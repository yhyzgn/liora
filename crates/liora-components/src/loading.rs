//! Loading module.
//!
//! This public module implements the Liora loading indicator and overlay components. It keeps the reusable
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

use crate::motion::{fade_in, spin_icon};
use gpui::{App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Fluent native GPUI component for rendering Liora loading.
pub struct Loading {
    text: Option<SharedString>,
    full_screen: bool,
}

impl Loading {
    /// Creates `Loading` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            text: None,
            full_screen: false,
        }
    }

    /// Applies the text-only visual variant.
    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the full screen value used by the component.
    pub fn full_screen(mut self) -> Self {
        self.full_screen = true;
        self
    }
}

impl RenderOnce for Loading {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let spinner_icon = spin_icon(
            "liora-loading-spinner-motion",
            Icon::new(IconName::LoaderCircle)
                .size(px(32.0))
                .color(theme.primary.base),
        );

        let spinner = div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(spinner_icon)
            .when_some(self.text, |s, t| {
                s.child(div().text_sm().text_color(theme.primary.base).child(t))
            });

        if self.full_screen {
            fade_in(
                "liora-loading-fullscreen-motion",
                div()
                    .absolute()
                    .size_full()
                    .bg(theme.neutral.mask)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(spinner),
            )
        } else {
            fade_in("liora-loading-inline-motion", spinner)
        }
    }
}

impl IntoElement for Loading {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn loading_uses_spin_and_fade_motion() {
        let source = include_str!("loading.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("spin_icon("));
        assert!(source.contains("fade_in("));
        assert!(source.contains("liora-loading-spinner-motion"));
    }
}
