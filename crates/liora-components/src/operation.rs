//! Operation module.
//!
//! This public module implements the Liora operation/action list component for compact command groups. It keeps the reusable
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

use crate::Label;
use gpui::{
    AnyElement, App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, LocalizedText};

/// Fluent native GPUI component for rendering Liora operation.
pub struct Operation {
    label: AnyElement,
    action: AnyElement,
    description: Option<SharedString>,
    status: Option<SharedString>,
    status_color: Option<Hsla>,
    gap: Pixels,
    padded: bool,
    disabled: bool,
}

impl Operation {
    /// Creates `Operation` initialized from the supplied label, and action.
    pub fn new(label: impl IntoElement, action: impl IntoElement) -> Self {
        Self {
            label: label.into_any_element(),
            action: action.into_any_element(),
            description: None,
            status: None,
            status_color: None,
            gap: px(16.0),
            padded: true,
            disabled: false,
        }
    }

    /// Applies the text preset.
    pub fn with_text(text: impl Into<LocalizedText>, action: impl IntoElement) -> Self {
        Self::new(Label::new(text), action)
    }
    /// Sets the spacing between child elements.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }
    /// Sets secondary descriptive text shown below the primary label.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }
    /// Sets the status value used by the component.
    pub fn status(mut self, status: impl Into<SharedString>) -> Self {
        self.status = Some(status.into());
        self
    }
    /// Sets the status color used by the rendered component.
    pub fn status_color(mut self, color: Hsla) -> Self {
        self.status_color = Some(color);
        self
    }
    /// Applies the success semantic visual variant.
    pub fn success(self) -> Self {
        self.status("正常").status_color(gpui::green())
    }
    /// Applies the warning semantic visual variant.
    pub fn warning(self) -> Self {
        self.status("注意").status_color(gpui::yellow())
    }
    /// Applies the danger semantic visual variant.
    pub fn danger(self) -> Self {
        self.status("异常").status_color(gpui::red())
    }
    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
    /// Disables padding rendering or behavior.
    pub fn no_padding(mut self) -> Self {
        self.padded = false;
        self
    }
}

impl RenderOnce for Operation {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let status_color = self.status_color.unwrap_or(theme.primary.base);
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(self.gap)
            .w_full()
            .when(self.disabled, |s| s.opacity(0.52))
            .when(self.padded, |s| {
                s.p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
            })
            .child(
                div()
                    .min_w_0()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(self.label)
                            .when_some(self.status, |s, status| {
                                s.child(
                                    div()
                                        .rounded_full()
                                        .px_2()
                                        .py(px(1.0))
                                        .text_xs()
                                        .bg(status_color.opacity(0.12))
                                        .text_color(status_color)
                                        .child(status),
                                )
                            }),
                    )
                    .when_some(self.description, |s, description| {
                        s.child(
                            div()
                                .text_sm()
                                .text_color(theme.neutral.text_3)
                                .child(description),
                        )
                    }),
            )
            .child(div().flex_none().child(self.action))
    }
}

impl IntoElement for Operation {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn operation_tracks_layout_options() {
        let op = Operation::with_text("Auto save", div())
            .gap(px(20.0))
            .description("Save changes automatically")
            .status("Enabled")
            .disabled(true)
            .no_padding();
        assert_eq!(op.gap, px(20.0));
        assert!(!op.padded);
        assert_eq!(
            op.description.as_ref().map(|text| text.as_ref()),
            Some("Save changes automatically")
        );
        assert_eq!(
            op.status.as_ref().map(|text| text.as_ref()),
            Some("Enabled")
        );
        assert!(op.disabled);
    }
}
