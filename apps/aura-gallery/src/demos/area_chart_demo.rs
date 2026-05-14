use aura_components::layout_helpers::{page, section};
use aura_components::{AreaChart, ChartPoint, ChartSeries, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AreaChartDemo).into()
}

struct AreaChartDemo;

impl Render for AreaChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "AreaChart 面积图",
            "使用填充路径展示趋势规模，支持叠加与堆叠面积。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础面积",
                    "突出趋势线下方的累计规模。",
                    AreaChart::new(visitor_series())
                        .id("area-chart-demo-basic")
                        .height(px(380.0)),
                ))
                .child(section(
                    "多序列叠加",
                    "多条半透明面积用于对比趋势。",
                    AreaChart::new(multi_series())
                        .id("area-chart-demo-overlay")
                        .height(px(400.0))
                        .y_domain(0.0, 100.0),
                ))
                .child(section(
                    "堆叠面积",
                    "展示多个渠道共同组成的总量变化。",
                    AreaChart::new(multi_series())
                        .id("area-chart-demo-stacked")
                        .height(px(400.0))
                        .stacked(),
                )),
        )
    }
}

pub fn visitor_series() -> Vec<ChartSeries> {
    vec![ChartSeries::new(
        "Visitors",
        [
            ChartPoint::new("Mon", 24.0),
            ChartPoint::new("Tue", 32.0),
            ChartPoint::new("Wed", 45.0),
            ChartPoint::new("Thu", 52.0),
            ChartPoint::new("Fri", 61.0),
            ChartPoint::new("Sat", 72.0),
            ChartPoint::new("Sun", 68.0),
        ],
    )]
}

pub fn multi_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "Desktop",
            [
                ChartPoint::new("Mon", 28.0),
                ChartPoint::new("Tue", 34.0),
                ChartPoint::new("Wed", 38.0),
                ChartPoint::new("Thu", 44.0),
                ChartPoint::new("Fri", 50.0),
            ],
        ),
        ChartSeries::new(
            "Mobile",
            [
                ChartPoint::new("Mon", 18.0),
                ChartPoint::new("Tue", 25.0),
                ChartPoint::new("Wed", 32.0),
                ChartPoint::new("Thu", 39.0),
                ChartPoint::new("Fri", 48.0),
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn area_chart_demo_uses_component_api() {
        let source = include_str!("area_chart_demo.rs");
        assert!(source.contains("AreaChart::new"));
        assert!(source.contains("ChartSeries::new"));
        assert!(source.contains("stacked()"));
    }
}
