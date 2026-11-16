//! Streaming Markdown rendered as native GPUI selectable text, code, tables and controls.
//! No HTML execution, implicit image fetching, or application-specific demo directives.

mod assets;
mod diagram;
mod editing;
mod math;
mod model;
mod render;
mod table_editor;

use gpui::{
    AppContext, Context, Entity, EventEmitter, Focusable, InteractiveElement, IntoElement,
    ParentElement, Render, ScrollHandle, SharedString, StatefulInteractiveElement, Styled, Window,
};
use model::{Block, Kind};
pub use model::{
    MarkdownDocument, MarkdownFootnote, MarkdownFootnoteReference, MarkdownHeading,
    MarkdownMetadata, MarkdownMetadataDiagnostic, MarkdownMetadataField, MarkdownTable,
    MarkdownTableAlignment, MarkdownTableCell, MarkdownTableEdit, MarkdownTableEditError,
    MarkdownTableRow, MetadataBlockKind, is_safe_markdown_link,
};
use std::{collections::HashMap, ops::Range, path::PathBuf, rc::Rc, sync::Arc};

/// Host-approved image content. Remote images must be fetched and decoded by the
/// host, which owns redirect, network permission, size and decoding limits.
#[derive(Clone)]
pub enum MarkdownImage {
    /// Explicitly approved local path, never interpreted as a network URL.
    File(PathBuf),
    /// Already decoded content, including images fetched by the host network layer.
    Render(Arc<gpui::RenderImage>),
}

type ImageResolver = Rc<dyn Fn(&str) -> Option<MarkdownImage>>;

/// Content or image policy changed; hosts can invalidate measured item height.
/// Subscribe and remeasure after layout (for example with Transcript::notify_item_changed).
#[derive(Clone, Copy, Debug)]
pub struct MarkdownChanged {
    /// Completed parse generation. Policy-only changes retain the same generation.
    pub generation: u64,
}
/// Host-owned task-list source mutation request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownTaskToggled {
    /// Stable block identity for the current parsed document.
    pub block_id: u64,
    /// Source range of the Markdown task marker.
    pub source_range: Range<usize>,
    /// Requested checked state.
    pub checked: bool,
}

/// Document-local fragment activation request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownNavigationRequested {
    /// Heading ID without the leading `#`.
    pub fragment: SharedString,
}

impl gpui::EventEmitter<MarkdownTaskToggled> for MarkdownView {}
impl gpui::EventEmitter<MarkdownNavigationRequested> for MarkdownView {}

impl gpui::EventEmitter<MarkdownChanged> for MarkdownView {}

/// Retain as `Entity<MarkdownView>` so document IDs, selection and parse cache survive renders.
pub struct MarkdownView {
    document: MarkdownDocument,
    plain_text: SharedString,
    image_resolver: Option<ImageResolver>,
    resolved_images: HashMap<SharedString, Option<MarkdownImage>>,
    selection_id: SharedString,
    continuous_selection: bool,
    show_toc: bool,
    navigation_anchor: Option<Entity<crate::Anchor>>,
    assets: assets::AssetCache,
}

impl MarkdownView {
    /// Construct inside `cx.new(|cx| MarkdownView::new(source, cx))`.
    pub fn new(source: impl Into<String>, _cx: &mut Context<Self>) -> Self {
        Self::from_document(MarkdownDocument::new(source))
    }

    /// Uses an existing parsed document without parsing again.
    pub fn from_document(document: MarkdownDocument) -> Self {
        Self {
            plain_text: document.plain_text().into(),
            document,
            continuous_selection: false,
            show_toc: false,
            navigation_anchor: None,
            assets: assets::AssetCache::default(),
            selection_id: liora_core::unique_id("markdown-selection"),
            image_resolver: None,
            resolved_images: HashMap::new(),
        }
    }

    /// Last parsed document; source includes pending streaming updates.
    pub fn document(&self) -> &MarkdownDocument {
        &self.document
    }
    /// Opts into a native contents list without changing existing document layout by default.
    pub fn set_show_toc(&mut self, enabled: bool, cx: &mut Context<Self>) {
        if self.show_toc != enabled {
            self.show_toc = enabled;
            cx.notify();
        }
    }
    /// Enables measured heading and footnote scrolling within a host-tracked viewport.
    pub fn set_navigation_scroll(&mut self, handle: ScrollHandle, cx: &mut Context<Self>) {
        self.navigation_anchor = Some(cx.new(|_| crate::Anchor::new(handle)));
        cx.notify();
    }

    /// Switches to one continuous native text range for cross-block selection/copy.
    /// Rich layout mode keeps each block's typography while sharing one selection scope.
    pub fn set_continuous_selection(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.continuous_selection = enabled;
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }

    /// Whether the flattened continuous text projection is displayed.
    pub fn continuous_selection(&self) -> bool {
        self.continuous_selection
    }

    /// Replace source. Parsing is deferred until the next render.
    pub fn set_source(&mut self, source: impl Into<String>, cx: &mut Context<Self>) {
        self.document.set_source(source);
        // Raster keys include source and theme; prose edits must not rerender unchanged assets.
        cx.notify();
    }

    /// Append without parsing. GPUI coalesces notifications; one render parses the batch once.
    pub fn append(&mut self, chunk: &str, cx: &mut Context<Self>) {
        self.document.append(chunk);
        cx.notify();
    }

    /// Explicitly grants image-loading policy to the host. The input is an untrusted
    /// Markdown destination; return an approved local file or decoded image, or None.
    /// Called once per distinct destination until the policy changes. Default: deny all.
    /// Do not blindly return the input for model-generated Markdown.
    pub fn set_image_resolver(
        &mut self,
        resolver: impl Fn(&str) -> Option<MarkdownImage> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.image_resolver = Some(Rc::new(resolver));
        self.resolved_images.clear();
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }

    /// Revokes image access and forgets previously approved destinations.
    pub fn clear_image_resolver(&mut self, cx: &mut Context<Self>) {
        self.image_resolver = None;
        self.resolved_images.clear();
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }
}

impl Render for MarkdownView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.document.flush() {
            self.plain_text = self.document.plain_text().into();
            cx.emit(MarkdownChanged {
                generation: self.document.parse_count(),
            });
            // Bound the policy cache to destinations still present in the parsed document.
            let mut destinations = std::collections::HashSet::new();
            render::image_destinations(&self.document.blocks, &mut destinations);
            self.resolved_images
                .retain(|destination, _| destinations.contains(destination));
        }
        let task_target = cx.weak_entity();
        let task_toggle: render::TaskToggleHandler =
            Rc::new(move |block_id, source_range, checked, cx| {
                let _ = task_target.update(cx, |_, cx| {
                    cx.emit(MarkdownTaskToggled {
                        block_id,
                        source_range,
                        checked,
                    });
                });
            });
        let navigation_target = cx.weak_entity();
        let anchor = self.navigation_anchor.clone();
        let navigation: render::NavigationHandler = Rc::new(move |fragment, cx| {
            if let Some(anchor) = &anchor {
                let _ = anchor.update(cx, |anchor, _| anchor.scroll_to_target(&fragment));
            }
            let _ = navigation_target.update(cx, |_, cx| {
                cx.emit(MarkdownNavigationRequested { fragment });
            });
        });
        let weak = cx.weak_entity();
        let toggle = gpui::div()
            .id(SharedString::from(format!(
                "{}-toggle-focus",
                self.selection_id
            )))
            .focusable()
            .tab_index(0)
            .on_key_down(cx.listener(|view, event: &gpui::KeyDownEvent, _, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    view.set_continuous_selection(!view.continuous_selection, cx);
                    cx.stop_propagation();
                }
            }))
            .child(
                crate::Button::new(if self.continuous_selection {
                    "Show rich document"
                } else {
                    "Select across blocks"
                })
                .id(SharedString::from(format!("{}-toggle", self.selection_id)))
                .text()
                .small()
                .on_click(move |_, _, cx| {
                    let _ = weak.update(cx, |view, cx| {
                        view.set_continuous_selection(!view.continuous_selection, cx)
                    });
                }),
            );
        let mut toc = gpui::div().flex().flex_col().gap_1().min_w_0();
        if self.show_toc && !self.continuous_selection {
            let headings = self.document.headings();
            if !headings.is_empty() {
                toc = toc.child(crate::Text::new("Contents"));
            }
            for heading in headings {
                let fragment = heading.id.clone();
                let keyboard_fragment = fragment.clone();
                let click_navigation = navigation.clone();
                let keyboard_navigation = navigation.clone();
                toc = toc.child(
                    gpui::div()
                        .id(SharedString::from(format!(
                            "{}-toc-{}-focus",
                            self.selection_id, heading.id
                        )))
                        .focusable()
                        .tab_index(0)
                        .on_key_down(move |event: &gpui::KeyDownEvent, _, cx| {
                            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                                keyboard_navigation(keyboard_fragment.clone(), cx);
                                cx.stop_propagation();
                            }
                        })
                        .child(
                            crate::Button::new(heading.title)
                                .id(SharedString::from(format!(
                                    "{}-toc-{}",
                                    self.selection_id, heading.id
                                )))
                                .text()
                                .small()
                                .on_click(move |_, _, cx| {
                                    click_navigation(fragment.clone(), cx);
                                }),
                        ),
                );
            }
        }
        let content = if self.continuous_selection {
            let mut options = crate::SelectableTextOptions::new(
                self.selection_id.clone(),
                self.plain_text.clone(),
            );
            options.text_color = cx.global::<liora_core::Config>().theme.neutral.text_1;
            options.font_family = liora_core::ui_font_family(cx);
            crate::SelectableText::view(options, window, cx)
        } else {
            render::document(
                &self.document.blocks,
                self.selection_id.clone(),
                self.image_resolver.as_ref(),
                &mut self.resolved_images,
                Some(&task_toggle),
                Some(&navigation),
                self.navigation_anchor.as_ref(),
                &self.assets,
                window,
                cx,
            )
        };
        gpui::div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .min_w_0()
            .child(toggle)
            .child(toc)
            .child(content)
    }
}

/// Presentation mode for [`MarkdownEditor`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkdownEditMode {
    /// Native multiline editing of the canonical Markdown source.
    Source,
    /// Native rendered projection of the canonical source.
    #[default]
    Rich,
}

/// Change notification emitted after the source editor changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownEditorChanged {
    /// Current canonical Markdown source.
    pub source: SharedString,
}

/// Native Markdown editor shell with one durable source and a live rich editing projection.
///
/// Inactive blocks retain rendered typography, inline styles, math and diagrams. Click a
/// paragraph or heading (or focus it and press Enter) to activate its native text input.
/// The active input edits visible text, not Markdown source; inline styling is restored
/// when focus leaves the block. Inputs retain native IME, selection and clipboard,
/// and soft-wrap at the document width. Edits preserve inline markers only when the
/// source mapping is unambiguous. Tables have literal-text cell editing; use Source mode for other structure.
pub struct MarkdownEditor {
    source: SharedString,
    mode: MarkdownEditMode,
    input: Entity<crate::Input>,
    preview: Entity<MarkdownView>,
    block_inputs: HashMap<u64, Entity<crate::Input>>,
    block_input_subscriptions: HashMap<u64, gpui::Subscription>,
    rich_focus_subscription: Option<gpui::Subscription>,
    table_editors: Vec<(u64, Entity<table_editor::TableEditor>)>,
    _input_subscription: gpui::Subscription,
    undo_stack: Vec<SharedString>,
    redo_stack: Vec<SharedString>,
    undo_group_open: bool,
    undo_group_recorded: bool,
    edit_error: Option<SharedString>,
    active_block: Option<u64>,
    id: SharedString,
}

impl EventEmitter<MarkdownEditorChanged> for MarkdownEditor {}

impl MarkdownEditor {
    /// Creates an editor with Rich mode selected.
    pub fn new(source: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        let source = source.into();
        let preview = cx.new(|cx| MarkdownView::new(source.clone(), cx));
        let input = cx.new(|cx| crate::Input::new(source.clone(), cx).min_rows(12));
        let input_subscription = cx.observe(&input, |editor, input, cx| {
            if input.read(cx).is_composing() {
                return;
            }
            let source = input.read(cx).value();
            editor.set_source_from_input(source, cx);
        });
        Self {
            source,
            mode: MarkdownEditMode::Rich,
            input,
            preview,
            block_inputs: HashMap::new(),
            block_input_subscriptions: HashMap::new(),
            rich_focus_subscription: None,
            _input_subscription: input_subscription,
            table_editors: Vec::new(),
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            undo_group_open: false,
            undo_group_recorded: false,
            edit_error: None,
            active_block: None,
            id: liora_core::unique_id("markdown-editor"),
        }
    }

    /// Returns the canonical Markdown source.
    pub fn source(&self) -> &str {
        self.source.as_ref()
    }

    /// Uses the same host-approved image policy in the rich projection as [`MarkdownView`].
    pub fn set_image_resolver(
        &mut self,
        resolver: impl Fn(&str) -> Option<MarkdownImage> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.preview
            .update(cx, |preview, cx| preview.set_image_resolver(resolver, cx));
        cx.notify();
    }

    /// Replaces the canonical source and updates both presentation modes.
    pub fn set_source(&mut self, source: impl Into<SharedString>, cx: &mut Context<Self>) {
        let source = source.into();
        if self.source == source {
            return;
        }
        self.record_undo();
        self.apply_source(source, cx);
    }

    fn record_undo(&mut self) {
        if !self.undo_group_open || !self.undo_group_recorded {
            if self.undo_stack.len() == 128 {
                self.undo_stack.remove(0);
            }
            self.undo_stack.push(self.source.clone());
        }
        self.undo_group_recorded = true;
        self.redo_stack.clear();
    }

    fn apply_source(&mut self, source: SharedString, cx: &mut Context<Self>) {
        self.edit_error = None;
        self.source = source.clone();
        cx.update_entity(&self.input, |input, cx| input.set_value(source.clone(), cx));
        cx.update_entity(&self.preview, |preview, cx| {
            preview.set_source(source.clone(), cx);
            preview.document.flush();
        });
        self.ensure_rich_inputs(cx);
        self.ensure_table_editors(cx);
        cx.emit(MarkdownEditorChanged { source });
        cx.notify();
    }

    fn set_source_from_input(&mut self, source: SharedString, cx: &mut Context<Self>) {
        if self.source == source {
            return;
        }
        self.record_undo();
        self.source = source.clone();
        cx.update_entity(&self.preview, |preview, cx| {
            preview.set_source(source.clone(), cx)
        });
        cx.emit(MarkdownEditorChanged { source });
        cx.notify();
    }

    /// Groups subsequent source updates into one undo transaction until `end_undo_group`.
    pub fn begin_undo_group(&mut self) {
        self.undo_group_open = true;
        self.undo_group_recorded = false;
    }

    /// Ends the current explicit undo transaction.
    pub fn end_undo_group(&mut self) {
        self.undo_group_open = false;
        self.undo_group_recorded = false;
    }

    /// Restores the previous canonical source revision.
    pub fn undo(&mut self, cx: &mut Context<Self>) -> bool {
        if self.is_composing(cx) {
            return false;
        }
        let Some(previous) = self.undo_stack.pop() else {
            return false;
        };
        self.undo_group_open = false;
        self.undo_group_recorded = false;
        self.redo_stack.push(self.source.clone());
        self.apply_source(previous, cx);
        true
    }

    /// Reapplies the last undone canonical source revision.
    pub fn redo(&mut self, cx: &mut Context<Self>) -> bool {
        if self.is_composing(cx) {
            return false;
        }
        let Some(next) = self.redo_stack.pop() else {
            return false;
        };
        self.undo_group_open = false;
        self.undo_group_recorded = false;
        self.undo_stack.push(self.source.clone());
        self.apply_source(next, cx);
        true
    }

    fn ensure_rich_inputs(&mut self, cx: &mut Context<Self>) {
        let blocks = cx.read_entity(&self.preview, |preview, _| {
            let mut editable = Vec::new();
            Self::collect_editable_blocks(&preview.document.blocks, &mut editable);
            editable
        });
        let active = blocks
            .iter()
            .map(|(id, _)| (*id, ()))
            .collect::<HashMap<_, _>>();
        self.block_inputs.retain(|id, _| active.contains_key(id));
        self.block_input_subscriptions
            .retain(|id, _| active.contains_key(id));
        for (id, value) in blocks {
            if let Some(input) = self.block_inputs.get(&id).cloned() {
                if !input.read(cx).is_composing() && input.read(cx).value() != value {
                    cx.update_entity(&input, |input, cx| input.set_value(value.clone(), cx));
                }
                continue;
            }
            let input = cx.new(|cx| {
                crate::Input::new(value.clone(), cx)
                    .id(format!("markdown-rich-{id}"))
                    .min_rows(1)
                    .soft_wrap(true)
            });
            let subscription = cx.observe(&input, move |editor, input, cx| {
                cx.notify();
                if input.read(cx).is_composing() {
                    return;
                }
                let value = input.read(cx).value();
                editor.update_block(id, value, cx);
            });
            self.block_inputs.insert(id, input);
            self.block_input_subscriptions.insert(id, subscription);
        }
    }

    fn is_editable_block(block: &Block) -> bool {
        matches!(block.kind, Kind::Paragraph | Kind::Heading(_)) && block.images.is_empty()
    }
    fn collect_editable_blocks(blocks: &[Block], output: &mut Vec<(u64, SharedString)>) {
        for block in blocks {
            if Self::is_editable_block(block) {
                output.push((block.id, block.text.clone()));
            }
            Self::collect_editable_blocks(&block.children, output);
        }
    }

    fn update_block(&mut self, id: u64, value: SharedString, cx: &mut Context<Self>) {
        let block = cx.update_entity(&self.preview, |preview, _| {
            preview.document.flush();
            find_block(&preview.document.blocks, id).map(|block| {
                (
                    block.source_range.clone(),
                    block.text.clone(),
                    block
                        .spans
                        .iter()
                        .map(|span| (span.range.clone(), span.source_range.clone()))
                        .collect::<Vec<_>>(),
                )
            })
        });
        let Some((source_range, old_text, spans)) = block else {
            return;
        };
        if source_range.start > source_range.end || source_range.end > self.source.len() {
            return;
        }
        let mut source = self.source.to_string();
        if !replace_display_text(
            &mut source,
            source_range,
            old_text.as_ref(),
            &spans,
            value.as_ref(),
        ) {
            if let Some(input) = self.block_inputs.get(&id).cloned() {
                cx.update_entity(&input, |input, cx| input.set_value(old_text, cx));
            }
            self.edit_error = Some("This edit crosses Markdown syntax. Use Edit Markdown source to preserve formatting.".into());
            cx.notify();
            return;
        }
        self.set_source(source, cx);
    }

    fn ensure_table_editors(&mut self, cx: &mut Context<Self>) {
        let tables = cx.read_entity(&self.preview, |preview, _| preview.document.tables());
        let mut previous = std::mem::take(&mut self.table_editors).into_iter();
        for table in tables {
            let id = table.id;
            let owner = cx.weak_entity();
            let callback: Rc<dyn Fn(MarkdownTableEdit, &mut gpui::App)> =
                Rc::new(move |edit, cx| {
                    let _ = owner.update(cx, |editor, cx| {
                        if editor.is_composing(cx) {
                            return;
                        }
                        if let Err(error) = editor.edit_table(id, edit, cx) {
                            editor.edit_error =
                                Some(format!("Table edit failed: {error:?}").into());
                            cx.notify();
                        }
                    });
                });
            let entity = if let Some((_, entity)) = previous.next() {
                entity.update(cx, |editor, cx| editor.sync(table, callback, cx));
                entity
            } else {
                cx.new(|cx| table_editor::TableEditor::new(table, callback, cx))
            };
            self.table_editors.push((id, entity));
        }
    }

    fn rich_content(&mut self, window: &mut Window, cx: &mut Context<Self>) -> gpui::AnyElement {
        cx.update_entity(&self.preview, |preview, _| preview.document.flush());
        self.ensure_rich_inputs(cx);
        self.ensure_table_editors(cx);
        let active_block = self
            .block_inputs
            .iter()
            .find_map(|(&id, input)| input.focus_handle(cx).is_focused(window).then_some(id));
        self.rich_focus_subscription = active_block.map(|id| {
            let focus = self.block_inputs[&id].focus_handle(cx);
            cx.on_focus_out(&focus, window, |_, _, _, cx| cx.notify())
        });
        if let Some(id) = active_block {
            self.active_block = Some(id);
        }
        let owner = cx.weak_entity();
        let editor_id = self.id.clone();
        let inputs = self.block_inputs.clone();
        let tables = self.table_editors.clone();
        let activation_owner = owner.clone();
        let activate: render::EditActivation = Rc::new(move |id, offset, window, cx| {
            let _ = activation_owner.update(cx, |editor, cx| {
                let Some(input) = editor.block_inputs.get(&id).cloned() else {
                    return;
                };
                input.update(cx, |input, cx| input.set_selection(offset..offset, cx));
                editor.active_block = Some(id);
                window.focus(&input.focus_handle(cx), cx);
                cx.notify();
            });
        });
        cx.update_entity(&self.preview, |preview, preview_cx| {
            let editable = |block: &Block| {
                if let Some((_, table)) = tables.iter().find(|(id, _)| *id == block.id) {
                    return Some(table.clone().into_any_element());
                }
                if Some(block.id) != active_block {
                    return None;
                }
                inputs
                    .get(&block.id)
                    .cloned()
                    .map(|input| input.into_any_element())
            };
            let decorate = |block: &Block, rendered: gpui::AnyElement| {
                let Some(input) = inputs.get(&block.id).cloned() else {
                    return rendered;
                };
                let id = block.id;
                let keyboard_owner = owner.clone();
                let keyboard_input = input.clone();
                gpui::div()
                    .id(SharedString::from(format!("{editor_id}-edit-{id}")))
                    .w_full()
                    .min_w_0()
                    .focusable()
                    .tab_index(0)
                    .aria_label("Markdown block; press Enter to edit")
                    .cursor_text()
                    .on_key_down(move |event, window, cx| {
                        if event.keystroke.key == "enter" {
                            let _ = keyboard_owner.update(cx, |editor, cx| {
                                editor.active_block = Some(id);
                                window.focus(&keyboard_input.focus_handle(cx), cx);
                                cx.notify();
                            });
                            cx.stop_propagation();
                        }
                    })
                    .child(rendered)
                    .into_any_element()
            };
            render::document_with_editable(
                &preview.document.blocks,
                preview.image_resolver.as_ref(),
                &mut preview.resolved_images,
                None,
                None,
                &editable,
                &decorate,
                activate,
                &preview.assets,
                window,
                preview_cx,
            )
        })
    }

    /// Returns the active presentation mode.
    pub fn mode(&self) -> MarkdownEditMode {
        self.mode
    }

    /// Switches between native source editing and the editable rich projection.
    pub fn set_mode(&mut self, mode: MarkdownEditMode, cx: &mut Context<Self>) {
        if self.is_composing(cx) {
            return;
        }
        if self.mode != mode {
            self.mode = mode;
            cx.notify();
        }
    }

    /// Applies a structural table edit to the canonical source and updates both modes.
    pub fn edit_table(
        &mut self,
        table_id: u64,
        edit: MarkdownTableEdit,
        cx: &mut Context<Self>,
    ) -> Result<(), MarkdownTableEditError> {
        if self.is_composing(cx) {
            return Err(MarkdownTableEditError::CompositionActive);
        }
        let source = cx.update_entity(&self.preview, |preview, cx| {
            preview.document.flush();
            preview.document.edit_table(table_id, edit)?;
            preview.plain_text = preview.document.plain_text().into();
            cx.emit(MarkdownChanged {
                generation: preview.document.parse_count(),
            });
            cx.notify();
            Ok::<_, MarkdownTableEditError>(preview.document.source().to_owned())
        })?;
        self.set_source(source, cx);
        Ok(())
    }

    /// Returns the underlying native source input for focus/selection integration.
    pub fn input(&self) -> Entity<crate::Input> {
        self.input.clone()
    }

    /// Returns a paragraph, heading or table-cell Rich input at a canonical UTF-8
    /// source offset. Table-cell edits replace formatting with literal plain text.
    /// Unsupported content returns `None`.
    pub fn rich_input_at(
        &mut self,
        source_offset: usize,
        cx: &mut Context<Self>,
    ) -> Option<Entity<crate::Input>> {
        cx.update_entity(&self.preview, |preview, _| preview.document.flush());
        self.ensure_rich_inputs(cx);
        self.ensure_table_editors(cx);
        if let Some(input) = self
            .table_editors
            .iter()
            .find_map(|(_, table)| table.read(cx).input_at(source_offset))
        {
            return Some(input);
        }
        let id = cx.read_entity(&self.preview, |preview, _| {
            fn find(blocks: &[Block], offset: usize) -> Option<u64> {
                for block in blocks {
                    if MarkdownEditor::is_editable_block(block)
                        && block.source_range.contains(&offset)
                    {
                        return Some(block.id);
                    }
                    if let Some(id) = find(&block.children, offset) {
                        return Some(id);
                    }
                }
                None
            }
            find(&preview.document.blocks, source_offset)
        })?;
        self.block_inputs.get(&id).cloned()
    }

    fn is_composing(&self, cx: &gpui::App) -> bool {
        self.input.read(cx).is_composing()
            || self
                .block_inputs
                .values()
                .any(|input| input.read(cx).is_composing())
            || self
                .table_editors
                .iter()
                .any(|(_, table)| table.read(cx).is_composing(cx))
    }
}

fn replace_display_text(
    source: &mut String,
    source_range: std::ops::Range<usize>,
    old_text: &str,
    spans: &[(std::ops::Range<usize>, std::ops::Range<usize>)],
    new_text: &str,
) -> bool {
    let prefix = common_prefix_bytes(old_text, new_text);
    let suffix = common_suffix_bytes(old_text, new_text, prefix);
    let old_end = old_text.len() - suffix;
    let new_end = new_text.len() - suffix;
    if prefix == old_end && prefix == new_end {
        return true;
    }
    if spans.is_empty() && old_text.is_empty() {
        if source_range.start <= source.len() && source.is_char_boundary(source_range.start) {
            source.insert_str(source_range.start, new_text);
            return true;
        }
        return false;
    }
    if spans.iter().any(|(display, source_span)| {
        display.end > old_text.len()
            || source_span.start < source_range.start
            || source_span.end > source_range.end
            || !old_text.is_char_boundary(display.start)
            || !old_text.is_char_boundary(display.end)
            || !source.is_char_boundary(source_span.start)
            || !source.is_char_boundary(source_span.end)
    }) {
        return false;
    }
    let start = map_display_offset(source, old_text, prefix, spans);
    let end = map_display_offset(source, old_text, old_end, spans);
    if let (Some(start), Some(end)) = (start, end) {
        if start <= end
            && source.is_char_boundary(start)
            && source.is_char_boundary(end)
            && (start == end || source[start..end] == old_text[prefix..old_end])
        {
            source.replace_range(start..end, &new_text[prefix..new_end]);
            return true;
        }
    }
    false
}

fn common_prefix_bytes(left: &str, right: &str) -> usize {
    let mut left_chars = left.char_indices();
    let mut right_chars = right.char_indices();
    let mut length = 0;
    while let (Some((_, left)), Some((_, right))) = (left_chars.next(), right_chars.next()) {
        if left != right {
            break;
        }
        length += left.len_utf8();
    }
    length
}

fn common_suffix_bytes(left: &str, right: &str, prefix: usize) -> usize {
    let mut left_chars = left[prefix..].char_indices().rev();
    let mut right_chars = right[prefix.min(right.len())..].char_indices().rev();
    let mut length = 0;
    while let (Some((_, left)), Some((_, right))) = (left_chars.next(), right_chars.next()) {
        if left != right {
            break;
        }
        length += left.len_utf8();
    }
    length.min(left.len().saturating_sub(prefix))
}

fn map_display_offset(
    source: &str,
    text: &str,
    offset: usize,
    spans: &[(std::ops::Range<usize>, std::ops::Range<usize>)],
) -> Option<usize> {
    for (display, source_span) in spans.iter().rev() {
        if display.start <= offset && offset <= display.end {
            let visible = &text[display.clone()];
            let original = &source[source_span.clone()];
            if source.as_bytes().get(source_span.start.wrapping_sub(1)) == Some(&b'\\')
                || (original.contains('\\') && original != visible)
            {
                return None;
            }
            let start = original.find(visible)?;
            if original[start + visible.len()..].contains(visible) {
                return None;
            }
            return Some(source_span.start + start + offset - display.start);
        }
    }
    None
}

fn find_block(blocks: &[Block], id: u64) -> Option<&Block> {
    for block in blocks {
        if block.id == id {
            return Some(block);
        }
        if let Some(found) = find_block(&block.children, id) {
            return Some(found);
        }
    }
    None
}

impl Render for MarkdownEditor {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let weak = cx.weak_entity();
        let next_mode = match self.mode {
            MarkdownEditMode::Source => MarkdownEditMode::Rich,
            MarkdownEditMode::Rich => MarkdownEditMode::Source,
        };
        let toggle = crate::Button::new(match self.mode {
            MarkdownEditMode::Source => "Show rich document",
            MarkdownEditMode::Rich => "Edit Markdown source",
        })
        .text()
        .small()
        .on_click(move |_, _, cx| {
            let _ = weak.update(cx, |editor, cx| editor.set_mode(next_mode, cx));
        });
        let content = match self.mode {
            MarkdownEditMode::Source => self.input.clone().into_any_element(),
            MarkdownEditMode::Rich => self.rich_content(window, cx),
        };
        let toolbar =
            (self.mode == MarkdownEditMode::Rich).then(|| self.editing_controls(window, cx));
        gpui::div()
            .id(self.id.clone())
            .on_key_down(cx.listener(|editor, event: &gpui::KeyDownEvent, _, cx| {
                let keys = &event.keystroke;
                if keys.key.eq_ignore_ascii_case("z")
                    && (keys.modifiers.control || keys.modifiers.platform)
                    && !keys.modifiers.alt
                {
                    if if keys.modifiers.shift {
                        editor.redo(cx)
                    } else {
                        editor.undo(cx)
                    } {
                        cx.stop_propagation();
                    }
                }
            }))
            .flex()
            .flex_col()
            .gap_2()
            .items_start()
            .w_full()
            .child(toggle)
            .children(toolbar)
            .child(crate::Text::new("Click a paragraph or heading to edit; other blocks remain rendered. Use source mode for unsupported structural edits.").sm())
            .child(content)
            .children(self.edit_error.clone().map(crate::Text::new))
    }
}
#[cfg(test)]
mod editor_mapping_tests {
    use super::replace_display_text;
    #[test]
    fn rich_edits_nested_paragraphs_without_removing_container_syntax() {
        for (original, visible, replacement, expected) in [
            (
                "- parent\n  - nested\n\n> quoted\n",
                "parent",
                "changed",
                "- changed\n  - nested\n\n> quoted\n",
            ),
            (
                "- parent\n  - nested\n\n> quoted\n",
                "nested",
                "changed",
                "- parent\n  - changed\n\n> quoted\n",
            ),
            (
                "- parent\n  - nested\n\n> quoted\n",
                "quoted",
                "changed",
                "- parent\n  - nested\n\n> changed\n",
            ),
            ("- **猫**\n", "猫", "猫!", "- **猫!**\n"),
            (
                "- **猫** and text\n",
                "猫 and text",
                "猫 and more",
                "- **猫** and more\n",
            ),
        ] {
            let document = super::MarkdownDocument::new(original);
            let mut blocks = Vec::new();
            super::MarkdownEditor::collect_editable_blocks(&document.blocks, &mut blocks);
            let (id, _) = blocks
                .iter()
                .find(|(_, text)| text.as_ref() == visible)
                .unwrap();
            let block = super::find_block(&document.blocks, *id).unwrap();
            let spans = block
                .spans
                .iter()
                .map(|span| (span.range.clone(), span.source_range.clone()))
                .collect::<Vec<_>>();
            let mut source = original.to_owned();
            assert!(replace_display_text(
                &mut source,
                block.source_range.clone(),
                visible,
                &spans,
                replacement
            ));
            assert_eq!(source, expected);
        }
    }

    #[test]
    fn edits_visible_text_without_removing_inline_markers() {
        let mut source = "**bold**\n".to_owned();
        replace_display_text(&mut source, 0..9, "bold", &[(0..4, 2..6)], "bold!");
        assert_eq!(source, "**bold!**\n");
    }

    #[test]
    fn edits_plain_text_without_removing_paragraph_separator() {
        let mut source = "first\n".to_owned();
        replace_display_text(&mut source, 0..6, "first", &[(0..5, 0..5)], "second");
        assert_eq!(source, "second\n");
    }
    #[test]
    fn edits_unicode_visible_text_on_character_boundaries() {
        let mut source = "**猫**\n".to_owned();
        replace_display_text(&mut source, 0..8, "猫", &[(0..3, 2..5)], "猫!");
        assert_eq!(source, "**猫!**\n");
    }
    #[test]
    fn edits_highlight_content_without_removing_markers() {
        let mut source = "==marked==\n".to_owned();
        assert!(replace_display_text(
            &mut source,
            0..11,
            "marked",
            &[(0..6, 0..10)],
            "marked!",
        ));
        assert_eq!(source, "==marked!==\n");
    }

    #[test]
    fn rejects_cross_style_deletion_without_mutating_source() {
        let mut source = "**bold** plain\n".to_owned();
        assert!(!replace_display_text(
            &mut source,
            0..15,
            "bold plain",
            &[(0..4, 2..6), (4..10, 8..14)],
            "",
        ));
        assert_eq!(source, "**bold** plain\n");
    }
    #[test]
    fn empty_paragraph_accepts_first_character() {
        let mut source = "\n".to_owned();
        assert!(replace_display_text(&mut source, 0..1, "", &[], "hello"));
        assert_eq!(source, "hello\n");
    }

    #[test]
    fn escaped_text_edits_do_not_change_meaning() {
        let mut source = r"\*".to_owned();
        assert!(!replace_display_text(
            &mut source,
            0..2,
            "*",
            &[(0..1, 0..2)],
            "x"
        ));
        assert_eq!(source, r"\*");
    }

    #[test]
    fn insertion_at_style_boundary_uses_following_span() {
        let mut source = "**bold** plain".to_owned();
        assert!(replace_display_text(
            &mut source,
            0..14,
            "bold plain",
            &[(0..4, 2..6), (4..10, 8..14)],
            "bold! plain",
        ));
        assert_eq!(source, "**bold**! plain");
    }
    #[test]
    fn editing_plain_text_keeps_neighboring_escaped_span() {
        let mut source = "word \\*".to_owned();
        assert!(replace_display_text(
            &mut source,
            0..7,
            "word *",
            &[(0..5, 0..5), (5..6, 5..7)],
            "next *",
        ));
        assert_eq!(source, "next \\*");
    }
}

#[cfg(test)]
mod editor_asset_tests {
    use super::*;

    #[test]
    fn prose_edits_reuse_unchanged_formula_images() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let editor = app.new(|cx| MarkdownEditor::new("Original paragraph.\n\n$x$\n", cx));
        app.update(|cx| {
            let cache = editor.read(cx).preview.read(cx).assets.clone();
            let kind = assets::AssetKind::Math(false, [0, 0, 0, 255]);
            let before = cache.render(kind, "x").unwrap();
            editor.update(cx, |editor, cx| {
                editor.set_source("Edited paragraph.\n\n$x$\n", cx);
            });
            let after = cache.render(kind, "x").unwrap();
            assert!(
                Arc::ptr_eq(&before, &after),
                "prose edits must retain cached formula rasters"
            );
        });
        app.quit();
    }
}

#[cfg(test)]
mod rich_pointer_tests {
    use super::*;
    use gpui::{
        FocusHandle, MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PlatformInput,
        TestAppContext, div, px, size,
    };

    struct Surface {
        editor: Entity<MarkdownEditor>,
        outside: FocusHandle,
    }
    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            div()
                .flex()
                .flex_col()
                .w_full()
                .child(self.editor.clone())
                .child(
                    div()
                        .id("outside-markdown")
                        .track_focus(&self.outside)
                        .child("Outside"),
                )
        }
    }

    fn click(window: &mut Window, position: gpui::Point<gpui::Pixels>, cx: &mut gpui::App) {
        window.dispatch_event(
            PlatformInput::MouseMove(MouseMoveEvent {
                position,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            PlatformInput::MouseDown(MouseDownEvent {
                position,
                button: MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            PlatformInput::MouseUp(MouseUpEvent {
                position,
                button: MouseButton::Left,
                click_count: 1,
                ..Default::default()
            }),
            cx,
        );
    }

    #[test]
    fn preview_pointer_places_display_caret_and_blur_restores_rendering() {
        let source = "Click this paragraph to edit it immediately.\n\nNative **bold猫**, *emphasis*.\n\n| A | B |\n| --- | --- |\n| cell | value |\n";
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let editor = app.new(|cx| MarkdownEditor::new(source, cx));
        let outside = app.update(|cx| cx.focus_handle());
        let first = app.update(|cx| {
            editor.update(cx, |editor, cx| {
                editor.ensure_rich_inputs(cx);
                editor.preview.read(cx).document.blocks[0].id
            })
        });
        let handle = app.open_window(size(px(900.), px(800.)), |_, _| Surface {
            editor: editor.clone(),
            outside: outside.clone(),
        });
        app.update_window(handle.into(), |_, window, cx| {
            window.draw(cx).clear(cx);
            let position = crate::selectable_text::test_display_point(
                &format!("markdown-block-{first}-text"),
                "Click this paragraph to edit it immediately.".len(),
                px(cx.global::<liora_core::Config>().theme.font_size.md) * 1.6,
            );
            click(window, position, cx);
            window.draw(cx).clear(cx);
            let input = editor.read(cx).block_inputs[&first].clone();
            assert!(input.focus_handle(cx).is_focused(window));
            input.update(cx, |input, cx| input.insert_text("!", cx));
        })
        .unwrap();
        app.update(|cx| {
            assert!(
                editor
                    .read(cx)
                    .source()
                    .starts_with("Click this paragraph to edit it immediately.!")
            )
        });
        app.update_window(handle.into(), |_, window, cx| {
            window.focus(&outside, cx);
            window.draw(cx).clear(cx);
            assert!(editor.read(cx).rich_focus_subscription.is_none());
            let second = editor.read(cx).preview.read(cx).document.blocks[1].id;
            let position = crate::selectable_text::test_display_point(
                &format!("markdown-block-{second}-text"),
                "Native bo".len(),
                px(cx.global::<liora_core::Config>().theme.font_size.md) * 1.6,
            );
            click(window, position, cx);
            window.draw(cx).clear(cx);
            let input = editor.read(cx).block_inputs[&second].clone();
            input.update(cx, |input, cx| input.insert_text("!", cx));
        })
        .unwrap();
        app.update(|cx| {
            assert!(
                editor
                    .read(cx)
                    .source()
                    .contains("Native **bo!ld猫**, *emphasis*.")
            )
        });
        app.update_window(handle.into(), |_, window, cx| {
            let cell_offset = editor.read(cx).source().find("cell").unwrap();
            let cell = editor.update(cx, |editor, cx| {
                editor.rich_input_at(cell_offset, cx).unwrap()
            });
            window.focus(&cell.focus_handle(cx), cx);
            window.draw(cx).clear(cx);
            assert!(
                editor.read(cx).rich_focus_subscription.is_none(),
                "table focus must restore styled paragraph preview"
            );
            let second = editor.read(cx).preview.read(cx).document.blocks[1].id;
            let position = crate::selectable_text::test_display_point(
                &format!("markdown-block-{second}-text"),
                "Native b".len(),
                px(cx.global::<liora_core::Config>().theme.font_size.md) * 1.6,
            );
            click(window, position, cx);
            window.draw(cx).clear(cx);
            let input = editor.read(cx).block_inputs[&second].clone();
            input.update(cx, |input, cx| input.insert_text("?", cx));
        })
        .unwrap();
        app.update(|cx| {
            assert!(
                editor.read(cx).source().contains("Native **b?o!ld猫**"),
                "revisited block must use clicked caret, not saved caret"
            )
        });
        app.quit();
    }
}
