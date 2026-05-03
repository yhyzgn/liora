use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    prelude::*, px, App, FocusHandle, Focusable, Hsla, Rgba, Render, SharedString,
    Window, Context, MouseButton, MouseDownEvent, MouseUpEvent, KeyDownEvent,
    actions, KeyBinding,
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
    cursor_visible: bool,
}

impl Input {
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            value: value.into(), placeholder: SharedString::default(), disabled: false,
            clearable: false, icon_prefix: None, icon_suffix: None,
            focus_handle: cx.focus_handle(), selected_range: 0..0, selection_reversed: false,
            cursor_visible: true,
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

    fn clear(&mut self, cx: &mut Context<Self>) { self.value = SharedString::default(); cx.notify(); }

    fn cursor_offset(&self) -> usize {
        if self.selection_reversed { self.selected_range.start } else { self.selected_range.end }
    }

    fn prev_char_boundary(&self, offset: usize) -> usize {
        if offset == 0 { return 0; }
        let mut prev = offset - 1;
        while prev > 0 && !self.value.is_char_boundary(prev) { prev -= 1; }
        prev
    }

    fn next_char_boundary(&self, offset: usize) -> usize {
        if offset >= self.value.len() { return self.value.len(); }
        let mut next = offset + 1;
        while next < self.value.len() && !self.value.is_char_boundary(next) { next += 1; }
        next
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset; self.cursor_visible = true; cx.notify()
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let prev = self.prev_char_boundary(self.cursor_offset());
            if prev == self.cursor_offset() { return; }
            self.select_to(prev, cx);
        }
        self.replace_text("", cx);
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let next = self.next_char_boundary(self.cursor_offset());
            if next == self.cursor_offset() { return; }
            self.select_to(next, cx);
        }
        self.replace_text("", cx);
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.prev_char_boundary(self.cursor_offset()), cx);
    }
    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.next_char_boundary(self.cursor_offset()), cx);
    }
    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) { self.move_to(0, cx); }
    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) { self.move_to(self.value.len(), cx); }
    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.value.len(); self.cursor_visible = true; cx.notify();
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

    fn replace_text(&mut self, new_text: &str, cx: &mut Context<Self>) {
        let range = self.selected_range.clone();
        let mut v = self.value.to_string();
        v.replace_range(range, new_text);
        self.value = SharedString::from(v);
        let new_pos = self.selected_range.start + new_text.len();
        self.selected_range = new_pos..new_pos;
        self.cursor_visible = true;
        cx.notify();
    }

    fn on_key_down(&mut self, event: &KeyDownEvent, _: &mut Window, cx: &mut Context<Self>) {
        if let Some(text) = &event.keystroke.key_char {
            self.replace_text(text, cx);
        }
    }
}

impl Focusable for Input {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for Input {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let h = 34.0; let icon_sz = 16.0;
        let focused = self.focus_handle(cx).is_focused(_window);

        let (bg, text_c, border_c) = if self.disabled {
            (theme.neutral.hover, theme.neutral.text_disabled, theme.neutral.border)
        } else if focused {
            (theme.neutral.card, theme.neutral.text_1, theme.primary.base)
        } else {
            (theme.neutral.card, theme.neutral.text_1, theme.neutral.border)
        };

        let is_empty = self.value.is_empty();
        let display = if is_empty { self.placeholder.clone() } else { self.value.clone() };
        let ph_color = theme.neutral.text_3;
        let fh = self.focus_handle(cx);

        let mut row = gpui::div()
            .flex().flex_row().items_center().gap_2()
            .h(px(h)).px(px(12.0)).rounded(px(theme.radius.md))
            .bg(bg).border_1().border_color(border_c).text_size(px(theme.font_size.md))
            .track_focus(&fh);

        if !self.disabled { row = row.cursor_text(); }
        else { row = row.cursor_not_allowed(); }

        if !self.disabled {
            let fh2 = fh.clone();
            row = row
                .on_mouse_down(MouseButton::Left, move |_, window, cx| { window.focus(&fh2, cx); })
                .on_key_down(cx.listener(Self::on_key_down))
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

        // Text + cursor
        let show_cursor = focused && self.cursor_visible && self.selected_range.is_empty();
        let cursor_w = 1.5;
        let cursor_color = theme.primary.base;

        if is_empty {
            row = row.child(
                gpui::div().flex_1().flex().items_center().relative().h_full().text_color(ph_color)
                    .child(display)
            );
        } else {
            let offset = self.cursor_offset();
            // Split text at cursor position for layout
            let before = &self.value[..offset];
            let after = &self.value[offset..];

            let mut text_div = gpui::div().flex_1().flex().flex_row().items_center().relative().h_full().text_color(text_c);

            text_div = text_div.child(gpui::div().child(before.to_string()));

            if show_cursor {
                text_div = text_div.child(
                    gpui::div().flex_none().w(px(cursor_w)).h(px(h * 0.6)).bg(cursor_color)
                );
            }

            text_div = text_div.child(gpui::div().child(after.to_string()));
            row = row.child(text_div);
        }

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
