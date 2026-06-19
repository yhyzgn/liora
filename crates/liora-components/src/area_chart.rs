use crate::chart::{
    ChartBoundsTracker, ChartOptions, ChartPalette, ChartSeries, ChartValueLabelContent,
    ChartValueLabelPlacement, collect_axis_labels, downsample_index_range,
    downsample_indexed_values, format_hit_tooltip, format_value_label, has_chart_data,
    label_domain_len, nearest_cartesian_hit_point, normalized_domain_with_baseline, series_total,
    sparse_indices, stacked_domain,
};
use crate::chart_frame::{paint_chart_frame, paint_chart_label_aligned};
use crate::chart_scale::{ScaleLinear, ScalePoint};
use crate::chart_shape::{
    area_path, finite_line_points, line_path, line_soft_edge_path, smooth_area_path,
    smooth_line_path,
};
use crate::gpui_compat::PixelsExt;
use crate::{Empty, Space, Text};
use gpui::{
    App, Bounds, Component, ElementId, InteractiveElement, IntoElement, ParentElement, Pixels,
    RenderOnce, SharedString, Styled, Window, canvas, div, point, px, size,
};
use liora_core::{Config, Placement, TooltipData, clear_tooltip, set_active_tooltip, unique_id};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AreaChartMode {
    Overlay,
    Stacked,
}

#[derive(Clone)]
pub struct AreaChart {
    series: Vec<ChartSeries>,
    options: ChartOptions,
    mode: AreaChartMode,
    line_stroke: bool,
    smooth: bool,
    stroke_width: Pixels,
}

impl AreaChart {
    pub fn new(series: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            series: series.into_iter().collect(),
            options: ChartOptions {
                id: unique_id("area-chart"),
                ..ChartOptions::default()
            },
            mode: AreaChartMode::Overlay,
            line_stroke: true,
            smooth: false,
            stroke_width: px(2.0),
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.options.id = id.into();
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.options.height = height.into();
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

    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.options.show_tooltip = show;
        self
    }

    pub fn tooltip_hit_radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.options.tooltip_hit_radius = radius.into().max(px(0.0));
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

    pub fn smooth(mut self, enabled: bool) -> Self {
        self.smooth = enabled;
        self
    }

    pub fn stroke_width(mut self, width: impl Into<Pixels>) -> Self {
        self.stroke_width = width.into();
        self
    }

    pub fn max_render_points(mut self, max_points: usize) -> Self {
        self.options.max_render_points = Some(max_points.max(3));
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

    pub fn disable_downsampling(mut self) -> Self {
        self.options.max_render_points = None;
        self
    }

    pub fn overlay(mut self) -> Self {
        self.mode = AreaChartMode::Overlay;
        self
    }

    pub fn stacked(mut self) -> Self {
        self.mode = AreaChartMode::Stacked;
        self
    }

    pub fn mode(mut self, mode: AreaChartMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn line_stroke(mut self, enabled: bool) -> Self {
        self.line_stroke = enabled;
        self
    }

    pub fn series(&self) -> &[ChartSeries] {
        &self.series
    }

    pub fn options(&self) -> &ChartOptions {
        &self.options
    }

    pub fn area_mode(&self) -> AreaChartMode {
        self.mode
    }
}

impl IntoElement for AreaChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for AreaChart {
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

        if self.options.show_legend {
            shell = shell.child(render_legend(&self.series, &palette));
        }

        shell
            .child(render_area_canvas(
                self.series,
                self.options,
                palette,
                self.mode,
                self.line_stroke,
                self.smooth,
                self.stroke_width,
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
                .child(
                    div()
                        .w(px(10.0))
                        .h(px(10.0))
                        .rounded_sm()
                        .bg(color.opacity(0.72)),
                )
                .child(Text::new(series.name.clone()).size(px(12.0)))
        }))
}

fn render_area_canvas(
    series: Vec<ChartSeries>,
    options: ChartOptions,
    palette: ChartPalette,
    mode: AreaChartMode,
    line_stroke: bool,
    smooth: bool,
    stroke_width: Pixels,
) -> impl IntoElement {
    let height = options.height;
    let bounds_cell: Rc<Cell<Bounds<Pixels>>> = Rc::new(Cell::new(Bounds::default()));
    let tooltip_bounds = bounds_cell.clone();
    let tooltip_series = series.clone();
    let tooltip_options = options.clone();
    let tooltip_mode = mode;
    let tooltip_id: SharedString = format!("{}-tooltip", options.id).into();
    let move_id = tooltip_id.clone();
    let chart = canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            let domain_len = label_domain_len(&series);
            if domain_len == 0 {
                return;
            }
            let axis_labels = collect_axis_labels(&series, options.max_axis_labels);

            let padding = options.padding;
            let left = bounds.left() + padding.left;
            let right = bounds.right() - padding.right;
            let top = bounds.top() + padding.top;
            let bottom = bounds.bottom() - padding.bottom;
            let width = (right - left).max(px(1.0));
            let plot_height = (bottom - top).max(px(1.0));

            let x = ScalePoint::from_len(domain_len, (0.0, width.as_f32()));
            let domain = if mode == AreaChartMode::Stacked {
                options
                    .y_domain
                    .or_else(|| stacked_domain(&series))
                    .map(|domain| normalized_domain_with_baseline(Some(domain), &[], true))
                    .unwrap_or_else(|| normalized_domain_with_baseline(None, &series, true))
            } else {
                normalized_domain_with_baseline(options.y_domain, &series, true)
            };
            let y = ScaleLinear::new(domain, (plot_height.as_f32(), 0.0));
            if options.show_grid || options.show_axis {
                paint_chart_frame(
                    left,
                    top,
                    width,
                    plot_height,
                    &axis_labels,
                    &x,
                    &y,
                    &palette,
                    &options,
                    window,
                    cx,
                );
            }

            match mode {
                AreaChartMode::Overlay => paint_overlay_areas(
                    left,
                    top,
                    plot_height,
                    &series,
                    &x,
                    &y,
                    &palette,
                    &options,
                    line_stroke,
                    smooth,
                    stroke_width,
                    window,
                    cx,
                ),
                AreaChartMode::Stacked => paint_stacked_areas(
                    left,
                    top,
                    &series,
                    &x,
                    &y,
                    &palette,
                    &options,
                    line_stroke,
                    smooth,
                    stroke_width,
                    window,
                    cx,
                ),
            }
        },
    )
    .w_full()
    .h(height);

    div()
        .relative()
        .w_full()
        .h(height)
        .on_mouse_move(move |event, _, cx| {
            if !tooltip_options.show_tooltip || tooltip_mode != AreaChartMode::Overlay {
                clear_tooltip(&move_id, cx);
                return;
            }
            let bounds = tooltip_bounds.get();
            if bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
                clear_tooltip(&move_id, cx);
                return;
            }
            let padding = tooltip_options.padding;
            let plot_width =
                (bounds.size.width.as_f32() - padding.left.as_f32() - padding.right.as_f32())
                    .max(1.0);
            let plot_height =
                (bounds.size.height.as_f32() - padding.top.as_f32() - padding.bottom.as_f32())
                    .max(1.0);
            let local_x = (event.position.x - bounds.left() - padding.left).as_f32();
            let local_y = (event.position.y - bounds.top() - padding.top).as_f32();
            let domain =
                normalized_domain_with_baseline(tooltip_options.y_domain, &tooltip_series, true);
            let Some(hit) = nearest_cartesian_hit_point(
                &tooltip_series,
                domain,
                plot_width,
                plot_height,
                local_x,
                local_y,
                tooltip_options.tooltip_hit_radius.as_f32(),
            ) else {
                clear_tooltip(&move_id, cx);
                return;
            };
            set_active_tooltip(
                TooltipData {
                    id: move_id.clone(),
                    content: format_hit_tooltip(&hit, tooltip_options.y_format),
                    anchor_bounds: Bounds::new(
                        point(event.position.x - px(1.0), event.position.y - px(1.0)),
                        size(px(2.0), px(2.0)),
                    ),
                    placement: Placement::Top,
                    offset: px(8.0),
                },
                cx,
            );
        })
        .child(ChartBoundsTracker::new(chart, bounds_cell))
}

fn sampled_point_indices(
    labels_len: usize,
    series: &[ChartSeries],
    max_points: Option<usize>,
) -> Vec<usize> {
    downsample_index_range(
        labels_len,
        |index| {
            series
                .iter()
                .filter_map(|series| series.points.get(index))
                .filter(|point| point.is_finite())
                .map(|point| point.value)
                .sum::<f64>()
        },
        max_points,
    )
    .into_iter()
    .map(|(index, _)| index)
    .collect()
}

#[allow(clippy::too_many_arguments)]
fn paint_overlay_areas(
    left: Pixels,
    top: Pixels,
    plot_height: Pixels,
    series: &[ChartSeries],
    x: &ScalePoint,
    y: &ScaleLinear,
    palette: &ChartPalette,
    options: &ChartOptions,
    line_stroke: bool,
    smooth: bool,
    stroke_width: Pixels,
    window: &mut Window,
    cx: &mut App,
) {
    let baseline = y.tick(0.0).clamp(0.0, plot_height.as_f32());
    for (series_index, current) in series.iter().enumerate() {
        let fallback = palette.series_color(series_index);
        let color = current.resolved_stroke_color(fallback);
        let fill_color = current.resolved_fill_color(fallback);
        let current_smooth = current.smooth.unwrap_or(smooth);
        let current_stroke_width = current.stroke_width.unwrap_or(stroke_width);
        let sampled_values = downsample_indexed_values(
            &current.points,
            |chart_point| chart_point.value,
            options.max_render_points,
        );
        let point_data = sampled_values
            .into_iter()
            .filter_map(|(index, value)| {
                let x_pos = x.tick_index(index)?;
                Some((
                    gpui::point(left + px(x_pos), top + px(y.tick(value))),
                    value,
                ))
            })
            .collect::<Vec<_>>();
        let points = point_data
            .iter()
            .map(|(position, _)| *position)
            .collect::<Vec<_>>();
        let area = if current_smooth {
            smooth_area_path(&points, top + px(baseline))
        } else {
            area_path(&points, top + px(baseline))
        };
        if let Some(path) = area {
            window.paint_path(path, fill_color.opacity(0.26));
        }
        if line_stroke {
            if let Some(path) = line_soft_edge_path(&points, current_stroke_width, current_smooth) {
                window.paint_path(path, color.opacity(0.20));
            }
            let line = if current_smooth {
                smooth_line_path(&points, current_stroke_width)
            } else {
                line_path(&points, current_stroke_width)
            };
            if let Some(path) = line {
                window.paint_path(path, color);
            }
        }
        if options.show_value_labels {
            let value_label_indices = sparse_indices(point_data.len(), options.max_value_labels);
            for (position, value) in value_label_indices
                .into_iter()
                .filter_map(|index| point_data.get(index))
            {
                paint_chart_label_aligned(
                    format_value_label(
                        *value,
                        series_total(current),
                        options.y_format,
                        &options.value_label_options,
                    ),
                    gpui::point(position.x - px(18.0), position.y - px(20.0)),
                    palette.label,
                    gpui::TextAlign::Center,
                    Some(px(36.0)),
                    window,
                    cx,
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn paint_stacked_areas(
    left: Pixels,
    top: Pixels,
    series: &[ChartSeries],
    x: &ScalePoint,
    y: &ScaleLinear,
    palette: &ChartPalette,
    options: &ChartOptions,
    line_stroke: bool,
    _smooth: bool,
    stroke_width: Pixels,
    window: &mut Window,
    cx: &mut App,
) {
    let labels_len = series
        .iter()
        .map(|series| series.points.len())
        .max()
        .unwrap_or(0);
    let sampled_indices = sampled_point_indices(labels_len, series, options.max_render_points);
    let mut previous = vec![0.0_f64; labels_len];
    for (series_index, current) in series.iter().enumerate() {
        let fallback = palette.series_color(series_index);
        let color = current.resolved_stroke_color(fallback);
        let fill_color = current.resolved_fill_color(fallback);
        let current_stroke_width = current.stroke_width.unwrap_or(stroke_width);
        let mut lower = Vec::new();
        let mut upper = Vec::new();
        for &point_index in &sampled_indices {
            let value = current
                .points
                .get(point_index)
                .filter(|point| point.is_finite())
                .map(|point| point.value)
                .unwrap_or(0.0);
            let from = previous[point_index];
            let to = from + value;
            previous[point_index] = to;
            if let Some(x_pos) = x.tick_index(point_index) {
                lower.push((left.as_f32() + x_pos, top.as_f32() + y.tick(from)));
                upper.push((left.as_f32() + x_pos, top.as_f32() + y.tick(to)));
            }
        }
        let lower = finite_line_points(lower);
        let upper = finite_line_points(upper);
        if let Some(path) = stacked_area_path(&lower, &upper) {
            window.paint_path(path, fill_color.opacity(0.32));
        }
        if line_stroke {
            if let Some(path) = line_soft_edge_path(&upper, current_stroke_width, false) {
                window.paint_path(path, color.opacity(0.20));
            }
            if let Some(path) = line_path(&upper, current_stroke_width) {
                window.paint_path(path, color);
            }
        }
        if options.show_value_labels {
            let value_label_indices = sparse_indices(upper.len(), options.max_value_labels);
            for sample_index in value_label_indices {
                let Some(position) = upper.get(sample_index) else {
                    continue;
                };
                let Some(&point_index) = sampled_indices.get(sample_index) else {
                    continue;
                };
                let value = current
                    .points
                    .get(point_index)
                    .filter(|point| point.is_finite())
                    .map(|point| point.value)
                    .unwrap_or(0.0);
                paint_chart_label_aligned(
                    format_value_label(
                        value,
                        series_total(current),
                        options.y_format,
                        &options.value_label_options,
                    ),
                    gpui::point(position.x - px(18.0), position.y - px(20.0)),
                    palette.label,
                    gpui::TextAlign::Center,
                    Some(px(36.0)),
                    window,
                    cx,
                );
            }
        }
    }
}

fn stacked_area_path(
    lower: &[gpui::Point<Pixels>],
    upper: &[gpui::Point<Pixels>],
) -> Option<gpui::Path<Pixels>> {
    let first = *upper.first()?;
    if lower.is_empty() || upper.len() != lower.len() {
        return None;
    }
    let mut builder = gpui::PathBuilder::fill();
    builder.move_to(first);
    for point in upper.iter().skip(1) {
        builder.line_to(*point);
    }
    for point in lower.iter().rev() {
        builder.line_to(*point);
    }
    builder.close();
    builder.build().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chart::ChartPoint;

    fn sample_series() -> Vec<ChartSeries> {
        vec![ChartSeries::new(
            "Visitors",
            [ChartPoint::new("Mon", 120.0), ChartPoint::new("Tue", 180.0)],
        )]
    }

    #[test]
    fn area_chart_builder_tracks_options_and_mode() {
        let chart = AreaChart::new(sample_series())
            .id("traffic-area")
            .height(px(320.0))
            .show_grid(false)
            .show_axis(false)
            .show_legend(false)
            .y_domain(0.0, 500.0)
            .line_stroke(false)
            .show_value_labels(false)
            .show_tooltip(false)
            .tooltip_hit_radius(px(20.0))
            .value_label_content(ChartValueLabelContent::Percentage)
            .value_label_placement(ChartValueLabelPlacement::OutsideFree)
            .percentage_decimals(2)
            .smooth(true)
            .stroke_width(px(3.0))
            .max_render_points(600)
            .max_axis_labels(6)
            .max_value_labels(10)
            .stacked();

        assert_eq!(chart.options().id, SharedString::from("traffic-area"));
        assert_eq!(chart.options().height, px(320.0));
        assert!(!chart.options().show_grid);
        assert!(!chart.options().show_axis);
        assert!(!chart.options().show_legend);
        assert_eq!(chart.options().y_domain, Some((0.0, 500.0)));
        assert!(!chart.options().show_value_labels);
        assert!(!chart.options().show_tooltip);
        assert_eq!(chart.options().tooltip_hit_radius, px(20.0));
        assert_eq!(
            chart.options().value_label_options.content,
            ChartValueLabelContent::Percentage
        );
        assert_eq!(
            chart.options().value_label_options.placement,
            ChartValueLabelPlacement::OutsideFree
        );
        assert_eq!(chart.options().value_label_options.percentage_decimals, 2);
        assert_eq!(chart.area_mode(), AreaChartMode::Stacked);
        assert!(!chart.line_stroke);
        assert!(chart.smooth);
        assert_eq!(chart.stroke_width, px(3.0));
        assert_eq!(chart.options().max_render_points, Some(600));
        assert_eq!(chart.options().max_axis_labels, 6);
        assert_eq!(chart.options().max_value_labels, 10);
    }

    #[test]
    fn area_chart_keeps_series_data() {
        let chart = AreaChart::new(sample_series());
        assert_eq!(chart.series().len(), 1);
        assert_eq!(chart.series()[0].name, SharedString::from("Visitors"));
    }

    #[test]
    fn stacked_area_samples_indices_from_total_series_shape() {
        let series = [
            ChartSeries::new(
                "a",
                (0..1_000).map(|index| ChartPoint::new(format!("T{index}"), index as f64)),
            ),
            ChartSeries::new(
                "b",
                (0..1_000).map(|index| {
                    let value = if index == 500 { 10_000.0 } else { 1.0 };
                    ChartPoint::new(format!("T{index}"), value)
                }),
            ),
        ];
        let indices = sampled_point_indices(1_000, &series, Some(80));

        assert!(indices.len() <= 80);
        assert_eq!(indices.first(), Some(&0));
        assert_eq!(indices.last(), Some(&999));
        assert!(indices.contains(&500));
    }
}
