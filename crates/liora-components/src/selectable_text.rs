use crate::gpui_compat::element_id;
use gpui::{
    App, Bounds, ClipboardItem, Component, Context, Element, ElementId, Entity, FocusHandle,
    Focusable, GlobalElementId, InspectorElementId, IntoElement, KeyDownEvent, LayoutId,
    MouseButton, MouseDownEvent, MouseMoveEvent, MouseUpEvent, PaintQuad, Pixels, Point, Render,
    RenderOnce, SharedString, Style, TextRun, TextStyle, WhiteSpace, Window, actions, div, fill,
    point, prelude::*, px, relative, size,
};
use liora_core::Config;
use std::{
    collections::HashMap,
    ops::Range,
    sync::{Arc, Mutex, MutexGuard, OnceLock},
};

actions!(
    selectable_text_actions,
    [SelectableTextSelectAll, SelectableTextCopy]
);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectableTextWrap {
    Normal,
    NoWrap,
}

impl SelectableTextWrap {
    fn white_space(self) -> WhiteSpace {
        match self {
            Self::Normal => WhiteSpace::Normal,
            Self::NoWrap => WhiteSpace::Nowrap,
        }
    }
}

#[derive(Clone)]
pub struct SelectableTextOptions {
    pub id: ElementId,
    pub text: SharedString,
    pub runs: Vec<TextRun>,
    pub font_size: Pixels,
    pub line_height: Pixels,
    pub text_color: gpui::Hsla,
    pub wrap: SelectableTextWrap,
    pub key_context: &'static str,
    pub fill_width: bool,
}

impl SelectableTextOptions {
    pub fn new(id: impl Into<ElementId>, text: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            text: text.into(),
            runs: Vec::new(),
            font_size: px(14.0),
            line_height: px(22.0),
            text_color: gpui::black(),
            wrap: SelectableTextWrap::Normal,
            key_context: "SelectableText",
            fill_width: true,
        }
    }
}

pub struct SelectableText;

impl SelectableText {
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            gpui::KeyBinding::new("cmd-a", SelectableTextSelectAll, Some("SelectableText")),
            gpui::KeyBinding::new("ctrl-a", SelectableTextSelectAll, Some("SelectableText")),
            gpui::KeyBinding::new("cmd-c", SelectableTextCopy, Some("SelectableText")),
            gpui::KeyBinding::new("ctrl-c", SelectableTextCopy, Some("SelectableText")),
        ]);
    }

    pub fn view(
        options: SelectableTextOptions,
        window: &mut Window,
        cx: &mut App,
    ) -> gpui::AnyElement {
        let input = window.use_keyed_state(options.id.clone(), cx, {
            let initial = options.clone();
            move |_, cx| SelectableTextState::new(cx, initial)
        });
        input.update(cx, |state, cx| state.update_options(options, cx));
        SelectableTextView { input }.into_any_element()
    }
}

struct SelectableTextView {
    input: Entity<SelectableTextState>,
}

impl IntoElement for SelectableTextView {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for SelectableTextView {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        self.input.into_any_element()
    }
}

struct SelectableTextSelectionState {
    selected_range: Range<usize>,
    selection_reversed: bool,
    selecting: bool,
    layout: Option<Arc<SelectableTextLayout>>,
    line_starts: Vec<(Pixels, usize)>,
    bounds: Option<Bounds<Pixels>>,
}

impl Default for SelectableTextSelectionState {
    fn default() -> Self {
        Self {
            selected_range: 0..0,
            selection_reversed: false,
            selecting: false,
            layout: None,
            line_starts: Vec::new(),
            bounds: None,
        }
    }
}

fn selection_state_map() -> &'static Mutex<HashMap<String, SelectableTextSelectionState>> {
    static STATES: OnceLock<Mutex<HashMap<String, SelectableTextSelectionState>>> = OnceLock::new();
    STATES.get_or_init(|| Mutex::new(HashMap::new()))
}

fn lock_selection_state_map() -> MutexGuard<'static, HashMap<String, SelectableTextSelectionState>>
{
    selection_state_map()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn selection_key(id: &ElementId) -> String {
    format!("{id:?}")
}

fn with_selection_state<R>(
    id: &ElementId,
    f: impl FnOnce(&mut SelectableTextSelectionState) -> R,
) -> R {
    let mut states = lock_selection_state_map();
    f(states.entry(selection_key(id)).or_default())
}

fn selected_range_snapshot(id: &ElementId) -> Range<usize> {
    lock_selection_state_map()
        .get(&selection_key(id))
        .map(|state| state.selected_range.clone())
        .unwrap_or(0..0)
}

fn set_layout_state(
    id: &ElementId,
    layout: Arc<SelectableTextLayout>,
    line_starts: Vec<(Pixels, usize)>,
    bounds: Bounds<Pixels>,
) {
    with_selection_state(id, |state| {
        state.layout = Some(layout);
        state.line_starts = line_starts;
        state.bounds = Some(bounds);
    });
}

struct SelectableTextState {
    id: ElementId,
    text: SharedString,
    runs: Vec<TextRun>,
    font_size: Pixels,
    line_height: Pixels,
    text_color: gpui::Hsla,
    wrap: SelectableTextWrap,
    key_context: &'static str,
    fill_width: bool,
    focus_handle: FocusHandle,
}

impl SelectableTextState {
    fn new(cx: &mut Context<Self>, options: SelectableTextOptions) -> Self {
        Self {
            id: options.id,
            runs: normalize_runs(options.runs, options.text.len(), options.text_color),
            text: options.text,
            font_size: options.font_size,
            line_height: options.line_height,
            text_color: options.text_color,
            wrap: options.wrap,
            key_context: options.key_context,
            fill_width: options.fill_width,
            focus_handle: cx.focus_handle(),
        }
    }

    fn update_options(&mut self, options: SelectableTextOptions, cx: &mut Context<Self>) {
        let runs = normalize_runs(options.runs, options.text.len(), options.text_color);
        let changed = self.id != options.id
            || self.text != options.text
            || self.runs != runs
            || self.font_size != options.font_size
            || self.line_height != options.line_height
            || self.text_color != options.text_color
            || self.wrap != options.wrap
            || self.key_context != options.key_context
            || self.fill_width != options.fill_width;
        if !changed {
            return;
        }

        let old_id = self.id.clone();
        self.id = options.id;
        self.text = options.text;
        self.runs = runs;
        self.font_size = options.font_size;
        self.line_height = options.line_height;
        self.text_color = options.text_color;
        self.wrap = options.wrap;
        self.key_context = options.key_context;
        self.fill_width = options.fill_width;

        if old_id != self.id {
            let old_range = selected_range_snapshot(&old_id);
            with_selection_state(&self.id, |state| state.selected_range = old_range);
        }
        with_selection_state(&self.id, |state| {
            state.selected_range.start = self.clamp_boundary(state.selected_range.start);
            state.selected_range.end = self.clamp_boundary(state.selected_range.end);
            if state.selected_range.end < state.selected_range.start {
                state.selected_range = state.selected_range.end..state.selected_range.start;
                state.selection_reversed = !state.selection_reversed;
            }
        });
        cx.notify();
    }

    fn text_style(&self, window: &Window) -> TextStyle {
        let mut style = window.text_style();
        style.color = self.text_color;
        style.font_size = self.font_size.into();
        style.line_height = self.line_height.into();
        style.white_space = self.wrap.white_space();
        style.text_overflow = None;
        style.line_clamp = None;
        style
    }

    fn move_to(&self, state: &mut SelectableTextSelectionState, offset: usize) -> bool {
        let offset = self.clamp_boundary(offset);
        if state.selected_range == (offset..offset) && !state.selection_reversed {
            return false;
        }
        state.selected_range = offset..offset;
        state.selection_reversed = false;
        true
    }

    fn select_to(&self, state: &mut SelectableTextSelectionState, offset: usize) -> bool {
        let offset = self.clamp_boundary(offset);
        let previous_range = state.selected_range.clone();
        let previous_reversed = state.selection_reversed;
        if state.selection_reversed {
            state.selected_range.start = offset;
        } else {
            state.selected_range.end = offset;
        }
        if state.selected_range.end < state.selected_range.start {
            state.selection_reversed = !state.selection_reversed;
            state.selected_range = state.selected_range.end..state.selected_range.start;
        }
        state.selected_range != previous_range || state.selection_reversed != previous_reversed
    }

    fn clamp_boundary(&self, mut offset: usize) -> usize {
        offset = offset.min(self.text.len());
        while offset > 0 && !self.text.is_char_boundary(offset) {
            offset -= 1;
        }
        offset
    }

    fn index_for_point(&self, pt: Point<Pixels>) -> usize {
        let states = lock_selection_state_map();
        let Some(state) = states.get(&selection_key(&self.id)) else {
            return self.text.len();
        };
        let Some(bounds) = state.bounds.as_ref() else {
            return self.text.len();
        };
        let Some(layout) = state.layout.as_ref() else {
            return self.text.len();
        };
        if layout.lines.is_empty() {
            return self.text.len();
        }

        let mut chosen = 0;
        for (ix, line) in layout.lines.iter().enumerate() {
            let y = state
                .line_starts
                .get(ix)
                .map(|(y, _)| *y)
                .unwrap_or(bounds.top());
            let line_bottom = y + line.size(self.line_height).height;
            if pt.y <= line_bottom {
                chosen = ix;
                break;
            }
            if pt.y >= y {
                chosen = ix;
            }
        }

        let line = &layout.lines[chosen];
        let (y, start) = state
            .line_starts
            .get(chosen)
            .copied()
            .unwrap_or((bounds.top(), 0));
        let position = point(pt.x - bounds.left(), pt.y - y);
        let line_index = line
            .closest_index_for_position(position, self.line_height)
            .unwrap_or_else(|idx| idx);
        self.clamp_boundary(start + line_index)
    }

    fn on_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        window.focus(&self.focus_handle);
        let idx = self.index_for_point(event.position);
        let changed = with_selection_state(&self.id, |state| {
            let was_selecting = state.selecting;
            state.selecting = true;
            if event.modifiers.shift {
                self.select_to(state, idx) || !was_selecting
            } else if event.click_count >= 3 {
                let changed = state.selected_range != (0..self.text.len())
                    || state.selection_reversed
                    || !was_selecting;
                state.selected_range = 0..self.text.len();
                state.selection_reversed = false;
                changed
            } else if event.click_count == 2 {
                let range = self.word_range_at(idx);
                let changed =
                    state.selected_range != range || state.selection_reversed || !was_selecting;
                state.selected_range = range;
                state.selection_reversed = false;
                changed
            } else {
                self.move_to(state, idx) || !was_selecting
            }
        });
        if changed {
            cx.notify();
        }
    }

    fn on_mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let dragging = event.pressed_button == Some(MouseButton::Left);
        let idx = dragging.then(|| self.index_for_point(event.position));
        let changed = with_selection_state(&self.id, |state| {
            if !dragging {
                let changed = state.selecting;
                state.selecting = false;
                changed
            } else if state.selecting {
                self.select_to(state, idx.unwrap_or(self.text.len()))
            } else {
                false
            }
        });
        if changed {
            cx.notify();
        }
    }

    fn on_mouse_up(&mut self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>) {
        let changed = with_selection_state(&self.id, |state| {
            let changed = state.selecting;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn clear_selection(&mut self, cx: &mut Context<Self>) {
        let changed = with_selection_state(&self.id, |state| {
            let changed = !state.selected_range.is_empty() || state.selecting;
            state.selected_range = 0..0;
            state.selection_reversed = false;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn set_select_all(&mut self, cx: &mut Context<Self>) {
        let changed = with_selection_state(&self.id, |state| {
            let changed = state.selected_range != (0..self.text.len())
                || state.selection_reversed
                || state.selecting;
            state.selected_range = 0..self.text.len();
            state.selection_reversed = false;
            state.selecting = false;
            changed
        });
        if changed {
            cx.notify();
        }
    }

    fn select_all(&mut self, _: &SelectableTextSelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.set_select_all(cx);
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if event.keystroke.key.eq_ignore_ascii_case("a")
            && (event.keystroke.modifiers.control || event.keystroke.modifiers.platform)
            && !event.keystroke.modifiers.alt
            && !event.keystroke.modifiers.shift
            && !event.keystroke.modifiers.function
        {
            self.set_select_all(cx);
            cx.stop_propagation();
        }
    }

    fn copy(&mut self, _: &SelectableTextCopy, _: &mut Window, cx: &mut Context<Self>) {
        let selected_range = selected_range_snapshot(&self.id);
        if !selected_range.is_empty() {
            cx.write_to_clipboard(ClipboardItem::new_string(
                self.text[selected_range].to_string(),
            ));
        }
    }

    fn word_range_at(&self, idx: usize) -> Range<usize> {
        let text = self.text.as_ref();
        if text.is_empty() {
            return 0..0;
        }
        let idx = self.clamp_boundary(idx);
        let mut start = idx;
        while start > 0 {
            let prev = self.prev_char(start);
            let ch = text[prev..start].chars().next().unwrap_or(' ');
            if !is_word_char(ch) {
                break;
            }
            start = prev;
        }
        let mut end = idx;
        while end < text.len() {
            let next = self.next_char(end);
            let ch = text[end..next].chars().next().unwrap_or(' ');
            if !is_word_char(ch) {
                break;
            }
            end = next;
        }
        start..end
    }

    fn prev_char(&self, offset: usize) -> usize {
        if offset == 0 {
            return 0;
        }
        let mut prev = offset - 1;
        while prev > 0 && !self.text.is_char_boundary(prev) {
            prev -= 1;
        }
        prev
    }

    fn next_char(&self, offset: usize) -> usize {
        if offset >= self.text.len() {
            return self.text.len();
        }
        let mut next = offset + 1;
        while next < self.text.len() && !self.text.is_char_boundary(next) {
            next += 1;
        }
        next
    }
}

impl Focusable for SelectableTextState {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for SelectableTextState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        cx.on_blur(&self.focus_handle, window, |this, _, cx| {
            this.clear_selection(cx);
        })
        .detach();

        div()
            .id(element_id(format!("{:?}-selectable", self.id)))
            .key_context(self.key_context)
            .track_focus(&self.focus_handle(cx))
            .cursor_text()
            .on_key_down(cx.listener(Self::on_key_down))
            .on_action(cx.listener(Self::select_all))
            .on_action(cx.listener(Self::copy))
            .child(SelectableTextElement {
                id: element_id(format!("{:?}-text", self.id)),
                input: cx.entity(),
            })
    }
}

struct SelectableTextLayout {
    lines: Vec<gpui::WrappedLine>,
    width: Pixels,
    height: Pixels,
}

struct SelectableTextElement {
    id: ElementId,
    input: Entity<SelectableTextState>,
}

struct SelectableTextPrepaint {
    layout: Arc<SelectableTextLayout>,
    selection: Vec<PaintQuad>,
    hitbox: gpui::Hitbox,
}

impl IntoElement for SelectableTextElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for SelectableTextElement {
    type RequestLayoutState = Arc<SelectableTextLayout>;
    type PrepaintState = SelectableTextPrepaint;

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
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
    ) -> (LayoutId, Arc<SelectableTextLayout>) {
        let input = self.input.read(cx);
        let layout = build_selectable_layout(input, window);
        let mut style = Style::default();
        style.size.width = layout.width.into();
        style.min_size.width = relative(1.).into();
        style.size.height = layout.height.into();
        if input.fill_width {
            style.size.width = relative(1.).into();
        }
        (window.request_layout(style, [], cx), Arc::new(layout))
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        layout: &mut Arc<SelectableTextLayout>,
        window: &mut Window,
        cx: &mut App,
    ) -> SelectableTextPrepaint {
        let input = self.input.read(cx);
        let mut selection_quads = Vec::new();
        let selected_range = selected_range_snapshot(&input.id);
        let mut y = bounds.top();
        let mut line_starts = Vec::new();
        let selection_color = cx.global::<Config>().theme.primary.base.opacity(0.28);

        let mut line_start = 0;
        for line in &layout.lines {
            if !selected_range.is_empty() {
                let line_end = line_start + line.len();
                let start = selected_range.start.max(line_start);
                let end = selected_range.end.min(line_end);
                if start < end {
                    add_wrapped_selection_quads(
                        line,
                        start - line_start,
                        end - line_start,
                        y,
                        input.line_height,
                        bounds,
                        selection_color,
                        &mut selection_quads,
                    );
                }
            }
            line_starts.push((y, line_start));
            y += line.size(input.line_height).height;
            line_start += line.len() + 1;
        }

        let hitbox = window.insert_hitbox(bounds, gpui::HitboxBehavior::Normal);
        set_layout_state(&input.id, layout.clone(), line_starts, bounds);

        SelectableTextPrepaint {
            layout: layout.clone(),
            selection: selection_quads,
            hitbox,
        }
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Arc<SelectableTextLayout>,
        prepaint: &mut SelectableTextPrepaint,
        window: &mut Window,
        cx: &mut App,
    ) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.set_cursor_style(gpui::CursorStyle::IBeam, &prepaint.hitbox);

        let input = self.input.clone();
        let focus_handle_for_down = focus_handle.clone();
        let hitbox = prepaint.hitbox.clone();
        window.on_mouse_event(move |event: &MouseDownEvent, phase, window, cx| {
            if phase.bubble() && event.button == MouseButton::Left && hitbox.is_hovered(window) {
                window.focus(&focus_handle_for_down);
                input.update(cx, |input, cx| input.on_mouse_down(event, window, cx));
                cx.stop_propagation();
            }
        });

        let input = self.input.clone();
        window.on_mouse_event(move |event: &MouseMoveEvent, phase, _window, cx| {
            if phase.capture() {
                input.update(cx, |input, cx| input.on_mouse_move(event, cx));
            }
        });

        let input = self.input.clone();
        window.on_mouse_event(move |event: &MouseUpEvent, phase, window, cx| {
            if phase.capture() && event.button == MouseButton::Left {
                input.update(cx, |input, cx| input.on_mouse_up(event, window, cx));
            }
        });

        for selection in prepaint.selection.drain(..) {
            window.paint_quad(selection);
        }

        let (line_height, text_align) = {
            let input = self.input.read(cx);
            (input.line_height, input.text_style(window).text_align)
        };
        let mut origin = bounds.origin;
        for line in &prepaint.layout.lines {
            let _ =
                line.paint_background(origin, line_height, text_align, Some(bounds), window, cx);
            let _ = line.paint(origin, line_height, text_align, Some(bounds), window, cx);
            origin.y += line.size(line_height).height;
        }
    }
}

fn build_selectable_layout(
    input: &SelectableTextState,
    window: &mut Window,
) -> SelectableTextLayout {
    let wrap_width = if input.wrap == SelectableTextWrap::Normal {
        Some(window.viewport_size().width.max(px(1.0)))
    } else {
        None
    };

    let lines: Vec<gpui::WrappedLine> = window
        .text_system()
        .shape_text(
            input.text.clone(),
            input.font_size,
            &input.runs,
            wrap_width,
            None,
        )
        .map(|lines| lines.into_iter().collect())
        .unwrap_or_default();

    let mut width = px(1.0);
    let mut height = px(0.0);
    for line in &lines {
        let line_size = line.size(input.line_height);
        width = width.max(line_size.width).ceil();
        height += line_size.height;
    }

    SelectableTextLayout {
        lines,
        width,
        height,
    }
}

fn add_wrapped_selection_quads(
    line: &gpui::WrappedLine,
    start: usize,
    end: usize,
    y: Pixels,
    line_height: Pixels,
    bounds: Bounds<Pixels>,
    color: gpui::Hsla,
    quads: &mut Vec<PaintQuad>,
) {
    let mut segment_start = start;
    while segment_start < end {
        let Some(start_pos) = line.position_for_index(segment_start, line_height) else {
            break;
        };
        let mut segment_end = end;
        let start_row = (start_pos.y / line_height).floor() as usize;
        while segment_end > segment_start {
            if let Some(end_pos) = line.position_for_index(segment_end, line_height) {
                let end_row = (end_pos.y / line_height).floor() as usize;
                if end_row == start_row {
                    let width = (end_pos.x - start_pos.x).max(px(1.0));
                    quads.push(fill(
                        Bounds::new(
                            point(bounds.left() + start_pos.x, y + start_pos.y),
                            size(width, line_height),
                        ),
                        color,
                    ));
                    break;
                }
            }
            segment_end = previous_boundary(line.text.as_ref(), segment_end);
        }
        if segment_end <= segment_start {
            break;
        }
        segment_start = segment_end;
    }
}

fn normalize_runs(mut runs: Vec<TextRun>, text_len: usize, color: gpui::Hsla) -> Vec<TextRun> {
    if text_len == 0 {
        return Vec::new();
    }
    if runs.is_empty() {
        let mut run = TextStyle::default().to_run(text_len);
        run.color = color;
        return vec![run];
    }
    let mut total = 0;
    for run in &mut runs {
        if run.color == gpui::transparent_black() {
            run.color = color;
        }
        total += run.len;
    }
    if total < text_len {
        let mut run = runs
            .last()
            .cloned()
            .unwrap_or_else(|| TextStyle::default().to_run(0));
        run.len = text_len - total;
        runs.push(run);
    } else if total > text_len {
        let mut remaining = text_len;
        runs.retain_mut(|run| {
            if remaining == 0 {
                return false;
            }
            if run.len > remaining {
                run.len = remaining;
            }
            remaining = remaining.saturating_sub(run.len);
            true
        });
    }
    runs
}

fn is_word_char(ch: char) -> bool {
    ch.is_alphanumeric() || ch == '_' || ('\u{4e00}'..='\u{9fff}').contains(&ch)
}

fn previous_boundary(text: &str, mut offset: usize) -> usize {
    offset = offset.saturating_sub(1).min(text.len());
    while offset > 0 && !text.is_char_boundary(offset) {
        offset -= 1;
    }
    offset
}

#[cfg(test)]
mod tests {

    #[test]
    fn selectable_text_actions_include_copy_shortcuts() {
        let source = include_str!("selectable_text.rs");
        assert!(source.contains("SelectableTextSelectAll"));
        assert!(source.contains("SelectableTextCopy"));
        assert!(source.contains("KeyBinding::new(\"ctrl-a\""));
        assert!(source.contains("KeyBinding::new(\"cmd-a\""));
        assert!(source.contains("KeyBinding::new(\"ctrl-c\""));
        assert!(source.contains("KeyBinding::new(\"cmd-c\""));
        assert!(source.contains("fn set_select_all"));
        assert!(source.contains("fn select_all"));
        assert!(source.contains("fn on_key_down"));
        assert!(source.contains("event.keystroke.modifiers.control"));
        assert!(source.contains("event.keystroke.modifiers.platform"));
        assert!(source.contains("event.click_count == 2"));
        assert!(source.contains("window.capture_pointer"));
        assert!(source.contains("phase.capture()"));
    }
}
