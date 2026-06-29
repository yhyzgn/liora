//! Button Group module.
//!
//! This public module implements the Liora button grouping helpers for adjacent actions. It keeps the reusable
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

use crate::Button;
use gpui::{App, Component, IntoElement, RenderOnce, Window, div, prelude::*};

/// Fluent native GPUI component for rendering Liora button group.
pub struct ButtonGroup {
    buttons: Vec<Button>,
}

impl ButtonGroup {
    /// Creates `ButtonGroup` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            buttons: Vec::new(),
        }
    }

    /// Adds the supplied button to the component.
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
                // Button currently owns its corner styling internally, so the group
                // renders adjacent actions in one flex row until Button exposes
                // explicit grouped-corner controls.
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
