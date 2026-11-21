//! Switch module.
//!
//! This public module implements the Liora binary switch/toggle component. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::motion::MotionSpring;
use gpui::{
    AnimationExt, App, Context, ElementId, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton,
    Render, Rgba, Window, prelude::*, px,
};
use liora_core::LocalizedText;

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
    id: Option<gpui::SharedString>,
    checked: bool,
    disabled: bool,
    aria_label: Option<LocalizedText>,
    pointer_armed: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Switch {
    /// Creates `Switch` initialized from the supplied checked.
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self {
            id: None,
            checked,
            disabled: false,
            aria_label: None,
            pointer_armed: false,
            focus_handle: cx.focus_handle().tab_stop(true),
            on_change: None,
        }
    }
    /// Assigns a stable root element ID for automation and interaction isolation.
    pub fn id(mut self, id: impl Into<gpui::SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Sets the accessible name without changing the switch ID or appearance.
    pub fn aria_label(mut self, label: impl Into<LocalizedText>) -> Self {
        self.aria_label = Some(label.into());
        self
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self.pointer_armed = false;
        self.focus_handle = self.focus_handle.tab_stop(!d);
        self
    }
    /// Updates disabled state while preserving the component and focus identities.
    pub fn set_disabled(&mut self, disabled: bool, cx: &mut Context<Self>) {
        self.disabled = disabled;
        self.pointer_armed = false;
        self.focus_handle = self.focus_handle.clone().tab_stop(!disabled);
        cx.notify();
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

    /// Synchronizes checked state without invoking the change callback.
    pub fn set_checked(&mut self, checked: bool, cx: &mut Context<Self>) {
        self.checked = checked;
        cx.notify();
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
        let to_left = if checked { thumb_end } else { thumb_start };
        let thumb_motion_id = ("liora-switch-thumb-motion", cx.entity().entity_id());

        let thumb_color = if self.disabled {
            theme.neutral.text_disabled
        } else {
            theme.neutral.inverted
        };
        let track_color = if self.disabled {
            theme.neutral.hover
        } else if self.checked {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        let track = gpui::div()
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
                    .with_spring(
                        thumb_motion_id,
                        MotionSpring::Snappy.animation_to(px(to_left)),
                        |thumb, left| thumb.left(left),
                    ),
            );

        let id = self.id.clone().map(ElementId::from).unwrap_or_else(|| ("liora-switch", cx.entity().entity_id()).into());
        let name = self.aria_label.as_ref().map(|label| label.resolve(cx));
        let mut el = gpui::div().id(id)
            .role(gpui::Role::Switch)
            .aria_toggled(self.checked.into())
            .when_some(name, |el, name| el.aria_label(name))
            .when(self.disabled, |el| el.a11y_synthetic_children(|builder| builder.parent_node().set_disabled()))
            .track_focus(&self.focus_handle)
            .p(px(2.0)).child(track);
        if focused && !self.disabled {
            el = el
                .rounded(px((h + 4.0) / 2.0))
                .border_2()
                .border_color(theme.primary.base);
        } else {
            el = el.border_2().border_color(rgba(0, 0, 0, 0.0));
        }

        if !self.disabled {
            el = el.cursor_pointer();
        } else {
            el = el.cursor_not_allowed().capture_any_mouse_down(|_, window, _| window.prevent_default());
        }

        el.on_action(cx.listener(Self::toggle))
        .on_mouse_down(MouseButton::Left, cx.listener(|this, _, _, _| this.pointer_armed = !this.disabled))
        .on_click(cx.listener(|this, event, window, cx| {
            // Keep key-down activation; an interrupted pointer press must not survive disabling.
            let armed = std::mem::take(&mut this.pointer_armed);
            if matches!(event, gpui::ClickEvent::Mouse(_)) && armed {
                this.toggle(&SwitchToggle, window, cx);
            }
        }))
        .on_key_down(|event, window, cx| {
            let modifiers = event.keystroke.modifiers;
            if event.keystroke.key == "tab" && !modifiers.control && !modifiers.alt && !modifiers.platform && !modifiers.function {
                if modifiers.shift { window.focus_prev(cx); } else { window.focus_next(cx); }
                cx.stop_propagation();
            }
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn switch_applies_explicit_root_id() {
        use super::Switch;
        use gpui::{Element, ElementId, IntoElement, Render, TestAppContext, px, size};

        let mut app = TestAppContext::single();
        app.update(|cx| liora_core::init_liora(cx, liora_theme::Theme::light()));
        let window = app.open_window(size(px(160.0), px(80.0)), |_, cx| {
            Switch::new(false, cx).id("settings-switch")
        });

        window
            .update(&mut app, |switch, window, cx| {
                let root = switch.render(window, cx).into_element();
                assert_eq!(root.id(), Some(ElementId::from("settings-switch")));
            })
            .unwrap();
        app.quit();
    }
}
