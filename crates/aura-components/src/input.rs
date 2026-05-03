use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    prelude::*, px, App, FocusHandle, Focusable, Hsla, Rgba, Render, SharedString,
    Window, Context, MouseButton, MouseUpEvent, KeyDownEvent, Bounds, Pixels,
    actions,
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
}

impl Input {
    pub fn new(value: impl Into<SharedString>, cx: &mut Context<Self>) -> Self {
        Self {
            value: value.into(), placeholder: SharedString::default(), disabled: false,
            clearable: false, icon_prefix: None, icon_suffix: None,
            focus_handle: cx.focus_handle(), selected_range: 0..0, selection_reversed: false,
        }
    }
    pub fn placeholder(mut self, p: impl Into<SharedString>) -> Self { self.placeholder = p.into(); self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn clearable(mut self, c: bool) -> Self { self.clearable = c; self }
    pub fn icon_prefix(mut self, icon: IconName) -> Self { self.icon_prefix = Some(icon); self }
    pub fn icon_suffix(mut self, icon: IconName) -> Self { self.icon_suffix = Some(icon); self }

    fn clear(&mut self, cx: &mut Context<Self>) { self.value = SharedString::default(); cx.notify(); }
    fn cursor_offset(&self) -> usize {
        if self.selection_reversed { self.selected_range.start } else { self.selected_range.end }
    }

    fn move_to(&mut self, offset: usize, cx: &mut Context<Self>) {
        self.selected_range = offset..offset; cx.notify()
    }

    fn backspace(&mut self, _: &Backspace, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let prev = self.cursor_offset().saturating_sub(1);
            self.select_to(prev, cx);
        }
        self.replace_text("", cx);
    }

    fn delete(&mut self, _: &Delete, _: &mut Window, cx: &mut Context<Self>) {
        if self.selected_range.is_empty() {
            let next = (self.cursor_offset() + 1).min(self.value.len());
            self.select_to(next, cx);
        }
        self.replace_text("", cx);
    }

    fn left(&mut self, _: &Left, _: &mut Window, cx: &mut Context<Self>) {
        self.move_to(self.cursor_offset().saturating_sub(1), cx);
    }

    fn right(&mut self, _: &Right, _: &mut Window, cx: &mut Context<Self>) {
        let off = (self.cursor_offset() + 1).min(self.value.len());
        self.move_to(off, cx);
    }

    fn home(&mut self, _: &Home, _: &mut Window, cx: &mut Context<Self>) { self.move_to(0, cx); }
    fn end(&mut self, _: &End, _: &mut Window, cx: &mut Context<Self>) { self.move_to(self.value.len(), cx); }
    fn select_all(&mut self, _: &SelectAll, _: &mut Window, cx: &mut Context<Self>) {
        self.selected_range = 0..self.value.len(); cx.notify();
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

        let (bg, text_c, border_c) = if self.disabled {
            (theme.neutral.hover, theme.neutral.text_disabled, theme.neutral.border)
        } else {
            (theme.neutral.card, theme.neutral.text_1, theme.neutral.border)
        };

        let display = if self.value.is_empty() { self.placeholder.clone() } else { self.value.clone() };
        let is_placeholder = self.value.is_empty();
        let ph_color = theme.neutral.text_3;

        let mut row = gpui::div()
            .flex().flex_row().items_center().gap_2()
            .h(px(h)).px(px(12.0)).rounded(px(theme.radius.md))
            .bg(bg).border_1().border_color(border_c).text_size(px(theme.font_size.md))
            .track_focus(&self.focus_handle(cx));

        if !self.disabled { row = row.cursor_text(); }
        else { row = row.cursor_not_allowed(); }

        if let Some(icon) = self.icon_prefix {
            row = row.child(Icon::new(icon).size(px(icon_sz)).color(theme.neutral.icon));
        }

        row = row.child(
            gpui::div().flex_1().h_full().flex().items_center()
                .text_color(if is_placeholder { ph_color } else { text_c })
                .child(display)
                .on_key_down(cx.listener(Self::on_key_down))
                .on_action(cx.listener(Self::backspace))
                .on_action(cx.listener(Self::delete))
                .on_action(cx.listener(Self::left))
                .on_action(cx.listener(Self::right))
                .on_action(cx.listener(Self::home))
                .on_action(cx.listener(Self::end))
                .on_action(cx.listener(Self::select_all))
        );

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
