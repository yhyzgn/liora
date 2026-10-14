//! Hover Card module.
//!
//! This public module implements the Liora hover-card preview wrapper for lightweight hover-triggered context. It keeps the reusable
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

use crate::{Popover, PopoverView};
use gpui::{
    AnyElement, App, Context, IntoElement, ParentElement, Pixels, RenderOnce, SharedString,
    ViewElement as Component, Window, div, px,
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
    trigger_id: Option<SharedString>,
}

impl HoverCard {
    /// Creates a hover-card with default placeholder content.
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Hover card").into_any_element()),
            placement: Placement::Top,
            offset: px(8.0),
            trigger_id: None,
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

    /// Assigns a stable trigger id forwarded to the underlying popover.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.trigger_id = Some(id.into());
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
        let popover = Popover::new(self.trigger)
            .content(move |window, cx| content(window, cx))
            .placement(self.placement)
            .offset(self.offset)
            .open_on_hover(true)
            .close_on_click_outside(true);

        if let Some(trigger_id) = self.trigger_id {
            popover.id(trigger_id)
        } else {
            popover
        }
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

    #[test]
    fn hover_card_configures_popover_for_hover_opening() {
        let source = include_str!("hover_card.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains(".open_on_hover(true)"));
    }

    #[test]
    fn hover_card_tracks_stable_id_for_tests_and_downstream() {
        let card = HoverCard::new(div()).id("profile-hover");
        assert_eq!(card.trigger_id.as_deref(), Some("profile-hover"));
    }
}
