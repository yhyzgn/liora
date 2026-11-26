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
    AnyElement, App, IntoElement, RenderOnce, SharedString, ViewElement as Component, Window, div,
    prelude::*, px,
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

    /// Sets the displayed text or integer count.
    pub fn value(mut self, value: impl Into<SharedString>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Sets the upper numeric boundary.
    pub fn max(mut self, max: i32) -> Self {
        self.max = Some(max);
        self
    }

    /// Displays a dot instead of the value when enabled.
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

fn display_value(value: Option<SharedString>, max: Option<i32>) -> SharedString {
    let value = value.unwrap_or_default();
    let Some(max) = max else {
        return value;
    };
    let exceeds_max = match value.parse::<i32>() {
        Ok(number) => number > max,
        Err(error) => {
            matches!(error.kind(), std::num::IntErrorKind::PosOverflow)
                && value
                    .strip_prefix('+')
                    .unwrap_or(&value)
                    .bytes()
                    .all(|digit| digit.is_ascii_digit())
        }
    };
    if exceeds_max {
        format!("{max}+").into()
    } else {
        value
    }
}

impl RenderOnce for Badge {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let base = div().relative().child(self.child);
        if self.hidden {
            return base;
        }
        let theme = &cx.global::<Config>().theme;

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
            let display_value = display_value(self.value, self.max);

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
                .text_color(liora_theme::filled_foreground(color))
                .text_size(px(10.0))
                .font_weight(gpui::FontWeight::BOLD)
                .child(display_value)
        };

        base.child(
            div()
                .absolute()
                .top(px(-6.0))
                .right(px(-6.0))
                .child(badge_content),
        )
    }
}

impl IntoElement for Badge {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_cap_preserves_text_and_handles_large_counts() {
        for (value, max, expected) in [
            ("200", Some(99), "99+"),
            ("99", Some(99), "99"),
            ("+000100", Some(99), "99+"),
            ("00099", Some(99), "00099"),
            ("0", Some(99), "0"),
            ("-50", Some(99), "-50"),
            ("100", None, "100"),
            ("New", Some(99), "New"),
            ("999999999999x", Some(99), "999999999999x"),
            ("-2147483649", Some(-1), "-2147483649"),
            ("2147483648", Some(99), "99+"),
            ("+4294967295", Some(99), "99+"),
            ("340282366920938463463374607431768211455", Some(99), "99+"),
        ] {
            assert_eq!(display_value(Some(value.into()), max), expected, "{value}");
        }
    }
}
