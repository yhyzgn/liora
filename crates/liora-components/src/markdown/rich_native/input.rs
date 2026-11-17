use super::*;
use gpui::{
    Bounds, Element, ElementId, ElementInputHandler, EntityInputHandler, GlobalElementId,
    InspectorElementId, KeyDownEvent, LayoutId, MouseDownEvent, MouseMoveEvent, Pixels, Point,
    Style, UTF16Selection, point,
};
use unicode_segmentation::UnicodeSegmentation;

pub(super) fn from_utf16(text: &str, target: usize) -> usize {
    let mut units = 0;
    for (offset, ch) in text.char_indices() {
        if units + ch.len_utf16() > target {
            return offset;
        }
        units += ch.len_utf16();
    }
    text.len()
}
fn to_utf16(text: &str, offset: usize) -> usize {
    text[..offset].encode_utf16().count()
}

impl MarkdownEditor {
    pub(super) fn history_key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let keys = &event.keystroke;
        if self.is_composing(cx)
            || self.find_input.focus_handle(cx).is_focused(window)
            || self.replacement_input.focus_handle(cx).is_focused(window)
            || !(keys.modifiers.control || keys.modifiers.platform)
            || keys.modifiers.alt
        {
            return;
        }
        match keys.key.as_str() {
            "z" => {
                if keys.modifiers.shift {
                    self.redo(cx);
                } else {
                    self.undo(cx);
                }
            }
            "y" => {
                self.redo(cx);
            }
            _ => return,
        }
        cx.stop_propagation();
    }
    fn utf16_range(&self, range: Range<usize>) -> Range<usize> {
        // Native callers can ask for reversed UTF-16 endpoints.
        let start = from_utf16(self.engine.source(), range.start);
        let end = from_utf16(self.engine.source(), range.end);
        start.min(end)..start.max(end)
    }
    pub(super) fn index_for_point(&self, point: Point<Pixels>) -> Option<usize> {
        let layout = self
            .layouts
            .values()
            .filter(|layout| {
                let viewport = self.list.viewport_bounds();
                layout.bounds.bottom() >= viewport.top() && layout.bounds.top() <= viewport.bottom()
            })
            .min_by(|a, b| {
                let distance = |layout: &text::TextLayout| {
                    if point.y < layout.bounds.top() {
                        layout.bounds.top() - point.y
                    } else if point.y > layout.bounds.bottom() {
                        point.y - layout.bounds.bottom()
                    } else {
                        px(0.)
                    }
                };
                distance(a)
                    .partial_cmp(&distance(b))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })?;
        if layout.id == 0 {
            Some(0)
        } else {
            self.engine.source_position(layout.id, layout.index(point))
        }
    }
    fn caret_geometry(&self) -> Option<(Rc<text::TextLayout>, usize)> {
        let (id, offset) = self.engine.display_position(self.engine.selection().head)?;
        Some((self.layouts.get(&id)?.clone(), offset))
    }
    fn move_to(&mut self, offset: usize, extend: bool, cx: &mut Context<Self>) {
        self.pending_vertical = None;
        self.pending_vertical_retry = None;
        let selection = self.engine.selection();
        self.engine.set_selection(Selection {
            anchor: if extend { selection.anchor } else { offset },
            head: offset,
        });
        self.slash_dismissed = false;
        self.reveal();
        cx.notify();
    }
    pub(super) fn mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_composing(cx) {
            return;
        }
        if !self.list.viewport_bounds().contains(&event.position) {
            return;
        }
        window.focus(&self.focus, cx);
        self.preferred_x = None;
        let Some(offset) = self.index_for_point(event.position) else {
            return;
        };
        if event.click_count == 2 {
            if let Some((id, display)) = self.engine.display_position(offset) {
                if let Some(block) = find_block(&self.engine.document().blocks, id) {
                    let range = word_range(&block.text, display);
                    if let (Some(start), Some(end)) = (
                        self.engine.source_position(id, range.start),
                        self.engine.source_position(id, range.end),
                    ) {
                        self.engine.set_selection(Selection {
                            anchor: start,
                            head: end,
                        });
                    }
                }
            }
            cx.notify();
        } else if event.click_count >= 3 {
            if let Some((id, _)) = self.engine.display_position(offset) {
                if let Some(block) = find_block(&self.engine.document().blocks, id) {
                    self.engine.set_selection(Selection {
                        anchor: block.source_range.start,
                        head: block.source_range.end,
                    });
                }
            }
            cx.notify();
        } else {
            self.move_to(offset, event.modifiers.shift, cx);
        }
        self.dragging = true;
    }
    pub(super) fn mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.dragging
            || event.pressed_button != Some(gpui::MouseButton::Left)
            || self.is_composing(cx)
        {
            return;
        }
        let viewport = self.list.viewport_bounds();
        if event.position.y < viewport.top() {
            self.list.scroll_by(-px(24.));
        } else if event.position.y > viewport.bottom() {
            self.list.scroll_by(px(24.));
        }
        if let Some(offset) = self.index_for_point(event.position) {
            self.move_to(offset, true, cx);
        }
    }
    fn horizontal(&mut self, right: bool, word: bool, extend: bool, cx: &mut Context<Self>) {
        self.preferred_x = None;
        let selection = self.engine.selection();
        if !extend && !selection.is_empty() {
            self.move_to(
                if right {
                    selection.range().end
                } else {
                    selection.range().start
                },
                false,
                cx,
            );
            return;
        }
        let Some((id, display)) = self.engine.display_position(selection.head) else {
            return;
        };
        let Some(block) = find_block(&self.engine.document().blocks, id) else {
            return;
        };
        let target = if word {
            if right {
                block
                    .text
                    .split_word_bound_indices()
                    .map(|(start, word)| start + word.len())
                    .find(|end| *end > display)
            } else {
                block
                    .text
                    .split_word_bound_indices()
                    .map(|(start, _)| start)
                    .take_while(|start| *start < display)
                    .last()
            }
        } else if right {
            block.text[display..]
                .grapheme_indices(true)
                .nth(1)
                .map(|(offset, _)| display + offset)
                .or_else(|| (display < block.text.len()).then_some(block.text.len()))
        } else {
            block.text[..display]
                .grapheme_indices(true)
                .next_back()
                .map(|(offset, _)| offset)
        };
        let target = target.map(|target| (id, target)).or_else(|| {
            let adjacent = Self::adjacent_text_block(&self.engine.document().blocks, block, right)?;
            Some((adjacent.id, if right { 0 } else { adjacent.text.len() }))
        });
        if let Some((id, target)) = target {
            if let Some(offset) = self.engine.source_position(id, target) {
                self.move_to(offset, extend, cx);
            }
        }
    }
    fn adjacent_text_block<'a>(
        blocks: &'a [Block],
        current: &Block,
        right: bool,
    ) -> Option<&'a Block> {
        fn visit<'a>(
            blocks: &'a [Block],
            current: &Block,
            right: bool,
            best: &mut Option<&'a Block>,
        ) {
            for block in blocks {
                let editable = text::is_native(block)
                    || matches!(
                        block.kind,
                        Kind::Paragraph
                            | Kind::Heading(_)
                            | Kind::DefinitionTitle
                            | Kind::DefinitionDefinition
                    );
                if editable && block.id != current.id {
                    let nearer = if right {
                        block.source_range.start >= current.source_range.end
                            && best.is_none_or(|best| {
                                (block.source_range.start, block.source_range.end)
                                    < (best.source_range.start, best.source_range.end)
                            })
                    } else {
                        block.source_range.end <= current.source_range.start
                            && best.is_none_or(|best| {
                                (
                                    block.source_range.end,
                                    std::cmp::Reverse(block.source_range.start),
                                ) > (
                                    best.source_range.end,
                                    std::cmp::Reverse(best.source_range.start),
                                )
                            })
                    };
                    if nearer {
                        *best = Some(block);
                    }
                }
                visit(&block.children, current, right, best);
            }
        }
        let mut best = None;
        visit(blocks, current, right, &mut best);
        best
    }
    pub(super) fn vertical(&mut self, down: bool, extend: bool, cx: &mut Context<Self>) {
        let Some((layout, offset)) = self.caret_geometry() else {
            self.pending_vertical_retry = Some((down, extend));
            self.reveal();
            cx.notify();
            return;
        };
        let position = layout.position(offset);
        let x = *self.preferred_x.get_or_insert(position.x);
        let target = point(
            x,
            position.y + layout.line_height * if down { 1.5 } else { -0.5 },
        );
        let display = layout.index(target);
        let next_y = layout.position(display).y;
        if (down && next_y > position.y) || (!down && next_y < position.y) {
            self.move_display_to(layout.id, display, extend, cx);
            return;
        }
        let Some(block) = find_block(&self.engine.document().blocks, layout.id) else {
            return;
        };
        let Some(adjacent) = Self::adjacent_text_block(&self.engine.document().blocks, block, down)
        else {
            return;
        };
        let id = adjacent.id;
        let edge = if down { 0 } else { adjacent.text.len() };
        if let Some(offset) = self.engine.source_position(id, edge) {
            let selection = self.engine.selection();
            self.engine.set_selection(Selection {
                anchor: if extend { selection.anchor } else { offset },
                head: offset,
            });
            self.pending_vertical = Some((id, down, extend));
            self.reveal();
            cx.notify();
        }
    }
    fn move_display_to(&mut self, id: u64, display: usize, extend: bool, cx: &mut Context<Self>) {
        let Some(block) = find_block(&self.engine.document().blocks, id) else {
            return;
        };
        let display = block
            .text
            .grapheme_indices(true)
            .map(|(offset, _)| offset)
            .chain(std::iter::once(block.text.len()))
            .take_while(|offset| *offset <= display)
            .last()
            .unwrap_or(0);
        if let Some(offset) = self.engine.source_position(id, display) {
            self.move_to(offset, extend, cx);
        }
    }
    pub(super) fn resolve_vertical(
        &mut self,
        layout: &text::TextLayout,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some((id, down, extend)) = self.pending_vertical.filter(|(id, _, _)| *id == layout.id)
        else {
            return;
        };
        let x = self.preferred_x.unwrap_or(layout.bounds.left());
        let y = if down {
            layout.bounds.top()
        } else {
            layout.bounds.bottom() - layout.line_height * 0.5
        };
        let display = layout.index(point(x, y));
        let Some(block) = find_block(&self.engine.document().blocks, id) else {
            return;
        };
        let display = block
            .text
            .grapheme_indices(true)
            .map(|(offset, _)| offset)
            .chain(std::iter::once(block.text.len()))
            .take_while(|offset| *offset <= display)
            .last()
            .unwrap_or(0);
        let Some(offset) = self.engine.source_position(id, display) else {
            return;
        };
        let selection = self.engine.selection();
        self.engine.set_selection(Selection {
            anchor: if extend { selection.anchor } else { offset },
            head: offset,
        });
        self.pending_vertical = None;
        self.reveal_pending = false;
        window.request_autoscroll(layout.range_bounds(display..display));
        cx.notify();
    }
    fn home_end(&mut self, end: bool, document: bool, extend: bool, cx: &mut Context<Self>) {
        self.preferred_x = None;
        let source = self.engine.source();
        let caret = self.engine.selection().head;
        let mut next = if end {
            source[caret..]
                .find('\n')
                .map_or(source.len(), |i| caret + i)
        } else {
            source[..caret].rfind('\n').map_or(0, |i| i + 1)
        };
        if document {
            next = if end { source.len() } else { 0 };
        } else if let Some((layout, offset)) = self.caret_geometry() {
            let position = layout.position(offset);
            let point = point(
                if end {
                    layout.bounds.right() + px(1.)
                } else {
                    layout.bounds.left() - px(1.)
                },
                position.y + layout.line_height * 0.5,
            );
            if let Some(offset) = self.engine.source_position(layout.id, layout.index(point)) {
                next = offset;
            }
        }
        self.move_to(next, extend, cx);
    }
    pub(super) fn key_down(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.focus.is_focused(window) || self.is_composing(cx) {
            return;
        }
        let keys = &event.keystroke;
        let control = keys.modifiers.control || keys.modifiers.platform;
        let shift = keys.modifiers.shift;
        let key = keys.key.as_str();
        if self.slash_range().is_some() {
            let count = self.slash_choices().len();
            match key {
                "escape" => {
                    self.slash_dismissed = true;
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                "up" | "down" if count > 0 => {
                    self.slash_index = if key == "up" {
                        (self.slash_index + count - 1) % count
                    } else {
                        (self.slash_index + 1) % count
                    };
                    cx.notify();
                    cx.stop_propagation();
                    return;
                }
                "enter" if count > 0 => {
                    self.apply_slash(self.slash_index, cx);
                    cx.stop_propagation();
                    return;
                }
                _ => {}
            }
        }
        let mut handled = true;
        if control && !keys.modifiers.alt {
            match key {
                "a" => self.move_all(cx),
                "c" | "x" => {
                    if !self.engine.selection().is_empty() {
                        cx.write_to_clipboard(gpui::ClipboardItem::new_string(
                            self.engine.selected_text(),
                        ));
                        if key == "x" {
                            self.replace("", cx);
                        }
                    }
                }
                "v" => {
                    if let Some(text) = cx.read_from_clipboard().and_then(|item| item.text()) {
                        self.replace(&text, cx);
                    }
                }
                "z" => {
                    if shift {
                        self.redo(cx);
                    } else {
                        self.undo(cx);
                    }
                }
                "y" => {
                    self.redo(cx);
                }
                "b" => {
                    self.execute(Command::Inline("**"), cx);
                }
                "i" => {
                    self.execute(Command::Inline("*"), cx);
                }
                "e" | "`" => {
                    self.execute(Command::Inline("`"), cx);
                }
                "s" if shift => {
                    self.execute(Command::Inline("~~"), cx);
                }
                "f" | "h" => {
                    self.find_open = true;
                    window.focus(&self.find_input.focus_handle(cx), cx);
                    cx.notify();
                }
                "g" => self.find_next(shift, cx),
                "home" | "end" => self.home_end(key == "end", true, shift, cx),
                "left" | "right" => self.horizontal(key == "right", true, shift, cx),
                _ => handled = false,
            }
        } else {
            match key {
                "backspace" => {
                    self.execute(Command::Backspace, cx);
                }
                "delete" => {
                    self.execute(Command::Delete, cx);
                }
                "enter" => {
                    self.execute(Command::Enter, cx);
                }
                "tab" => {
                    self.execute(
                        if shift {
                            Command::Outdent
                        } else {
                            Command::Indent
                        },
                        cx,
                    );
                }
                "left" | "right" => self.horizontal(key == "right", keys.modifiers.alt, shift, cx),
                "up" | "down" if keys.modifiers.alt => {
                    self.move_block(if key == "up" { -1 } else { 1 }, cx)
                }
                "up" | "down" => self.vertical(key == "down", shift, cx),
                "home" | "end" => self.home_end(key == "end", false, shift, cx),
                "escape" => {
                    self.find_open = false;
                    self.slash_dismissed = true;
                    cx.notify();
                }
                _ => handled = false,
            }
        }
        if handled {
            cx.stop_propagation();
        }
    }
    fn move_all(&mut self, cx: &mut Context<Self>) {
        self.engine.set_selection(Selection {
            anchor: 0,
            head: self.engine.source().len(),
        });
        cx.notify();
    }
    fn finish_composition(&mut self, cx: &mut Context<Self>) {
        self.marked = None;
        if self.composition {
            self.composition = false;
            self.engine.end_undo_group();
            self.publish(cx);
        }
    }
}

impl EntityInputHandler for MarkdownEditor {
    fn text_for_range(
        &mut self,
        range: Range<usize>,
        actual: &mut Option<Range<usize>>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<String> {
        let range = self.utf16_range(range);
        *actual = Some(
            to_utf16(self.engine.source(), range.start)..to_utf16(self.engine.source(), range.end),
        );
        self.engine.source().get(range).map(str::to_owned)
    }
    fn selected_text_range(
        &mut self,
        _: bool,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<UTF16Selection> {
        let selection = self.engine.selection();
        let range = selection.range();
        Some(UTF16Selection {
            range: to_utf16(self.engine.source(), range.start)
                ..to_utf16(self.engine.source(), range.end),
            reversed: selection.anchor > selection.head,
        })
    }
    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked.as_ref().map(|range| {
            to_utf16(self.engine.source(), range.start)..to_utf16(self.engine.source(), range.end)
        })
    }
    fn unmark_text(&mut self, _: &mut Window, cx: &mut Context<Self>) {
        self.finish_composition(cx);
    }
    fn replace_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.mode != MarkdownEditMode::Rich
            || self.input.read(cx).is_composing()
            || self
                .tables
                .iter()
                .any(|(_, table)| table.read(cx).is_composing(cx))
        {
            return;
        }
        let range = range
            .map(|range| self.utf16_range(range))
            .or_else(|| self.marked.clone())
            .unwrap_or_else(|| self.selection());
        self.engine.set_selection(Selection {
            anchor: range.start,
            head: range.end,
        });
        if self.engine.type_markdown(text) {
            self.error = None;
            self.changed(cx);
        }
        if self.composition {
            self.finish_composition(cx);
        }
    }
    fn replace_and_mark_text_in_range(
        &mut self,
        range: Option<Range<usize>>,
        text: &str,
        selected: Option<Range<usize>>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.mode != MarkdownEditMode::Rich
            || self.input.read(cx).is_composing()
            || self
                .tables
                .iter()
                .any(|(_, table)| table.read(cx).is_composing(cx))
        {
            return;
        }
        // XIM sends an empty preedit after commit; it must not reopen a transaction.
        if text.is_empty() && !self.composition {
            return;
        }
        if !self.composition {
            self.pending_vertical = None;
            self.pending_vertical_retry = None;
            self.engine.begin_undo_group();
            self.composition = true;
        }
        let range = range
            .map(|range| self.utf16_range(range))
            .or_else(|| self.marked.clone())
            .unwrap_or_else(|| self.selection());
        self.engine.set_selection(Selection {
            anchor: range.start,
            head: range.end,
        });
        self.engine.type_markdown(text);
        let end = self.engine.selection().head;
        let start = range.start;
        self.marked = (!text.is_empty()).then_some(start..end);
        if let Some(selected) = selected {
            let anchor = start + from_utf16(text, selected.start);
            let head = start + from_utf16(text, selected.end);
            self.engine.set_selection(Selection { anchor, head });
        }
        self.changed(cx);
        if text.is_empty() {
            self.finish_composition(cx);
        }
    }
    fn bounds_for_range(
        &mut self,
        range: Range<usize>,
        _: Bounds<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<Bounds<Pixels>> {
        let range = self.utf16_range(range);
        if self.engine.document().blocks.is_empty() {
            return Some(self.layouts.get(&0)?.range_bounds(0..0));
        }
        let (id, start) = self.engine.display_position(range.start)?;
        let end = self
            .engine
            .display_position(range.end)
            .filter(|(next, _)| *next == id)
            .map_or(start, |(_, offset)| offset);
        Some(self.layouts.get(&id)?.range_bounds(start..end))
    }
    fn character_index_for_point(
        &mut self,
        point: Point<Pixels>,
        _: &mut Window,
        _: &mut Context<Self>,
    ) -> Option<usize> {
        self.index_for_point(point)
            .map(|offset| to_utf16(self.engine.source(), offset))
    }
}

fn word_range(text: &str, offset: usize) -> Range<usize> {
    text.split_word_bound_indices()
        .find_map(|(start, word)| {
            (start <= offset && offset < start + word.len()).then_some(start..start + word.len())
        })
        .unwrap_or(text.len()..text.len())
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

pub(super) struct InputLayer {
    pub editor: Entity<MarkdownEditor>,
}
impl Element for InputLayer {
    type RequestLayoutState = ();
    type PrepaintState = ();
    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }
    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let mut style = Style::default();
        style.position = gpui::Position::Absolute;
        style.size.width = gpui::relative(1.).into();
        style.size.height = gpui::relative(1.).into();
        (window.request_layout(style, [], cx), ())
    }
    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut Window,
        cx: &mut App,
    ) {
        self.editor.update(cx, |editor, _| editor.layouts.clear());
    }
    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus = self.editor.read(cx).focus.clone();
        window.handle_input(
            &focus,
            ElementInputHandler::new(bounds, self.editor.clone()),
            cx,
        );
    }
}

/// GPUI Div scrolls on wheel input but does not consume child autoscroll requests.
pub(super) struct SourceViewport {
    pub child: AnyElement,
    pub editor: Entity<MarkdownEditor>,
}

impl Element for SourceViewport {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.prepaint(window, cx);
        let Some(caret) = window.take_autoscroll() else {
            return;
        };
        let scroll = self.editor.read(cx).source_scroll.clone();
        let viewport = scroll.bounds();
        let delta = if caret.top() < viewport.top() {
            viewport.top() - caret.top()
        } else if caret.bottom() > viewport.bottom() {
            viewport.bottom() - caret.bottom()
        } else {
            px(0.)
        };
        if delta != px(0.) {
            let mut offset = scroll.offset();
            offset.y = (offset.y + delta).min(px(0.));
            scroll.set_offset(offset);
            self.editor.update(cx, |_, cx| cx.notify());
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.paint(window, cx);
    }
}
