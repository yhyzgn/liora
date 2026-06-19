//! Label module.
//!
//! This public module implements the Liora label component for text plus optional icon decoration. It keeps the reusable
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
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Public builder and render state for the Liora label component.
pub struct Label {
    text: SharedString,
    icon: Option<IconName>,
    custom_icon: Option<AnyElement>,
    gap: Pixels,
    color: Option<Hsla>,
    size: Pixels,
}

impl Label {
    /// Creates a new value with the required baseline configuration.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            icon: None,
            custom_icon: None,
            gap: px(6.0),
            color: None,
            size: px(13.0),
        }
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }
    /// Configures the custom icon option.
    pub fn custom_icon(mut self, icon: impl IntoElement) -> Self {
        self.custom_icon = Some(icon.into_any_element());
        self
    }
    /// Configures the gap option.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }
    /// Configures the color option.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().max(px(8.0));
        self
    }
}

impl RenderOnce for Label {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let color = self.color.unwrap_or(theme.neutral.text_2);
        let has_custom_icon = self.custom_icon.is_some();
        div()
            .flex()
            .items_center()
            .gap(self.gap)
            .text_size(self.size)
            .text_color(color)
            .when_some(self.custom_icon, |s, icon| s.child(icon))
            .when(!has_custom_icon, |s| {
                if let Some(icon) = self.icon {
                    s.child(Icon::new(icon).size(self.size).color(color))
                } else {
                    s
                }
            })
            .child(self.text)
    }
}

impl IntoElement for Label {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn label_builders_track_state() {
        let label = Label::new("CPU")
            .icon(IconName::Activity)
            .gap(px(10.0))
            .size(px(15.0));
        assert_eq!(label.gap, px(10.0));
        assert_eq!(label.size, px(15.0));
        assert_eq!(label.icon, Some(IconName::Activity));
    }
}
