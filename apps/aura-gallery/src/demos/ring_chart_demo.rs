use aura_components::layout_helpers::{page, section};
use aura_components::{ChartPoint, ChartSeries, RingChart, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| RingChartDemo).into()
}

struct RingChartDemo;

impl Render for RingChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "RingChart 圆环图",
            "使用中心留空的扇形 path 展示圆环占比。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础圆环",
                    "适合 KPI 占比展示。",
                    RingChart::new(slices())
                        .id("ring-chart-demo-basic")
                        .height(px(420.0)),
                ))
                .child(section(
                    "更厚圆环",
                    "增强中心空间感。",
                    RingChart::new(slices())
                        .id("ring-chart-demo-thick")
                        .height(px(420.0))
                        .inner_ratio(0.44),
                )),
        )
    }
}

fn slices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("A", [ChartPoint::new("A", 30.0)]),
        ChartSeries::new("B", [ChartPoint::new("B", 20.0)]),
        ChartSeries::new("C", [ChartPoint::new("C", 50.0)]),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn ring_chart_demo_uses_component_api() {
        let source = include_str!("ring_chart_demo.rs");
        assert!(source.contains("RingChart::new"));
    }
}
