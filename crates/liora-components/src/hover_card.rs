//! Hover Card module.
//!
//! This public module implements the Liora hover-card preview wrapper for lightweight hover-triggered context. It keeps the reusable
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

use crate::{Popover, PopoverView};
use gpui::{
    AnyElement, App, Component, Context, IntoElement, ParentElement, Pixels, RenderOnce, Window,
    div, px,
};
use liora_core::Placement;
use std::sync::Arc;

type HoverCardContent = dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement + 'static;

/// Hover-card facade over Popover for profile/link previews.
pub struct HoverCard {
    trigger: AnyElement,
    content: Arc<HoverCardContent>,
    placement: Placement,
    offset: Pixels,
}

impl HoverCard {
    /// Creates a hover-card with default placeholder content.
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Hover card").into_any_element()),
            placement: Placement::Top,
            offset: px(8.0),
        }
    }

    /// Sets the preview content callback.
    pub fn content<F, E>(mut self, content: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<PopoverView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| content(window, cx).into_any_element());
        self
    }

    /// Selects the popup placement.
    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    /// Sets the offset from the trigger.
    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }
}

impl IntoElement for HoverCard {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for HoverCard {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let content = self.content.clone();
        Popover::new(self.trigger)
            .content(move |window, cx| content(window, cx))
            .placement(self.placement)
            .offset(self.offset)
            .close_on_click_outside(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hover_card_tracks_placement() {
        let card = HoverCard::new(div()).placement(Placement::Bottom);
        assert_eq!(card.placement, Placement::Bottom);
    }
}
