use aura_components::layout_helpers::{page, section};
use aura_components::{BarChart, ChartPoint, ChartSeries, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BarChartDemo).into()
}

struct BarChartDemo;

impl Render for BarChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "BarChart 柱状图",
            "使用 GPUI 原生矩形绘制分类统计，支持分组与堆叠。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础分组",
                    "比较不同分类下的单项指标。",
                    BarChart::new(revenue_series())
                        .id("bar-chart-demo-basic")
                        .height(px(380.0)),
                ))
                .child(section(
                    "多序列分组",
                    "多组指标共享同一个分类坐标轴。",
                    BarChart::new(multi_series())
                        .id("bar-chart-demo-grouped")
                        .height(px(400.0))
                        .y_domain(0.0, 120.0),
                ))
                .child(section(
                    "堆叠柱状图",
                    "在同一个分类柱中展示构成占比。",
                    BarChart::new(multi_series())
                        .id("bar-chart-demo-stacked")
                        .height(px(400.0))
                        .stacked(),
                )),
        )
    }
}

pub fn revenue_series() -> Vec<ChartSeries> {
    vec![ChartSeries::new(
        "Revenue",
        [
            ChartPoint::new("Q1", 42.0),
            ChartPoint::new("Q2", 58.0),
            ChartPoint::new("Q3", 73.0),
            ChartPoint::new("Q4", 96.0),
        ],
    )]
}

pub fn multi_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "Online",
            [
                ChartPoint::new("Jan", 42.0),
                ChartPoint::new("Feb", 58.0),
                ChartPoint::new("Mar", 64.0),
                ChartPoint::new("Apr", 72.0),
            ],
        ),
        ChartSeries::new(
            "Retail",
            [
                ChartPoint::new("Jan", 28.0),
                ChartPoint::new("Feb", 34.0),
                ChartPoint::new("Mar", 39.0),
                ChartPoint::new("Apr", 45.0),
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn bar_chart_demo_uses_component_api() {
        let source = include_str!("bar_chart_demo.rs");
        assert!(source.contains("BarChart::new"));
        assert!(source.contains("ChartSeries::new"));
        assert!(source.contains("stacked()"));
    }
}
