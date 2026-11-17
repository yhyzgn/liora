//! One source-backed native rich editor; source-mode Input remains an integration surface.
mod controls;
mod input;
mod special;
#[cfg(test)]
mod tests;
mod text;

use super::rich_engine::{Command, Engine, Selection};
use super::{
    ImageResolver, MarkdownEditMode, MarkdownEditorChanged, MarkdownImage, MarkdownTableEdit,
    MarkdownTableEditError, assets,
    model::{Block, Kind},
    render, table_editor,
};
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement,
    ListAlignment, ListState, Render, SharedString, Subscription, Window, div, prelude::*, px,
};
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    rc::Rc,
};

/// Retained native Markdown editing surface with canonical UTF-8 source selection.
pub struct MarkdownEditor {
    engine: Engine,
    committed: SharedString,
    mode: MarkdownEditMode,
    input: Entity<crate::Input>,
    _source_subscription: Subscription,
    focus: FocusHandle,
    cursor_visible: bool,
    blink_task: Option<gpui::Task<()>>,
    marked: Option<Range<usize>>,
    composition: bool,
    dragging: bool,
    preferred_x: Option<gpui::Pixels>,
    pending_vertical: Option<(u64, bool, bool)>,
    pending_vertical_retry: Option<(bool, bool)>,
    retained_visible_row: Option<usize>,
    layouts: HashMap<u64, Rc<text::TextLayout>>,
    list: ListState,
    source_scroll: gpui::ScrollHandle,
    reveal_pending: bool,
    rows: Vec<(u64, SharedString)>,
    visible_rows: Vec<usize>,
    folded: HashSet<u64>,
    assets: assets::AssetCache,
    resolver: Option<ImageResolver>,
    images: render::ImageCache,
    footnotes: HashMap<usize, render::FootnoteInfo>,
    tables: Vec<(u64, Entity<table_editor::TableEditor>)>,
    error: Option<SharedString>,
    find_input: Entity<crate::Input>,
    replacement_input: Entity<crate::Input>,
    find_open: bool,
    slash_index: usize,
    slash_dismissed: bool,
    id: SharedString,
}

impl EventEmitter<MarkdownEditorChanged> for MarkdownEditor {}
impl Focusable for MarkdownEditor {
    fn focus_handle(&self, cx: &App) -> FocusHandle {
        if self.mode == MarkdownEditMode::Source {
            self.input.focus_handle(cx)
        } else {
            self.focus.clone()
        }
    }
}

impl MarkdownEditor {
    /// Creates a rich editor and its separate source-mode integration input.
    pub fn new(source: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let source = source.into();
        let input = cx.new(|cx| {
            crate::Input::new(source.clone(), cx)
                .min_rows(12)
                .soft_wrap(true)
        });
        let subscription = cx.observe(&input, |editor, input, cx| {
            if input.read(cx).is_composing() || editor.composition {
                return;
            }
            let value = input.read(cx).value();
            if value != editor.committed {
                let selection = input.read(cx).directed_selection();
                editor.engine.set_source(value.as_ref());
                editor.engine.set_selection(Selection {
                    anchor: selection.start,
                    head: selection.end,
                });
                editor.publish(cx);
            } else if editor.mode == MarkdownEditMode::Source {
                let range = input.read(cx).directed_selection();
                editor.engine.set_selection(Selection {
                    anchor: range.start,
                    head: range.end,
                });
            }
        });
        let mut editor = Self {
            engine: Engine::new(source.to_string()),
            committed: source,
            mode: MarkdownEditMode::Rich,
            input,
            _source_subscription: subscription,
            focus: cx.focus_handle(),
            cursor_visible: true,
            blink_task: None,
            marked: None,
            composition: false,
            dragging: false,
            preferred_x: None,
            pending_vertical: None,
            pending_vertical_retry: None,
            retained_visible_row: None,
            layouts: HashMap::new(),
            list: ListState::new(0, ListAlignment::Top, px(320.)),
            rows: Vec::new(),
            source_scroll: gpui::ScrollHandle::new(),
            reveal_pending: false,
            visible_rows: Vec::new(),
            folded: HashSet::new(),
            assets: Default::default(),
            resolver: None,
            images: HashMap::new(),
            tables: Vec::new(),
            error: None,
            footnotes: HashMap::new(),
            find_input: cx.new(|cx| crate::Input::new("", cx).placeholder("Find text")),
            replacement_input: cx.new(|cx| crate::Input::new("", cx).placeholder("Replace with")),
            find_open: false,
            slash_index: 0,
            slash_dismissed: false,
            id: liora_core::unique_id("markdown-editor"),
        };
        editor.reconcile_rows();
        editor
    }

    /// Last committed canonical Markdown; preedit text never leaks into persistence.
    pub fn source(&self) -> &str {
        self.committed.as_ref()
    }
    /// Source-mode integration only; rich text is owned by this editor's input handler.
    pub fn input(&self) -> Entity<crate::Input> {
        self.input.clone()
    }
    /// Returns the current rich or source presentation mode.
    pub fn mode(&self) -> MarkdownEditMode {
        self.mode
    }
    /// Changes mode unless native composition is active.
    pub fn set_mode(&mut self, mode: MarkdownEditMode, cx: &mut Context<Self>) {
        if !self.is_composing(cx) && mode != self.mode {
            self.pending_vertical = None;
            self.pending_vertical_retry = None;
            if mode == MarkdownEditMode::Source {
                let selection = self.engine.selection();
                self.input.update(cx, |input, cx| {
                    input.set_directed_selection(selection.anchor..selection.head, cx)
                });
            } else {
                let range = self.input.read(cx).directed_selection();
                self.engine.set_selection(Selection {
                    anchor: range.start,
                    head: range.end,
                });
                self.reveal();
            }
            self.mode = mode;
            self.layouts.clear();
            cx.notify();
        }
    }
    /// Replaces canonical source as one undoable edit unless composition is active.
    pub fn set_source(&mut self, source: impl Into<SharedString>, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        let source = source.into();
        if self.engine.set_source(source.as_ref()) {
            self.publish(cx);
        }
    }
    /// Installs the host's explicit image authorization and resolution policy.
    pub fn set_image_resolver(
        &mut self,
        resolver: impl Fn(&str) -> Option<MarkdownImage> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.resolver = Some(Rc::new(resolver));
        self.images.clear();
        self.list.remeasure();
        cx.notify();
    }
    /// Revokes the host image resolver and invalidates previously approved images.
    pub fn clear_image_resolver(&mut self, cx: &mut Context<Self>) {
        self.resolver = None;
        self.images.clear();
        self.list.remeasure();
        cx.notify();
    }
    /// Begins an explicit undo transaction.
    pub fn begin_undo_group(&mut self) {
        if !self.composition {
            self.engine.begin_undo_group();
        }
    }
    /// Ends the explicit undo transaction.
    pub fn end_undo_group(&mut self) {
        if !self.composition {
            self.engine.end_undo_group();
        }
    }
    /// Restores the previous committed source and selection.
    pub fn undo(&mut self, cx: &mut Context<Self>) -> bool {
        self.execute(Command::Undo, cx)
    }
    /// Reapplies the last undone source and selection.
    pub fn redo(&mut self, cx: &mut Context<Self>) -> bool {
        self.execute(Command::Redo, cx)
    }
    /// Returns the selected canonical UTF-8 source byte range.
    pub fn selection(&self) -> Range<usize> {
        self.engine.selection().range()
    }
    /// Selects canonical source bytes, clamping to UTF-8 boundaries.
    pub fn set_selection(&mut self, range: Range<usize>, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        self.pending_vertical = None;
        self.pending_vertical_retry = None;
        self.engine.set_selection(Selection {
            anchor: range.start,
            head: range.end,
        });
        self.sync_source_selection(cx);
        self.preferred_x = None;
        self.reveal();
        cx.notify();
    }
    /// Reveals and focuses a canonical source caret.
    pub fn focus_source_offset(
        &mut self,
        offset: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_composing(cx) {
            return;
        }
        self.set_selection(offset..offset, cx);
        if self.mode == MarkdownEditMode::Source {
            self.input
                .update(cx, |input, cx| input.set_selection(offset..offset, cx));
        }
        window.focus(&self.focus_handle(cx), cx);
    }
    /// Replaces the current selection with literal text, preserving Markdown styles.
    pub fn insert_text(&mut self, text: &str, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        self.replace(text, cx);
    }
    /// Official GPUI variable-height list state, also usable by host scrollbars.
    pub fn scroll_handle(&self) -> ListState {
        self.list.clone()
    }
    /// Reports active rich, source, or table-cell native composition.
    pub fn is_composing(&self, cx: &App) -> bool {
        self.composition
            || self.input.read(cx).is_composing()
            || self.find_input.read(cx).is_composing()
            || self.replacement_input.read(cx).is_composing()
            || self
                .tables
                .iter()
                .any(|(_, table)| table.read(cx).is_composing(cx))
    }
    /// Retains real native cell inputs; paragraphs never expose a per-block Input.
    pub fn table_input_at(
        &mut self,
        offset: usize,
        cx: &mut Context<Self>,
    ) -> Option<Entity<crate::Input>> {
        self.ensure_tables(cx);
        self.tables
            .iter()
            .find_map(|(_, table)| table.read(cx).input_at(offset))
    }
    /// Applies one source-backed table transaction, rejecting active composition.
    pub fn edit_table(
        &mut self,
        id: u64,
        edit: MarkdownTableEdit,
        cx: &mut Context<Self>,
    ) -> Result<(), MarkdownTableEditError> {
        if self.is_composing(cx) {
            return Err(MarkdownTableEditError::CompositionActive);
        }
        let scroll = self.list.logical_scroll_top();
        if self.engine.edit_table(id, edit)? {
            self.publish(cx);
            self.reveal_pending = false;
            self.list.scroll_to(scroll);
        }
        Ok(())
    }

    fn replace(&mut self, text: &str, cx: &mut Context<Self>) {
        match self.engine.replace(text) {
            Ok(changed) => {
                self.error = None;
                self.slash_dismissed = false;
                self.slash_index = 0;
                if changed {
                    self.changed(cx);
                } else {
                    cx.notify();
                }
            }
            Err(error) => {
                self.error = Some(error.into());
                cx.notify();
            }
        }
    }
    fn execute(&mut self, command: Command, cx: &mut Context<Self>) -> bool {
        if self.is_composing(cx) {
            return false;
        }
        match self.engine.command(command) {
            Ok(changed) => {
                self.error = None;
                if changed {
                    self.publish(cx);
                } else {
                    self.reveal();
                    cx.notify();
                }
                self.sync_source_selection(cx);
                changed
            }
            Err(error) => {
                self.error = Some(error.into());
                cx.notify();
                false
            }
        }
    }
    fn changed(&mut self, cx: &mut Context<Self>) {
        self.pending_vertical_retry = None;
        if self.composition {
            self.reconcile_rows();
            self.reveal();
            self.layouts.clear();
            cx.notify();
        } else {
            self.publish(cx);
        }
    }
    fn publish(&mut self, cx: &mut Context<Self>) {
        self.pending_vertical = None;
        self.pending_vertical_retry = None;
        self.reconcile_rows();
        let source: SharedString = self.engine.source().to_owned().into();
        let changed = source != self.committed;
        self.committed = source.clone();
        if self.input.read(cx).value() != source {
            self.input
                .update(cx, |input, cx| input.set_value(source.clone(), cx));
        }
        self.sync_source_selection(cx);
        self.reveal();
        self.layouts.clear();
        if changed {
            cx.emit(MarkdownEditorChanged { source });
        }
        cx.notify();
    }
    fn sync_source_selection(&mut self, cx: &mut Context<Self>) {
        let selection = self.engine.selection();
        let range = selection.anchor..selection.head;
        if self.mode == MarkdownEditMode::Source
            && self.input.read(cx).directed_selection() != range
        {
            self.input
                .update(cx, |input, cx| input.set_directed_selection(range, cx));
        }
    }
    fn reconcile_rows(&mut self) {
        let blocks = &self.engine.document().blocks;
        self.footnotes = render::footnote_info(blocks);
        self.folded
            .retain(|id| blocks.iter().any(|block| block.id == *id));
        let mut hidden_level = None;
        let mut visible: Vec<usize> = blocks
            .iter()
            .enumerate()
            .filter_map(|(index, block)| {
                if let Some(level) = hidden_level {
                    if matches!(block.kind, Kind::Heading(next) if next <= level) {
                        hidden_level = None;
                    } else {
                        return None;
                    }
                }
                if self.folded.contains(&block.id) {
                    if let Kind::Heading(level) = block.kind {
                        hidden_level = Some(level);
                    }
                }
                Some(index)
            })
            .collect();
        if visible.is_empty() {
            visible.push(usize::MAX);
        }
        let rows: Vec<_> = visible
            .iter()
            .enumerate()
            .map(|(row, &index)| {
                if index == usize::MAX {
                    return (0, SharedString::default());
                }
                let block = &blocks[index];
                let source = self
                    .engine
                    .source()
                    .get(block.source_range.clone())
                    .unwrap_or("");
                let source = self
                    .rows
                    .get(row)
                    .filter(|(id, previous)| *id == block.id && previous.as_ref() == source)
                    .map(|(_, previous)| previous.clone())
                    .unwrap_or_else(|| SharedString::from(source.to_owned()));
                (block.id, source)
            })
            .collect();
        let prefix = self
            .rows
            .iter()
            .zip(&rows)
            .take_while(|(a, b)| a == b)
            .count();
        let suffix = self.rows[prefix..]
            .iter()
            .rev()
            .zip(rows[prefix..].iter().rev())
            .take_while(|(a, b)| a == b)
            .count();
        self.retained_visible_row = None;
        if prefix + suffix < self.rows.len().max(rows.len()) {
            let scroll = self.list.logical_scroll_top();
            let replaced = prefix..self.rows.len() - suffix;
            if self.rows.len() == rows.len() {
                let viewport = self.list.viewport_bounds();
                self.retained_visible_row = visible
                    .iter()
                    .position(|&index| {
                        index != usize::MAX && {
                            let range = &blocks[index].source_range;
                            range.start <= self.engine.selection().head
                                && self.engine.selection().head <= range.end
                        }
                    })
                    .filter(|&row| {
                        self.list.bounds_for_item(row).is_some_and(|bounds| {
                            bounds.bottom() > viewport.top() && bounds.top() < viewport.bottom()
                        })
                    });
            }
            self.list.splice(
                prefix..self.rows.len() - suffix,
                rows.len() - prefix - suffix,
            );
            if replaced.contains(&scroll.item_ix) && self.rows.len() == rows.len() {
                self.list.scroll_to(scroll);
            }
        }
        self.rows = rows;
        self.visible_rows = visible;
    }
    fn reveal(&mut self) {
        self.cursor_visible = true;
        self.blink_task = None;
        self.reveal_pending = true;
        let offset = self.engine.selection().head;
        let Some(index) = self.visible_rows.iter().position(|&index| {
            if index == usize::MAX {
                return true;
            }
            let range = &self.engine.document().blocks[index].source_range;
            range.start <= offset && offset <= range.end
        }) else {
            return;
        };
        // Splicing invalidates measured heights, but a painted caret row is still visible.
        let viewport = self.list.viewport_bounds();
        let painted = self.engine.display_position(offset).is_some_and(|(id, _)| {
            self.layouts.get(&id).is_some_and(|layout| {
                layout.bounds.bottom() > viewport.top() && layout.bounds.top() < viewport.bottom()
            })
        });
        if self.list.logical_scroll_top().item_ix != index
            && self.retained_visible_row.take() != Some(index)
            && !painted
            && !self.list.bounds_for_item(index).is_some_and(|bounds| {
                let viewport = self.list.viewport_bounds();
                bounds.bottom() > viewport.top() && bounds.top() < viewport.bottom()
            })
        {
            self.list.scroll_to(gpui::ListOffset {
                item_ix: index,
                offset_in_item: px(0.),
            });
        }
    }
    fn ensure_tables(&mut self, cx: &mut Context<Self>) {
        let snapshots = self.engine.document().tables();
        let mut previous: HashMap<_, _> = std::mem::take(&mut self.tables).into_iter().collect();
        for snapshot in snapshots {
            let id = snapshot.id;
            let owner = cx.weak_entity();
            let callback: Rc<dyn Fn(MarkdownTableEdit, &mut App)> = Rc::new(move |edit, cx| {
                let _ = owner.update(cx, |editor, cx| {
                    if let Err(error) = editor.edit_table(id, edit, cx) {
                        editor.error = Some(format!("Table edit failed: {error:?}").into());
                        cx.notify();
                    }
                });
            });
            let table = if let Some(table) = previous.remove(&id) {
                table.update(cx, |table, cx| table.sync(snapshot, callback, cx));
                table
            } else {
                cx.new(|cx| table_editor::TableEditor::new(snapshot, callback, cx))
            };
            self.tables.push((id, table));
        }
    }
    fn render_row(
        &mut self,
        row: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(&index) = self.visible_rows.get(row) else {
            return div().into_any_element();
        };
        if index == usize::MAX {
            return text::TextElement::empty(cx.entity()).into_any_element();
        }
        let block = &self.engine.document().blocks[index];
        let id = block.id;
        let folded = self.folded.contains(&id);
        let owner = cx.entity();
        let tables = self.tables.clone();
        let editable = |block: &Block| {
            if let Some((_, table)) = tables.iter().find(|(id, _)| *id == block.id) {
                return Some(
                    div()
                        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| cx.stop_propagation())
                        .child(table.clone())
                        .into_any_element(),
                );
            }
            if folded && block.id == id && !matches!(block.kind, Kind::Heading(_)) {
                return Some(div().child("Folded block").into_any_element());
            }
            if text::is_native(block) {
                return Some(text::TextElement::new(block, owner.clone()).into_any_element());
            }
            None
        };
        let decorate = |block: &Block, rendered: AnyElement| {
            if matches!(
                block.kind,
                Kind::Paragraph
                    | Kind::Heading(_)
                    | Kind::DefinitionTitle
                    | Kind::DefinitionDefinition
            ) && text::is_special_text(block)
            {
                special::SpecialElement::new(block, owner.clone()).into_any_element()
            } else {
                rendered
            }
        };
        let weak = cx.weak_entity();
        let activate: render::EditActivation = Rc::new(move |id, offset, window, cx| {
            let _ = weak.update(cx, |editor, cx| {
                if let Some(offset) = editor.engine.source_position(id, offset) {
                    editor.focus_source_offset(offset, window, cx);
                }
            });
        });
        let weak = cx.weak_entity();
        let tasks: render::TaskToggleHandler = Rc::new(move |id, _, _, cx| {
            let _ = weak.update(cx, |editor, cx| {
                editor.execute(Command::ToggleTask(id), cx);
            });
        });
        let weak = cx.weak_entity();
        let navigate: render::NavigationHandler = Rc::new(move |fragment, cx| {
            let _ = weak.update(cx, |editor, cx| {
                if let Some(offset) =
                    find_fragment(&editor.engine.document().blocks, fragment.as_ref())
                {
                    editor.set_selection(offset..offset, cx);
                }
            });
        });
        let content = render::document_row_with_editable(
            &self.engine.document().blocks,
            index,
            &self.footnotes,
            self.resolver.as_ref(),
            &mut self.images,
            Some(&tasks),
            Some(&navigate),
            &editable,
            &decorate,
            activate,
            &self.assets,
            window,
            cx,
        );
        div()
            .w_full()
            .min_w_0()
            .pb_3()
            .child(content)
            .into_any_element()
    }
}

fn find_fragment(blocks: &[Block], fragment: &str) -> Option<usize> {
    for block in blocks {
        if block.heading_id.as_deref() == Some(fragment)
            || block
                .footnote_number
                .is_some_and(|n| fragment == format!("_footnote-{n}"))
        {
            return Some(block.source_range.start);
        }
        if let Some(offset) = find_fragment(&block.children, fragment) {
            return Some(offset);
        }
    }
    None
}

impl Render for MarkdownEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let focused = self.mode == MarkdownEditMode::Rich && self.focus.is_focused(window);
        if focused && self.blink_task.is_none() {
            self.cursor_visible = true;
            let executor = cx.background_executor().clone();
            self.blink_task = Some(cx.spawn(async move |this, cx| {
                loop {
                    executor.timer(std::time::Duration::from_millis(500)).await;
                    if this
                        .update(cx, |editor, cx| {
                            editor.cursor_visible = !editor.cursor_visible;
                            cx.notify();
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }));
        } else if !focused {
            self.blink_task = None;
        }
        self.ensure_tables(cx);
        let content = if self.mode == MarkdownEditMode::Source {
            let viewport = div()
                .id(SharedString::from(format!("{}-source", self.id)))
                .w_full()
                .flex_1()
                .min_h_0()
                .overflow_y_scroll()
                .track_scroll(&self.source_scroll)
                .child(self.input.clone())
                .into_any_element();
            input::SourceViewport {
                child: viewport,
                editor: cx.entity(),
            }
            .into_any_element()
        } else {
            let weak = cx.weak_entity();
            let list = gpui::list(self.list.clone(), move |row, window, cx| {
                weak.update(cx, |editor, cx| editor.render_row(row, window, cx))
                    .unwrap_or_else(|_| div().into_any_element())
            })
            .w_full()
            .h_full();
            div()
                .id(SharedString::from(format!("{}-native", self.id)))
                .relative()
                .w_full()
                .h_full()
                .min_h_0()
                .flex_1()
                .track_focus(&self.focus)
                .tab_index(0)
                .key_context("MarkdownEditor")
                .aria_label("Markdown document editor")
                .cursor_text()
                .on_key_down(cx.listener(Self::key_down))
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(Self::mouse_down))
                .on_mouse_move(cx.listener(Self::mouse_move))
                .on_mouse_up(
                    gpui::MouseButton::Left,
                    cx.listener(|editor, _, _, _| editor.dragging = false),
                )
                .child(input::InputLayer {
                    editor: cx.entity(),
                })
                .child(list)
                .into_any_element()
        };
        let find = self.find_open.then(|| self.find_controls(cx));
        let slash = self.slash_controls(cx);
        let _ = window;
        div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .h_full()
            .min_h_0()
            .on_key_down(cx.listener(Self::history_key_down))
            .children(find)
            .children(slash)
            .child(content)
            .children(self.error.clone().map(|error| crate::Text::new(error).sm()))
    }
}
