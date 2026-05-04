use gpui::{prelude::*, px, App, Hsla, Rgba, Render, Window, Context, MouseButton, Focusable, FocusHandle, KeyBinding};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba { r: r as f32 / 255.0, g: g as f32 / 255.0, b: b as f32 / 255.0, a }.into()
}

gpui::actions!(switch, [SwitchToggle]);

pub struct Switch {
    checked: bool,
    disabled: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self { checked, disabled: false, focus_handle: cx.focus_handle(), on_change: None }
    }

    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
    pub fn on_change(mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb)); self
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("space", SwitchToggle, None),
            KeyBinding::new("enter", SwitchToggle, None),
        ]);
    }

    fn toggle(&mut self, _: &SwitchToggle, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(self.checked, window, cx);
            }
        }
    }
}

impl Focusable for Switch {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for Switch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let w = 40.0; let h = 22.0; let thumb_sz = 16.0;
        let thumb_offset = if self.checked { w - thumb_sz - 3.0 } else { 3.0 };

        let thumb_color = if self.disabled {
            theme.neutral.text_disabled
        } else {
            rgba(255, 255, 255, 1.0)
        };
        let track_color = if self.disabled {
            theme.neutral.hover
        } else if self.checked {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        let mut el = gpui::div()
            .flex_none().w(px(w)).h(px(h)).rounded(px(h / 2.0))
            .bg(track_color)
            .on_action(cx.listener(Self::toggle));

        if focused && !self.disabled {
            el = el.border_2().border_color(theme.primary.base.opacity(0.5));
        } else {
            el = el.border_0();
        }

        if !self.disabled { 
            el = el.cursor_pointer().track_focus(&self.focus_handle);
            el = el.on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                window.focus(&this.focus_handle, cx);
            }));
            el = el.on_mouse_up(MouseButton::Left, cx.listener(|this, _, window, cx| {
                this.toggle(&SwitchToggle, window, cx);
            }));
        } else { 
            el = el.cursor_not_allowed(); 
        }

        el.child(
                gpui::div()
                    .absolute().left(px(thumb_offset)).top(px((h - thumb_sz) / 2.0))
                    .w(px(thumb_sz)).h(px(thumb_sz)).rounded(px(thumb_sz / 2.0))
                    .bg(thumb_color)
            )
    }
}
