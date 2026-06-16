use crate::chart::{ChartLineStyle, ChartPoint};
use crate::chart_shape::{
    area_path, line_path_with_style, smooth_area_path, smooth_line_path_with_style,
};
use aura_core::{Config, unique_id};
use gpui::{
    App, Background, Component, ElementId, Hsla, IntoElement, Pixels, RenderOnce, SharedString,
    Window, canvas, div, fill, point, prelude::*, px, size,
};

#[derive(Clone, Debug)]
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
}

impl Sparkline {
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
        }
    }

    pub fn from_points(points: impl IntoIterator<Item = ChartPoint>) -> Self {
        Self::new(points.into_iter().map(|point| point.value))
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().max(px(12.0));
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into().max(px(12.0)));
        self
    }

    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into().max(px(0.0));
        self
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn positive_color(mut self, color: Hsla) -> Self {
        self.positive_color = Some(color);
        self
    }

    pub fn negative_color(mut self, color: Hsla) -> Self {
        self.negative_color = Some(color);
        self
    }

    pub fn trend_colors(mut self, positive: Hsla, negative: Hsla) -> Self {
        self.positive_color = Some(positive);
        self.negative_color = Some(negative);
        self
    }

    pub fn fill_color(mut self, color: Hsla) -> Self {
        self.fill_color = Some(color);
        self.area_fill = true;
        self
    }

    pub fn baseline_color(mut self, color: Hsla) -> Self {
        self.baseline_color = Some(color);
        self.show_baseline = true;
        self
    }

    pub fn stroke_width(mut self, width: impl Into<Pixels>) -> Self {
        self.stroke_width = width.into().max(px(0.5));
        self
    }

    pub fn smooth(mut self, smooth: bool) -> Self {
        self.smooth = smooth;
        self
    }

    pub fn area_fill(mut self, enabled: bool) -> Self {
        self.area_fill = enabled;
        self
    }

    pub fn show_last_point(mut self, show: bool) -> Self {
        self.show_last_point = show;
        self
    }

    pub fn show_baseline(mut self, show: bool) -> Self {
        self.show_baseline = show;
        self
    }

    pub fn y_domain(mut self, min: f64, max: f64) -> Self {
        self.y_domain = Some((min, max));
        self
    }

    pub fn line_style(mut self, style: ChartLineStyle) -> Self {
        self.line_style = style;
        if !matches!(style, ChartLineStyle::Dashed) {
            self.dash_pattern = None;
        }
        self
    }

    pub fn dashed(mut self) -> Self {
        self.line_style = ChartLineStyle::Dashed;
        self
    }

    pub fn dotted(mut self) -> Self {
        self.line_style = ChartLineStyle::Dotted;
        self
    }

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

    pub fn values(&self) -> &[f64] {
        &self.values
    }

    pub fn trend_delta(&self) -> Option<f64> {
        trend_delta(&self.values)
    }

    pub fn resolved_domain(&self) -> Option<(f64, f64)> {
        sparkline_domain(self.y_domain, &finite_values(&self.values))
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
        let finite = finite_values(&self.values);
        if finite.is_empty() {
            return div()
                .id(ElementId::from(id))
                .h(height)
                .when_some(width, |s, width| s.w(width))
                .when(width.is_none(), |s| s.w_full())
                .rounded_sm()
                .bg(theme.neutral.hover.opacity(0.28))
                .into_any_element();
        }

        let domain = sparkline_domain(self.y_domain, &finite).unwrap_or((0.0, 1.0));
        let positive = self.positive_color.unwrap_or(theme.success.base);
        let negative = self.negative_color.unwrap_or(theme.danger.base);
        let trend_color = if self.trend_delta().unwrap_or(0.0) < 0.0 {
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

        let chart = canvas(
            |_, _, _| (),
            move |bounds, _, window, _cx| {
                let left = bounds.left() + padding;
                let right = bounds.right() - padding;
                let top = bounds.top() + padding;
                let bottom = bounds.bottom() - padding;
                let plot_width = (right - left).max(px(1.0));
                let plot_height = (bottom - top).max(px(1.0));
                let finite = finite_values(&values);
                let points = sparkline_points(&values, domain, left, top, plot_width, plot_height);
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

                let _ = finite;
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

fn finite_values(values: &[f64]) -> Vec<f64> {
    values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .collect()
}

fn trend_delta(values: &[f64]) -> Option<f64> {
    let finite = finite_values(values);
    Some(finite.last()? - finite.first()?)
}

fn sparkline_domain(domain: Option<(f64, f64)>, values: &[f64]) -> Option<(f64, f64)> {
    if let Some((min, max)) = domain.filter(|(min, max)| min.is_finite() && max.is_finite()) {
        if max > min {
            return Some((min, max));
        }
    }
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for value in values.iter().copied().filter(|value| value.is_finite()) {
        min = min.min(value);
        max = max.max(value);
    }
    if !min.is_finite() || !max.is_finite() {
        return None;
    }
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
) -> Vec<gpui::Point<Pixels>> {
    let finite = finite_values(values);
    let last_index = finite.len().saturating_sub(1).max(1) as f32;
    finite
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let x = left + px(plot_width.as_f32() * (index as f32 / last_index));
            let y = y_for_value(*value, domain, top, plot_height);
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
            .dashed();

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
    }

    #[test]
    fn sparkline_domain_ignores_invalid_values_and_expands_flat_data() {
        assert_eq!(sparkline_domain(None, &[f64::NAN]), None);
        let domain = sparkline_domain(None, &[4.0, f64::NAN, 4.0]).unwrap();
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
        let points = sparkline_points(&[5.0], (0.0, 10.0), px(0.0), px(0.0), px(100.0), px(50.0));
        assert_eq!(points.len(), 1);
        assert_eq!(points[0].x, px(0.0));
    }
}
