use crate::chart::{
    ChartOptions, ChartPalette, ChartSeries, ChartValueLabelContent, ChartValueLabelPlacement,
    collect_axis_labels, collect_labels, format_value_label, has_chart_data, normalized_domain,
    series_total, stacked_domain,
};
use crate::chart_frame::{paint_chart_frame, paint_chart_label_aligned};
use crate::chart_scale::{ScaleBand, ScaleLinear, ScalePoint};
use crate::{Empty, Space, Text};
use aura_core::{Config, unique_id};
use gpui::{
    App, Background, BorderStyle, Bounds, Component, Corners, Edges, ElementId, Hsla,
    InteractiveElement, IntoElement, ParentElement, Pixels, RenderOnce, SharedString, Styled,
    Window, canvas, div, fill, point, prelude::*, px, quad, size,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BarChartMode {
    Grouped,
    Stacked,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BarChartValueColorRange {
    pub min: f64,
    pub max: f64,
    pub color: Hsla,
}

impl BarChartValueColorRange {
    pub fn new(min: f64, max: f64, color: Hsla) -> Self {
        Self { min, max, color }
    }

    fn contains(self, value: f64) -> bool {
        value >= self.min && value <= self.max
    }
}

#[derive(Clone)]
pub struct BarChart {
    series: Vec<ChartSeries>,
    options: ChartOptions,
    mode: BarChartMode,
    bar_gap_ratio: f32,
    standalone: bool,
    bar_radius: Pixels,
    bar_width: Option<Pixels>,
    bar_gap: Option<Pixels>,
    value_color_ranges: Vec<BarChartValueColorRange>,
}

impl BarChart {
    pub fn new(series: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            series: series.into_iter().collect(),
            options: ChartOptions {
                id: unique_id("bar-chart"),
                ..ChartOptions::default()
            },
            mode: BarChartMode::Grouped,
            bar_gap_ratio: 0.18,
            standalone: false,
            bar_radius: px(0.0),
            bar_width: None,
            bar_gap: None,
            value_color_ranges: Vec::new(),
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.options.id = id.into();
        self
    }

    pub fn height(mut self, height: Pixels) -> Self {
        self.options.height = height;
        self
    }

    pub fn show_grid(mut self, show: bool) -> Self {
        self.options.show_grid = show;
        self
    }

    pub fn show_axis(mut self, show: bool) -> Self {
        self.options.show_axis = show;
        self
    }

    pub fn show_legend(mut self, show: bool) -> Self {
        self.options.show_legend = show;
        self
    }

    pub fn y_domain(mut self, min: f64, max: f64) -> Self {
        self.options.y_domain = Some((min, max));
        self
    }

    pub fn y_format(mut self, formatter: fn(f64) -> SharedString) -> Self {
        self.options.y_format = Some(formatter);
        self
    }

    pub fn show_value_labels(mut self, show: bool) -> Self {
        self.options.show_value_labels = show;
        self
    }

    pub fn value_label_content(mut self, content: ChartValueLabelContent) -> Self {
        self.options.value_label_options.content = content;
        self
    }

    pub fn value_label_placement(mut self, placement: ChartValueLabelPlacement) -> Self {
        self.options.value_label_options.placement = placement;
        self
    }

    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.options.value_label_options.percentage_decimals = decimals.min(4);
        self
    }

    pub fn bar_gap_ratio(mut self, ratio: f32) -> Self {
        self.bar_gap_ratio = ratio.clamp(0.0, 0.8);
        self
    }

    pub fn max_axis_labels(mut self, max_labels: usize) -> Self {
        self.options.max_axis_labels = max_labels.max(2);
        self
    }

    pub fn max_value_labels(mut self, max_labels: usize) -> Self {
        self.options.max_value_labels = max_labels.max(2);
        self
    }

    pub fn standalone(mut self) -> Self {
        self.standalone = true;
        self.options.show_grid = false;
        self.options.show_axis = false;
        self.options.show_legend = false;
        self.options.show_value_labels = false;
        self.options.padding = crate::chart::ChartPadding {
            top: px(6.0),
            right: px(6.0),
            bottom: px(6.0),
            left: px(6.0),
        };
        self.options.height = px(112.0);
        self.bar_radius = px(4.0);
        self
    }

    pub fn bar_radius(mut self, radius: Pixels) -> Self {
        self.bar_radius = radius.max(px(0.0));
        self
    }

    pub fn bar_width(mut self, width: Pixels) -> Self {
        self.bar_width = Some(width.max(px(1.0)));
        self
    }

    pub fn bar_gap(mut self, gap: Pixels) -> Self {
        self.bar_gap = Some(gap.max(px(0.0)));
        self
    }

    pub fn value_color_ranges(
        mut self,
        ranges: impl IntoIterator<Item = BarChartValueColorRange>,
    ) -> Self {
        self.value_color_ranges = ranges.into_iter().collect();
        self
    }

    pub fn grouped(mut self) -> Self {
        self.mode = BarChartMode::Grouped;
        self
    }

    pub fn stacked(mut self) -> Self {
        self.mode = BarChartMode::Stacked;
        self
    }

    pub fn mode(mut self, mode: BarChartMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn series(&self) -> &[ChartSeries] {
        &self.series
    }

    pub fn options(&self) -> &ChartOptions {
        &self.options
    }

    pub fn bar_mode(&self) -> BarChartMode {
        self.mode
    }

    pub fn is_standalone(&self) -> bool {
        self.standalone
    }

    pub fn bar_radius_value(&self) -> Pixels {
        self.bar_radius
    }
}

#[derive(Clone)]
struct BarPaintOptions {
    radius: Pixels,
    width: Option<Pixels>,
    gap: Option<Pixels>,
    value_color_ranges: Vec<BarChartValueColorRange>,
}

impl BarPaintOptions {
    fn resolve_color(&self, value: f64, fallback: Hsla) -> Hsla {
        self.value_color_ranges
            .iter()
            .copied()
            .find(|range| range.contains(value))
            .map(|range| range.color)
            .unwrap_or(fallback)
    }
}

fn paint_bar(window: &mut Window, bounds: Bounds<Pixels>, color: Hsla, radius: Pixels) {
    if radius > px(0.0) {
        window.paint_quad(quad(
            bounds,
            Corners::all(radius).clamp_radii_for_quad_size(bounds.size),
            Background::from(color),
            Edges::all(px(0.0)),
            gpui::transparent_black(),
            BorderStyle::Solid,
        ));
    } else {
        window.paint_quad(fill(bounds, Background::from(color)));
    }
}

impl IntoElement for BarChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for BarChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let palette = ChartPalette::from_config(cx.global::<Config>());
        let has_data = has_chart_data(&self.series);
        let height = self.options.height;
        let id = self.options.id.clone();

        let mut shell = div()
            .id(ElementId::from(id.clone()))
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .when(!self.standalone, |s| {
                s.p_3()
                    .rounded_md()
                    .border_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.card)
            });

        if !has_data {
            return shell
                .h(height)
                .items_center()
                .justify_center()
                .child(Empty::new().description("暂无图表数据"))
                .into_any_element();
        }

        if self.options.show_legend {
            shell = shell.child(render_legend(&self.series, &palette));
        }

        shell
            .child(render_bar_canvas(
                self.series,
                self.options,
                palette,
                self.mode,
                self.bar_gap_ratio,
                BarPaintOptions {
                    radius: self.bar_radius,
                    width: self.bar_width,
                    gap: self.bar_gap,
                    value_color_ranges: self.value_color_ranges,
                },
            ))
            .into_any_element()
    }
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

fn render_bar_canvas(
    series: Vec<ChartSeries>,
    options: ChartOptions,
    palette: ChartPalette,
    mode: BarChartMode,
    bar_gap_ratio: f32,
    paint_options: BarPaintOptions,
) -> impl IntoElement {
    let height = options.height;
    canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            let labels = collect_labels(&series);
            if labels.is_empty() {
                return;
            }

            let padding = options.padding;
            let left = bounds.left() + padding.left;
            let right = bounds.right() - padding.right;
            let top = bounds.top() + padding.top;
            let bottom = bounds.bottom() - padding.bottom;
            let width = (right - left).max(px(1.0));
            let plot_height = (bottom - top).max(px(1.0));

            let frame_x = ScalePoint::new(labels.clone(), (0.0, width.as_f32()));
            let band = ScaleBand::new(labels.clone(), (0.0, width.as_f32()))
                .padding_inner(bar_gap_ratio)
                .padding_outer((bar_gap_ratio * 0.58).max(0.02));
            let domain = if mode == BarChartMode::Stacked {
                options
                    .y_domain
                    .or_else(|| stacked_domain(&series))
                    .map(|domain| normalized_domain(Some(domain), &[]))
                    .unwrap_or_else(|| normalized_domain(None, &series))
            } else {
                normalized_domain(options.y_domain, &series)
            };
            let y = ScaleLinear::new(domain, (plot_height.as_f32(), 0.0));
            if options.show_grid || options.show_axis {
                paint_chart_frame(
                    left,
                    top,
                    width,
                    plot_height,
                    &collect_axis_labels(&series, options.max_axis_labels),
                    &frame_x,
                    &y,
                    &palette,
                    &options,
                    window,
                    cx,
                );
            }

            match mode {
                BarChartMode::Grouped => paint_grouped_bars(
                    left,
                    top,
                    plot_height,
                    &series,
                    &band,
                    &y,
                    &palette,
                    &options,
                    &paint_options,
                    window,
                    cx,
                ),
                BarChartMode::Stacked => paint_stacked_bars(
                    left,
                    top,
                    plot_height,
                    &series,
                    &band,
                    &y,
                    &palette,
                    &options,
                    &paint_options,
                    window,
                    cx,
                ),
            }
        },
    )
    .w_full()
    .h(height)
}

fn paint_grouped_bars(
    left: Pixels,
    top: Pixels,
    plot_height: Pixels,
    series: &[ChartSeries],
    band: &ScaleBand,
    y: &ScaleLinear,
    palette: &ChartPalette,
    options: &ChartOptions,
    paint_options: &BarPaintOptions,
    window: &mut Window,
    cx: &mut App,
) {
    let baseline = y.tick(0.0).clamp(0.0, plot_height.as_f32());
    let series_count = series.len().max(1) as f32;
    let group_width = band.band_width().max(1.0);
    let default_width = (group_width / series_count * 0.82).max(1.0);
    let bar_width = paint_options
        .width
        .map(|width| width.as_f32().min(group_width / series_count).max(1.0))
        .unwrap_or(default_width);
    let gap = paint_options
        .gap
        .map(|gap| gap.as_f32())
        .unwrap_or_else(|| (group_width / series_count - bar_width).max(0.0));

    for (series_index, current) in series.iter().enumerate() {
        for (point_index, chart_point) in current.points.iter().enumerate() {
            if !chart_point.is_finite() {
                continue;
            }
            let Some(group_x) = band.tick_index(point_index) else {
                continue;
            };
            let color = paint_options.resolve_color(
                chart_point.value,
                current.resolved_fill_color(palette.series_color(series_index)),
            );
            let value_y = y.tick(chart_point.value).clamp(0.0, plot_height.as_f32());
            let top_y = baseline.min(value_y);
            let height = (baseline - value_y).abs().max(1.0);
            let x = group_x + series_index as f32 * (bar_width + gap) + gap * 0.5;
            paint_bar(
                window,
                Bounds::new(
                    point(left + px(x), top + px(top_y)),
                    size(px(bar_width), px(height)),
                ),
                color,
                paint_options.radius,
            );
            if options.show_value_labels {
                let label_y = if chart_point.value >= 0.0 {
                    top_y - 17.0
                } else {
                    top_y + height + 3.0
                };
                paint_chart_label_aligned(
                    format_value_label(
                        chart_point.value,
                        series_total(current),
                        options.y_format,
                        &options.value_label_options,
                    ),
                    point(left + px(x + bar_width * 0.5 - 24.0), top + px(label_y)),
                    palette.label,
                    gpui::TextAlign::Center,
                    Some(px(48.0)),
                    window,
                    cx,
                );
            }
        }
    }
}

fn paint_stacked_bars(
    left: Pixels,
    top: Pixels,
    plot_height: Pixels,
    series: &[ChartSeries],
    band: &ScaleBand,
    y: &ScaleLinear,
    palette: &ChartPalette,
    options: &ChartOptions,
    paint_options: &BarPaintOptions,
    window: &mut Window,
    cx: &mut App,
) {
    let baseline = y.tick(0.0).clamp(0.0, plot_height.as_f32());
    let labels_len = series
        .iter()
        .map(|series| series.points.len())
        .max()
        .unwrap_or(0);
    for point_index in 0..labels_len {
        let Some(group_x) = band.tick_index(point_index) else {
            continue;
        };
        let mut positive_base = 0.0_f64;
        let mut negative_base = 0.0_f64;
        for (series_index, current) in series.iter().enumerate() {
            let Some(chart_point) = current.points.get(point_index) else {
                continue;
            };
            if !chart_point.is_finite() {
                continue;
            }
            let color = paint_options.resolve_color(
                chart_point.value,
                current.resolved_fill_color(palette.series_color(series_index)),
            );
            let (from, to) = if chart_point.value >= 0.0 {
                let from = positive_base;
                positive_base += chart_point.value;
                (from, positive_base)
            } else {
                let from = negative_base;
                negative_base += chart_point.value;
                (from, negative_base)
            };
            let y0 = y.tick(from).clamp(0.0, plot_height.as_f32());
            let y1 = y.tick(to).clamp(0.0, plot_height.as_f32());
            let top_y = y0.min(y1).min(baseline.max(y1));
            let height = (y0 - y1).abs().max(1.0);
            let width = paint_options
                .width
                .map(|width| width.as_f32().min(band.band_width()).max(1.0))
                .unwrap_or_else(|| band.band_width().max(1.0));
            let x = group_x + (band.band_width().max(1.0) - width) * 0.5;
            paint_bar(
                window,
                Bounds::new(
                    point(left + px(x), top + px(top_y)),
                    size(px(width), px(height)),
                ),
                color,
                paint_options.radius,
            );
            if options.show_value_labels {
                paint_chart_label_aligned(
                    format_value_label(
                        chart_point.value,
                        series_total(current),
                        options.y_format,
                        &options.value_label_options,
                    ),
                    point(
                        left + px(group_x + band.band_width().max(1.0) * 0.5 - 24.0),
                        top + px(top_y + height * 0.5 - 7.0),
                    ),
                    gpui::white(),
                    gpui::TextAlign::Center,
                    Some(px(48.0)),
                    window,
                    cx,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::ChartPoint;

    fn sample_series() -> Vec<ChartSeries> {
        vec![
            ChartSeries::new(
                "Revenue",
                [ChartPoint::new("Q1", 12.0), ChartPoint::new("Q2", 18.0)],
            ),
            ChartSeries::new(
                "Cost",
                [ChartPoint::new("Q1", 7.0), ChartPoint::new("Q2", 9.0)],
            ),
        ]
    }

    #[test]
    fn bar_chart_builder_tracks_options_and_mode() {
        let chart = BarChart::new(sample_series())
            .id("sales-bars")
            .height(px(320.0))
            .show_grid(false)
            .show_axis(false)
            .show_legend(false)
            .y_domain(0.0, 100.0)
            .show_value_labels(false)
            .value_label_content(ChartValueLabelContent::ValueAndPercentage)
            .value_label_placement(ChartValueLabelPlacement::Inside)
            .percentage_decimals(2)
            .bar_gap_ratio(0.3)
            .bar_radius(px(3.0))
            .bar_width(px(8.0))
            .bar_gap(px(4.0))
            .value_color_ranges([BarChartValueColorRange::new(0.0, 50.0, gpui::green())])
            .stacked();

        assert_eq!(chart.options().id, SharedString::from("sales-bars"));
        assert_eq!(chart.options().height, px(320.0));
        assert!(!chart.options().show_grid);
        assert!(!chart.options().show_axis);
        assert!(!chart.options().show_legend);
        assert_eq!(chart.options().y_domain, Some((0.0, 100.0)));
        assert!(!chart.options().show_value_labels);
        assert_eq!(
            chart.options().value_label_options.content,
            ChartValueLabelContent::ValueAndPercentage
        );
        assert_eq!(
            chart.options().value_label_options.placement,
            ChartValueLabelPlacement::Inside
        );
        assert_eq!(chart.options().value_label_options.percentage_decimals, 2);
        assert_eq!(chart.bar_gap_ratio, 0.3);
        assert_eq!(chart.bar_radius_value(), px(3.0));
        assert_eq!(chart.bar_width, Some(px(8.0)));
        assert_eq!(chart.bar_gap, Some(px(4.0)));
        assert_eq!(chart.value_color_ranges.len(), 1);
        assert_eq!(chart.bar_mode(), BarChartMode::Stacked);
    }

    #[test]
    fn bar_chart_keeps_series_data() {
        let chart = BarChart::new(sample_series());
        assert_eq!(chart.series().len(), 2);
        assert_eq!(chart.series()[0].name, SharedString::from("Revenue"));
    }

    #[test]
    fn standalone_mode_disables_chart_chrome() {
        let chart = BarChart::new(sample_series()).standalone();
        assert!(chart.is_standalone());
        assert!(!chart.options().show_grid);
        assert!(!chart.options().show_axis);
        assert!(!chart.options().show_legend);
        assert!(!chart.options().show_value_labels);
        assert_eq!(chart.bar_radius_value(), px(4.0));
    }
}
