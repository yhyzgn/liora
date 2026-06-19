//! Button Group module.
//!
//! This public module implements the Liora button grouping helpers for adjacent actions. It keeps the reusable
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

use crate::Button;
use gpui::{App, Component, IntoElement, RenderOnce, Window, div, prelude::*};

pub struct ButtonGroup {
    buttons: Vec<Button>,
}

impl ButtonGroup {
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    pub fn button(mut self, button: Button) -> Self {
        self.buttons.push(button);
        self
    }
}

impl RenderOnce for ButtonGroup {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        // let theme = &cx.global::<liora_core::Config>().theme;
        let count = self.buttons.len();

        div()
            .flex()
            .flex_row()
            .items_center()
            .children(self.buttons.into_iter().enumerate().map(|(i, btn)| {
                // Note: In GPUI 0.2.2, we don't have an easy way to override
                // internal parts of Button from outside without adding more fields to Button.
                // For now, we'll just render them side-by-side.
                // Real ButtonGroup implementation would need Button to support custom corners.

                // To keep it simple for now, we'll just use the flex row.
                // In a future update, we can add .rounded_none(), .rounded_left(), etc. to Button.
                if i > 0 && i < count {
                    // btn = btn.margin_left(px(-1.0)); // overlap borders
                }
                btn
            }))
    }
}

impl IntoElement for ButtonGroup {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
