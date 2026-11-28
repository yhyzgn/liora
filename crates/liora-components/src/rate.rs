//! Rate module.
//!
//! This public module implements the Liora rating component for star-like score selection. It keeps the reusable
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

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{App, Context, FocusHandle, Focusable, MouseButton, Render, Window, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Fluent native GPUI component for rendering Liora rate.
pub struct Rate {
    value: f32,
    max: usize,
    hover_value: Option<f32>,
    keyboard_value: Option<f32>,
    disabled: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
}

impl Rate {
    /// Creates `Rate` initialized from the supplied value.
    pub fn new(value: f32, cx: &mut Context<Self>) -> Self {
        Self {
            value,
            max: 5,
            hover_value: None,
            keyboard_value: None,
            disabled: false,
            focus_handle: cx.focus_handle().tab_stop(true).tab_index(0),
            on_change: None,
        }
    }

    /// Sets the upper numeric boundary.
    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self.focus_handle = self.focus_handle.clone().tab_stop(!d);
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(f32, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn set_value(&mut self, val: f32, window: &mut Window, cx: &mut Context<Self>) {
        if (val - self.value).abs() > f32::EPSILON {
            self.value = val;
            if let Some(ref cb) = self.on_change {
                cb(self.value, window, cx);
            }
            cx.notify();
        }
    }
    fn set_keyboard_value(&mut self, value: f32, cx: &mut Context<Self>) {
        if self.keyboard_value != Some(value) {
            self.keyboard_value = Some(value);
            cx.notify();
        }
    }
    fn active_value(&self, window: &Window) -> f32 {
        let keyboard_value = self.keyboard_value.unwrap_or(self.value);
        if self.focus_handle.is_focused(window) && window.last_input_was_keyboard() {
            keyboard_value
        } else {
            self.hover_value.unwrap_or(self.value)
        }
    }

    fn handle_key_down(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        let key = event.keystroke.key.as_str();
        let modifiers = event.keystroke.modifiers;
        if key == "tab" {
            if !modifiers.control && !modifiers.alt && !modifiers.platform && !modifiers.function {
                if self.keyboard_value.take().is_some() {
                    cx.notify();
                }
                if modifiers.shift {
                    window.focus_prev(cx);
                } else {
                    window.focus_next(cx);
                }
                cx.stop_propagation();
            }
            return;
        }
        if modifiers.modified() {
            return;
        }

        match key {
            "left" | "down" | "right" | "up" | "home" | "end" if self.max > 0 => {
                let max = self.max as f32;
                let current = self.keyboard_value.unwrap_or(self.value);
                let active = if current.is_nan() {
                    0.0
                } else {
                    current.floor().clamp(0.0, max)
                };
                let next = match key {
                    "left" | "down" if active == 0.0 => 0.0,
                    "left" | "down" => (active - 1.0).max(1.0),
                    "right" | "up" if active == 0.0 => 1.0,
                    "right" | "up" => (active + 1.0).min(max),
                    "home" => 1.0,
                    "end" => max,
                    _ => unreachable!(),
                };
                self.set_keyboard_value(next, cx);
                cx.stop_propagation();
            }
            "enter" | "space" => {
                self.set_value(self.keyboard_value.unwrap_or(self.value), window, cx);
                cx.stop_propagation();
            }
            _ => {}
        }
    }
}

impl Focusable for Rate {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Rate {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if !self.focus_handle.is_focused(window) {
            self.keyboard_value = None;
        }
        let theme = &cx.global::<Config>().theme;
        let icon_sz = 20.0;

        let view_id = cx.entity().entity_id().as_u64();

        let mut row = gpui::div()
            .id(element_id(format!("rate-container-{view_id}")))
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .gap_1();
        let focus_handle = self.focus_handle.clone();
        let focus_color = theme.primary.base;
        let focus_ring = gpui::canvas(
            |_, _, _| (),
            move |bounds, (), window, _| {
                if focus_handle.is_focused(window) && window.last_input_was_keyboard() {
                    window.paint_quad(
                        gpui::outline(
                            bounds.dilate(px(2.0)),
                            focus_color,
                            gpui::BorderStyle::default(),
                        )
                        .border_widths(px(2.0))
                        .corner_radii(px(2.0)),
                    );
                }
            },
        )
        .absolute()
        .inset(-px(2.0));

        if !self.disabled {
            row = row
                .track_focus(&self.focus_handle)
                .tab_index(0)
                .on_key_down(cx.listener(Self::handle_key_down))
                .on_hover(cx.listener(|this, hovered, _, cx| {
                    if !hovered && this.hover_value.is_some() {
                        this.hover_value = None;
                        cx.notify();
                    }
                }));
        }

        for i in 1..=self.max {
            let active_val = self.active_value(window);
            let is_active = i as f32 <= active_val;

            let color = if is_active {
                theme.warning.base
            } else {
                theme.neutral.border
            };

            let mut star = gpui::div()
                .id(element_id(format!("rate-star-{view_id}-{i}")))
                .flex()
                .items_center()
                .justify_center()
                .child({
                    let icon = Icon::new(IconName::Star).size(px(icon_sz)).color(color);
                    let icon_shell = gpui::div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(icon);
                    if is_active {
                        pop_in(
                            element_id(format!("rate-star-motion-{view_id}-{i}")),
                            icon_shell,
                        )
                        .into_any_element()
                    } else {
                        icon_shell.into_any_element()
                    }
                });

            if !self.disabled {
                star = star
                    .cursor_pointer()
                    .on_hover(cx.listener(move |this, hovered, _, cx| {
                        let hover_value = Some(i as f32);
                        match (*hovered, this.hover_value == hover_value) {
                            (true, false) => {
                                this.hover_value = hover_value;
                                cx.notify();
                            }
                            (false, true) => {
                                this.hover_value = None;
                                cx.notify();
                            }
                            _ => {}
                        }
                    }))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(move |this, _, window, cx| {
                            this.keyboard_value = None;
                            this.set_value(i as f32, window, cx);
                        }),
                    );
            } else {
                star = star.cursor_not_allowed();
            }

            row = row.child(star);
        }

        row.child(focus_ring)
    }
}
#[cfg(test)]
#[path = "rate_keyboard_tests.rs"]
mod keyboard_tests;
