//! Switch module.
//!
//! This public module implements the Liora binary switch/toggle component. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::gpui_compat::element_id;
use crate::motion::{MotionDuration, MotionEasing, motion_animation, slide_snap};
use gpui::{
    AnimationExt, App, Context, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, Render,
    Rgba, Window, prelude::*, px,
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

gpui::actions!(
    switch,
    [
        #[doc = "Keyboard action that toggles the focused switch."]
        SwitchToggle
    ]
);

/// Fluent native GPUI component for rendering Liora switch.
pub struct Switch {
    checked: bool,
    thumb_from_checked: bool,
    disabled: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    /// Creates `Switch` initialized from the supplied checked.
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self {
            checked,
            thumb_from_checked: checked,
            disabled: false,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(&mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Box::new(cb));
    }

    /// Returns whether checked is currently enabled or available.
    pub fn checked(&self) -> bool {
        self.checked
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("space", SwitchToggle, None),
            KeyBinding::new("enter", SwitchToggle, None),
        ]);
    }

    fn toggle(&mut self, _: &SwitchToggle, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.thumb_from_checked = self.checked;
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
        let theme = &cx.global::<liora_core::Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let w = 40.0;
        let h = 22.0;
        let thumb_sz = 16.0;
        let thumb_start = 3.0;
        let thumb_end = w - thumb_sz - 3.0;
        let checked = self.checked;
        let from_checked = self.thumb_from_checked;
        let from_left = if from_checked { thumb_end } else { thumb_start };
        let to_left = if checked { thumb_end } else { thumb_start };
        let thumb_motion_id = format!(
            "liora-switch-thumb-motion:{:?}:{from_checked}:{checked}",
            self.focus_handle
        );

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
                    .w(px(thumb_sz))
                    .h(px(thumb_sz))
                    .rounded(px(thumb_sz / 2.0))
                    .bg(thumb_color)
                    .with_animation(
                        element_id(thumb_motion_id),
                        motion_animation(MotionDuration::Normal, MotionEasing::Linear),
                        move |thumb, delta| {
                            let left = if (to_left - from_left).abs() < f32::EPSILON {
                                to_left
                            } else {
                                slide_snap(from_left, to_left, delta)
                            };

                            thumb.left(px(left))
                        },
                    ),
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
                cx.listener(|this, _, window, _cx| {
                    window.focus(&this.focus_handle);
                }),
            );
        } else {
            el = el.cursor_not_allowed();
        }

        el.on_action(cx.listener(Self::toggle))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn switch_thumb_uses_elastic_motion() {
        let source = include_str!("switch.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("with_animation("));
        assert!(source.contains("MotionEasing::Linear"));
        assert!(source.contains("slide_snap(from_left, to_left, delta)"));
        assert!(source.contains("thumb_from_checked"));
    }
}
