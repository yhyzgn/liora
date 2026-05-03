use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    prelude::*, px, App, Bounds, Context, Element, ElementId, ElementInputHandler, Entity,
    EntityInputHandler, FocusHandle, Focusable, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, LayoutId,     MouseButton, MouseDownEvent, MouseUpEvent,
    Pixels, Point, Render, Rgba, SharedString, ShapedLine, Size, Style, TextRun,
    UTF16Selection, UnderlineStyle, Window, actions, KeyBinding, fill, point, size,
};
use std::ops::Range;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
}

actions!(input, [Backspace, Delete, Left, Right, Home, End, SelectAll]);

pub struct Input {
    value: SharedString,
    placeholder: SharedString,
    disabled: bool,
    clearable: bool,
    icon_prefix: Option<IconName>,
    icon_suffix: Option<IconName>,
    focus_handle: FocusHandle,
    selected_range: Range<usize>,
    selection_reversed: bool,
    marked_range: Option<Range<usize>>,
    last_layout: Option<ShapedLine>,
    last_bounds: Option<Bounds<Pixels>>,
}

impl Input {
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            value: value.into(), placeholder: SharedString::default(), disabled: false,
            clearable: false, icon_prefix: None, icon_suffix: None,
            focus_handle: cx.focus_handle(), selected_range: 0..0, selection_reversed: false,
            marked_range: None, last_layout: None, last_bounds: None,
        }
    }
    pub fn placeholder(mut self, p: impl Into<SharedString>) -> Self { self.placeholder = p.into(); self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn clearable(mut self, c: bool) -> Self { self.clearable = c; self }
    pub fn icon_prefix(mut self, icon: IconName) -> Self { self.icon_prefix = Some(icon); self }
    pub fn icon_suffix(mut self, icon: IconName) -> Self { self.icon_suffix = Some(icon); self }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("backspace", Backspace, None),  KeyBinding::new("delete", Delete, None),
            KeyBinding::new("left", Left, None),            KeyBinding::new("right", Right, None),
            KeyBinding::new("home", Home, None),            KeyBinding::new("end", End, None),
            KeyBinding::new("cmd-a", SelectAll, None),
        ]);
    }

    fn clear(&mut self, cx: &mut Context<Self>) {
        self.value = SharedString::default();
        self.selected_range = 0..0; cx.notify();
    }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed { self.selected_range.start } else { self.selected_range.end }
    }

    fn prev_char(&self, offset: usize) -> usize {
        if offset == 0 { return 0; }
        let mut p = offset - 1;
        while p > 0 && !self.value.is_char_boundary(p) { p -= 1; }
        p
    }
    fn next_char(&self, offset: usize) -> usize {
        if offset >= self.value.len() { return self.value.len(); }
        let mut n = offset + 1;
        while n < self.value.len() && !self.value.is_char_boundary(n) { n += 1; }
        n
    }
    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset; cx.notify();
    }
    fn select_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        if self.selection_reversed { self.selected_range.start = offset }
        else { self.selected_range.end = offset }
        if self.selected_range.end < self.selected_range.start {
            self.selection_reversed = !self.selection_reversed;
            self.selected_range = self.selected_range.end..self.selected_range.start;
        }
        cx.notify()
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let p = self.prev_char(self.cursor_offset());
            if p == self.cursor_offset() { return; }
            self.select_to(p, cx);
        }
        self.internal_replace("", cx);
    }
    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let n = self.next_char(self.cursor_offset());
            if n == self.cursor_offset() { return; }
            self.select_to(n, cx);
        }
        self.internal_replace("", cx);
    }
    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) { self.move_to(self.prev_char(self.cursor_offset()), cx); }
    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) { self.move_to(self.next_char(self.cursor_offset()), cx); }
    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) { self.move_to(0, cx); }
    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) { self.move_to(self.value.len(), cx); }
    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.value.len(); cx.notify();
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        window.focus(&self.focus_handle, cx);
        if self.value.is_empty() {
            self.move_to(0, cx);
            return;
        }
        let idx = if let (Some(bounds), Some(line)) = (self.last_bounds.as_ref(), self.last_layout.as_ref()) {
            let x = event.position.x - bounds.left();
            line.index_for_x(x).unwrap_or(self.value.len())
        } else {
            self.value.len()
        };
        self.move_to(idx, cx);
    }

    fn internal_replace(&mut self, new_text: &str, cx: &mut Context<Self>) {
        let range = self.selected_range.clone();
        let mut v = self.value.to_string();
        v.replace_range(range, new_text);
        self.value = SharedString::from(v);
        let pos = self.selected_range.start + new_text.len();
        self.selected_range = pos..pos;
        cx.notify();
    }
}

impl Focusable for Input {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl EntityInputHandler for Input {
    fn text_for_range(&mut self, range_utf16: Range<usize>, _: &mut Option<Range<usize>>, _: &mut Window, _: &mut Context<Self>) -> Option<String> {
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        if start <= self.value.len() && end <= self.value.len() {
            Some(self.value[start..end].to_string())
        } else { None }
    }

    fn selected_text_range(&mut self, _: bool, _: &mut Window, _: &mut Context<Self>) -> Option<UTF16Selection> {
        Some(UTF16Selection {
            range: self.offset_to_utf16(self.selected_range.start)..self.offset_to_utf16(self.selected_range.end),
            reversed: self.selection_reversed,
        })
    }

    fn marked_text_range(&self, _: &mut Window, _: &mut Context<Self>) -> Option<Range<usize>> {
        self.marked_range.as_ref().map(|r| self.offset_to_utf16(r.start)..self.offset_to_utf16(r.end))
    }

    fn unmark_text(&mut self, _: &mut Window, _: &mut Context<Self>) { self.marked_range = None; }

    fn replace_text_in_range(&mut self, range_utf16: Option<Range<usize>>, new_text: &str, _: &mut Window, cx: &mut Context<Self>) {
        let range = range_utf16
            .map(|r| self.offset_from_utf16(r.start)..self.offset_from_utf16(r.end))
            .or_else(|| self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let mut v = self.value.to_string();
        v.replace_range(range.clone(), new_text);
        self.value = SharedString::from(v);
        self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        self.marked_range = None;
        cx.notify();
    }

    fn replace_and_mark_text_in_range(&mut self, range_utf16: Option<Range<usize>>, new_text: &str, new_selected: Option<Range<usize>>, _: &mut Window, cx: &mut Context<Self>) {
        let range = range_utf16
            .map(|r| self.offset_from_utf16(r.start)..self.offset_from_utf16(r.end))
            .or(self.marked_range.clone())
            .unwrap_or(self.selected_range.clone());
        let mut v = self.value.to_string();
        v.replace_range(range.clone(), new_text);
        self.value = SharedString::from(v);
        if !new_text.is_empty() {
            self.marked_range = Some(range.start..range.start + new_text.len());
        } else { self.marked_range = None; }
        if let Some(sel) = new_selected {
            self.selected_range = range.start + sel.start..range.start + sel.end;
        } else {
            self.selected_range = range.start + new_text.len()..range.start + new_text.len();
        }
        cx.notify();
    }

    fn bounds_for_range(&mut self, range_utf16: Range<usize>, bounds: Bounds<Pixels>, _: &mut Window, _: &mut Context<Self>) -> Option<Bounds<Pixels>> {
        let line = self.last_layout.as_ref()?;
        let start = self.offset_from_utf16(range_utf16.start);
        let end = self.offset_from_utf16(range_utf16.end);
        Some(Bounds::from_corners(
            point(bounds.left() + line.x_for_index(start), bounds.top()),
            point(bounds.left() + line.x_for_index(end), bounds.bottom()),
        ))
    }

    fn character_index_for_point(&mut self, pt: Point<Pixels>, _: &mut Window, _: &mut Context<Self>) -> Option<usize> {
        let line_pt = self.last_bounds?.localize(&pt)?;
        let line = self.last_layout.as_ref()?;
        let idx = line.index_for_x(pt.x - line_pt.x)?;
        Some(self.offset_to_utf16(idx))
    }
}

impl Input {
    fn offset_to_utf16(&self, offset: usize) -> usize {
        self.value[..offset].chars().map(|c| c.len_utf16()).sum()
    }
    fn offset_from_utf16(&self, target: usize) -> usize {
        let mut utf8 = 0; let mut utf16 = 0;
        for c in self.value.chars() {
            if utf16 >= target { break; }
            utf16 += c.len_utf16();
            utf8 += c.len_utf8();
        }
        utf8
    }
    fn text_for_display(&self) -> SharedString {
        if self.value.is_empty() { self.placeholder.clone() } else { self.value.clone() }
    }
}

// ── Custom Element for IME + cursor ──

struct InputElement {
    input: Entity<Input>,
    disabled: bool,
}

struct InputPrepaint {
    line: Option<ShapedLine>,
    cursor: Option<gpui::PaintQuad>,
    selection: Option<gpui::PaintQuad>,
}

impl IntoElement for InputElement {
    type Element = Self;
    fn into_element(self) -> Self::Element { self }
}

impl Element for InputElement {
    type RequestLayoutState = ();
    type PrepaintState = InputPrepaint;

    fn id(&self) -> Option<ElementId> { None }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

    fn request_layout(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, ()) {
        let mut style = Style::default();
        style.size.width = gpui::relative(1.).into();
        style.size.height = window.line_height().into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, bounds: Bounds<Pixels>, _: &mut (), window: &mut Window, cx: &mut App) -> InputPrepaint {
        let input = self.input.read(cx);
        let text = input.text_for_display();
        let cursor = input.cursor_offset();
        let selected = input.selected_range.clone();
        let style = window.text_style();
        let theme = &cx.global::<Config>().theme;
        let text_c = if self.disabled { theme.neutral.text_disabled } else { style.color };
        let (display, text_color) = if input.value.is_empty() {
            (input.placeholder.clone(), rgba(0,0,0,0.3))
        } else {
            (text, text_c)
        };
        let run = TextRun { len: display.len(), font: style.font(), color: text_color, background_color: None, underline: None, strikethrough: None };
        let runs = if let Some(ref marked) = input.marked_range {
            vec![
                TextRun { len: marked.start, ..run.clone() },
                TextRun { len: marked.end - marked.start, underline: Some(UnderlineStyle { color: Some(run.color), thickness: px(1.0), wavy: false }), ..run.clone() },
                TextRun { len: display.len() - marked.end, ..run },
            ].into_iter().filter(|r| r.len > 0).collect()
        } else { vec![run] };

        let font_size = style.font_size.to_pixels(window.rem_size());
        let line = window.text_system().shape_line(display, font_size, &runs, None);
        let cursor_pos = line.x_for_index(cursor);
        let cursor_h = (bounds.bottom() - bounds.top()) * 0.65;
        let cursor_top = bounds.top() + (bounds.bottom() - bounds.top() - cursor_h) / 2.0;
        let (selection, cursor_quad) = if selected.is_empty() {
            (None, Some(fill(Bounds::new(point(bounds.left() + cursor_pos, cursor_top), size(px(2.), cursor_h)), theme.primary.base)))
        } else {
            (Some(fill(Bounds::from_corners(point(bounds.left() + line.x_for_index(selected.start), bounds.top()), point(bounds.left() + line.x_for_index(selected.end), bounds.bottom())), gpui::rgba(0x3311ff30))), None)
        };
        InputPrepaint { line: Some(line), cursor: cursor_quad, selection }
    }

    fn paint(&mut self, _: Option<&GlobalElementId>, _: Option<&InspectorElementId>, bounds: Bounds<Pixels>, _: &mut (), prepaint: &mut InputPrepaint, window: &mut Window, cx: &mut App) {
        let focus_handle = self.input.read(cx).focus_handle.clone();
        window.handle_input(&focus_handle, ElementInputHandler::new(bounds, self.input.clone()), cx);
        if let Some(s) = prepaint.selection.take() { window.paint_quad(s); }
        let line = prepaint.line.take().unwrap();
        line.paint(bounds.origin, window.line_height(), gpui::TextAlign::Left, None, window, cx).unwrap();
        if focus_handle.is_focused(window) {
            if let Some(c) = prepaint.cursor.take() { window.paint_quad(c); }
        }
        self.input.update(cx, |input, _| { input.last_layout = Some(line); input.last_bounds = Some(bounds); });
    }
}

// ── View render ──

impl Render for Input {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let h = 34.0; let icon_sz = 16.0;
        let focused = self.focus_handle(cx).is_focused(_window);
        let (bg, border_c) = if self.disabled {
            (theme.neutral.hover, theme.neutral.border)
        } else if focused {
            (theme.neutral.card, theme.primary.base)
        } else {
            (theme.neutral.card, theme.neutral.border)
        };
        let fh = self.focus_handle(cx);

        let mut row = gpui::div()
            .flex().flex_row().items_center().gap_2()
            .h(px(h)).px(px(12.0)).rounded(px(theme.radius.md))
            .bg(bg).border_1().border_color(border_c).text_size(px(theme.font_size.md));

        if !self.disabled {
            row = row.track_focus(&fh);
            row = row.cursor_text();
        } else {
            row = row.cursor_not_allowed();
        }

        if !self.disabled {
            row = row
                .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::delete))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::right))
                .on_action(cx.listener(Self::home))
                .on_action(cx.listener(Self::end))
                .on_action(cx.listener(Self::select_all));
        }

        if let Some(icon) = self.icon_prefix {
            row = row.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        row = row.child(InputElement {
            input: cx.entity(),
            disabled: self.disabled,
        });

        if self.clearable && !self.value.is_empty() && !self.disabled {
            row = row.child(
                gpui::div().cursor_pointer().flex_none()
                    .child(Icon::new(IconName::X).size(px(14.0)).color(theme.neutral.icon))
                    .on_mouse_up(MouseButton::Left, cx.listener(move |this: &mut Self, _: &MouseUpEvent, _: &mut Window, cx: &mut Context<Self>| {
                        this.clear(cx);
                    }))
            );
        }

        if let Some(icon) = self.icon_suffix {
            row = row.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        row
    }
}

use gpui::hsla;
