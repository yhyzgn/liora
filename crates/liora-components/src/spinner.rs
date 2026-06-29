//! Spinner module.
//!
//! This public module implements the Liora inline loading spinner component. It keeps the reusable
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

use crate::motion::spin_icon_with_duration;
use gpui::{App, Component, Hsla, IntoElement, Pixels, RenderOnce, Window, px};
use liora_core::{Config, stable_unique_id};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Fluent native GPUI component for rendering a compact inline loading spinner.
pub struct Spinner {
    size: Pixels,
    color: Option<Hsla>,
    icon: IconName,
}

impl Spinner {
    /// Creates `Spinner` with theme-primary color and medium size.
    pub fn new() -> Self {
        Self {
            size: px(16.0),
            color: None,
            icon: IconName::LoaderCircle,
        }
    }

    /// Sets the spinner icon size.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into().max(px(8.0));
        self
    }

    /// Applies an explicit spinner color instead of the theme primary color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Uses a custom Lucide icon while preserving the spin motion.
    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = icon;
        self
    }

    /// Applies the small sizing preset.
    pub fn small(self) -> Self {
        self.size(px(12.0))
    }

    /// Applies the large sizing preset.
    pub fn large(self) -> Self {
        self.size(px(24.0))
    }
}

impl RenderOnce for Spinner {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let color = self.color.unwrap_or(theme.primary.base);
        let motion_id = stable_unique_id(
            format!(
                "liora-spinner-motion:{:?}:{:?}:{:?}",
                self.icon, self.size, color
            ),
            "liora-spinner-motion",
            _window,
            cx,
        );

        spin_icon_with_duration(
            motion_id,
            Icon::new(self.icon).size(self.size).color(color),
            std::time::Duration::from_millis(1350),
        )
    }
}

impl IntoElement for Spinner {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn spinner_builders_track_state() {
        let spinner = Spinner::new().large().icon(IconName::RefreshCw);

        assert_eq!(spinner.size, px(24.0));
        assert_eq!(spinner.icon, IconName::RefreshCw);
    }

    #[test]
    fn spinner_uses_stable_motion_ids_so_animation_can_continue() {
        let source = include_str!("spinner.rs");
        assert!(source.contains("stable_unique_id("));
        assert!(source.contains("liora-spinner-motion:{:?}:{:?}:{:?}"));
        assert!(source.contains("spin_icon_with_duration("));
        assert!(source.contains("Duration::from_millis(1350)"));
        let render_body = source
            .split("impl RenderOnce for Spinner")
            .nth(1)
            .expect("Spinner should implement RenderOnce")
            .split("impl IntoElement for Spinner")
            .next()
            .expect("RenderOnce block should end before IntoElement");
        assert!(!render_body.contains(r#"liora_core::unique_id("liora-spinner-motion")"#));
        assert!(render_body.contains("Icon::new(self.icon).size(self.size).color(color)"));
        let stale_field = ["motion_id", ": &'static str"].join("");
        assert!(!source.contains(&stale_field));
    }
}
