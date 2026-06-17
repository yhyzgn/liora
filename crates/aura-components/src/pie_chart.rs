use crate::chart::{
    ChartBoundsTracker, ChartHitPoint, ChartPalette, ChartSeries, ChartValueLabelContent,
    ChartValueLabelOptions, ChartValueLabelPlacement, format_hit_tooltip, format_value_label,
    has_chart_data,
};
use crate::chart_frame::paint_chart_label_aligned;
use crate::{Empty, Space, Text};
use aura_core::{Config, Placement, TooltipData, clear_tooltip, set_active_tooltip, unique_id};
use gpui::{
    App, Bounds, Component, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement,
    Pixels, Point, RenderOnce, SharedString, Styled, Window, canvas, div, point, px, size,
};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PieChartLabelOptions {
    pub value: ChartValueLabelOptions,
}

impl Default for PieChartLabelOptions {
    fn default() -> Self {
        Self {
            value: ChartValueLabelOptions {
                content: ChartValueLabelContent::ValueOverTotalAndPercentage,
                placement: ChartValueLabelPlacement::Auto,
                percentage_decimals: 1,
                outside_threshold_degrees: 28,
            },
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
    show_tooltip: bool,
    tooltip_hit_radius: Pixels,
}

#[derive(Clone)]
pub struct RingChart {
    slices: Vec<ChartSeries>,
    id: SharedString,
    height: Pixels,
    show_legend: bool,
    show_value_labels: bool,
    label_options: PieChartLabelOptions,
    show_tooltip: bool,
    tooltip_hit_radius: Pixels,
    inner_ratio: f32,
    external_legend: Option<RingExternalLegendOptions>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingExternalLegendLayout {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RingExternalLegendSide {
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PieSliceHitRegion {
    pub series_index: usize,
    pub series_name: SharedString,
    pub label: SharedString,
    pub value: f64,
    pub start_deg: f32,
    pub end_deg: f32,
    pub inner_radius: f32,
    pub outer_radius: f32,
}

pub fn pie_slice_hit_regions(
    slices: &[ChartSeries],
    inner_ratio: f32,
    outer_radius: f32,
) -> Vec<PieSliceHitRegion> {
    if slices.is_empty() || !outer_radius.is_finite() || outer_radius <= 0.0 {
        return Vec::new();
    }
    let total = series_total(slices);
    if total <= f64::EPSILON {
        return Vec::new();
    }

    let inner_radius = (outer_radius * inner_ratio.clamp(0.0, 0.95)).max(0.0);
    let mut start = -90.0_f32;
    let mut regions = Vec::new();
    for (series_index, series) in slices.iter().enumerate() {
        let value = series_value(series).max(0.0);
        if value <= 0.0 {
            continue;
        }
        let sweep = (value / total) as f32 * 360.0;
        let end = start + sweep;
        let label = series
            .finite_points()
            .next()
            .map(|point| point.label.clone())
            .unwrap_or_else(|| series.name.clone());
        regions.push(PieSliceHitRegion {
            series_index,
            series_name: series.name.clone(),
            label,
            value,
            start_deg: start,
            end_deg: end,
            inner_radius,
            outer_radius,
        });
        start = end;
    }
    regions
}

pub fn nearest_pie_slice_hit_point(
    slices: &[ChartSeries],
    inner_ratio: f32,
    outer_radius: f32,
    center_x: f32,
    center_y: f32,
    pointer_x: f32,
    pointer_y: f32,
    hit_radius: f32,
) -> Option<ChartHitPoint> {
    if !center_x.is_finite()
        || !center_y.is_finite()
        || !pointer_x.is_finite()
        || !pointer_y.is_finite()
        || !hit_radius.is_finite()
        || hit_radius < 0.0
    {
        return None;
    }

    let dx = pointer_x - center_x;
    let dy = pointer_y - center_y;
    let radius = (dx * dx + dy * dy).sqrt();
    let mut angle = dy.atan2(dx).to_degrees();
    while angle < -90.0 {
        angle += 360.0;
    }
    while angle >= 270.0 {
        angle -= 360.0;
    }

    let mut best: Option<(PieSliceHitRegion, f32)> = None;
    for region in pie_slice_hit_regions(slices, inner_ratio, outer_radius) {
        if angle < region.start_deg || angle > region.end_deg {
            continue;
        }
        let inner_distance = region.inner_radius - radius;
        let outer_distance = radius - region.outer_radius;
        let distance = if radius < region.inner_radius {
            inner_distance
        } else if radius > region.outer_radius {
            outer_distance
        } else {
            0.0
        };
        if distance <= hit_radius
            && best
                .as_ref()
                .is_none_or(|(_, best_distance)| distance < *best_distance)
        {
            best = Some((region, distance));
        }
    }

    best.map(|(region, distance)| {
        let mid_deg = (region.start_deg + region.end_deg) * 0.5;
        let hit_radius = (region.inner_radius + region.outer_radius) * 0.5;
        ChartHitPoint {
            series_index: region.series_index,
            point_index: 0,
            series_name: region.series_name.clone(),
            label: region.label.clone(),
            value: region.value,
            x: center_x + hit_radius * mid_deg.to_radians().cos(),
            y: center_y + hit_radius * mid_deg.to_radians().sin(),
            distance,
        }
    })
}

#[derive(Clone, Debug, PartialEq)]
pub struct RingExternalLegendOptions {
    layout: RingExternalLegendLayout,
    side: RingExternalLegendSide,
    content: ChartValueLabelContent,
    percentage_decimals: usize,
    max_items: Option<usize>,
}

impl Default for RingExternalLegendOptions {
    fn default() -> Self {
        Self {
            layout: RingExternalLegendLayout::Vertical,
            side: RingExternalLegendSide::Right,
            content: ChartValueLabelContent::ValueOverTotalAndPercentage,
            percentage_decimals: 1,
            max_items: None,
        }
    }
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
            show_tooltip: true,
            tooltip_hit_radius: px(0.0),
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

    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }

    pub fn tooltip_hit_radius(mut self, radius: Pixels) -> Self {
        self.tooltip_hit_radius = radius.max(px(0.0));
        self
    }

    pub fn show_percentage_labels(mut self, show: bool) -> Self {
        self.label_options.value.content = if show {
            ChartValueLabelContent::ValueOverTotalAndPercentage
        } else {
            ChartValueLabelContent::ValueOverTotal
        };
        self
    }

    pub fn value_label_content(mut self, content: ChartValueLabelContent) -> Self {
        self.label_options.value.content = content;
        self
    }

    pub fn value_label_placement(mut self, placement: ChartValueLabelPlacement) -> Self {
        self.label_options.value.placement = placement;
        self
    }

    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.label_options.value.percentage_decimals = decimals.min(4);
        self
    }

    pub fn outside_label_threshold_degrees(mut self, degrees: u16) -> Self {
        self.label_options.value.outside_threshold_degrees = degrees.min(120);
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
            show_tooltip: true,
            tooltip_hit_radius: px(0.0),
            inner_ratio: 0.62,
            external_legend: None,
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

    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.show_tooltip = show;
        self
    }

    pub fn tooltip_hit_radius(mut self, radius: Pixels) -> Self {
        self.tooltip_hit_radius = radius.max(px(0.0));
        self
    }

    pub fn show_percentage_labels(mut self, show: bool) -> Self {
        self.label_options.value.content = if show {
            ChartValueLabelContent::ValueOverTotalAndPercentage
        } else {
            ChartValueLabelContent::ValueOverTotal
        };
        self
    }

    pub fn value_label_content(mut self, content: ChartValueLabelContent) -> Self {
        self.label_options.value.content = content;
        self
    }

    pub fn value_label_placement(mut self, placement: ChartValueLabelPlacement) -> Self {
        self.label_options.value.placement = placement;
        self
    }

    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.label_options.value.percentage_decimals = decimals.min(4);
        self
    }

    pub fn outside_label_threshold_degrees(mut self, degrees: u16) -> Self {
        self.label_options.value.outside_threshold_degrees = degrees.min(120);
        self
    }

    pub fn label_options(&self) -> &PieChartLabelOptions {
        &self.label_options
    }

    pub fn inner_ratio(mut self, ratio: f32) -> Self {
        self.inner_ratio = ratio.clamp(0.2, 0.9);
        self
    }

    pub fn external_legend(mut self, options: RingExternalLegendOptions) -> Self {
        self.external_legend = Some(options);
        self.show_value_labels = false;
        self.show_legend = false;
        self
    }

    pub fn external_vertical_legend(self) -> Self {
        self.external_legend(RingExternalLegendOptions::default())
    }

    pub fn external_horizontal_legend(self) -> Self {
        self.external_legend(
            RingExternalLegendOptions::default().layout(RingExternalLegendLayout::Horizontal),
        )
    }

    pub fn external_legend_side(mut self, side: RingExternalLegendSide) -> Self {
        let mut options = self.external_legend.unwrap_or_default();
        options.side = side;
        self.external_legend = Some(options);
        self
    }

    pub fn external_legend_left(self) -> Self {
        self.external_legend_side(RingExternalLegendSide::Left)
    }

    pub fn external_legend_right(self) -> Self {
        self.external_legend_side(RingExternalLegendSide::Right)
    }

    pub fn external_legend_max_items(mut self, max_items: usize) -> Self {
        let mut options = self.external_legend.unwrap_or_default();
        options.max_items = Some(max_items.max(1));
        self.external_legend = Some(options);
        self
    }

    pub fn external_legend_content(mut self, content: ChartValueLabelContent) -> Self {
        let mut options = self.external_legend.unwrap_or_default();
        options.content = content;
        self.external_legend = Some(options);
        self
    }

    pub fn external_legend_percentage_decimals(mut self, decimals: usize) -> Self {
        let mut options = self.external_legend.unwrap_or_default();
        options.percentage_decimals = decimals.min(4);
        self.external_legend = Some(options);
        self
    }

    pub fn slices(&self) -> &[ChartSeries] {
        &self.slices
    }
}

impl RingExternalLegendOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn layout(mut self, layout: RingExternalLegendLayout) -> Self {
        self.layout = layout;
        self
    }

    pub fn vertical(self) -> Self {
        self.layout(RingExternalLegendLayout::Vertical)
    }

    pub fn horizontal(self) -> Self {
        self.layout(RingExternalLegendLayout::Horizontal)
    }

    pub fn side(mut self, side: RingExternalLegendSide) -> Self {
        self.side = side;
        self
    }

    pub fn left(self) -> Self {
        self.side(RingExternalLegendSide::Left)
    }

    pub fn right(self) -> Self {
        self.side(RingExternalLegendSide::Right)
    }

    pub fn content(mut self, content: ChartValueLabelContent) -> Self {
        self.content = content;
        self
    }

    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.percentage_decimals = decimals.min(4);
        self
    }

    pub fn max_items(mut self, max_items: usize) -> Self {
        self.max_items = Some(max_items.max(1));
        self
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
            self.show_tooltip,
            self.tooltip_hit_radius,
            0.0,
            None,
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
            self.show_tooltip,
            self.tooltip_hit_radius,
            self.inner_ratio,
            self.external_legend,
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
    show_tooltip: bool,
    tooltip_hit_radius: Pixels,
    inner_ratio: f32,
    external_legend: Option<RingExternalLegendOptions>,
    cx: &mut App,
) -> impl IntoElement {
    let theme = cx.global::<Config>().theme.clone();
    let palette = ChartPalette::from_config(cx.global::<Config>());
    let has_data = has_chart_data(&slices);
    let tooltip_id: SharedString = format!("{}-tooltip", id).into();

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

    let side_legend = external_legend
        .as_ref()
        .is_some_and(|options| options.layout == RingExternalLegendLayout::Vertical);
    let canvas_height = if side_legend { px(280.0) } else { height };
    let canvas = render_canvas(
        slices.clone(),
        palette.clone(),
        theme.neutral.card,
        inner_ratio,
        show_value_labels,
        label_options,
        tooltip_id,
        show_tooltip,
        tooltip_hit_radius,
        canvas_height,
    );

    match external_legend {
        Some(options) if options.layout == RingExternalLegendLayout::Vertical => {
            let legend = render_external_legend(&slices, &palette, options.clone());
            let content = div()
                .flex()
                .items_center()
                .gap_2()
                .children(match options.side {
                    RingExternalLegendSide::Left => vec![
                        legend.into_any_element(),
                        div()
                            .flex_none()
                            .w(canvas_height)
                            .child(canvas)
                            .into_any_element(),
                    ],
                    RingExternalLegendSide::Right => vec![
                        div()
                            .flex_none()
                            .w(canvas_height)
                            .child(canvas)
                            .into_any_element(),
                        legend.into_any_element(),
                    ],
                });
            shell.child(content).into_any_element()
        }
        Some(options) => shell
            .child(canvas)
            .child(render_external_legend(&slices, &palette, options))
            .into_any_element(),
        None => shell.child(canvas).into_any_element(),
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

fn render_external_legend(
    series: &[ChartSeries],
    palette: &ChartPalette,
    options: RingExternalLegendOptions,
) -> impl IntoElement {
    let total = series_total(series);
    let items = series
        .iter()
        .enumerate()
        .take(options.max_items.unwrap_or(usize::MAX))
        .filter_map(|(index, series)| {
            let value = series_value(series).max(0.0);
            (value > 0.0).then_some((index, series, value))
        })
        .map(|(index, series, value)| {
            let color = series.color.unwrap_or_else(|| palette.series_color(index));
            let text = format_value_label(
                value,
                total,
                None,
                &ChartValueLabelOptions {
                    content: options.content,
                    placement: ChartValueLabelPlacement::OutsideAligned,
                    percentage_decimals: options.percentage_decimals,
                    outside_threshold_degrees: 0,
                },
            );
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_3()
                .min_w(px(160.0))
                .child(
                    Space::new()
                        .gap_xs()
                        .align_center()
                        .child(div().w(px(10.0)).h(px(10.0)).rounded_full().bg(color))
                        .child(Text::new(series.name.clone()).size(px(12.0))),
                )
                .child(Text::new(text).size(px(12.0)))
        });

    match options.layout {
        RingExternalLegendLayout::Vertical => div()
            .flex()
            .flex_col()
            .gap_2()
            .flex_none()
            .w(px(180.0))
            .children(items),
        RingExternalLegendLayout::Horizontal => div()
            .flex()
            .gap_2()
            .w_full()
            .flex_row()
            .flex_wrap()
            .gap_4()
            .children(items),
    }
}

fn render_canvas(
    slices: Vec<ChartSeries>,
    palette: ChartPalette,
    hole_color: Hsla,
    inner_ratio: f32,
    show_value_labels: bool,
    label_options: PieChartLabelOptions,
    tooltip_id: SharedString,
    show_tooltip: bool,
    tooltip_hit_radius: Pixels,
    height: Pixels,
) -> impl IntoElement {
    let bounds_cell: Rc<Cell<Bounds<Pixels>>> = Rc::new(Cell::new(Bounds::default()));
    let tooltip_bounds = bounds_cell.clone();
    let tooltip_slices = slices.clone();
    let move_id = tooltip_id.clone();
    let chart = canvas(
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
                .map(|series| series_value(series).max(0.0))
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
    .h(height);

    div()
        .relative()
        .w_full()
        .h(height)
        .on_mouse_move(move |event, _, cx| {
            if !show_tooltip {
                clear_tooltip(&move_id, cx);
                return;
            }
            let bounds = tooltip_bounds.get();
            if bounds.size.width <= px(0.0) || bounds.size.height <= px(0.0) {
                clear_tooltip(&move_id, cx);
                return;
            }
            let inset = if show_value_labels {
                px(56.0)
            } else {
                px(18.0)
            };
            let width = (bounds.right() - bounds.left() - inset * 2.0).max(px(1.0));
            let chart_height = (bounds.bottom() - bounds.top() - inset * 2.0).max(px(1.0));
            let radius = (width.min(chart_height).as_f32() / 2.0).max(1.0);
            let center = point(
                bounds.left() + width / 2.0 + inset,
                bounds.top() + chart_height / 2.0 + inset,
            );
            let Some(hit) = nearest_pie_slice_hit_point(
                &tooltip_slices,
                inner_ratio,
                radius,
                center.x.as_f32(),
                center.y.as_f32(),
                event.position.x.as_f32(),
                event.position.y.as_f32(),
                tooltip_hit_radius.as_f32(),
            ) else {
                clear_tooltip(&move_id, cx);
                return;
            };
            set_active_tooltip(
                TooltipData {
                    id: move_id.clone(),
                    content: format_hit_tooltip(&hit, None),
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

fn series_value(series: &ChartSeries) -> f64 {
    series
        .finite_points()
        .next()
        .map(|point| point.value.max(0.0))
        .unwrap_or(0.0)
}

fn series_total(series: &[ChartSeries]) -> f64 {
    series.iter().map(series_value).sum()
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
    let force_outside = matches!(
        options.value.placement,
        ChartValueLabelPlacement::OutsideFree | ChartValueLabelPlacement::OutsideAligned
    );
    if force_outside || sweep < options.value.outside_threshold_degrees as f32 {
        paint_outside_slice_label(
            center,
            radius,
            mid_deg,
            text,
            label.color,
            palette,
            options.value.placement,
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
    placement: ChartValueLabelPlacement,
    window: &mut Window,
    cx: &mut App,
) {
    let edge = polar_point(center, radius, mid_deg);
    let elbow = polar_point(center, radius + 14.0, mid_deg);
    let right_side = mid_deg.to_radians().cos() >= 0.0;
    let label_anchor = if placement == ChartValueLabelPlacement::OutsideAligned {
        point(
            center.x
                + if right_side {
                    px(radius + 62.0)
                } else {
                    px(-(radius + 62.0))
                },
            elbow.y,
        )
    } else {
        point(
            elbow.x + if right_side { px(34.0) } else { px(-34.0) },
            elbow.y,
        )
    };

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
    format_value_label(value, total, None, &options.value)
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
            .show_tooltip(false)
            .tooltip_hit_radius(px(6.0))
            .show_percentage_labels(false)
            .percentage_decimals(2)
            .outside_label_threshold_degrees(36);
        assert_eq!(chart.slices().len(), 3);
        assert!(!chart.show_value_labels);
        assert!(!chart.show_tooltip);
        assert_eq!(chart.tooltip_hit_radius, px(6.0));
        assert!(!matches!(
            chart.label_options().value.content,
            ChartValueLabelContent::ValueOverTotalAndPercentage
        ));
        assert_eq!(chart.label_options().value.percentage_decimals, 2);
        assert_eq!(chart.label_options().value.outside_threshold_degrees, 36);
    }

    #[test]
    fn ring_chart_tracks_inner_ratio() {
        let chart = RingChart::new(slices())
            .inner_ratio(0.5)
            .show_value_labels(false)
            .show_tooltip(false)
            .tooltip_hit_radius(px(8.0))
            .percentage_decimals(3);
        assert_eq!(chart.slices().len(), 3);
        assert!(chart.inner_ratio >= 0.2 && chart.inner_ratio <= 0.9);
        assert!(!chart.show_value_labels);
        assert!(!chart.show_tooltip);
        assert_eq!(chart.tooltip_hit_radius, px(8.0));
        assert_eq!(chart.label_options().value.percentage_decimals, 3);
    }

    #[test]
    fn ring_chart_external_legend_disables_inline_labels() {
        let chart = RingChart::new(slices())
            .external_horizontal_legend()
            .external_legend_content(ChartValueLabelContent::Percentage)
            .external_legend_percentage_decimals(2);
        assert!(!chart.show_legend);
        assert!(!chart.show_value_labels);
        let options = chart.external_legend.unwrap();
        assert_eq!(options.layout, RingExternalLegendLayout::Horizontal);
        assert_eq!(options.side, RingExternalLegendSide::Right);
        assert_eq!(options.content, ChartValueLabelContent::Percentage);
        assert_eq!(options.percentage_decimals, 2);
    }

    #[test]
    fn ring_chart_external_legend_tracks_side_and_limit() {
        let chart = RingChart::new(slices())
            .external_vertical_legend()
            .external_legend_left()
            .external_legend_max_items(2);
        let options = chart.external_legend.unwrap();
        assert_eq!(options.layout, RingExternalLegendLayout::Vertical);
        assert_eq!(options.side, RingExternalLegendSide::Left);
        assert_eq!(options.max_items, Some(2));
    }

    #[test]
    fn slice_labels_use_value_total_and_configurable_percentage_precision() {
        let options = PieChartLabelOptions {
            value: ChartValueLabelOptions {
                percentage_decimals: 2,
                ..PieChartLabelOptions::default().value
            },
        };
        assert_eq!(
            format_slice_label(1.0, 3.0, &options),
            SharedString::from("1 / 3 (33.33%)")
        );

        let options = PieChartLabelOptions {
            value: ChartValueLabelOptions {
                content: ChartValueLabelContent::ValueOverTotal,
                ..PieChartLabelOptions::default().value
            },
        };
        assert_eq!(
            format_slice_label(1.0, 3.0, &options),
            SharedString::from("1 / 3")
        );
    }

    #[test]
    fn pie_slice_hit_testing_returns_slice_under_pointer() {
        let regions = pie_slice_hit_regions(&slices(), 0.0, 100.0);
        assert_eq!(regions.len(), 3);
        assert_eq!(regions[0].series_name, SharedString::from("A"));
        assert_eq!(regions[0].start_deg, -90.0);
        assert!((regions[0].end_deg - 18.0).abs() < 0.001);

        let hit = nearest_pie_slice_hit_point(&slices(), 0.0, 100.0, 0.0, 0.0, 70.0, -70.0, 0.0)
            .expect("pointer inside first slice should hit");
        assert_eq!(hit.series_index, 0);
        assert_eq!(hit.series_name, SharedString::from("A"));
        assert_eq!(hit.label, SharedString::from("A"));
        assert_eq!(hit.value, 30.0);
    }

    #[test]
    fn ring_slice_hit_testing_excludes_inner_hole_and_hits_ring_segment() {
        assert_eq!(
            nearest_pie_slice_hit_point(&slices(), 0.62, 100.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            None
        );

        let hit = nearest_pie_slice_hit_point(&slices(), 0.62, 100.0, 0.0, 0.0, 70.0, -70.0, 0.0)
            .expect("pointer inside ring segment should hit");
        assert_eq!(hit.series_index, 0);
    }
}
