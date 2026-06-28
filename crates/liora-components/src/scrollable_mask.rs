//! Scrollable Mask module.
//!
//! This public module implements the Liora scrollable-mask visual wrapper for edge-fade scroll affordances. It keeps the reusable
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

use gpui::{
    AnyElement, App, Component, ElementId, InteractiveElement, IntoElement, Pixels, RenderOnce,
    Styled, Window, div, prelude::*, px,
};
use liora_core::{Config, stable_unique_id};

/// Scroll container with subtle top/bottom mask indicators.
pub struct ScrollableMask {
    child: AnyElement,
    height: Option<Pixels>,
    fade: Pixels,
}
impl ScrollableMask {
    /// Wraps a child element in a scroll container with edge fade affordances.
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            height: None,
            fade: px(18.0),
        }
    }
    /// Sets a fixed viewport height for the scrollable region.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }
    /// Sets the height of the top and bottom fade masks.
    pub fn fade(mut self, fade: impl Into<Pixels>) -> Self {
        self.fade = fade.into().max(px(0.0));
        self
    }
}
impl IntoElement for ScrollableMask {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
impl RenderOnce for ScrollableMask {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_id = ElementId::from(stable_unique_id("scrollable-mask", "scroll", _window, cx));
        div()
            .relative()
            .overflow_hidden()
            .when_some(self.height, |s, h| s.h(h))
            .child(
                div()
                    .id(scroll_id)
                    .overflow_y_scroll()
                    .size_full()
                    .child(self.child),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .right_0()
                    .h(self.fade)
                    .bg(theme.neutral.card.opacity(0.62)),
            )
            .child(
                div()
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .right_0()
                    .h(self.fade)
                    .bg(theme.neutral.card.opacity(0.62)),
            )
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use gpui::div;
    #[test]
    fn scrollable_mask_tracks_height_and_fade() {
        let mask = ScrollableMask::new(div()).height(px(120.0)).fade(px(12.0));
        assert_eq!(mask.height, Some(px(120.0)));
        assert_eq!(mask.fade, px(12.0));
    }
}
