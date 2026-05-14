use aura_core::{Config, unique_id};
use gpui::{Hsla, Pixels, SharedString, px};

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

#[derive(Clone, Debug)]
pub struct ChartSeries {
    pub name: SharedString,
    pub points: Vec<ChartPoint>,
    pub color: Option<Hsla>,
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
        }
    }

    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
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
        }
    }
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
    let (mut min, mut max) = domain
        .filter(|(min, max)| min.is_finite() && max.is_finite())
        .or_else(|| finite_domain(series))
        .unwrap_or((0.0, 1.0));

    if min > 0.0 {
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

pub fn has_chart_data(series: &[ChartSeries]) -> bool {
    series.iter().any(|series| !series.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
