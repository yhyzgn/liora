use crate::chart::{ChartPalette, ChartSeries, has_chart_data};
use crate::{Empty, Space, Text};
use aura_core::{Config, unique_id};
use gpui::{
    App, Component, ElementId, Hsla, InteractiveElement, IntoElement, ParentElement, Pixels, Point,
    RenderOnce, SharedString, Styled, Window, canvas, div, point, px,
};

const PIE_STEPS_PER_SLICE: usize = 36;

#[derive(Clone)]
pub struct PieChart {
    slices: Vec<ChartSeries>,
    id: SharedString,
    height: Pixels,
    show_legend: bool,
}

#[derive(Clone)]
pub struct RingChart {
    slices: Vec<ChartSeries>,
    id: SharedString,
    height: Pixels,
    show_legend: bool,
    inner_ratio: f32,
}

impl PieChart {
    pub fn new(slices: impl IntoIterator<Item = ChartSeries>) -> Self {
        Self {
            slices: slices.into_iter().collect(),
            id: unique_id("pie-chart"),
            height: px(280.0),
            show_legend: true,
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
        render_shell(self.slices, self.id, self.height, self.show_legend, 0.0, cx)
    }
}

impl RenderOnce for RingChart {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        render_shell(
            self.slices,
            self.id,
            self.height,
            self.show_legend,
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
    height: Pixels,
) -> impl IntoElement {
    canvas(
        |_, _, _| (),
        move |bounds, _, window, _cx| {
            let inset = px(18.0);
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

            let mut start = -90.0_f32;
            for (index, (series, value)) in slices.iter().zip(values).enumerate() {
                if value <= 0.0 {
                    continue;
                }
                let sweep = (value / total) as f32 * 360.0;
                let color = series.color.unwrap_or_else(|| palette.series_color(index));
                if let Some(path) = pie_slice_path(center, radius, start, start + sweep) {
                    window.paint_path(path, color);
                }
                start += sweep;
            }

            if inner_ratio > 0.0 {
                let hole_radius = (radius * inner_ratio).max(0.0);
                if let Some(path) = circle_path(center, hole_radius) {
                    window.paint_path(path, hole_color);
                }
            }
        },
    )
    .w_full()
    .h(height)
}

fn pie_slice_path(
    center: Point<Pixels>,
    radius: f32,
    start_deg: f32,
    end_deg: f32,
) -> Option<gpui::Path<Pixels>> {
    let points = arc_points(center, radius, start_deg, end_deg, PIE_STEPS_PER_SLICE);
    let first = *points.first()?;
    let mut builder = gpui::PathBuilder::fill();
    builder.move_to(center);
    builder.line_to(first);
    for point in points.iter().skip(1) {
        builder.line_to(*point);
    }
    builder.close();
    builder.build().ok()
}

fn circle_path(center: Point<Pixels>, radius: f32) -> Option<gpui::Path<Pixels>> {
    let points = arc_points(center, radius, 0.0, 360.0, 64);
    let first = *points.first()?;
    let mut builder = gpui::PathBuilder::fill();
    builder.move_to(first);
    for point in points.iter().skip(1) {
        builder.line_to(*point);
    }
    builder.close();
    builder.build().ok()
}

fn arc_points(
    center: Point<Pixels>,
    radius: f32,
    start_deg: f32,
    end_deg: f32,
    steps: usize,
) -> Vec<Point<Pixels>> {
    let mut points = Vec::with_capacity(steps + 1);
    let start = start_deg.to_radians();
    let end = end_deg.to_radians();
    let span = (end - start).abs().max(0.001);
    let steps = ((span / std::f32::consts::TAU) * steps as f32)
        .ceil()
        .max(2.0) as usize;
    for step in 0..=steps {
        let t = step as f32 / steps as f32;
        let angle = start + (end - start) * t;
        points.push(point(
            center.x + px(radius * angle.cos()),
            center.y + px(radius * angle.sin()),
        ));
    }
    points
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
            .show_legend(false);
        assert_eq!(chart.slices().len(), 3);
    }

    #[test]
    fn ring_chart_tracks_inner_ratio() {
        let chart = RingChart::new(slices()).inner_ratio(0.5);
        assert_eq!(chart.slices().len(), 3);
        assert!(chart.inner_ratio >= 0.2 && chart.inner_ratio <= 0.9);
    }
}
