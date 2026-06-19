//! Rate module.
//!
//! This public module implements the Liora rating component for star-like score selection. It keeps the reusable
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
use crate::motion::pop_in;
use gpui::{App, Context, FocusHandle, Focusable, MouseButton, Render, Window, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct Rate {
    value: f32,
    max: usize,
    hover_value: Option<f32>,
    disabled: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(f32, &mut Window, &mut App) + 'static>>,
}

impl Rate {
    pub fn new(value: f32, cx: &mut Context<Self>) -> Self {
        Self {
            value,
            max: 5,
            hover_value: None,
            disabled: false,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    pub fn max(mut self, max: usize) -> Self {
        self.max = max;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }

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
}

impl Focusable for Rate {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Rate {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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

        if !self.disabled {
            row = row.track_focus(&self.focus_handle).on_hover(cx.listener(
                |this, hovered, _, cx| {
                    if !hovered && this.hover_value.is_some() {
                        this.hover_value = None;
                        cx.notify();
                    }
                },
            ));
        }

        for i in 1..=self.max {
            let active_val = self.hover_value.unwrap_or(self.value);
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
                            this.set_value(i as f32, window, cx);
                        }),
                    );
            } else {
                star = star.cursor_not_allowed();
            }

            row = row.child(star);
        }

        row
    }
}
