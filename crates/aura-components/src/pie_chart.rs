use crate::chart::{ChartPalette, ChartSeries, default_y_format, has_chart_data};
use crate::chart_frame::paint_chart_label_aligned;
use crate::{Empty, Space, Text};
use aura_core::{Config, unique_id};
use gpui::{
    App, Component, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, Point,
    RenderOnce, SharedString, Styled, Window, canvas, div, point, px,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieChartLabelOptions {
    pub show_percentage: bool,
    pub percentage_decimals: usize,
    pub outside_threshold_degrees: u16,
}

impl Default for PieChartLabelOptions {
    fn default() -> Self {
        Self {
            show_percentage: true,
            percentage_decimals: 1,
            outside_threshold_degrees: 28,
        }
    }
}

#[derive(Clone)]
pub struct PieChart {
    slices: Vec<ChartSeries>,
    id: SharedString,
    height: Pixels,
    show_legend: bool,
    show_value_labels: bool,
    label_options: PieChartLabelOptions,
}

#[derive(Clone)]
pub struct RingChart {
    slices: Vec<ChartSeries>,
    id: SharedString,
    height: Pixels,
    show_legend: bool,
    show_value_labels: bool,
    label_options: PieChartLabelOptions,
    inner_ratio: f32,
}

impl PieChart {
    pub fn new(slices: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            slices: slices.into_iter().collect(),
            id: unique_id("pie-chart"),
            height: px(280.0),
            show_legend: true,
            show_value_labels: true,
            label_options: PieChartLabelOptions::default(),
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn show_value_labels(mut self, show: bool) -> Self {
        self.show_value_labels = show;
        self
    }

    pub fn show_percentage_labels(mut self, show: bool) -> Self {
        self.label_options.show_percentage = show;
        self
    }

    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.label_options.percentage_decimals = decimals.min(4);
        self
    }

    pub fn outside_label_threshold_degrees(mut self, degrees: u16) -> Self {
        self.label_options.outside_threshold_degrees = degrees.min(120);
        self
    }

    pub fn label_options(&self) -> &PieChartLabelOptions {
        &self.label_options
    }

    pub fn slices(&self) -> &[ChartSeries] {
        &self.slices
    }
}

impl RingChart {
    pub fn new(slices: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            slices: slices.into_iter().collect(),
            id: unique_id("ring-chart"),
            height: px(280.0),
            show_legend: true,
            show_value_labels: true,
            label_options: PieChartLabelOptions::default(),
            inner_ratio: 0.62,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.show_legend = show;
        self
    }

    pub fn show_value_labels(mut self, show: bool) -> Self {
        self.show_value_labels = show;
        self
    }

    pub fn show_percentage_labels(mut self, show: bool) -> Self {
        self.label_options.show_percentage = show;
        self
    }

    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.label_options.percentage_decimals = decimals.min(4);
        self
    }

    pub fn outside_label_threshold_degrees(mut self, degrees: u16) -> Self {
        self.label_options.outside_threshold_degrees = degrees.min(120);
        self
    }

    pub fn label_options(&self) -> &PieChartLabelOptions {
        &self.label_options
    }

    pub fn inner_ratio(mut self, ratio: f32) -> Self {
        self.inner_ratio = ratio.clamp(0.2, 0.9);
        self
    }

    pub fn slices(&self) -> &[ChartSeries] {
        &self.slices
    }
}

impl IntoElement for PieChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl IntoElement for RingChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for PieChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        render_shell(
            self.slices,
            self.id,
            self.height,
            self.show_legend,
            self.show_value_labels,
            self.label_options,
            0.0,
            cx,
        )
    }
}

impl RenderOnce for RingChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        render_shell(
            self.slices,
            self.id,
            self.height,
            self.show_legend,
            self.show_value_labels,
            self.label_options,
            self.inner_ratio,
            cx,
        )
    }
}

fn render_shell(
    slices: Vec<ChartSeries>,
    id: SharedString,
    height: Pixels,
    show_legend: bool,
    show_value_labels: bool,
    label_options: PieChartLabelOptions,
    inner_ratio: f32,
    cx: &mut App,
) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();
    let palette = ChartPalette::from_config(cx.global::<Config>());
    let has_data = has_chart_data(&slices);

    let mut shell = div()
        .id(ElementId::from(id))
        .flex()
        .flex_col()
        .gap_2()
        .w_full()
        .p_3()
        .rounded_md()
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card);

    if !has_data {
        return shell
            .h(height)
            .items_center()
            .justify_center()
            .child(Empty::new().description("暂无图表数据"))
            .into_any_element();
    }

    if show_legend {
        shell = shell.child(render_legend(&slices, &palette));
    }

    shell
        .child(render_canvas(
            slices,
            palette,
            theme.neutral.card,
            inner_ratio,
            show_value_labels,
            label_options,
            height,
        ))
        .into_any_element()
}

fn render_legend(series: &[ChartSeries], palette: &ChartPalette) -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .children(series.iter().enumerate().map(|(index, series)| {
            let color = series.color.unwrap_or_else(|| palette.series_color(index));
            Space::new()
                .gap_xs()
                .align_center()
                .child(div().w(px(10.0)).h(px(10.0)).rounded_sm().bg(color))
                .child(Text::new(series.name.clone()).size(px(12.0)))
        }))
}

fn render_canvas(
    slices: Vec<ChartSeries>,
    palette: ChartPalette,
    hole_color: Hsla,
    inner_ratio: f32,
    show_value_labels: bool,
    label_options: PieChartLabelOptions,
    height: Pixels,
) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            let inset = if show_value_labels {
                px(56.0)
            } else {
                px(18.0)
            };
            let width = (bounds.right() - bounds.left() - inset * 2.0).max(px(1.0));
            let height = (bounds.bottom() - bounds.top() - inset * 2.0).max(px(1.0));
            let radius = (width.min(height).as_f32() / 2.0).max(1.0);
            let center = point(
                bounds.left() + width / 2.0 + inset,
                bounds.top() + height / 2.0 + inset,
            );

            let values = slices
                .iter()
                .map(|series| {
                    series
                        .finite_points()
                        .next()
                        .map(|point| point.value.max(0.0))
                        .unwrap_or(0.0)
                })
                .collect::<Vec<_>>();
            let total: f64 = values.iter().sum();
            if total <= f64::EPSILON {
                return;
            }

            let mut slice_labels = Vec::new();
            let mut start = -90.0_f32;
            for (index, (series, value)) in slices.iter().zip(values).enumerate() {
                if value <= 0.0 {
                    continue;
                }
                let sweep = (value / total) as f32 * 360.0;
                let color = series.color.unwrap_or_else(|| palette.series_color(index));
                let end = start + sweep;
                if let Some(path) = pie_slice_path(center, radius, start, end) {
                    window.paint_path(path, color);
                }
                slice_labels.push(SliceLabel {
                    start_deg: start,
                    end_deg: end,
                    value,
                    total,
                    color,
                });
                start = end;
            }

            if inner_ratio > 0.0 {
                let hole_radius = (radius * inner_ratio).max(0.0);
                if let Some(path) = circle_path(center, hole_radius) {
                    window.paint_path(path, hole_color);
                }
            }

            if show_value_labels {
                for label in slice_labels {
                    paint_slice_value_label(
                        center,
                        radius,
                        inner_ratio,
                        label,
                        &label_options,
                        &palette,
                        window,
                        cx,
                    );
                }
            }
        },
    )
    .w_full()
    .h(height)
}

#[derive(Clone, Copy)]
struct SliceLabel {
    start_deg: f32,
    end_deg: f32,
    value: f64,
    total: f64,
    color: Hsla,
}

fn paint_slice_value_label(
    center: Point<Pixels>,
    radius: f32,
    inner_ratio: f32,
    label: SliceLabel,
    options: &PieChartLabelOptions,
    palette: &ChartPalette,
    window: &mut Window,
    cx: &mut App,
) {
    let sweep = (label.end_deg - label.start_deg).abs();
    if sweep <= f32::EPSILON {
        return;
    }

    let mid_deg = (label.start_deg + label.end_deg) * 0.5;
    let text = format_slice_label(label.value, label.total, options);
    if sweep < options.outside_threshold_degrees as f32 {
        paint_outside_slice_label(
            center,
            radius,
            mid_deg,
            text,
            label.color,
            palette,
            window,
            cx,
        );
        return;
    }

    let label_radius = if inner_ratio > 0.0 {
        radius * (inner_ratio + 1.0) * 0.5
    } else {
        radius * 0.62
    };
    let position = polar_point(center, label_radius, mid_deg);
    paint_chart_label_aligned(
        text,
        point(position.x - px(36.0), position.y - px(7.0)),
        gpui::white(),
        gpui::TextAlign::Center,
        Some(px(72.0)),
        window,
        cx,
    );
}

fn paint_outside_slice_label(
    center: Point<Pixels>,
    radius: f32,
    mid_deg: f32,
    text: SharedString,
    color: Hsla,
    palette: &ChartPalette,
    window: &mut Window,
    cx: &mut App,
) {
    let edge = polar_point(center, radius, mid_deg);
    let elbow = polar_point(center, radius + 14.0, mid_deg);
    let right_side = mid_deg.to_radians().cos() >= 0.0;
    let label_anchor = point(
        elbow.x + if right_side { px(34.0) } else { px(-34.0) },
        elbow.y,
    );

    if let Some(path) = leader_line_path(edge, elbow, label_anchor) {
        window.paint_path(path, color.opacity(0.82));
    }

    let (origin, align) = if right_side {
        (
            point(label_anchor.x + px(4.0), label_anchor.y - px(7.0)),
            gpui::TextAlign::Left,
        )
    } else {
        (
            point(label_anchor.x - px(76.0), label_anchor.y - px(7.0)),
            gpui::TextAlign::Right,
        )
    };
    paint_chart_label_aligned(
        text,
        origin,
        palette.label,
        align,
        Some(px(72.0)),
        window,
        cx,
    );
}

fn leader_line_path(
    edge: Point<Pixels>,
    elbow: Point<Pixels>,
    label_anchor: Point<Pixels>,
) -> Option<gpui::Path<Pixels>> {
    let mut builder = gpui::PathBuilder::stroke(px(1.2));
    builder.move_to(edge);
    builder.line_to(elbow);
    builder.line_to(label_anchor);
    builder.build().ok()
}

fn format_slice_label(value: f64, total: f64, options: &PieChartLabelOptions) -> SharedString {
    let base = format!("{} / {}", default_y_format(value), default_y_format(total));
    if options.show_percentage {
        let percentage = if total > f64::EPSILON {
            value / total * 100.0
        } else {
            0.0
        };
        format!("{} ({:.*}%)", base, options.percentage_decimals, percentage).into()
    } else {
        base.into()
    }
}

fn pie_slice_path(
    center: Point<Pixels>,
    radius: f32,
    start_deg: f32,
    end_deg: f32,
) -> Option<gpui::Path<Pixels>> {
    if radius <= 0.0 || !radius.is_finite() || !start_deg.is_finite() || !end_deg.is_finite() {
        return None;
    }

    let sweep_deg = end_deg - start_deg;
    if sweep_deg.abs() >= 359.999 {
        return circle_path(center, radius);
    }

    let start = polar_point(center, radius, start_deg);
    let mut builder = gpui::PathBuilder::fill();
    builder.move_to(center);
    builder.line_to(start);
    append_arc(&mut builder, center, radius, start_deg, end_deg);
    builder.line_to(center);
    builder.close();
    Some(builder.build().ok()?)
}

fn circle_path(center: Point<Pixels>, radius: f32) -> Option<gpui::Path<Pixels>> {
    if radius <= 0.0 || !radius.is_finite() {
        return None;
    }

    let start = polar_point(center, radius, -90.0);
    let mid = polar_point(center, radius, 90.0);
    let mut builder = gpui::PathBuilder::fill();
    builder.move_to(start);
    builder.arc_to(point(px(radius), px(radius)), px(0.0), false, true, mid);
    builder.arc_to(point(px(radius), px(radius)), px(0.0), false, true, start);
    builder.close();
    builder.build().ok()
}

fn append_arc(
    builder: &mut gpui::PathBuilder,
    center: Point<Pixels>,
    radius: f32,
    start_deg: f32,
    end_deg: f32,
) {
    let sweep_deg = end_deg - start_deg;
    let large_arc = sweep_deg.abs() > 180.0;
    let sweep = sweep_deg >= 0.0;
    let end = polar_point(center, radius, end_deg);
    builder.arc_to(
        point(px(radius), px(radius)),
        px(0.0),
        large_arc,
        sweep,
        end,
    );
}

fn polar_point(center: Point<Pixels>, radius: f32, deg: f32) -> Point<Pixels> {
    let radians = deg.to_radians();
    point(
        center.x + px(radius * radians.cos()),
        center.y + px(radius * radians.sin()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::ChartPoint;

    fn slices() -> Vec<ChartSeries> {
        vec![
            ChartSeries::new("A", [ChartPoint::new("A", 30.0)]),
            ChartSeries::new("B", [ChartPoint::new("B", 20.0)]),
            ChartSeries::new("C", [ChartPoint::new("C", 50.0)]),
        ]
    }

    #[test]
    fn pie_chart_tracks_slices() {
        let chart = PieChart::new(slices())
            .id("pie")
            .height(px(240.0))
            .show_legend(false)
            .show_value_labels(false)
            .show_percentage_labels(false)
            .percentage_decimals(2)
            .outside_label_threshold_degrees(36);
        assert_eq!(chart.slices().len(), 3);
        assert!(!chart.show_value_labels);
        assert!(!chart.label_options().show_percentage);
        assert_eq!(chart.label_options().percentage_decimals, 2);
        assert_eq!(chart.label_options().outside_threshold_degrees, 36);
    }

    #[test]
    fn ring_chart_tracks_inner_ratio() {
        let chart = RingChart::new(slices())
            .inner_ratio(0.5)
            .show_value_labels(false)
            .percentage_decimals(3);
        assert_eq!(chart.slices().len(), 3);
        assert!(chart.inner_ratio >= 0.2 && chart.inner_ratio <= 0.9);
        assert!(!chart.show_value_labels);
        assert_eq!(chart.label_options().percentage_decimals, 3);
    }

    #[test]
    fn slice_labels_use_value_total_and_configurable_percentage_precision() {
        let options = PieChartLabelOptions {
            percentage_decimals: 2,
            ..PieChartLabelOptions::default()
        };
        assert_eq!(
            format_slice_label(1.0, 3.0, &options),
            SharedString::from("1 / 3 (33.33%)")
        );

        let options = PieChartLabelOptions {
            show_percentage: false,
            ..PieChartLabelOptions::default()
        };
        assert_eq!(
            format_slice_label(1.0, 3.0, &options),
            SharedString::from("1 / 3")
        );
    }
}
