//! Candlestick Chart module.
//!
//! This public module implements the Liora native candlestick chart component for OHLC market and telemetry data. It keeps the reusable
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

use crate::chart::{
    ChartAxisLabel, ChartBoundsTracker, ChartOptions, ChartPalette, ChartValueLabelContent,
    ChartValueLabelPlacement, format_value_label, sparse_indices,
};
use crate::chart_frame::{paint_chart_frame, paint_chart_label_aligned};
use crate::chart_scale::{ScaleLinear, ScalePoint};
use crate::{Empty, Space, Text};
use gpui::{
    App, Background, Bounds, Component, ElementId, Hsla, InteractiveElement, IntoElement,
    ParentElement, Pixels, RenderOnce, SharedString, Styled, Window, canvas, div, fill, point, px,
    size,
};
use liora_core::{Config, Placement, TooltipData, clear_tooltip, set_active_tooltip, unique_id};
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
/// Ordered open-high-low-close record rendered by [`CandlestickChart`].
pub struct CandlestickPoint {
    /// User-facing x-axis label, usually a date, time, or trading session.
    pub label: SharedString,
    /// Opening value for the interval.
    pub open: f64,
    /// Highest value reached during the interval.
    pub high: f64,
    /// Lowest value reached during the interval.
    pub low: f64,
    /// Closing value for the interval.
    pub close: f64,
    /// Optional volume rendered only by external summaries or custom legends.
    pub volume: Option<f64>,
}

impl CandlestickPoint {
    /// Creates an OHLC point without volume metadata.
    pub fn new(label: impl Into<SharedString>, open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            label: label.into(),
            open,
            high,
            low,
            close,
            volume: None,
        }
    }

    /// Adds volume metadata that can be displayed in tooltips.
    pub fn volume(mut self, volume: f64) -> Self {
        self.volume = Some(volume);
        self
    }

    /// Returns true when all OHLC numbers are finite and high/low bound the body.
    pub fn is_valid(&self) -> bool {
        self.open.is_finite()
            && self.high.is_finite()
            && self.low.is_finite()
            && self.close.is_finite()
            && self.high >= self.low
            && self.high >= self.open.max(self.close)
            && self.low <= self.open.min(self.close)
    }

    /// Returns true when the close is greater than or equal to the open.
    pub fn is_rising(&self) -> bool {
        self.close >= self.open
    }

    /// Returns the largest absolute price change in the interval body.
    pub fn body_delta(&self) -> f64 {
        (self.close - self.open).abs()
    }
}

#[derive(Clone, Debug, PartialEq)]
/// Pixel hit target for a rendered candlestick candle.
pub struct CandlestickHitBox {
    /// Index of the underlying OHLC point.
    pub point_index: usize,
    /// User-facing label associated with the candle.
    pub label: SharedString,
    /// Opening value for the candle.
    pub open: f64,
    /// Highest value for the candle.
    pub high: f64,
    /// Lowest value for the candle.
    pub low: f64,
    /// Closing value for the candle.
    pub close: f64,
    /// Optional volume metadata for tooltip content.
    pub volume: Option<f64>,
    /// X coordinate in chart-local pixels.
    pub x: f32,
    /// Y coordinate in chart-local pixels.
    pub y: f32,
    /// Width used by layout or hit-testing calculations.
    pub width: f32,
    /// Height used by layout or hit-testing calculations.
    pub height: f32,
}

impl CandlestickHitBox {
    /// Returns the horizontal center of the hit target in chart-local pixels.
    pub fn center_x(&self) -> f32 {
        self.x + self.width / 2.0
    }

    /// Returns true when the candle body closed at or above its opening value.
    pub fn is_rising(&self) -> bool {
        self.close >= self.open
    }
}

/// Returns the normalized y-domain used by candlestick rendering.
pub fn candlestick_domain(points: &[CandlestickPoint], explicit: Option<(f64, f64)>) -> (f64, f64) {
    if let Some((min, max)) = explicit.filter(|(min, max)| min.is_finite() && max.is_finite()) {
        return expand_flat_domain(min, max);
    }

    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for point in points.iter().filter(|point| point.is_valid()) {
        min = min.min(point.low);
        max = max.max(point.high);
    }
    if !min.is_finite() || !max.is_finite() {
        return (0.0, 1.0);
    }
    let span = (max - min).abs();
    let padding = (span * 0.08).max(if max.abs() < f64::EPSILON {
        1.0
    } else {
        max.abs() * 0.01
    });
    expand_flat_domain(min - padding, max + padding)
}

fn expand_flat_domain(min: f64, max: f64) -> (f64, f64) {
    if (max - min).abs() >= f64::EPSILON {
        return (min, max);
    }
    let pad = if max.abs() < f64::EPSILON {
        1.0
    } else {
        max.abs() * 0.1
    };
    (min - pad, max + pad)
}

/// Computes rendered candlestick hit boxes for pointer interaction and tests.
pub fn candlestick_hit_boxes(
    points: &[CandlestickPoint],
    domain: (f64, f64),
    plot_width: f32,
    plot_height: f32,
    body_width: Option<Pixels>,
) -> Vec<CandlestickHitBox> {
    if points.is_empty()
        || !domain.0.is_finite()
        || !domain.1.is_finite()
        || (domain.1 - domain.0).abs() < f64::EPSILON
        || !plot_width.is_finite()
        || !plot_height.is_finite()
        || plot_width <= 0.0
        || plot_height <= 0.0
    {
        return Vec::new();
    }

    let x = ScalePoint::from_len(points.len(), (0.0, plot_width));
    let y = ScaleLinear::new(domain, (plot_height, 0.0));
    let automatic_width = if points.len() <= 1 {
        (plot_width * 0.18).clamp(4.0, 16.0)
    } else {
        (plot_width / points.len() as f32 * 0.56).clamp(3.0, 18.0)
    };
    let width = body_width
        .map(|width| width.as_f32().clamp(1.0, plot_width.max(1.0)))
        .unwrap_or(automatic_width);

    points
        .iter()
        .enumerate()
        .filter(|(_, point)| point.is_valid())
        .filter_map(|(index, point_data)| {
            let center = x.tick_index(index)?;
            let high_y = y.tick(point_data.high).clamp(0.0, plot_height);
            let low_y = y.tick(point_data.low).clamp(0.0, plot_height);
            let open_y = y.tick(point_data.open).clamp(0.0, plot_height);
            let close_y = y.tick(point_data.close).clamp(0.0, plot_height);
            let top = high_y.min(open_y.min(close_y));
            let bottom = low_y.max(open_y.max(close_y));
            Some(CandlestickHitBox {
                point_index: index,
                label: point_data.label.clone(),
                open: point_data.open,
                high: point_data.high,
                low: point_data.low,
                close: point_data.close,
                volume: point_data.volume,
                x: center - width / 2.0,
                y: top,
                width,
                height: (bottom - top).max(1.0),
            })
        })
        .collect()
}

/// Returns the nearest candlestick hit target for a pointer position.
pub fn nearest_candlestick_hit_box(
    points: &[CandlestickPoint],
    domain: (f64, f64),
    plot_width: f32,
    plot_height: f32,
    body_width: Option<Pixels>,
    pointer_x: f32,
    pointer_y: f32,
    hit_radius: f32,
) -> Option<CandlestickHitBox> {
    if !pointer_x.is_finite()
        || !pointer_y.is_finite()
        || !hit_radius.is_finite()
        || hit_radius < 0.0
    {
        return None;
    }
    candlestick_hit_boxes(points, domain, plot_width, plot_height, body_width)
        .into_iter()
        .filter_map(|hit| {
            let inside_x = pointer_x >= hit.x && pointer_x <= hit.x + hit.width;
            let inside_y = pointer_y >= hit.y && pointer_y <= hit.y + hit.height;
            let dx = if inside_x {
                0.0
            } else if pointer_x < hit.x {
                hit.x - pointer_x
            } else {
                pointer_x - (hit.x + hit.width)
            };
            let dy = if inside_y {
                0.0
            } else if pointer_y < hit.y {
                hit.y - pointer_y
            } else {
                pointer_y - (hit.y + hit.height)
            };
            let distance = (dx * dx + dy * dy).sqrt();
            (distance <= hit_radius).then_some((hit, distance))
        })
        .min_by(|(_, left), (_, right)| left.total_cmp(right))
        .map(|(hit, _)| hit)
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering OHLC candlestick charts.
pub struct CandlestickChart {
    points: Vec<CandlestickPoint>,
    options: ChartOptions,
    up_color: Option<Hsla>,
    down_color: Option<Hsla>,
    wick_width: Pixels,
    body_width: Option<Pixels>,
}

impl CandlestickChart {
    /// Creates `CandlestickChart` that renders the supplied OHLC point collection.
    pub fn new(points: impl IntoIterator<Item = CandlestickPoint>) -> Self {
        Self {
            points: points.into_iter().collect(),
            options: ChartOptions {
                id: unique_id("candlestick-chart"),
                show_legend: false,
                show_value_labels: false,
                ..ChartOptions::default()
            },
            up_color: None,
            down_color: None,
            wick_width: px(1.4),
            body_width: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.options.id = id.into();
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.options.height = height.into();
        self
    }

    /// Configures whether grid is visible in the rendered component.
    pub fn show_grid(mut self, show: bool) -> Self {
        self.options.show_grid = show;
        self
    }

    /// Configures whether axis is visible in the rendered component.
    pub fn show_axis(mut self, show: bool) -> Self {
        self.options.show_axis = show;
        self
    }

    /// Configures whether legend is visible in the rendered component.
    pub fn show_legend(mut self, show: bool) -> Self {
        self.options.show_legend = show;
        self
    }

    /// Overrides automatic y-axis bounds with an explicit numeric domain.
    pub fn y_domain(mut self, min: f64, max: f64) -> Self {
        self.options.y_domain = Some((min, max));
        self
    }

    /// Installs the formatter used for y-axis tick labels and tooltip values.
    pub fn y_format(mut self, formatter: fn(f64) -> SharedString) -> Self {
        self.options.y_format = Some(formatter);
        self
    }

    /// Configures whether close-value labels are visible above candles.
    pub fn show_value_labels(mut self, show: bool) -> Self {
        self.options.show_value_labels = show;
        self
    }

    /// Configures whether pointer tooltip is visible in the rendered component.
    pub fn show_tooltip(mut self, show: bool) -> Self {
        self.options.show_tooltip = show;
        self
    }

    /// Sets the pointer distance used when resolving chart tooltip hits.
    pub fn tooltip_hit_radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.options.tooltip_hit_radius = radius.into().max(px(0.0));
        self
    }

    /// Chooses whether value labels show raw values, percentages, or both.
    pub fn value_label_content(mut self, content: ChartValueLabelContent) -> Self {
        self.options.value_label_options.content = content;
        self
    }

    /// Chooses where value labels are positioned relative to chart marks.
    pub fn value_label_placement(mut self, placement: ChartValueLabelPlacement) -> Self {
        self.options.value_label_options.placement = placement;
        self
    }

    /// Sets the number of fractional digits used for percentage labels.
    pub fn percentage_decimals(mut self, decimals: usize) -> Self {
        self.options.value_label_options.percentage_decimals = decimals.min(4);
        self
    }

    /// Sets the color used by rising or unchanged candles.
    pub fn up_color(mut self, color: Hsla) -> Self {
        self.up_color = Some(color);
        self
    }

    /// Sets the color used by falling candles.
    pub fn down_color(mut self, color: Hsla) -> Self {
        self.down_color = Some(color);
        self
    }

    /// Sets the body width applied to every candle.
    pub fn body_width(mut self, width: impl Into<Pixels>) -> Self {
        self.body_width = Some(width.into().max(px(1.0)));
        self
    }

    /// Sets the wick stroke width applied to high-low stems.
    pub fn wick_width(mut self, width: impl Into<Pixels>) -> Self {
        self.wick_width = width.into().max(px(1.0));
        self
    }

    /// Caps the number of rendered candles after even sampling.
    pub fn max_render_points(mut self, max_points: usize) -> Self {
        self.options.max_render_points = Some(max_points.max(3));
        self
    }

    /// Caps axis labels to keep dense charts readable.
    pub fn max_axis_labels(mut self, max_labels: usize) -> Self {
        self.options.max_axis_labels = max_labels.max(2);
        self
    }

    /// Caps close-value labels to avoid chart text collisions.
    pub fn max_value_labels(mut self, max_labels: usize) -> Self {
        self.options.max_value_labels = max_labels.max(2);
        self
    }

    /// Disables candle sampling for exact rendering.
    pub fn disable_downsampling(mut self) -> Self {
        self.options.max_render_points = None;
        self
    }

    /// Returns the OHLC data owned by this chart.
    pub fn points(&self) -> &[CandlestickPoint] {
        &self.points
    }

    /// Returns shared chart options configured through the builder API.
    pub fn options(&self) -> &ChartOptions {
        &self.options
    }

    /// Returns the configured body width override.
    pub fn body_width_value(&self) -> Option<Pixels> {
        self.body_width
    }
}

impl IntoElement for CandlestickChart {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

impl RenderOnce for CandlestickChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let palette = ChartPalette::from_config(cx.global::<Config>());
        let valid_points = self
            .points
            .iter()
            .filter(|point| point.is_valid())
            .cloned()
            .collect::<Vec<_>>();
        let has_data = !valid_points.is_empty();
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
                .child(Empty::new().description("暂无 K 线数据"))
                .into_any_element();
        }

        if self.options.show_legend {
            shell = shell.child(render_legend(
                self.up_color.unwrap_or(theme.success.base),
                self.down_color.unwrap_or(theme.danger.base),
            ));
        }

        shell
            .child(render_candlestick_canvas(
                valid_points,
                self.options,
                palette,
                self.up_color.unwrap_or(theme.success.base),
                self.down_color.unwrap_or(theme.danger.base),
                self.wick_width,
                self.body_width,
            ))
            .into_any_element()
    }
}

fn render_legend(up_color: Hsla, down_color: Hsla) -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Space::new()
                .gap_xs()
                .align_center()
                .child(div().w(px(10.0)).h(px(10.0)).rounded_sm().bg(up_color))
                .child(Text::new("Rising / unchanged").size(px(12.0))),
        )
        .child(
            Space::new()
                .gap_xs()
                .align_center()
                .child(div().w(px(10.0)).h(px(10.0)).rounded_sm().bg(down_color))
                .child(Text::new("Falling").size(px(12.0))),
        )
}

fn render_candlestick_canvas(
    points: Vec<CandlestickPoint>,
    options: ChartOptions,
    palette: ChartPalette,
    up_color: Hsla,
    down_color: Hsla,
    wick_width: Pixels,
    body_width: Option<Pixels>,
) -> impl IntoElement {
    let height = options.height;
    let render_points = sample_candles(&points, options.max_render_points);
    let axis_points = render_points.clone();
    let tooltip_points = render_points.clone();
    let tooltip_options = options.clone();
    let bounds_cell: Rc<Cell<Bounds<Pixels>>> = Rc::new(Cell::new(Bounds::default()));
    let tooltip_bounds = bounds_cell.clone();
    let tooltip_id: SharedString = format!("{}-tooltip", options.id).into();
    let move_id = tooltip_id.clone();
    let chart = canvas(
        |_, _, _| (),
        move |bounds, _, window, cx| {
            if render_points.is_empty() {
                return;
            }
            let padding = options.padding;
            let left = bounds.left() + padding.left;
            let right = bounds.right() - padding.right;
            let top = bounds.top() + padding.top;
            let bottom = bounds.bottom() - padding.bottom;
            let width = (right - left).max(px(1.0));
            let plot_height = (bottom - top).max(px(1.0));
            let x = ScalePoint::from_len(render_points.len(), (0.0, width.as_f32()));
            let domain = candlestick_domain(&render_points, options.y_domain);
            let y = ScaleLinear::new(domain, (plot_height.as_f32(), 0.0));
            if options.show_grid || options.show_axis {
                paint_chart_frame(
                    left,
                    top,
                    width,
                    plot_height,
                    &candlestick_axis_labels(&axis_points, options.max_axis_labels),
                    &x,
                    &y,
                    &palette,
                    &options,
                    window,
                    cx,
                );
            }

            let candle_width = resolved_body_width(render_points.len(), width.as_f32(), body_width);
            for (index, candle) in render_points.iter().enumerate() {
                let Some(center_x) = x.tick_index(index) else {
                    continue;
                };
                let color = if candle.is_rising() {
                    up_color
                } else {
                    down_color
                };
                let high_y = y.tick(candle.high).clamp(0.0, plot_height.as_f32());
                let low_y = y.tick(candle.low).clamp(0.0, plot_height.as_f32());
                let open_y = y.tick(candle.open).clamp(0.0, plot_height.as_f32());
                let close_y = y.tick(candle.close).clamp(0.0, plot_height.as_f32());
                let body_top = open_y.min(close_y);
                let body_height = (open_y - close_y).abs().max(1.0);
                let wick_x = left + px(center_x) - wick_width / 2.0;
                window.paint_quad(fill(
                    Bounds::new(
                        point(wick_x, top + px(high_y)),
                        size(wick_width, px((low_y - high_y).max(1.0))),
                    ),
                    Background::from(color.opacity(0.78)),
                ));
                window.paint_quad(fill(
                    Bounds::new(
                        point(left + px(center_x - candle_width / 2.0), top + px(body_top)),
                        size(px(candle_width), px(body_height)),
                    ),
                    Background::from(color),
                ));
            }

            if options.show_value_labels {
                let indices = sparse_indices(render_points.len(), options.max_value_labels);
                for index in indices {
                    let Some(point_data) = render_points.get(index) else {
                        continue;
                    };
                    let Some(x_pos) = x.tick_index(index) else {
                        continue;
                    };
                    let close_y = y.tick(point_data.close).clamp(0.0, plot_height.as_f32());
                    paint_chart_label_aligned(
                        format_value_label(
                            point_data.close,
                            0.0,
                            options.y_format,
                            &options.value_label_options,
                        ),
                        point(left + px(x_pos) - px(18.0), top + px(close_y) - px(20.0)),
                        palette.label,
                        gpui::TextAlign::Center,
                        Some(px(36.0)),
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
            let domain = candlestick_domain(&tooltip_points, tooltip_options.y_domain);
            let Some(hit) = nearest_candlestick_hit_box(
                &tooltip_points,
                domain,
                plot_width,
                plot_height,
                body_width,
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
                    content: format_candlestick_tooltip(&hit, tooltip_options.y_format),
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

fn resolved_body_width(point_len: usize, plot_width: f32, body_width: Option<Pixels>) -> f32 {
    body_width
        .map(|width| width.as_f32().clamp(1.0, plot_width.max(1.0)))
        .unwrap_or_else(|| {
            if point_len <= 1 {
                (plot_width * 0.18).clamp(4.0, 16.0)
            } else {
                (plot_width / point_len as f32 * 0.56).clamp(3.0, 18.0)
            }
        })
}

fn sample_candles(points: &[CandlestickPoint], max_points: Option<usize>) -> Vec<CandlestickPoint> {
    let valid = points
        .iter()
        .filter(|point| point.is_valid())
        .cloned()
        .collect::<Vec<_>>();
    let Some(max_points) = max_points else {
        return valid;
    };
    if valid.len() <= max_points.max(2) {
        return valid;
    }
    sparse_indices(valid.len(), max_points)
        .into_iter()
        .filter_map(|index| valid.get(index).cloned())
        .collect()
}

fn candlestick_axis_labels(points: &[CandlestickPoint], max_labels: usize) -> Vec<ChartAxisLabel> {
    sparse_indices(points.len(), max_labels)
        .into_iter()
        .filter_map(|index| {
            points
                .get(index)
                .map(|point| ChartAxisLabel::new(index, point.label.clone()))
        })
        .collect()
}

fn format_candlestick_tooltip(
    hit: &CandlestickHitBox,
    value_format: Option<fn(f64) -> SharedString>,
) -> SharedString {
    let format = value_format.unwrap_or(crate::chart::default_y_format);
    let mut content = format!(
        "{}\nO {}  H {}\nL {}  C {}",
        hit.label,
        format(hit.open),
        format(hit.high),
        format(hit.low),
        format(hit.close)
    );
    if let Some(volume) = hit.volume {
        content.push_str(&format!("\nVol {}", format(volume)));
    }
    content.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{green, red};

    fn sample_points() -> Vec<CandlestickPoint> {
        vec![
            CandlestickPoint::new("Mon", 100.0, 110.0, 96.0, 108.0).volume(12_000.0),
            CandlestickPoint::new("Tue", 108.0, 112.0, 101.0, 103.0),
            CandlestickPoint::new("Wed", 103.0, 118.0, 102.0, 116.0),
        ]
    }

    #[test]
    fn candlestick_point_validates_ohlc_invariants() {
        assert!(CandlestickPoint::new("ok", 10.0, 12.0, 9.0, 11.0).is_valid());
        assert!(!CandlestickPoint::new("bad", 10.0, 9.0, 8.0, 11.0).is_valid());
        assert!(!CandlestickPoint::new("nan", 10.0, f64::NAN, 8.0, 9.0).is_valid());
    }

    #[test]
    fn candlestick_domain_uses_high_low_and_expands_flat_data() {
        let domain = candlestick_domain(&sample_points(), None);
        assert!(domain.0 < 96.0);
        assert!(domain.1 > 118.0);

        let flat = vec![CandlestickPoint::new("Flat", 10.0, 10.0, 10.0, 10.0)];
        let flat_domain = candlestick_domain(&flat, None);
        assert!(flat_domain.0 < 10.0);
        assert!(flat_domain.1 > 10.0);
    }

    #[test]
    fn candlestick_hit_boxes_classify_rising_and_falling_candles() {
        let points = sample_points();
        let domain = candlestick_domain(&points, None);
        let boxes = candlestick_hit_boxes(&points, domain, 300.0, 180.0, Some(px(8.0)));
        assert_eq!(boxes.len(), 3);
        assert!(boxes[0].is_rising());
        assert!(!boxes[1].is_rising());
        assert_eq!(boxes[0].width, 8.0);
    }

    #[test]
    fn candlestick_builder_tracks_options() {
        let chart = CandlestickChart::new(sample_points())
            .id("ohlc")
            .height(px(360.0))
            .show_grid(false)
            .show_axis(false)
            .show_legend(true)
            .show_value_labels(true)
            .show_tooltip(false)
            .tooltip_hit_radius(px(18.0))
            .value_label_content(ChartValueLabelContent::ValueAndPercentage)
            .value_label_placement(ChartValueLabelPlacement::OutsideFree)
            .percentage_decimals(2)
            .y_domain(90.0, 130.0)
            .up_color(green())
            .down_color(red())
            .body_width(px(9.0))
            .wick_width(px(2.0))
            .max_render_points(100)
            .max_axis_labels(6)
            .max_value_labels(8);

        assert_eq!(chart.options().id, SharedString::from("ohlc"));
        assert_eq!(chart.options().height, px(360.0));
        assert!(!chart.options().show_grid);
        assert!(!chart.options().show_axis);
        assert!(chart.options().show_legend);
        assert!(chart.options().show_value_labels);
        assert!(!chart.options().show_tooltip);
        assert_eq!(chart.options().tooltip_hit_radius, px(18.0));
        assert_eq!(chart.options().y_domain, Some((90.0, 130.0)));
        assert_eq!(chart.body_width_value(), Some(px(9.0)));
        assert_eq!(chart.points().len(), 3);
    }
}
