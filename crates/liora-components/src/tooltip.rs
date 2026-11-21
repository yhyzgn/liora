//! Tooltip module.
//!
//! This public module implements the Liora tooltip component for anchored help text. It keeps the reusable
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

use gpui::{
    AnyElement, App, Bounds, IntoElement, Pixels, RenderOnce, SharedString,
    ViewElement as Component, Window, px,
};
use liora_core::{Placement, TooltipData, render_tooltip_trigger, stable_unique_id};

/// Fluent native GPUI component for rendering Liora tooltip.
pub struct Tooltip {
    trigger: AnyElement,
    content: SharedString,
    placement: Placement,
    offset: Pixels,
    id: Option<SharedString>,
}

impl Tooltip {
    /// Creates `Tooltip` initialized from the supplied trigger.
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: SharedString::default(),
            placement: Placement::Top,
            offset: px(8.0),
            id: None,
        }
    }

    /// Sets the rendered content element or text for this component.
    pub fn content(mut self, content: impl Into<SharedString>) -> Self {
        self.content = content.into();
        self
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the pixel offset used when positioning the component.
    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }
}

impl RenderOnce for Tooltip {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let id = self.id.unwrap_or_else(|| {
            stable_unique_id(
                format!("tooltip:{}:{:?}", self.content, self.placement),
                "tooltip",
                window,
                cx,
            )
        });
        render_tooltip_trigger(
            self.trigger,
            TooltipData {
                id,
                content: self.content,
                anchor_bounds: Bounds::default(),
                placement: self.placement,
                offset: self.offset,
            },
            window,
            cx,
        )
    }
}

impl IntoElement for Tooltip {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
#[path = "tooltip/interaction_tests.rs"]
mod interaction_tests;
