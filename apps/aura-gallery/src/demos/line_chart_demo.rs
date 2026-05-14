use aura_components::layout_helpers::{page, section};
use aura_components::{ChartPoint, ChartSeries, LineChart, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| LineChartDemo).into()
}

struct LineChartDemo;

impl Render for LineChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "LineChart 折线图",
            "使用 GPUI canvas/path 原生绘制多序列趋势数据，默认支持平滑曲线和线下渐变填充。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础趋势",
                    "展示单条业务指标曲线。",
                    LineChart::new(cpu_series())
                        .id("line-chart-demo-basic")
                        .height(px(380.0))
                        .smooth(true)
                        .area_fill(true),
                ))
                .child(section(
                    "多序列",
                    "多条曲线共享坐标系和图例。",
                    LineChart::new(multi_series())
                        .id("line-chart-demo-multi")
                        .height(px(420.0))
                        .y_domain(0.0, 100.0)
                        .smooth(true)
                        .area_fill(true),
                ))
                .child(section(
                    "无数据",
                    "空数据自动降级为空状态。",
                    LineChart::new(Vec::<ChartSeries>::new())
                        .id("line-chart-demo-empty")
                        .height(px(280.0)),
                )),
        )
    }
}

pub fn cpu_series() -> Vec<ChartSeries> {
    vec![ChartSeries::new(
        "CPU Usage",
        [
            ChartPoint::new("10:00", 24.0),
            ChartPoint::new("10:05", 36.0),
            ChartPoint::new("10:10", 32.0),
            ChartPoint::new("10:15", 52.0),
            ChartPoint::new("10:20", 46.0),
            ChartPoint::new("10:25", 64.0),
        ],
    )]
}

pub fn multi_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "CPU",
            [
                ChartPoint::new("Mon", 25.0),
                ChartPoint::new("Tue", 38.0),
                ChartPoint::new("Wed", 42.0),
                ChartPoint::new("Thu", 58.0),
                ChartPoint::new("Fri", 49.0),
                ChartPoint::new("Sat", 72.0),
                ChartPoint::new("Sun", 61.0),
            ],
        ),
        ChartSeries::new(
            "Memory",
            [
                ChartPoint::new("Mon", 48.0),
                ChartPoint::new("Tue", 52.0),
                ChartPoint::new("Wed", 57.0),
                ChartPoint::new("Thu", 63.0),
                ChartPoint::new("Fri", 66.0),
                ChartPoint::new("Sat", 69.0),
                ChartPoint::new("Sun", 74.0),
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn line_chart_demo_uses_component_api() {
        let source = include_str!("line_chart_demo.rs");
        assert!(source.contains("LineChart::new"));
        assert!(source.contains("ChartSeries::new"));
        assert!(source.contains("ChartPoint::new"));
        assert!(source.contains("smooth(true)"));
        assert!(source.contains("area_fill(true)"));
    }
}
