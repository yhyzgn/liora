//! Splitter module.
//!
//! This public module implements the Liora resizable splitter layout component. It keeps the reusable
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

use gpui::{App, Component, IntoElement, RenderOnce, Window, prelude::*, px};

/// Public builder and render state for the Liora splitter component.
pub struct Splitter {
    left: Option<gpui::AnyElement>,
    right: Option<gpui::AnyElement>,
    height: Option<gpui::Pixels>,
    bordered: bool,
}

impl Splitter {
    /// Creates a new value with the required baseline configuration.
    pub fn new() -> Self {
        Self {
            left: None,
            right: None,
            height: None,
            bordered: false,
        }
    }
    /// Configures the left option.
    pub fn left(mut self, el: impl IntoElement) -> Self {
        self.left = Some(el.into_any_element());
        self
    }
    /// Configures the right option.
    pub fn right(mut self, el: impl IntoElement) -> Self {
        self.right = Some(el.into_any_element());
        self
    }

    /// Returns the height token used for component sizing.
    pub fn height(mut self, height: impl Into<gpui::Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Applies the predefined height md sizing preset.
    pub fn height_md(self) -> Self {
        self.height(px(200.0))
    }

    /// Configures the bordered option.
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }
}

impl RenderOnce for Splitter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        let left = self.left.unwrap_or_else(|| gpui::div().into_any_element());
        let right = self.right.unwrap_or_else(|| gpui::div().into_any_element());

        gpui::div()
            .flex()
            .flex_row()
            .size_full()
            .when_some(self.height, |s, height| s.h(height))
            .when(self.bordered, |s| {
                s.border_1()
                    .border_color(theme.neutral.border)
                    .rounded(px(theme.radius.sm))
            })
            .child(gpui::div().flex_none().w(px(300.0)).h_full().child(left))
            .child(
                gpui::div()
                    .flex_none()
                    .w(px(4.0))
                    .h_full()
                    .bg(theme.neutral.border),
            )
            .child(gpui::div().flex_1().h_full().child(right))
    }
}

impl IntoElement for Splitter {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitter_presentation_helpers_track_state() {
        let splitter = Splitter::new().height_md().bordered();

        assert_eq!(splitter.height, Some(px(200.0)));
        assert!(splitter.bordered);
    }
}
