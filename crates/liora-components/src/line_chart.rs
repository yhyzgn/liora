use crate::chart::{
    ChartBoundsTracker, ChartOptions, ChartPalette, ChartSeries, ChartValueLabelContent,
    ChartValueLabelPlacement, collect_axis_labels, downsample_indexed_values, format_hit_tooltip,
    format_value_label, has_chart_data, label_domain_len, nearest_cartesian_hit_point,
    normalized_domain, series_total, sparse_indices,
};
use crate::chart_frame::{paint_chart_frame, paint_chart_label_aligned};
use crate::chart_scale::{ScaleLinear, ScalePoint};
use crate::chart_shape::{
    area_path, line_path_with_style, line_soft_edge_path_with_style, smooth_area_path,
    smooth_line_path_with_style,
};
use crate::{Empty, Space, Text};
use gpui::{
    App, Background, Bounds, Component, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement, Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, fill, point, px,
    size,
};
use liora_core::{Config, Placement, TooltipData, clear_tooltip, set_active_tooltip, unique_id};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct LineChart {
    series: Vec<ChartSeries>,
    options: ChartOptions,
    point_markers: bool,
    smooth: bool,
    area_fill: bool,
    stroke_width: Pixels,
}

impl LineChart {
    pub fn new(series: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            series: series.into_iter().collect(),
            options: ChartOptions {
                id: unique_id("line-chart"),
                ..ChartOptions::default()
            },
            point_markers: true,
            smooth: true,
            area_fill: true,
            stroke_width: px(2.4),
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

    pub fn point_markers(mut self, enabled: bool) -> Self {
        self.point_markers = enabled;
        self
    }

    pub fn smooth(mut self, enabled: bool) -> Self {
        self.smooth = enabled;
        self
    }

    pub fn area_fill(mut self, enabled: bool) -> Self {
        self.area_fill = enabled;
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

    pub fn series(&self) -> &[ChartSeries] {
        &self.series
    }

    pub fn options(&self) -> &ChartOptions {
        &self.options
    }
}

impl IntoElement for LineChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for LineChart {
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
            .child(render_line_canvas(
                self.series,
                self.options,
                palette,
                self.point_markers,
                self.smooth,
                self.area_fill,
                self.stroke_width,
            ))
            .into_any_element()
    }
}

fn gradient_for_series(color: Hsla) -> gpui::Background {
    // GPUI uses CSS-like linear gradient angles. 180deg keeps the strongest
    // color on the curve edge and fades vertically toward the chart baseline.
    gpui::linear_gradient(
        180.0,
        gpui::linear_color_stop(color.opacity(0.28), 0.0),
        gpui::linear_color_stop(color.opacity(0.0), 1.0),
    )
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
                .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(color))
                .child(Text::new(series.name.clone()).size(px(12.0)))
        }))
}

fn render_line_canvas(
    series: Vec<ChartSeries>,
    options: ChartOptions,
    palette: ChartPalette,
    point_markers: bool,
    smooth: bool,
    area_fill: bool,
    stroke_width: Pixels,
) -> impl IntoElement {
    let height = options.height;
    let bounds_cell: Rc<Cell<Bounds<Pixels>>> = Rc::new(Cell::new(Bounds::default()));
    let tooltip_bounds = bounds_cell.clone();
    let tooltip_series = series.clone();
    let tooltip_options = options.clone();
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
            let domain = normalized_domain(options.y_domain, &series);
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

            for (series_index, current) in series.iter().enumerate() {
                let fallback = palette.series_color(series_index);
                let color = current.resolved_stroke_color(fallback);
                let fill_color = current.resolved_fill_color(fallback);
                let current_smooth = current.smooth.unwrap_or(smooth);
                let current_stroke_width = current.stroke_width.unwrap_or(stroke_width);
                let current_line_style = current
                    .line_style
                    .unwrap_or(crate::chart::ChartLineStyle::Solid);
                let current_dash_pattern = current.dash_pattern.as_deref();
                let sampled_values = downsample_indexed_values(
                    &current.points,
                    |chart_point| chart_point.value,
                    options.max_render_points,
                );
                let point_data = sampled_values
                    .into_iter()
                    .filter_map(|(index, value)| {
                        let x_pos = x.tick_index(index)?;
                        let position = point(
                            left + px(x_pos),
                            top + px(y.tick(value).clamp(0.0, plot_height.as_f32())),
                        );
                        Some((position, value))
                    })
                    .collect::<Vec<_>>();
                let points = point_data
                    .iter()
                    .map(|(position, _)| *position)
                    .collect::<Vec<_>>();
                if area_fill {
                    let baseline_y = top + px(plot_height.as_f32());
                    let area = if current_smooth {
                        smooth_area_path(&points, baseline_y)
                    } else {
                        area_path(&points, baseline_y)
                    };
                    if let Some(path) = area {
                        let gradient = gradient_for_series(fill_color);
                        window.paint_path(path, gradient);
                    }
                }
                if let Some(path) = line_soft_edge_path_with_style(
                    &points,
                    current_stroke_width,
                    current_smooth,
                    current_line_style,
                    current_dash_pattern,
                ) {
                    window.paint_path(path, color.opacity(0.20));
                }
                if let Some(path) = if current_smooth {
                    smooth_line_path_with_style(
                        &points,
                        current_stroke_width,
                        current_line_style,
                        current_dash_pattern,
                    )
                } else {
                    line_path_with_style(
                        &points,
                        current_stroke_width,
                        current_line_style,
                        current_dash_pattern,
                    )
                } {
                    window.paint_path(path, color);
                }
                if point_markers {
                    for (point_pos, _) in &point_data {
                        window.paint_quad(fill(
                            gpui::Bounds::new(
                                point(point_pos.x - px(3.0), point_pos.y - px(3.0)),
                                size(px(6.0), px(6.0)),
                            ),
                            Background::from(color),
                        ));
                    }
                }
                if options.show_value_labels {
                    let value_label_indices =
                        sparse_indices(point_data.len(), options.max_value_labels);
                    for (point_pos, value) in value_label_indices
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
                            point(point_pos.x - px(18.0), point_pos.y - px(20.0)),
                            palette.label,
                            gpui::TextAlign::Center,
                            Some(px(36.0)),
                            window,
                            cx,
                        );
                    }
                }
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
            if !tooltip_options.show_tooltip {
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
            let domain = normalized_domain(tooltip_options.y_domain, &tooltip_series);
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
            let anchor = Bounds::new(
                point(event.position.x - px(1.0), event.position.y - px(1.0)),
                size(px(2.0), px(2.0)),
            );
            set_active_tooltip(
                TooltipData {
                    id: move_id.clone(),
                    content: format_hit_tooltip(&hit, tooltip_options.y_format),
                    anchor_bounds: anchor,
                    placement: Placement::Top,
                    offset: px(8.0),
                },
                cx,
            );
        })
        .child(ChartBoundsTracker::new(chart, bounds_cell))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_series() -> Vec<ChartSeries> {
        vec![ChartSeries::new(
            "CPU",
            [
                ChartPoint::new("10:00", 20.0),
                ChartPoint::new("10:05", 35.0),
                ChartPoint::new("10:10", 28.0),
            ],
        )]
    }

    use crate::chart::ChartPoint;

    #[test]
    fn line_chart_builder_tracks_options() {
        let chart = LineChart::new(sample_series())
            .id("cpu-line")
            .height(px(320.0))
            .show_grid(false)
            .show_axis(false)
            .show_legend(false)
            .y_domain(0.0, 100.0)
            .point_markers(false)
            .show_value_labels(false)
            .show_tooltip(false)
            .tooltip_hit_radius(px(18.0))
            .value_label_content(ChartValueLabelContent::ValueAndPercentage)
            .value_label_placement(ChartValueLabelPlacement::OutsideFree)
            .percentage_decimals(2)
            .stroke_width(px(3.0))
            .max_render_points(1200)
            .max_axis_labels(6)
            .max_value_labels(10);

        assert_eq!(chart.options().id, SharedString::from("cpu-line"));
        assert_eq!(chart.options().height, px(320.0));
        assert!(!chart.options().show_grid);
        assert!(!chart.options().show_axis);
        assert!(!chart.options().show_legend);
        assert_eq!(chart.options().y_domain, Some((0.0, 100.0)));
        assert!(!chart.point_markers);
        assert!(!chart.options().show_value_labels);
        assert!(!chart.options().show_tooltip);
        assert_eq!(chart.options().tooltip_hit_radius, px(18.0));
        assert_eq!(
            chart.options().value_label_options.content,
            ChartValueLabelContent::ValueAndPercentage
        );
        assert_eq!(
            chart.options().value_label_options.placement,
            ChartValueLabelPlacement::OutsideFree
        );
        assert_eq!(chart.options().value_label_options.percentage_decimals, 2);
        assert_eq!(chart.stroke_width, px(3.0));
        assert_eq!(chart.options().max_render_points, Some(1200));
        assert_eq!(chart.options().max_axis_labels, 6);
        assert_eq!(chart.options().max_value_labels, 10);
    }

    #[test]
    fn line_chart_keeps_series_data() {
        let chart = LineChart::new(sample_series());
        assert_eq!(chart.series().len(), 1);
        assert_eq!(chart.series()[0].name, SharedString::from("CPU"));
        assert_eq!(chart.series()[0].points.len(), 3);
    }
}
