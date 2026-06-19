//! Sparkline module.
//!
//! This public module implements the Liora compact trend line component for inline metrics. It keeps the reusable
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

use crate::chart::{ChartLineStyle, ChartPoint, downsample_index_range};
use crate::chart_shape::{
    area_path, line_path_with_style, smooth_area_path, smooth_line_path_with_style,
};
use crate::gpui_compat::PixelsExt;
use gpui::{
    App, Background, Component, ElementId, Hsla, IntoElement, Pixels, RenderOnce, SharedString,
    Window, canvas, div, fill, point, prelude::*, px, size,
};
use liora_core::{Config, unique_id};

#[derive(Clone, Debug)]
/// Fluent native GPUI component for rendering Liora sparkline.
pub struct Sparkline {
    id: SharedString,
    values: Vec<f64>,
    height: Pixels,
    width: Option<Pixels>,
    padding: Pixels,
    color: Option<Hsla>,
    positive_color: Option<Hsla>,
    negative_color: Option<Hsla>,
    fill_color: Option<Hsla>,
    baseline_color: Option<Hsla>,
    stroke_width: Pixels,
    smooth: bool,
    area_fill: bool,
    show_last_point: bool,
    show_baseline: bool,
    y_domain: Option<(f64, f64)>,
    line_style: ChartLineStyle,
    dash_pattern: Option<Vec<Pixels>>,
    max_render_points: Option<usize>,
}

impl Sparkline {
    /// Creates `Sparkline` with default theme-driven styling and no optional callbacks attached.
    pub fn new(values: impl IntoIterator<Item = f64>) -> Self {
        Self {
            id: unique_id("sparkline"),
            values: values.into_iter().collect(),
            height: px(56.0),
            width: None,
            padding: px(4.0),
            color: None,
            positive_color: None,
            negative_color: None,
            fill_color: None,
            baseline_color: None,
            stroke_width: px(2.0),
            smooth: true,
            area_fill: false,
            show_last_point: true,
            show_baseline: false,
            y_domain: None,
            line_style: ChartLineStyle::Solid,
            dash_pattern: None,
            max_render_points: Some(240),
        }
    }

    /// Creates this value from points.
    pub fn from_points(points: impl IntoIterator<Item = ChartPoint>) -> Self {
        Self::new(points.into_iter().map(|point| point.value))
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(12.0));
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().max(px(12.0)));
        self
    }

    /// Sets inner padding on all sides of the component.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().max(px(0.0));
        self
    }

    /// Applies an explicit color instead of the theme-derived default.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the positive color used by the rendered component.
    pub fn positive_color(mut self, color: Hsla) -> Self {
        self.positive_color = Some(color);
        self
    }

    /// Sets the negative color used by the rendered component.
    pub fn negative_color(mut self, color: Hsla) -> Self {
        self.negative_color = Some(color);
        self
    }

    /// Sets the trend colors value used by the component.
    pub fn trend_colors(mut self, positive: Hsla, negative: Hsla) -> Self {
        self.positive_color = Some(positive);
        self.negative_color = Some(negative);
        self
    }

    /// Sets the optional fill color used by the chart series.
    pub fn fill_color(mut self, color: Hsla) -> Self {
        self.fill_color = Some(color);
        self.area_fill = true;
        self
    }

    /// Sets the baseline color used by the rendered component.
    pub fn baseline_color(mut self, color: Hsla) -> Self {
        self.baseline_color = Some(color);
        self.show_baseline = true;
        self
    }

    /// Sets the stroke width used for rendered chart paths.
    pub fn stroke_width(mut self, width: impl Into<Pixels>) -> Self {
        self.stroke_width = width.into().max(px(0.5));
        self
    }

    /// Toggles smoothed curve interpolation for line and area paths.
    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    /// Sets the area fill value used by the component.
    pub fn area_fill(mut self, enabled: bool) -> Self {
        self.area_fill = enabled;
        self
    }

    /// Configures whether last point is visible in the rendered component.
    pub fn show_last_point(mut self, show: bool) -> Self {
        self.show_last_point = show;
        self
    }

    /// Configures whether baseline is visible in the rendered component.
    pub fn show_baseline(mut self, show: bool) -> Self {
        self.show_baseline = show;
        self
    }

    /// Overrides automatic y-axis bounds with an explicit numeric domain.
    pub fn y_domain(mut self, min: f64, max: f64) -> Self {
        self.y_domain = Some((min, max));
        self
    }

    /// Selects solid, dashed, or dotted stroke rendering.
    pub fn line_style(mut self, style: ChartLineStyle) -> Self {
        self.line_style = style;
        if !matches!(style, ChartLineStyle::Dashed) {
            self.dash_pattern = None;
        }
        self
    }

    /// Applies the dashed preset.
    pub fn dashed(mut self) -> Self {
        self.line_style = ChartLineStyle::Dashed;
        self
    }

    /// Applies the dotted preset.
    pub fn dotted(mut self) -> Self {
        self.line_style = ChartLineStyle::Dotted;
        self
    }

    /// Sets the custom dash pattern for chart strokes.
    pub fn dash_pattern(mut self, pattern: impl IntoIterator<Item = Pixels>) -> Self {
        self.dash_pattern = Some(
            pattern
                .into_iter()
                .map(|value| value.max(px(0.1)))
                .collect(),
        );
        self.line_style = ChartLineStyle::Dashed;
        self
    }

    /// Caps the number of rendered chart points after downsampling.
    pub fn max_render_points(mut self, max_points: usize) -> Self {
        self.max_render_points = Some(max_points.max(3));
        self
    }

    /// Disables chart point downsampling for exact rendering.
    pub fn disable_downsampling(mut self) -> Self {
        self.max_render_points = None;
        self
    }

    /// Performs the values operation used by this component.
    pub fn values(&self) -> &[f64] {
        &self.values
    }

    /// Performs the trend delta operation used by this component.
    pub fn trend_delta(&self) -> Option<f64> {
        trend_delta(&self.values)
    }

    /// Performs the resolved domain operation used by this component.
    pub fn resolved_domain(&self) -> Option<(f64, f64)> {
        finite_stats(&self.values).and_then(|stats| sparkline_domain(self.y_domain, stats))
    }
}

impl IntoElement for Sparkline {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for Sparkline {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let height = self.height;
        let width = self.width;
        let Some(stats) = finite_stats(&self.values) else {
            return div()
                .id(ElementId::from(id))
                .h(height)
                .when_some(width, |s, width| s.w(width))
                .when(width.is_none(), |s| s.w_full())
                .rounded_sm()
                .bg(theme.neutral.hover.opacity(0.28))
                .into_any_element();
        };

        let domain = sparkline_domain(self.y_domain, stats).unwrap_or((0.0, 1.0));
        let positive = self.positive_color.unwrap_or(theme.success.base);
        let negative = self.negative_color.unwrap_or(theme.danger.base);
        let trend_color = if stats.last - stats.first < 0.0 {
            negative
        } else {
            positive
        };
        let line_color = self.color.unwrap_or(trend_color);
        let fill_color = self.fill_color.unwrap_or(line_color.opacity(0.18));
        let baseline_color = self
            .baseline_color
            .unwrap_or(theme.neutral.border.opacity(0.72));
        let padding = self.padding;
        let stroke_width = self.stroke_width;
        let smooth = self.smooth;
        let area_fill_enabled = self.area_fill;
        let show_last_point = self.show_last_point;
        let show_baseline = self.show_baseline;
        let line_style = self.line_style;
        let dash_pattern = self.dash_pattern.clone();
        let values = self.values.clone();
        let max_render_points = self.max_render_points;

        let chart = canvas(
            |_, _, _| (),
            move |bounds, _, window, _cx| {
                let left = bounds.left() + padding;
                let right = bounds.right() - padding;
                let top = bounds.top() + padding;
                let bottom = bounds.bottom() - padding;
                let plot_width = (right - left).max(px(1.0));
                let plot_height = (bottom - top).max(px(1.0));
                let points = sparkline_points(
                    &values,
                    domain,
                    left,
                    top,
                    plot_width,
                    plot_height,
                    max_render_points,
                );
                if points.is_empty() {
                    return;
                }

                if show_baseline {
                    let baseline_y = y_for_value(0.0, domain, top, plot_height);
                    if let Some(path) = line_path_with_style(
                        &[point(left, baseline_y), point(right, baseline_y)],
                        px(1.0),
                        ChartLineStyle::Dashed,
                        None,
                    ) {
                        window.paint_path(path, baseline_color);
                    }
                }

                if area_fill_enabled {
                    let baseline_y = if domain.0 < 0.0 && domain.1 > 0.0 {
                        y_for_value(0.0, domain, top, plot_height)
                    } else if domain.1 <= 0.0 {
                        top
                    } else {
                        bottom
                    };
                    let area = if smooth {
                        smooth_area_path(&points, baseline_y)
                    } else {
                        area_path(&points, baseline_y)
                    };
                    if let Some(path) = area {
                        window.paint_path(path, Background::from(fill_color));
                    }
                }

                if let Some(path) = if smooth {
                    smooth_line_path_with_style(
                        &points,
                        stroke_width,
                        line_style,
                        dash_pattern.as_deref(),
                    )
                } else {
                    line_path_with_style(&points, stroke_width, line_style, dash_pattern.as_deref())
                } {
                    window.paint_path(path, line_color);
                }

                if show_last_point {
                    if let Some(last_point) = points.last().copied() {
                        let radius = (stroke_width.as_f32() + 2.0).max(3.0);
                        window.paint_quad(fill(
                            gpui::Bounds::new(
                                point(last_point.x - px(radius), last_point.y - px(radius)),
                                size(px(radius * 2.0), px(radius * 2.0)),
                            ),
                            Background::from(line_color),
                        ));
                    }
                }
            },
        )
        .h(height)
        .when_some(width, |s, width| s.w(width))
        .when(width.is_none(), |s| s.w_full());

        div()
            .id(ElementId::from(id))
            .child(chart)
            .into_any_element()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct FiniteStats {
    first: f64,
    last: f64,
    min: f64,
    max: f64,
}

fn finite_stats(values: &[f64]) -> Option<FiniteStats> {
    let mut first = None;
    let mut last = 0.0;
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        first.get_or_insert(value);
        last = value;
        min = min.min(value);
        max = max.max(value);
    }
    Some(FiniteStats {
        first: first?,
        last,
        min,
        max,
    })
}

fn trend_delta(values: &[f64]) -> Option<f64> {
    let stats = finite_stats(values)?;
    Some(stats.last - stats.first)
}

fn sparkline_domain(domain: Option<(f64, f64)>, stats: FiniteStats) -> Option<(f64, f64)> {
    if let Some((min, max)) = domain.filter(|(min, max)| min.is_finite() && max.is_finite()) {
        if max > min {
            return Some((min, max));
        }
    }
    let mut min = stats.min;
    let mut max = stats.max;
    if (max - min).abs() < f64::EPSILON {
        let pad = if max.abs() < f64::EPSILON {
            1.0
        } else {
            max.abs() * 0.1
        };
        min -= pad;
        max += pad;
    }
    Some((min, max))
}

fn y_for_value(value: f64, domain: (f64, f64), top: Pixels, plot_height: Pixels) -> Pixels {
    let ratio = ((value - domain.0) / (domain.1 - domain.0)).clamp(0.0, 1.0) as f32;
    top + px(plot_height.as_f32() * (1.0 - ratio))
}

fn sparkline_points(
    values: &[f64],
    domain: (f64, f64),
    left: Pixels,
    top: Pixels,
    plot_width: Pixels,
    plot_height: Pixels,
    max_render_points: Option<usize>,
) -> Vec<gpui::Point<Pixels>> {
    let last_index = values.len().saturating_sub(1).max(1) as f32;
    downsample_index_range(values.len(), |index| values[index], max_render_points)
        .into_iter()
        .map(|(index, value)| {
            let x = left + px(plot_width.as_f32() * (index as f32 / last_index));
            let y = y_for_value(value, domain, top, plot_height);
            point(x, y)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sparkline_tracks_builder_options() {
        let chart = Sparkline::new([1.0, 2.0, 3.0])
            .id("revenue-spark")
            .height(px(72.0))
            .width(px(180.0))
            .padding(px(8.0))
            .color(gpui::blue())
            .trend_colors(gpui::green(), gpui::red())
            .fill_color(gpui::blue().opacity(0.2))
            .baseline_color(gpui::black().opacity(0.2))
            .stroke_width(px(3.0))
            .smooth(false)
            .area_fill(true)
            .show_last_point(false)
            .show_baseline(true)
            .y_domain(0.0, 10.0)
            .dashed()
            .max_render_points(120);

        assert_eq!(chart.id, SharedString::from("revenue-spark"));
        assert_eq!(chart.height, px(72.0));
        assert_eq!(chart.width, Some(px(180.0)));
        assert_eq!(chart.padding, px(8.0));
        assert_eq!(chart.stroke_width, px(3.0));
        assert!(!chart.smooth);
        assert!(chart.area_fill);
        assert!(!chart.show_last_point);
        assert!(chart.show_baseline);
        assert_eq!(chart.y_domain, Some((0.0, 10.0)));
        assert_eq!(chart.line_style, ChartLineStyle::Dashed);
        assert_eq!(chart.max_render_points, Some(120));
    }

    #[test]
    fn sparkline_domain_ignores_invalid_values_and_expands_flat_data() {
        assert_eq!(
            finite_stats(&[f64::NAN]).and_then(|stats| sparkline_domain(None, stats)),
            None
        );
        let domain = sparkline_domain(None, finite_stats(&[4.0, f64::NAN, 4.0]).unwrap()).unwrap();
        assert!(domain.0 < 4.0);
        assert!(domain.1 > 4.0);
    }

    #[test]
    fn sparkline_trend_delta_uses_finite_values() {
        let chart = Sparkline::new([f64::NAN, 10.0, 7.0]);
        assert_eq!(chart.trend_delta(), Some(-3.0));
    }

    #[test]
    fn sparkline_points_keep_single_value_visible() {
        let points = sparkline_points(
            &[5.0],
            (0.0, 10.0),
            px(0.0),
            px(0.0),
            px(100.0),
            px(50.0),
            Some(240),
        );
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].x, px(0.0));
    }

    #[test]
    fn sparkline_downsamples_dense_values() {
        let values = (0..1000).map(|index| index as f64).collect::<Vec<_>>();
        let points = sparkline_points(
            &values,
            (0.0, 1000.0),
            px(0.0),
            px(0.0),
            px(100.0),
            px(50.0),
            Some(80),
        );
        assert!(points.len() <= 80);
    }
}
