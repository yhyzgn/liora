//! Badge module.
//!
//! This public module implements the Liora small count and status marker component. It keeps the reusable
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

use gpui::{
    AnyElement, App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control badge type behavior.
pub enum BadgeType {
    #[default]
    /// Uses danger semantic color tokens.
    Danger,
    /// Uses the primary brand-accent treatment.
    Primary,
    /// Uses success semantic color tokens.
    Success,
    /// Uses warning semantic color tokens.
    Warning,
    /// Uses informational semantic color tokens.
    Info,
}

/// Fluent native GPUI component for rendering Liora badge.
pub struct Badge {
    child: AnyElement,
    value: Option<SharedString>,
    max: Option<i32>,
    is_dot: bool,
    hidden: bool,
    badge_type: BadgeType,
}

impl Badge {
    /// Creates `Badge` initialized from the supplied child.
    pub fn new(child: impl IntoElement) -> Self {
        Self {
            child: child.into_any_element(),
            value: None,
            max: None,
            is_dot: false,
            hidden: false,
            badge_type: BadgeType::Danger,
        }
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets the upper numeric boundary.
    pub fn max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    /// Returns whether dot is currently true for this value.
    pub fn is_dot(mut self, is_dot: bool) -> Self {
        self.is_dot = is_dot;
        self
    }

    /// Toggles hidden behavior.
    pub fn hidden(mut self, hidden: bool) -> Self {
        self.hidden = hidden;
        self
    }

    /// Sets the badge type value used by the component.
    pub fn badge_type(mut self, t: BadgeType) -> Self {
        self.badge_type = t;
        self
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let color = match self.badge_type {
            BadgeType::Danger => theme.danger.base,
            BadgeType::Primary => theme.primary.base,
            BadgeType::Success => theme.success.base,
            BadgeType::Warning => theme.warning.base,
            BadgeType::Info => theme.info.base,
        };

        let badge_content = if self.is_dot {
            div()
                .size(px(8.0))
                .bg(color)
                .rounded_full()
                .border_1()
                .border_color(theme.neutral.body)
        } else {
            let display_value = if let Some(val) = self.value {
                // Try to parse as i32 if max is set
                if let Some(max) = self.max {
                    if let Ok(num) = val.parse::<i32>() {
                        if num > max {
                            format!("{}+", max).into()
                        } else {
                            val
                        }
                    } else {
                        val
                    }
                } else {
                    val
                }
            } else {
                "".into()
            };

            div()
                .flex()
                .items_center()
                .justify_center()
                .h(px(18.0))
                .min_w(px(18.0))
                .px(px(6.0))
                .bg(color)
                .rounded_full()
                .border_1()
                .border_color(theme.neutral.body)
                .text_color(theme.neutral.inverted)
                .text_size(px(10.0))
                .font_weight(gpui::FontWeight::BOLD)
                .child(display_value)
        };

        div().relative().child(self.child).when(!self.hidden, |s| {
            s.child(
                div()
                    .absolute()
                    .top(px(-6.0))
                    .right(px(-6.0))
                    // We use a small negative offset to put it in the top-right corner
                    .child(badge_content),
            )
        })
    }
}

impl IntoElement for Badge {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
