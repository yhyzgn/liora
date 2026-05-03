use gpui::{prelude::*, px, App, Hsla, Rgba, Render, Window, Context, MouseButton, MouseUpEvent, Focusable, FocusHandle, SharedString};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
}

pub struct Radio {
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Radio {
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self { checked, disabled: false, label: None, focus_handle: cx.focus_handle(), on_change: None }
    }

    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn label(mut self, text: impl Into<SharedString>) -> Self { self.label = Some(text.into()); self }
    pub fn on_change(mut self, cb: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb)); self
    }

    fn select(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && !self.checked {
            self.checked = true;
            cx.notify();
            if let Some(ref cb) = self.on_change { cb(window, cx); }
        }
    }
}

impl Focusable for Radio {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for Radio {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let sz = 16.0;
        let inner_sz = 8.0;

        let (border_color, dot_color) = if self.disabled {
            (theme.neutral.border, theme.neutral.text_disabled)
        } else if self.checked {
            (theme.primary.base, theme.primary.base)
        } else {
            (theme.neutral.border, rgba(0,0,0,0.0))
        };

        let mut row = gpui::div().flex().flex_row().items_center().gap_2();

        if !self.disabled { row = row.cursor_pointer(); }
        else { row = row.cursor_not_allowed(); }

        if !self.disabled {
            row = row.on_mouse_up(MouseButton::Left, cx.listener(move |this: &mut Self, _: &MouseUpEvent, window: &mut Window, cx: &mut Context<Self>| {
                this.select(window, cx);
            }));
        }

        let circle = gpui::div()
            .flex_none().w(px(sz)).h(px(sz)).rounded(px(sz / 2.0))
            .border_1().border_color(border_color)
            .flex().items_center().justify_center()
            .child(
                gpui::div()
                    .w(px(inner_sz)).h(px(inner_sz)).rounded(px(inner_sz / 2.0))
                    .bg(dot_color)
            );

        row = row.child(circle);

        if let Some(ref label) = self.label {
            row = row.child(gpui::div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_1).child(label.clone()));
        }

        row
    }
}
