//! Statistic module.
//!
//! This public module implements the Liora statistic/metric display component with optional icon. It keeps the reusable
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
    AnyElement, App, Hsla, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use liora_core::Config;
use liora_icons::{Icon, IntoIconPath};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported statistic layout modes and options.
pub enum StatisticLayout {
    /// Lays out content in the vertical direction.
    Vertical,
    /// Uses the horizontal compact variant.
    HorizontalCompact,
    /// Uses the horizontal between variant.
    HorizontalBetween,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Enumerates the supported statistic icon position modes and options.
pub enum StatisticIconPosition {
    /// Uses the left variant.
    Left,
    /// Uses the right variant.
    Right,
}

/// Public builder and render state for the Liora statistic component.
pub struct Statistic {
    title: SharedString,
    value: SharedString,
    prefix: Option<AnyElement>,
    suffix: Option<AnyElement>,
    value_color: Option<Hsla>,
    icon: Option<String>,
    icon_position: StatisticIconPosition,
    icon_color: Option<Hsla>,
    layout: StatisticLayout,
}

impl Statistic {
    /// Creates a new value with the required baseline configuration.
    pub fn new(title: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            value: value.into(),
            prefix: None,
            suffix: None,
            value_color: None,
            icon: None,
            icon_position: StatisticIconPosition::Right,
            icon_color: None,
            layout: StatisticLayout::Vertical,
        }
    }

    /// Configures the prefix option.
    pub fn prefix(mut self, prefix: impl IntoElement) -> Self {
        self.prefix = Some(prefix.into_any_element());
        self
    }

    /// Configures the suffix option.
    pub fn suffix(mut self, suffix: impl IntoElement) -> Self {
        self.suffix = Some(suffix.into_any_element());
        self
    }

    /// Configures the value color option.
    pub fn value_color(mut self, color: Hsla) -> Self {
        self.value_color = Some(color);
        self
    }

    /// Sets the tray icon configuration value.
    pub fn icon(mut self, icon: impl IntoIconPath) -> Self {
        self.icon = Some(icon.icon_path().into_owned());
        self
    }

    /// Configures the icon position option.
    pub fn icon_position(mut self, position: StatisticIconPosition) -> Self {
        self.icon_position = position;
        self
    }

    /// Configures the icon left option.
    pub fn icon_left(self) -> Self {
        self.icon_position(StatisticIconPosition::Left)
    }

    /// Configures the icon right option.
    pub fn icon_right(self) -> Self {
        self.icon_position(StatisticIconPosition::Right)
    }

    /// Configures the icon color option.
    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }

    /// Configures the layout option.
    pub fn layout(mut self, layout: StatisticLayout) -> Self {
        self.layout = layout;
        self
    }

    /// Configures the vertical option.
    pub fn vertical(self) -> Self {
        self.layout(StatisticLayout::Vertical)
    }

    /// Configures the horizontal option.
    pub fn horizontal(self) -> Self {
        self.horizontal_compact()
    }

    /// Configures the horizontal compact option.
    pub fn horizontal_compact(self) -> Self {
        self.layout(StatisticLayout::HorizontalCompact)
    }

    /// Configures the horizontal between option.
    pub fn horizontal_between(self) -> Self {
        self.layout(StatisticLayout::HorizontalBetween)
    }

    fn resolved_icon_color(&self, value_color: Hsla) -> Hsla {
        self.icon_color.unwrap_or(value_color)
    }
}

impl RenderOnce for Statistic {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let value_line_height = px(32.0);
        let value_color = self.value_color.unwrap_or(theme.neutral.text_1);
        let icon_color = self.resolved_icon_color(value_color);
        let icon_position = self.icon_position;

        let title = div()
            .text_sm()
            .text_color(theme.neutral.text_3)
            .child(self.title);

        let icon = self.icon.map(|path| {
            Icon::new(path)
                .size(px(18.0))
                .color(icon_color)
                .into_any_element()
        });
        let (leading_icon, trailing_icon) = match icon_position {
            StatisticIconPosition::Left => (icon, None),
            StatisticIconPosition::Right => (None, icon),
        };

        let value = div()
            .text_2xl()
            .line_height(value_line_height)
            .font_weight(gpui::FontWeight::BOLD)
            .text_color(value_color)
            .child(self.value);

        let value_line = div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .when_some(self.prefix, |s, p| {
                s.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(value_line_height)
                        .child(p),
                )
            })
            .when_some(leading_icon, |s, icon| {
                s.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(value_line_height)
                        .child(icon),
                )
            })
            .child(value)
            .when_some(trailing_icon, |s, icon| {
                s.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(value_line_height)
                        .child(icon),
                )
            })
            .when_some(self.suffix, |s, p| {
                s.child(
                    div()
                        .flex()
                        .items_center()
                        .justify_center()
                        .h(value_line_height)
                        .child(p),
                )
            });

        match self.layout {
            StatisticLayout::Vertical => div()
                .flex()
                .flex_col()
                .gap_1()
                .child(title)
                .child(value_line),
            StatisticLayout::HorizontalCompact => div()
                .flex()
                .flex_row()
                .items_center()
                .gap_4()
                .child(title)
                .child(value_line),
            StatisticLayout::HorizontalBetween => div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_4()
                .w_full()
                .child(title)
                .child(value_line),
        }
    }
}

impl IntoElement for Statistic {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use liora_icons_lucide::IconName;

    #[test]
    fn statistic_horizontal_helpers_set_layout() {
        assert_eq!(
            Statistic::new("Visitors", "1,024")
                .horizontal_compact()
                .layout,
            StatisticLayout::HorizontalCompact
        );
        assert_eq!(
            Statistic::new("Visitors", "1,024")
                .horizontal_between()
                .layout,
            StatisticLayout::HorizontalBetween
        );
    }

    #[test]
    fn statistic_icon_helpers_set_position_and_color() {
        let icon_color = gpui::red();
        let statistic = Statistic::new("Growth", "12.5")
            .icon(IconName::TrendingUp)
            .icon_left()
            .icon_color(icon_color);

        assert_eq!(statistic.icon_position, StatisticIconPosition::Left);
        assert!(statistic.icon.is_some());
        assert_eq!(statistic.icon_color, Some(icon_color));
    }

    #[test]
    fn statistic_icon_color_defaults_to_value_color() {
        let value_color = gpui::green();
        assert_eq!(
            Statistic::new("Growth", "12.5")
                .icon(IconName::TrendingUp)
                .resolved_icon_color(value_color),
            value_color
        );
    }
}
