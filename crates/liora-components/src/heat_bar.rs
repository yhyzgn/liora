//! Heat Bar module.
//!
//! This public module implements the Liora heat bar component for compact intensity/threshold visualization. It keeps the reusable
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
#[cfg(not(liora_gpui_latest_api))]
use crate::gpui_compat::PixelsExt;
use gpui::{
    App, Background, BorderStyle, Bounds, Component, Corners, Edges, Hsla, IntoElement, Pixels,
    RenderOnce, SharedString, Window, div, point, prelude::*, px, quad, size,
};
use liora_core::Config;

#[derive(Clone, Debug)]
/// Data model used by heat bar item rendering.
pub struct HeatBarItem {
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Machine-readable value represented by this item.
    pub value: f64,
    /// Color token or explicit color applied to the visual element.
    pub color: Hsla,
}
impl HeatBarItem {
    /// Creates `HeatBarItem` initialized from the supplied label, value, and color.
    pub fn new(label: impl Into<SharedString>, value: f64, color: Hsla) -> Self {
        Self {
            label: label.into(),
            value,
            color,
        }
    }
}

#[derive(Clone, Debug)]
/// Fluent native GPUI component for rendering Liora heat bar legend.
pub struct HeatBarLegend {
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Count displayed by this legend item.
    pub count: usize,
    /// Color token or explicit color applied to the visual element.
    pub color: Hsla,
}
impl HeatBarLegend {
    /// Creates `HeatBarLegend` initialized from the supplied label, count, and color.
    pub fn new(label: impl Into<SharedString>, count: usize, color: Hsla) -> Self {
        Self {
            label: label.into(),
            count,
            color,
        }
    }
}

#[derive(Clone, Debug)]
/// Fluent native GPUI component for rendering Liora heat bar color range.
pub struct HeatBarColorRange {
    /// Lower bound of the numeric range.
    pub min: f64,
    /// Upper bound of the numeric range.
    pub max: f64,
    /// Color token or explicit color applied to the visual element.
    pub color: Hsla,
}

impl HeatBarColorRange {
    /// Creates `HeatBarColorRange` initialized from the supplied min, max, and color.
    pub fn new(min: f64, max: f64, color: Hsla) -> Self {
        Self { min, max, color }
    }

    /// Sets the up to value used by the component.
    pub fn up_to(max: f64, color: Hsla) -> Self {
        Self::new(f64::NEG_INFINITY, max, color)
    }

    /// Sets the above value used by the component.
    pub fn above(min: f64, color: Hsla) -> Self {
        Self::new(min, f64::INFINITY, color)
    }

    fn contains(&self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering Liora heat bar.
pub struct HeatBar {
    items: Vec<HeatBarItem>,
    legends: Vec<HeatBarLegend>,
    color_ranges: Vec<HeatBarColorRange>,
    height: Pixels,
    bar_width: Pixels,
    gap: Pixels,
    max_value: Option<f64>,
    show_axis: bool,
    x_labels: Vec<SharedString>,
}

impl HeatBar {
    /// Creates `HeatBar` that renders the supplied items collection.
    pub fn new(items: impl IntoIterator<Item = HeatBarItem>) -> Self {
        Self {
            items: items.into_iter().collect(),
            legends: Vec::new(),
            color_ranges: Vec::new(),
            height: px(180.0),
            bar_width: px(4.0),
            gap: px(3.0),
            max_value: None,
            show_axis: true,
            x_labels: Vec::new(),
        }
    }
    /// Sets the legends value used by the component.
    pub fn legends(mut self, legends: impl IntoIterator<Item = HeatBarLegend>) -> Self {
        self.legends = legends.into_iter().collect();
        self
    }
    /// Sets the color ranges value used by the component.
    pub fn color_ranges(mut self, ranges: impl IntoIterator<Item = HeatBarColorRange>) -> Self {
        self.color_ranges = ranges.into_iter().collect();
        self
    }
    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(60.0));
        self
    }
    /// Sets a fixed bar width instead of automatic band sizing.
    pub fn bar_width(mut self, width: impl Into<Pixels>) -> Self {
        self.bar_width = width.into().max(px(1.0));
        self
    }
    /// Sets the spacing between child elements.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into().max(px(0.0));
        self
    }
    /// Sets the maximum value limit.
    pub fn max_value(mut self, value: f64) -> Self {
        self.max_value = value.is_finite().then_some(value.max(1.0));
        self
    }
    /// Configures whether axis is visible in the rendered component.
    pub fn show_axis(mut self, show: bool) -> Self {
        self.show_axis = show;
        self
    }
    /// Sets the x labels value used by the component.
    pub fn x_labels(mut self, labels: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.x_labels = labels.into_iter().map(Into::into).collect();
        self
    }
}

impl RenderOnce for HeatBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let chart_width = self.bar_width * self.items.len() as f32
            + self.gap * self.items.len().saturating_sub(1) as f32
            + if self.show_axis { px(36.0) } else { px(0.0) };
        let items = self.items.clone();
        let max = self.max_value.unwrap_or_else(|| {
            items
                .iter()
                .map(|i| i.value)
                .fold(0.0_f64, f64::max)
                .max(1.0)
        });
        let bar_width = self.bar_width;
        let gap = self.gap;
        let color_ranges = self.color_ranges.clone();
        let show_axis = self.show_axis;
        let axis_color = theme.neutral.text_3;
        let grid = theme.neutral.divider.opacity(0.55);
        div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .when(!self.legends.is_empty(), |s| {
                s.child(
                    div()
                        .flex()
                        .gap_4()
                        .children(self.legends.into_iter().map(|legend| {
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(div().w(px(8.0)).h(px(8.0)).rounded_full().bg(legend.color))
                                .child(
                                    Text::new(format!("{} {}", legend.label, legend.count))
                                        .size(px(12.0)),
                                )
                        })),
                )
            })
            .child(
                gpui::canvas(
                    |_, _, _| (),
                    move |bounds, _, window, _| {
                        let left_pad = if show_axis { px(30.0) } else { px(0.0) };
                        let top = bounds.top() + px(8.0);
                        let bottom = bounds.bottom() - px(22.0);
                        let plot_h = (bottom - top).max(px(1.0));
                        if show_axis {
                            for tick in [0.0, 0.5, 1.0] {
                                let y = bottom - plot_h * tick;
                                window.paint_quad(gpui::fill(
                                    Bounds::new(
                                        point(bounds.left() + left_pad, y),
                                        size(bounds.size.width - left_pad, px(1.0)),
                                    ),
                                    Background::from(grid),
                                ));
                            }
                        }
                        for (index, item) in items.iter().enumerate() {
                            if !item.value.is_finite() {
                                continue;
                            }
                            let h = (plot_h.as_f32() * (item.value / max).clamp(0.0, 1.0) as f32)
                                .max(1.0);
                            let x = bounds.left() + left_pad + (bar_width + gap) * index as f32;
                            let rect =
                                Bounds::new(point(x, bottom - px(h)), size(bar_width, px(h)));
                            let color = color_ranges
                                .iter()
                                .find(|range| range.contains(item.value))
                                .map_or(item.color, |range| range.color);
                            window.paint_quad(quad(
                                rect,
                                Corners::all(bar_width / 2.0).clamp_radii_for_quad_size(rect.size),
                                Background::from(color),
                                Edges::all(px(0.0)),
                                gpui::transparent_black(),
                                BorderStyle::Solid,
                            ));
                        }
                    },
                )
                .w(chart_width)
                .h(self.height),
            )
            .when(!self.x_labels.is_empty(), |s| {
                s.child(
                    div()
                        .ml(if self.show_axis { px(30.0) } else { px(0.0) })
                        .flex()
                        .justify_between()
                        .text_color(axis_color)
                        .text_size(px(11.0))
                        .children(self.x_labels.into_iter().map(|l| div().child(l))),
                )
            })
    }
}

impl IntoElement for HeatBar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn heat_bar_tracks_visual_options() {
        let heat = HeatBar::new([HeatBarItem::new("10:00", 3.0, gpui::red())])
            .max_value(10.0)
            .height(px(120.0))
            .bar_width(px(5.0))
            .gap(px(2.0))
            .show_axis(false)
            .color_ranges([HeatBarColorRange::new(0.0, 5.0, gpui::blue())]);
        assert_eq!(heat.max_value, Some(10.0));
        assert_eq!(heat.height, px(120.0));
        assert_eq!(heat.bar_width, px(5.0));
        assert_eq!(heat.gap, px(2.0));
        assert!(!heat.show_axis);
        assert_eq!(heat.color_ranges.len(), 1);
    }

    #[test]
    fn heat_bar_color_ranges_match_inclusive_bounds() {
        let range = HeatBarColorRange::new(3.0, 7.0, gpui::yellow());
        assert!(!range.contains(2.9));
        assert!(range.contains(3.0));
        assert!(range.contains(7.0));
        assert!(!range.contains(7.1));
    }
}
