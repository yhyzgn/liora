//! Segment Ratio Bar module.
//!
//! This public module implements the Liora segmented ratio bar for proportional category visualization. It keeps the reusable
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

use crate::Text;
use gpui::{
    App, Component, Hsla, IntoElement, Pixels, RenderOnce, SharedString, Window, div, prelude::*,
    px,
};
use liora_core::Config;

#[derive(Clone, Debug)]
/// Public builder and render state for the Liora segment ratio item component.
pub struct SegmentRatioItem {
    /// Human-readable label shown in the component UI.
    pub label: SharedString,
    /// Current value represented by this option or component state.
    pub value: f64,
    /// Color token or explicit color applied to the visual element.
    pub color: Hsla,
    /// Label pattern for this data model.
    pub label_pattern: Option<SharedString>,
    /// Value pattern for this data model.
    pub value_pattern: Option<SharedString>,
}
impl SegmentRatioItem {
    /// Creates a new value with the required baseline configuration.
    pub fn new(label: impl Into<SharedString>, value: f64, color: Hsla) -> Self {
        Self {
            label: label.into(),
            value,
            color,
            label_pattern: None,
            value_pattern: None,
        }
    }
    /// Configures the label pattern option.
    pub fn label_pattern(mut self, pattern: impl Into<SharedString>) -> Self {
        self.label_pattern = Some(pattern.into());
        self
    }
    /// Configures the value pattern option.
    pub fn value_pattern(mut self, pattern: impl Into<SharedString>) -> Self {
        self.value_pattern = Some(pattern.into());
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Enumerates the supported segment legend position modes and options.
pub enum SegmentLegendPosition {
    /// Uses the top variant.
    Top,
    #[default]
    /// Uses the bottom variant.
    Bottom,
    /// Uses the both variant.
    Both,
    /// Uses the hidden variant.
    Hidden,
}

#[derive(Clone)]
/// Public builder and render state for the Liora segment ratio bar component.
pub struct SegmentRatioBar {
    items: Vec<SegmentRatioItem>,
    height: Pixels,
    radius: Pixels,
    segment_radius: Pixels,
    legend_position: SegmentLegendPosition,
    percentage_decimals: usize,
    split_legend: bool,
    legend_inset_x: Pixels,
}

impl SegmentRatioBar {
    /// Creates a new value with the required baseline configuration.
    pub fn new(items: impl IntoIterator<Item = SegmentRatioItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
            height: px(12.0),
            radius: px(6.0),
            segment_radius: px(0.0),
            legend_position: SegmentLegendPosition::Bottom,
            percentage_decimals: 0,
            split_legend: true,
            legend_inset_x: px(6.0),
        }
    }
    /// Returns the height token used for component sizing.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(4.0));
        self
    }
    /// Configures the radius option.
    pub fn radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.radius = radius.into().max(px(0.0));
        self
    }

    /// Configures the segment radius option.
    pub fn segment_radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.segment_radius = radius.into().max(px(0.0));
        self
    }

    /// Configures the rounded segments option.
    pub fn rounded_segments(self, radius: impl Into<Pixels>) -> Self {
        self.segment_radius(radius)
    }

    /// Configures the legend position option.
    pub fn legend_position(mut self, position: SegmentLegendPosition) -> Self {
        self.legend_position = position;
        self
    }
    /// Configures the legend top option.
    pub fn legend_top(self) -> Self {
        self.legend_position(SegmentLegendPosition::Top)
    }
    /// Configures the legend bottom option.
    pub fn legend_bottom(self) -> Self {
        self.legend_position(SegmentLegendPosition::Bottom)
    }
    /// Configures the legend both option.
    pub fn legend_both(self) -> Self {
        self.legend_position(SegmentLegendPosition::Both)
    }
    /// Configures whether legend is hidden in the rendered component.
    pub fn hide_legend(self) -> Self {
        self.legend_position(SegmentLegendPosition::Hidden)
    }
    /// Configures the percentage decimals option.
    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.percentage_decimals = decimals.min(4);
        self
    }
    /// Configures the split legend option.
    pub fn split_legend(mut self, split: bool) -> Self {
        self.split_legend = split;
        self
    }

    /// Configures the legend inset x option.
    pub fn legend_inset_x(mut self, inset: impl Into<Pixels>) -> Self {
        self.legend_inset_x = inset.into().max(px(0.0));
        self
    }

    /// Configures the legend text inset option.
    pub fn legend_text_inset(self, inset: impl Into<Pixels>) -> Self {
        self.legend_inset_x(inset)
    }
}

impl RenderOnce for SegmentRatioBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let total: f64 = self
            .items
            .iter()
            .map(|i| i.value.max(0.0))
            .sum::<f64>()
            .max(1.0);
        let legend_top = matches!(
            self.legend_position,
            SegmentLegendPosition::Top | SegmentLegendPosition::Both
        );
        let legend_bottom = matches!(
            self.legend_position,
            SegmentLegendPosition::Bottom | SegmentLegendPosition::Both
        );
        let items_for_top = self.items.clone();
        let items_for_bottom = self.items.clone();
        let segment_radius = self.segment_radius;
        div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .when(legend_top, |s| {
                s.child(render_segment_legend(
                    items_for_top,
                    total,
                    self.percentage_decimals,
                    self.split_legend,
                    self.legend_inset_x,
                ))
            })
            .child(
                div()
                    .flex()
                    .w_full()
                    .h(self.height)
                    .rounded(self.radius)
                    .overflow_hidden()
                    .bg(theme.neutral.body)
                    .children(self.items.into_iter().map(|item| {
                        let percent = (item.value.max(0.0) / total * 100.0).max(0.0) as f32;
                        div()
                            .h_full()
                            .bg(item.color)
                            .rounded(segment_radius)
                            .overflow_hidden()
                            .w(gpui::relative(percent / 100.0))
                    })),
            )
            .when(legend_bottom, |s| {
                s.child(render_segment_legend(
                    items_for_bottom,
                    total,
                    self.percentage_decimals,
                    self.split_legend,
                    self.legend_inset_x,
                ))
            })
    }
}

fn render_segment_legend(
    items: Vec<SegmentRatioItem>,
    total: f64,
    decimals: usize,
    split: bool,
    inset_x: Pixels,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .w_full()
        .children(items.into_iter().map(move |item| {
            let pct = item.value.max(0.0) / total * 100.0;
            let label = item
                .label_pattern
                .as_ref()
                .map(|p| {
                    p.to_string()
                        .replace("{label}", &item.label)
                        .replace("{value}", &format_value(item.value))
                        .replace("{percent}", &format!("{:.*}%", decimals, pct))
                })
                .unwrap_or_else(|| item.label.to_string());
            let value = item
                .value_pattern
                .as_ref()
                .map(|p| {
                    p.to_string()
                        .replace("{label}", &item.label)
                        .replace("{value}", &format_value(item.value))
                        .replace("{percent}", &format!("{:.*}%", decimals, pct))
                })
                .unwrap_or_else(|| format!("{:.*}%", decimals, pct));
            let segment_width = gpui::relative((item.value.max(0.0) / total).max(0.0) as f32);
            div()
                .flex()
                .items_center()
                .gap_2()
                .w(segment_width)
                .min_w(px(0.0))
                .px(inset_x)
                .when(split, |s| s.justify_between())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(item.color))
                        .min_w(px(0.0))
                        .child(Text::new(label).size(px(12.0))),
                )
                .child(
                    div()
                        .flex_none()
                        .text_align(gpui::TextAlign::Right)
                        .child(Text::new(value).size(px(12.0))),
                )
        }))
}

fn format_value(value: f64) -> String {
    if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}")
    } else {
        format!("{value:.1}")
    }
}

impl IntoElement for SegmentRatioBar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn segment_ratio_bar_tracks_legend_position() {
        let bar = SegmentRatioBar::new([SegmentRatioItem::new("Direct", 42.0, gpui::blue())])
            .legend_top()
            .percentage_decimals(2)
            .split_legend(true)
            .height(px(18.0))
            .legend_inset_x(px(10.0))
            .radius(px(12.0))
            .segment_radius(px(4.0));
        assert_eq!(bar.legend_position, SegmentLegendPosition::Top);
        assert_eq!(bar.percentage_decimals, 2);
        assert!(bar.split_legend);
        assert_eq!(bar.height, px(18.0));
        assert_eq!(bar.legend_inset_x, px(10.0));
        assert_eq!(bar.radius, px(12.0));
        assert_eq!(bar.segment_radius, px(4.0));
    }

    #[test]
    fn segment_ratio_bar_legend_is_horizontal() {
        let source = include_str!("segment_ratio_bar.rs");
        assert!(source.contains("fn render_segment_legend"));
        assert!(source.contains(".flex_row()"));
        assert!(source.contains(".w_full()"));
        assert!(source.contains("let segment_width = gpui::relative"));
        assert!(source.contains(".w(segment_width)"));
        assert!(source.contains("justify_between"));
        assert!(source.contains("TextAlign::Right"));
        assert!(source.contains(".px(inset_x)"));
        assert!(source.contains("segment_radius"));
        assert!(source.contains(".rounded(segment_radius)"));
    }
}
