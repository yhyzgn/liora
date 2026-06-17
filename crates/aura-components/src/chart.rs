use aura_core::{Config, unique_id};
use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, LayoutId, Pixels, SharedString, Window, px,
};
use std::cell::Cell;
use std::rc::Rc;

pub struct ChartBoundsTracker {
    pub child: AnyElement,
    pub bounds: Rc<Cell<Bounds<Pixels>>>,
}

impl ChartBoundsTracker {
    pub fn new(child: impl IntoElement, bounds: Rc<Cell<Bounds<Pixels>>>) -> Self {
        Self {
            child: child.into_any_element(),
            bounds,
        }
    }
}

impl IntoElement for ChartBoundsTracker {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ChartBoundsTracker {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        self.child.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.bounds.set(bounds);
        self.child.paint(window, cx);
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartPoint {
    pub label: SharedString,
    pub value: f64,
}

impl ChartPoint {
    pub fn new(label: impl Into<SharedString>, value: f64) -> Self {
        Self {
            label: label.into(),
            value,
        }
    }

    pub fn is_finite(&self) -> bool {
        self.value.is_finite()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartLineStyle {
    Solid,
    Dashed,
    Dotted,
}

#[derive(Clone, Debug)]
pub struct ChartSeries {
    pub name: SharedString,
    pub points: Vec<ChartPoint>,
    pub color: Option<Hsla>,
    pub fill_color: Option<Hsla>,
    pub stroke_color: Option<Hsla>,
    pub stroke_width: Option<Pixels>,
    pub line_style: Option<ChartLineStyle>,
    pub dash_pattern: Option<Vec<Pixels>>,
    pub smooth: Option<bool>,
}

impl ChartSeries {
    pub fn new(
        name: impl Into<SharedString>,
        points: impl IntoIterator<Item = ChartPoint>,
    ) -> Self {
        Self {
            name: name.into(),
            points: points.into_iter().collect(),
            color: None,
            fill_color: None,
            stroke_color: None,
            stroke_width: None,
            line_style: None,
            dash_pattern: None,
            smooth: None,
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    pub fn fill_color(mut self, color: Hsla) -> Self {
        self.fill_color = Some(color);
        self
    }

    pub fn stroke_color(mut self, color: Hsla) -> Self {
        self.stroke_color = Some(color);
        self
    }

    pub fn stroke_width(mut self, width: Pixels) -> Self {
        self.stroke_width = Some(width);
        self
    }

    pub fn line_style(mut self, style: ChartLineStyle) -> Self {
        self.line_style = Some(style);
        self
    }

    pub fn dashed(self) -> Self {
        self.line_style(ChartLineStyle::Dashed)
    }

    pub fn dotted(self) -> Self {
        self.line_style(ChartLineStyle::Dotted)
    }

    pub fn solid(self) -> Self {
        self.line_style(ChartLineStyle::Solid)
    }

    pub fn dash_pattern(mut self, pattern: impl IntoIterator<Item = Pixels>) -> Self {
        self.dash_pattern = Some(
            pattern
                .into_iter()
                .map(|value| value.max(px(0.1)))
                .collect(),
        );
        self.line_style = Some(ChartLineStyle::Dashed);
        self
    }

    pub fn smooth(mut self, enabled: bool) -> Self {
        self.smooth = Some(enabled);
        self
    }

    pub fn resolved_fill_color(&self, fallback: Hsla) -> Hsla {
        self.fill_color.or(self.color).unwrap_or(fallback)
    }

    pub fn resolved_stroke_color(&self, fallback: Hsla) -> Hsla {
        self.stroke_color.or(self.color).unwrap_or(fallback)
    }

    pub fn finite_points(&self) -> impl Iterator<Item = &ChartPoint> {
        self.points.iter().filter(|point| point.is_finite())
    }

    pub fn is_empty(&self) -> bool {
        self.finite_points().next().is_none()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChartPadding {
    pub top: Pixels,
    pub right: Pixels,
    pub bottom: Pixels,
    pub left: Pixels,
}

impl Default for ChartPadding {
    fn default() -> Self {
        Self {
            top: px(18.0),
            right: px(18.0),
            bottom: px(34.0),
            left: px(44.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ChartPalette {
    pub series: Vec<Hsla>,
    pub axis: Hsla,
    pub grid: Hsla,
    pub label: Hsla,
}

impl ChartPalette {
    pub fn from_config(config: &Config) -> Self {
        let theme = &config.theme;
        Self {
            series: vec![
                theme.primary.base,
                theme.info.base,
                theme.success.base,
                theme.warning.base,
                theme.danger.base,
                theme.primary.hover,
                theme.info.hover,
                theme.warning.hover,
            ],
            axis: theme.neutral.border,
            grid: theme.neutral.divider.opacity(0.72),
            label: theme.neutral.text_3,
        }
    }

    pub fn series_color(&self, index: usize) -> Hsla {
        self.series
            .get(index % self.series.len().max(1))
            .copied()
            .unwrap_or_else(|| gpui::blue())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartValueLabelContent {
    Value,
    Percentage,
    ValueAndPercentage,
    ValueOverTotal,
    ValueOverTotalAndPercentage,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChartValueLabelPlacement {
    Auto,
    Inside,
    OutsideFree,
    OutsideAligned,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChartValueLabelOptions {
    pub content: ChartValueLabelContent,
    pub placement: ChartValueLabelPlacement,
    pub percentage_decimals: usize,
    pub outside_threshold_degrees: u16,
}

impl Default for ChartValueLabelOptions {
    fn default() -> Self {
        Self {
            content: ChartValueLabelContent::Value,
            placement: ChartValueLabelPlacement::Auto,
            percentage_decimals: 1,
            outside_threshold_degrees: 28,
        }
    }
}

#[derive(Clone)]
pub struct ChartOptions {
    pub id: SharedString,
    pub height: Pixels,
    pub padding: ChartPadding,
    pub show_grid: bool,
    pub show_axis: bool,
    pub show_legend: bool,
    pub y_domain: Option<(f64, f64)>,
    pub y_tick_count: usize,
    pub y_format: Option<fn(f64) -> SharedString>,
    pub show_value_labels: bool,
    pub value_label_options: ChartValueLabelOptions,
    pub max_render_points: Option<usize>,
    pub max_axis_labels: usize,
    pub max_value_labels: usize,
    pub show_tooltip: bool,
    pub tooltip_hit_radius: Pixels,
}

impl Default for ChartOptions {
    fn default() -> Self {
        Self {
            id: unique_id("chart"),
            height: px(280.0),
            padding: ChartPadding::default(),
            show_grid: true,
            show_axis: true,
            show_legend: true,
            y_domain: None,
            y_tick_count: 4,
            y_format: None,
            show_value_labels: true,
            value_label_options: ChartValueLabelOptions::default(),
            max_render_points: Some(800),
            max_axis_labels: 8,
            max_value_labels: 32,
            show_tooltip: true,
            tooltip_hit_radius: px(12.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartHitPoint {
    pub series_index: usize,
    pub point_index: usize,
    pub series_name: SharedString,
    pub label: SharedString,
    pub value: f64,
    pub x: f32,
    pub y: f32,
    pub distance: f32,
}

pub fn nearest_cartesian_hit_point(
    series: &[ChartSeries],
    domain: (f64, f64),
    plot_width: f32,
    plot_height: f32,
    pointer_x: f32,
    pointer_y: f32,
    max_distance: f32,
) -> Option<ChartHitPoint> {
    if series.is_empty()
        || !plot_width.is_finite()
        || !plot_height.is_finite()
        || plot_width <= 0.0
        || plot_height <= 0.0
        || !pointer_x.is_finite()
        || !pointer_y.is_finite()
        || !max_distance.is_finite()
        || max_distance < 0.0
        || pointer_x < 0.0
        || pointer_y < 0.0
        || pointer_x > plot_width
        || pointer_y > plot_height
    {
        return None;
    }

    let domain_len = label_domain_len(series);
    if domain_len == 0 {
        return None;
    }

    let span = domain.1 - domain.0;
    if !domain.0.is_finite() || !domain.1.is_finite() || span.abs() < f64::EPSILON {
        return None;
    }

    let x_for_index = |index: usize| -> Option<f32> {
        if index >= domain_len {
            return None;
        }
        if domain_len == 1 {
            Some(plot_width / 2.0)
        } else {
            Some(plot_width * index as f32 / (domain_len - 1) as f32)
        }
    };
    let y_for_value = |value: f64| -> Option<f32> {
        if !value.is_finite() {
            return None;
        }
        let t = ((value - domain.0) / span) as f32;
        Some((plot_height - plot_height * t).clamp(0.0, plot_height))
    };

    let mut best: Option<ChartHitPoint> = None;
    let mut best_distance_sq = max_distance * max_distance;

    for (series_index, current) in series.iter().enumerate() {
        for (point_index, point) in current.points.iter().enumerate() {
            if !point.is_finite() {
                continue;
            }
            let Some(x) = x_for_index(point_index) else {
                continue;
            };
            let Some(y) = y_for_value(point.value) else {
                continue;
            };
            let dx = x - pointer_x;
            let dy = y - pointer_y;
            let distance_sq = dx * dx + dy * dy;
            if distance_sq <= best_distance_sq {
                best_distance_sq = distance_sq;
                best = Some(ChartHitPoint {
                    series_index,
                    point_index,
                    series_name: current.name.clone(),
                    label: point.label.clone(),
                    value: point.value,
                    x,
                    y,
                    distance: distance_sq.sqrt(),
                });
            }
        }
    }

    best
}

pub fn format_hit_tooltip(
    hit: &ChartHitPoint,
    formatter: Option<fn(f64) -> SharedString>,
) -> SharedString {
    let format_value = formatter.unwrap_or(default_y_format);
    format!(
        "{} · {}: {}",
        hit.series_name,
        hit.label,
        format_value(hit.value)
    )
    .into()
}

pub fn default_y_format(value: f64) -> SharedString {
    if value.abs() >= 1000.0 {
        format!("{value:.0}").into()
    } else if value.fract().abs() < f64::EPSILON {
        format!("{value:.0}").into()
    } else {
        format!("{value:.1}").into()
    }
}

pub fn format_value_label(
    value: f64,
    total: f64,
    formatter: Option<fn(f64) -> SharedString>,
    options: &ChartValueLabelOptions,
) -> SharedString {
    let format_value = formatter.unwrap_or(default_y_format);
    let value_text = format_value(value);
    let total_text = format_value(total);
    let percentage = if total.abs() > f64::EPSILON {
        value / total * 100.0
    } else {
        0.0
    };
    match options.content {
        ChartValueLabelContent::Value => value_text,
        ChartValueLabelContent::Percentage => {
            format!("{:.*}%", options.percentage_decimals, percentage).into()
        }
        ChartValueLabelContent::ValueAndPercentage => format!(
            "{} ({:.*}%)",
            value_text, options.percentage_decimals, percentage
        )
        .into(),
        ChartValueLabelContent::ValueOverTotal => format!("{} / {}", value_text, total_text).into(),
        ChartValueLabelContent::ValueOverTotalAndPercentage => format!(
            "{} / {} ({:.*}%)",
            value_text, total_text, options.percentage_decimals, percentage
        )
        .into(),
    }
}

pub fn series_total(series: &ChartSeries) -> f64 {
    series
        .finite_points()
        .map(|point| point.value.max(0.0))
        .sum()
}

pub fn finite_domain(series: &[ChartSeries]) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    for value in series
        .iter()
        .flat_map(|series| series.finite_points().map(|point| point.value))
    {
        min = min.min(value);
        max = max.max(value);
    }
    if min.is_finite() && max.is_finite() {
        Some((min, max))
    } else {
        None
    }
}

pub fn normalized_domain(domain: Option<(f64, f64)>, series: &[ChartSeries]) -> (f64, f64) {
    normalized_domain_with_baseline(domain, series, true)
}

pub fn normalized_domain_with_baseline(
    domain: Option<(f64, f64)>,
    series: &[ChartSeries],
    include_zero: bool,
) -> (f64, f64) {
    let (mut min, mut max) = domain
        .filter(|(min, max)| min.is_finite() && max.is_finite())
        .or_else(|| finite_domain(series))
        .unwrap_or((0.0, 1.0));

    if include_zero && min > 0.0 {
        min = 0.0;
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
    (min, max)
}

pub fn stacked_domain(series: &[ChartSeries]) -> Option<(f64, f64)> {
    let labels_len = label_domain_len(series);
    if labels_len == 0 {
        return finite_domain(series);
    }

    let mut max_total = 0.0_f64;
    let mut min_total = 0.0_f64;
    let mut seen = false;
    for index in 0..labels_len {
        let mut positive = 0.0_f64;
        let mut negative = 0.0_f64;
        for point in series.iter().filter_map(|series| series.points.get(index)) {
            if !point.is_finite() {
                continue;
            }
            seen = true;
            if point.value >= 0.0 {
                positive += point.value;
            } else {
                negative += point.value;
            }
        }
        max_total = max_total.max(positive);
        min_total = min_total.min(negative);
    }

    if seen {
        Some((min_total, max_total))
    } else {
        None
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChartAxisLabel {
    pub index: usize,
    pub label: SharedString,
}

impl ChartAxisLabel {
    pub fn new(index: usize, label: impl Into<SharedString>) -> Self {
        Self {
            index,
            label: label.into(),
        }
    }
}

pub fn collect_labels(series: &[ChartSeries]) -> Vec<SharedString> {
    series
        .iter()
        .max_by_key(|series| series.points.len())
        .map(|series| {
            series
                .points
                .iter()
                .map(|point| point.label.clone())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default()
}

pub fn label_domain_len(series: &[ChartSeries]) -> usize {
    series
        .iter()
        .map(|series| series.points.len())
        .max()
        .unwrap_or(0)
}

pub fn collect_axis_labels(series: &[ChartSeries], max_labels: usize) -> Vec<ChartAxisLabel> {
    let Some(longest) = series.iter().max_by_key(|series| series.points.len()) else {
        return Vec::new();
    };
    sparse_axis_labels(&longest.points, max_labels)
}

pub fn sparse_indices(len: usize, max_count: usize) -> Vec<usize> {
    if len == 0 {
        return Vec::new();
    }
    let max_count = max_count.max(2);
    if len <= max_count {
        return (0..len).collect();
    }

    let last = len - 1;
    let intervals = max_count - 1;
    let mut indices = Vec::with_capacity(max_count);
    let mut previous = None;
    for slot in 0..=intervals {
        let mut index = ((slot * last) + intervals / 2) / intervals;
        if slot == 0 {
            index = 0;
        } else if slot == intervals {
            index = last;
        }
        if previous == Some(index) {
            continue;
        }
        indices.push(index);
        previous = Some(index);
    }
    indices
}

pub fn sparse_axis_labels(points: &[ChartPoint], max_labels: usize) -> Vec<ChartAxisLabel> {
    sparse_indices(points.len(), max_labels)
        .into_iter()
        .map(|index| ChartAxisLabel::new(index, points[index].label.clone()))
        .collect()
}

pub fn has_chart_data(series: &[ChartSeries]) -> bool {
    series.iter().any(|series| !series.is_empty())
}

/// Downsample an indexed value slice without first allocating every finite
/// `(index, value)` pair. It makes one cheap count pass and one bucket pass,
/// returning only the bounded render set while preserving first/last finite
/// values and local min/max extrema.
pub fn downsample_index_range<F>(
    len: usize,
    value_at: F,
    max_points: Option<usize>,
) -> Vec<(usize, f64)>
where
    F: Fn(usize) -> f64,
{
    let collect_finite = || {
        (0..len)
            .filter_map(|index| {
                let value = value_at(index);
                value.is_finite().then_some((index, value))
            })
            .collect::<Vec<_>>()
    };

    let Some(max_points) = max_points.filter(|max| *max >= 3) else {
        return collect_finite();
    };

    let finite_count = (0..len)
        .map(&value_at)
        .filter(|value| value.is_finite())
        .count();
    if finite_count == 0 {
        return Vec::new();
    }
    if finite_count <= max_points {
        return collect_finite();
    }

    let bucket_count = ((max_points.saturating_sub(2)) / 2).max(1);
    let middle_len = finite_count.saturating_sub(2);
    let bucket_size = (middle_len as f64 / bucket_count as f64).ceil() as usize;
    let mut sampled = Vec::with_capacity(max_points.min(finite_count));
    let mut finite_ordinal = 0usize;
    let mut first = None;
    let mut last = None;
    let mut bucket_start = 1usize;
    let mut bucket_end = (bucket_start + bucket_size).min(finite_count - 1);
    let mut bucket_min: Option<(usize, f64, usize)> = None;
    let mut bucket_max: Option<(usize, f64, usize)> = None;

    let flush_bucket = |sampled: &mut Vec<(usize, f64)>,
                        bucket_min: &mut Option<(usize, f64, usize)>,
                        bucket_max: &mut Option<(usize, f64, usize)>| {
        let (Some(min), Some(max)) = (*bucket_min, *bucket_max) else {
            return;
        };
        if min.2 <= max.2 {
            sampled.push((min.0, min.1));
            if min.2 != max.2 && sampled.len() + 1 < max_points {
                sampled.push((max.0, max.1));
            }
        } else {
            sampled.push((max.0, max.1));
            if sampled.len() + 1 < max_points {
                sampled.push((min.0, min.1));
            }
        }
        *bucket_min = None;
        *bucket_max = None;
    };

    for index in 0..len {
        let current_value = value_at(index);
        if !current_value.is_finite() {
            continue;
        }

        if finite_ordinal == 0 {
            first = Some((index, current_value));
        }
        if finite_ordinal == finite_count - 1 {
            last = Some((index, current_value));
        } else if finite_ordinal >= bucket_start && finite_ordinal < finite_count - 1 {
            while finite_ordinal >= bucket_end && sampled.len() + 1 < max_points {
                flush_bucket(&mut sampled, &mut bucket_min, &mut bucket_max);
                bucket_start = bucket_end;
                bucket_end = (bucket_start + bucket_size).min(finite_count - 1);
            }
            let candidate = (index, current_value, finite_ordinal);
            if bucket_min
                .as_ref()
                .is_none_or(|(_, min_value, _)| current_value < *min_value)
            {
                bucket_min = Some(candidate);
            }
            if bucket_max
                .as_ref()
                .is_none_or(|(_, max_value, _)| current_value > *max_value)
            {
                bucket_max = Some(candidate);
            }
        }
        finite_ordinal += 1;
    }

    if let Some(first) = first {
        sampled.insert(0, first);
    }
    if sampled.len() + 1 < max_points {
        flush_bucket(&mut sampled, &mut bucket_min, &mut bucket_max);
    }
    if sampled.len() >= max_points {
        sampled.pop();
    }
    if let Some(last) = last {
        sampled.push(last);
    }
    sampled
}

pub fn downsample_indexed_values<T, F>(
    items: &[T],
    value: F,
    max_points: Option<usize>,
) -> Vec<(usize, f64)>
where
    F: Fn(&T) -> f64,
{
    downsample_index_range(items.len(), |index| value(&items[index]), max_points)
}

/// Downsample a finite point stream while preserving first/last points and
/// local min/max extrema in each bucket. This keeps long native path rendering
/// bounded without hiding short spikes in monitoring-style charts.
pub fn downsample_points<T>(points: &[(T, f64)], max_points: Option<usize>) -> Vec<(T, f64)>
where
    T: Copy,
{
    let finite = points
        .iter()
        .copied()
        .filter(|(_, value)| value.is_finite())
        .collect::<Vec<_>>();
    let Some(max_points) = max_points.filter(|max| *max >= 3) else {
        return finite;
    };
    if finite.len() <= max_points {
        return finite;
    }

    let bucket_count = ((max_points.saturating_sub(2)) / 2).max(1);
    let middle_len = finite.len().saturating_sub(2);
    let bucket_size = (middle_len as f64 / bucket_count as f64).ceil() as usize;
    let mut sampled = Vec::with_capacity(max_points.min(finite.len()));
    sampled.push(finite[0]);

    let mut start = 1;
    while start < finite.len() - 1 && sampled.len() + 1 < max_points {
        let end = (start + bucket_size).min(finite.len() - 1);
        let bucket = &finite[start..end];
        if !bucket.is_empty() {
            let (min_offset, _) = bucket
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.1.total_cmp(&b.1))
                .unwrap();
            let (max_offset, _) = bucket
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.1.total_cmp(&b.1))
                .unwrap();
            if min_offset <= max_offset {
                sampled.push(bucket[min_offset]);
                if min_offset != max_offset && sampled.len() + 1 < max_points {
                    sampled.push(bucket[max_offset]);
                }
            } else {
                sampled.push(bucket[max_offset]);
                if sampled.len() + 1 < max_points {
                    sampled.push(bucket[min_offset]);
                }
            }
        }
        start = end;
    }

    let last = *finite.last().unwrap();
    if sampled.len() >= max_points {
        sampled.pop();
    }
    sampled.push(last);
    sampled
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chart_series_builder_tracks_visual_overrides() {
        let series = ChartSeries::new("metrics", [ChartPoint::new("a", 1.0)])
            .fill_color(gpui::red())
            .stroke_color(gpui::blue())
            .stroke_width(px(3.0))
            .smooth(false);
        assert_eq!(series.fill_color, Some(gpui::red()));
        assert_eq!(series.stroke_color, Some(gpui::blue()));
        assert_eq!(series.stroke_width, Some(px(3.0)));
        assert_eq!(series.smooth, Some(false));
    }

    #[test]
    fn value_labels_format_content_variants() {
        let options = ChartValueLabelOptions {
            content: ChartValueLabelContent::ValueOverTotalAndPercentage,
            percentage_decimals: 2,
            ..ChartValueLabelOptions::default()
        };
        assert_eq!(
            format_value_label(1.0, 4.0, None, &options),
            SharedString::from("1 / 4 (25.00%)")
        );
    }

    #[test]
    fn chart_options_enable_tooltip_by_default() {
        let options = ChartOptions::default();
        assert!(options.show_tooltip);
        assert_eq!(options.tooltip_hit_radius, px(12.0));
    }

    #[test]
    fn hit_tooltip_uses_series_label_and_formatter() {
        let hit = ChartHitPoint {
            series_index: 0,
            point_index: 1,
            series_name: "CPU".into(),
            label: "10:05".into(),
            value: 42.25,
            x: 10.0,
            y: 20.0,
            distance: 2.0,
        };

        assert_eq!(
            format_hit_tooltip(&hit, Some(|value| format!("{value:.1}%").into())),
            SharedString::from("CPU · 10:05: 42.2%")
        );
    }

    #[test]
    fn chart_series_filters_non_finite_points() {
        let series = ChartSeries::new(
            "metrics",
            [
                ChartPoint::new("a", 1.0),
                ChartPoint::new("bad", f64::NAN),
                ChartPoint::new("b", 2.0),
                ChartPoint::new("inf", f64::INFINITY),
            ],
        );

        let values = series
            .finite_points()
            .map(|point| point.value)
            .collect::<Vec<_>>();
        assert_eq!(values, vec![1.0, 2.0]);
    }

    #[test]
    fn normalized_domain_includes_zero_and_expands_single_value() {
        let series = [ChartSeries::new("one", [ChartPoint::new("a", 10.0)])];
        assert_eq!(normalized_domain(None, &series), (0.0, 10.0));

        let negative = [ChartSeries::new("negative", [ChartPoint::new("a", -4.0)])];
        assert_eq!(normalized_domain(None, &negative), (-4.4, -3.6));
    }

    #[test]
    fn stacked_domain_sums_same_index_values() {
        let series = [
            ChartSeries::new(
                "a",
                [ChartPoint::new("Q1", 2.0), ChartPoint::new("Q2", -1.0)],
            ),
            ChartSeries::new(
                "b",
                [ChartPoint::new("Q1", 3.0), ChartPoint::new("Q2", -4.0)],
            ),
        ];
        assert_eq!(stacked_domain(&series), Some((-5.0, 5.0)));
    }

    #[test]
    fn collect_labels_uses_longest_series() {
        let labels = collect_labels(&[
            ChartSeries::new("a", [ChartPoint::new("Q1", 1.0)]),
            ChartSeries::new(
                "b",
                [ChartPoint::new("Q1", 2.0), ChartPoint::new("Q2", 3.0)],
            ),
        ]);
        assert_eq!(
            labels,
            vec![SharedString::from("Q1"), SharedString::from("Q2")]
        );
    }

    #[test]
    fn sparse_indices_preserve_edges_and_cap_count() {
        let indices = sparse_indices(100, 8);
        assert_eq!(indices.len(), 8);
        assert_eq!(indices.first(), Some(&0));
        assert_eq!(indices.last(), Some(&99));
    }

    #[test]
    fn collect_axis_labels_caps_dense_domains() {
        let series = [ChartSeries::new(
            "dense",
            (0..100).map(|index| ChartPoint::new(format!("T{index}"), index as f64)),
        )];
        let labels = collect_axis_labels(&series, 8);

        assert_eq!(labels.len(), 8);
        assert_eq!(labels.first().map(|label| label.index), Some(0));
        assert_eq!(labels.last().map(|label| label.index), Some(99));
        assert_eq!(label_domain_len(&series), 100);
    }

    #[test]
    fn downsample_index_range_preserves_edges_and_extrema_without_dense_output() {
        let sampled = downsample_index_range(
            10_000,
            |index| {
                if index == 5_432 {
                    999_999.0
                } else {
                    index as f64
                }
            },
            Some(101),
        );

        assert!(sampled.len() <= 101);
        assert_eq!(sampled.first(), Some(&(0, 0.0)));
        assert_eq!(sampled.last(), Some(&(9_999, 9_999.0)));
        assert!(sampled.contains(&(5_432, 999_999.0)));
    }

    #[test]
    fn downsample_indexed_values_preserves_edges_and_extrema_without_dense_output() {
        let values = (0..10_000)
            .map(|index| {
                if index == 5_432 {
                    999_999.0
                } else {
                    index as f64
                }
            })
            .collect::<Vec<_>>();
        let sampled = downsample_indexed_values(&values, |value| *value, Some(101));

        assert!(sampled.len() <= 101);
        assert_eq!(sampled.first(), Some(&(0, 0.0)));
        assert_eq!(sampled.last(), Some(&(9_999, 9_999.0)));
        assert!(sampled.contains(&(5_432, 999_999.0)));
    }

    #[test]
    fn downsample_indexed_values_filters_non_finite_values() {
        let values = [0.0, f64::NAN, 2.0, f64::INFINITY, 4.0];
        assert_eq!(
            downsample_indexed_values(&values, |value| *value, Some(10)),
            vec![(0, 0.0), (2, 2.0), (4, 4.0)]
        );
    }

    #[test]
    fn downsample_points_preserves_edges_and_extrema() {
        let points = (0..100)
            .map(|index| {
                let value = if index == 42 { 1000.0 } else { index as f64 };
                (index, value)
            })
            .collect::<Vec<_>>();
        let sampled = downsample_points(&points, Some(21));

        assert!(sampled.len() <= 21);
        assert_eq!(sampled.first(), Some(&(0, 0.0)));
        assert_eq!(sampled.last(), Some(&(99, 99.0)));
        assert!(sampled.contains(&(42, 1000.0)));
    }

    #[test]
    fn downsample_points_can_be_disabled() {
        let points = (0..10)
            .map(|index| (index, index as f64))
            .collect::<Vec<_>>();
        assert_eq!(downsample_points(&points, None), points);
        assert_eq!(downsample_points(&points, Some(2)), points);
    }

    #[test]
    fn nearest_cartesian_hit_point_returns_closest_finite_point() {
        let series = [
            ChartSeries::new(
                "cpu",
                [
                    ChartPoint::new("t0", 0.0),
                    ChartPoint::new("t1", 50.0),
                    ChartPoint::new("t2", f64::NAN),
                ],
            ),
            ChartSeries::new(
                "mem",
                [
                    ChartPoint::new("t0", 10.0),
                    ChartPoint::new("t1", 80.0),
                    ChartPoint::new("t2", 100.0),
                ],
            ),
        ];

        let hit = nearest_cartesian_hit_point(&series, (0.0, 100.0), 200.0, 100.0, 198.0, 2.0, 8.0)
            .expect("pointer near last mem point should hit");

        assert_eq!(hit.series_index, 1);
        assert_eq!(hit.point_index, 2);
        assert_eq!(hit.series_name, SharedString::from("mem"));
        assert_eq!(hit.label, SharedString::from("t2"));
        assert_eq!(hit.value, 100.0);
        assert!(hit.distance <= 8.0);
    }

    #[test]
    fn nearest_cartesian_hit_point_respects_threshold_and_bounds() {
        let series = [ChartSeries::new(
            "cpu",
            [ChartPoint::new("t0", 0.0), ChartPoint::new("t1", 100.0)],
        )];

        assert_eq!(
            nearest_cartesian_hit_point(&series, (0.0, 100.0), 100.0, 100.0, 50.0, 50.0, 10.0),
            None
        );
        assert_eq!(
            nearest_cartesian_hit_point(&series, (0.0, 100.0), 100.0, 100.0, -1.0, 0.0, 10.0),
            None
        );
    }
}
