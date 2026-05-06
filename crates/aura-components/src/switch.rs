use gpui::{
    App, Context, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, Render, Rgba, Window,
    prelude::*, px,
};

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
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
        Self {
            checked,
            disabled: false,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn on_change(mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
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
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Switch {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let w = 40.0;
        let h = 22.0;
        let thumb_sz = 16.0;
        let thumb_offset = if self.checked {
            w - thumb_sz - 3.0
        } else {
            3.0
        };

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

        // Focus ring color
        let focus_color = if self.checked {
            theme.primary.base.opacity(0.5)
        } else {
            theme.neutral.border.opacity(0.5)
        };

        let mut track = gpui::div()
            .relative()
            .flex_none()
            .w(px(w))
            .h(px(h))
            .rounded(px(h / 2.0))
            .bg(track_color)
            .child(
                gpui::div()
                    .absolute()
                    .top(px((h - thumb_sz) / 2.0))
                    .left(px(thumb_offset))
                    .w(px(thumb_sz))
                    .h(px(thumb_sz))
                    .rounded(px(thumb_sz / 2.0))
                    .bg(thumb_color),
            );

        if !self.disabled {
            track = track.on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle(&SwitchToggle, window, cx);
                }),
            );
        }

        let mut el = gpui::div().p(px(2.0)).child(track);

        if focused && !self.disabled {
            el = el
                .rounded(px((h + 4.0) / 2.0))
                .border_2()
                .border_color(focus_color);
        } else {
            el = el.border_2().border_color(rgba(0, 0, 0, 0.0));
        }

        if !self.disabled {
            el = el.cursor_pointer().track_focus(&self.focus_handle);
            el = el.on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    window.focus(&this.focus_handle, cx);
                }),
            );
        } else {
            el = el.cursor_not_allowed();
        }

        el.on_action(cx.listener(Self::toggle))
    }
}
