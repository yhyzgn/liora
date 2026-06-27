//! Group Box module.
//!
//! This public module implements the Liora group-box container for labeled settings and form sections. It keeps the reusable
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
    AnyElement, App, Component, IntoElement, ParentElement, RenderOnce, SharedString, Styled,
    Window, div, px,
};
use liora_core::Config;

/// Labeled container for form/settings subsections.
pub struct GroupBox {
    title: SharedString,
    description: Option<SharedString>,
    child: AnyElement,
    compact: bool,
}
impl GroupBox {
    /// Creates a group box with title and content.
    pub fn new(title: impl Into<SharedString>, child: impl IntoElement) -> Self {
        Self {
            title: title.into(),
            description: None,
            child: child.into_any_element(),
            compact: false,
        }
    }
    /// Adds helper description text.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Enables compact spacing.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }
}
impl IntoElement for GroupBox {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
impl RenderOnce for GroupBox {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p(if self.compact { px(10.0) } else { px(14.0) })
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(gpui::FontWeight::BOLD)
                            .text_color(theme.neutral.text_1)
                            .child(self.title),
                    )
                    .children(
                        self.description
                            .map(|d| div().text_xs().text_color(theme.neutral.text_3).child(d)),
                    ),
            )
            .child(self.child)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn group_box_tracks_description_and_compact() {
        let box_ = GroupBox::new("A", div()).description("B").compact();
        assert!(box_.description.is_some());
        assert!(box_.compact);
    }
}
