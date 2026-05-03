use aura_core::Config;
use gpui::{prelude::*, px, App, Hsla, Rgba, Render, Window, Context, MouseButton, MouseUpEvent, Focusable, FocusHandle, SharedString, Entity};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
}

pub struct RadioGroup {
    selected: usize,
    disabled: bool,
    options: Vec<SharedString>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl RadioGroup {
    pub fn new(options: Vec<impl Into<SharedString>>, selected: usize, cx: &mut Context<Self>) -> Self {
        Self {
            selected, disabled: false,
            options: options.into_iter().map(|o| o.into()).collect(),
            focus_handle: cx.focus_handle(), on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb)); self
    }
}

impl Focusable for RadioGroup {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for RadioGroup {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let sz = 16.0; let inner_sz = 8.0;

        let mut col = gpui::div().flex().flex_col().gap_2();

        for (idx, label) in self.options.iter().enumerate() {
            let checked = idx == self.selected;
            let (border_color, dot_color) = if self.disabled {
                (theme.neutral.border, theme.neutral.text_disabled)
            } else if checked {
                (theme.primary.base, theme.primary.base)
            } else {
                (theme.neutral.border, rgba(0,0,0,0.0))
            };

            let label_text = label.clone();
            let mut row = gpui::div().flex().flex_row().items_center().gap_2();

            if !self.disabled { row = row.cursor_pointer(); }
            else { row = row.cursor_not_allowed(); }

            let circle = gpui::div()
                .flex_none().w(px(sz)).h(px(sz)).rounded(px(sz / 2.0))
                .border_1().border_color(border_color)
                .flex().items_center().justify_center()
                .child(gpui::div().w(px(inner_sz)).h(px(inner_sz)).rounded(px(inner_sz / 2.0)).bg(dot_color));

            row = row.child(circle);
            row = row.child(gpui::div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_1).child(label_text));

            if !self.disabled {
                row = row.on_mouse_up(MouseButton::Left, cx.listener(move |this: &mut Self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>| {
                    if idx != this.selected {
                        this.selected = idx;
                        cx.notify();
                        if let Some(ref cb) = this.on_change { cb(idx, window, cx); }
                    }
                }));
            }

            col = col.child(row);
        }

        col
    }
}
