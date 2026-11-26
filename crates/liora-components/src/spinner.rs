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

use crate::motion::spin_icon;
use gpui::{
    App, Hsla, IntoElement, ParentElement, Pixels, RenderOnce, Styled, ViewElement as Component,
    Window, div, px,
};
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
        let color = self
            .color
            .unwrap_or(cx.global::<Config>().theme.primary.base);
        let motion_id = stable_unique_id(
            format!(
                "liora-spinner-motion:{:?}:{:?}:{:?}",
                self.icon, self.size, color
            ),
            "liora-spinner-motion",
            _window,
            cx,
        );

        let oversample_scale = spinner_oversample_scale(self.size);
        let render_size = self.size * oversample_scale;

        div()
            .flex_none()
            .w(self.size)
            .h(self.size)
            .flex()
            .items_center()
            .justify_center()
            .child(spin_icon(
                motion_id,
                Icon::new(self.icon)
                    .size(render_size)
                    .render_scale(1.0 / oversample_scale)
                    .color(color),
            ))
    }
}

fn spinner_oversample_scale(size: Pixels) -> f32 {
    if size <= px(18.0) {
        2.0
    } else if size < px(32.0) {
        1.5
    } else {
        1.0
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
        let render_body = source
            .split("impl RenderOnce for Spinner")
            .nth(1)
            .expect("Spinner should implement RenderOnce")
            .split("impl IntoElement for Spinner")
            .next()
            .expect("RenderOnce block should end before IntoElement");
        assert!(render_body.contains("spin_icon("));
        assert!(render_body.contains("spinner_oversample_scale(self.size)"));
        assert!(render_body.contains(".render_scale(1.0 / oversample_scale)"));
        assert!(!render_body.contains("Duration::from_millis(1350)"));
        assert!(!render_body.contains("spin_icon_with_duration("));
        assert!(!render_body.contains(r#"liora_core::unique_id("liora-spinner-motion")"#));
        assert!(render_body.contains("Icon::new(self.icon)"));
        assert!(render_body.contains(".size(render_size)"));
        assert!(render_body.contains(".color(color)"));
        let stale_field = ["motion_id", ": &'static str"].join("");
        assert!(!source.contains(&stale_field));
    }

    #[test]
    fn spinner_oversamples_small_svg_icons_for_smoother_rotation() {
        assert_eq!(spinner_oversample_scale(px(12.0)), 2.0);
        assert_eq!(spinner_oversample_scale(px(16.0)), 2.0);
        assert_eq!(spinner_oversample_scale(px(24.0)), 1.5);
        assert_eq!(spinner_oversample_scale(px(32.0)), 1.0);
    }
}
