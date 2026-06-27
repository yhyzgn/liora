//! Radio module.
//!
//! This public module implements the Liora single radio option component. It keeps the reusable
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
use crate::gpui_compat::focus_window;
use crate::motion::pop_in;
use gpui::{
    App, Context, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, Render, Rgba,
    SharedString, Window, prelude::*, px,
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
    radio,
    [
        #[doc = "Keyboard action that selects the focused radio option."]
        RadioSelect
    ]
);

/// Fluent native GPUI component for rendering Liora radio.
pub struct Radio {
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Radio {
    /// Creates `Radio` initialized from the supplied checked.
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self {
            checked,
            disabled: false,
            label: None,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    /// Returns the stable user-facing label for this value.
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = Some(text.into());
        self
    }
    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("space", RadioSelect, None),
            KeyBinding::new("enter", RadioSelect, None),
        ]);
    }

    fn select(&mut self, _: &RadioSelect, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled && !self.checked {
            self.checked = true;
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(window, cx);
            }
        }
    }
}

impl Focusable for Radio {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Radio {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let sz = 16.0;
        let inner_sz = 8.0;

        let (border_color, dot_color) = if self.disabled {
            (theme.neutral.border, theme.neutral.text_disabled)
        } else if self.checked {
            (theme.primary.base, theme.primary.base)
        } else {
            (
                if focused {
                    theme.primary.base
                } else {
                    theme.neutral.border
                },
                rgba(0, 0, 0, 0.0),
            )
        };

        let mut row = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .on_action(cx.listener(Self::select));

        if !self.disabled {
            row = row.cursor_pointer().track_focus(&self.focus_handle);
            row = row.on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, _cx| {
                    focus_window(window, &this.focus_handle, _cx);
                }),
            );
            row = row.on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.select(&RadioSelect, window, cx);
                }),
            );
        } else {
            row = row.cursor_not_allowed();
        }

        let dot = || {
            gpui::div()
                .w(px(inner_sz))
                .h(px(inner_sz))
                .rounded(px(inner_sz / 2.0))
                .bg(dot_color)
        };

        let circle = gpui::div()
            .flex_none()
            .w(px(sz))
            .h(px(sz))
            .rounded(px(sz / 2.0))
            .border_1()
            .border_color(border_color)
            .flex()
            .items_center()
            .justify_center()
            .when(self.checked, |s| {
                s.child(pop_in(
                    element_id(format!(
                        "liora-radio-dot-motion-{}",
                        cx.entity().entity_id()
                    )),
                    dot(),
                ))
            })
            .when(!self.checked, |s| s.child(dot()));

        row = row.child(circle);

        if let Some(ref label) = self.label {
            row = row.child(
                gpui::div()
                    .text_size(px(theme.font_size.md))
                    .text_color(theme.neutral.text_1)
                    .child(label.clone()),
            );
        }

        row
    }
}
