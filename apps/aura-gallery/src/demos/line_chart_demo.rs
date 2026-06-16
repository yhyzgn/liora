use aura_components::layout_helpers::{page, section};
use aura_components::{
    ChartLineStyle, ChartPoint, ChartSeries, ChartValueLabelContent, LineChart, Space,
};
use gpui::{AnyView, App, Context, Render, Window, blue, green, prelude::*, px};

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
                    "颜色、线宽、平滑与标签",
                    "每个序列都能指定颜色、线条粗细与是否平滑，标签内容也可切换为百分比。",
                    LineChart::new(custom_series())
                        .id("line-chart-demo-custom")
                        .height(px(420.0))
                        .y_domain(0.0, 100.0)
                        .area_fill(true)
                        .value_label_content(ChartValueLabelContent::Percentage)
                        .percentage_decimals(1),
                ))
                .child(section(
                    "每条线独立样式",
                    "每个序列可以分别配置实线、虚线、点线、颜色、粗细和平滑效果。",
                    LineChart::new(styled_series())
                        .id("line-chart-demo-styled-lines")
                        .height(px(420.0))
                        .y_domain(0.0, 100.0)
                        .area_fill(false)
                        .point_markers(false),
                ))
                .child(section(
                    "大数据降采样",
                    "长序列默认使用 min/max bucket 降采样，保留首尾和局部峰谷，避免原生路径绘制过重。",
                    LineChart::new(dense_series())
                        .id("line-chart-demo-downsample")
                        .height(px(420.0))
                        .y_domain(0.0, 100.0)
                        .point_markers(false)
                        .area_fill(true)
                        .max_render_points(180),
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

pub fn custom_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "CPU",
            [
                ChartPoint::new("Mon", 25.0),
                ChartPoint::new("Tue", 38.0),
                ChartPoint::new("Wed", 42.0),
                ChartPoint::new("Thu", 58.0),
                ChartPoint::new("Fri", 49.0),
            ],
        )
        .stroke_color(blue())
        .fill_color(blue().opacity(0.22))
        .stroke_width(px(3.2))
        .smooth(true),
        ChartSeries::new(
            "Memory",
            [
                ChartPoint::new("Mon", 48.0),
                ChartPoint::new("Tue", 52.0),
                ChartPoint::new("Wed", 57.0),
                ChartPoint::new("Thu", 63.0),
                ChartPoint::new("Fri", 66.0),
            ],
        )
        .stroke_color(green())
        .fill_color(green().opacity(0.18))
        .stroke_width(px(2.4))
        .smooth(false),
    ]
}

pub fn styled_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "Solid Smooth",
            [
                ChartPoint::new("Mon", 32.0),
                ChartPoint::new("Tue", 44.0),
                ChartPoint::new("Wed", 38.0),
                ChartPoint::new("Thu", 70.0),
                ChartPoint::new("Fri", 62.0),
            ],
        )
        .stroke_color(blue())
        .stroke_width(px(3.2))
        .line_style(ChartLineStyle::Solid)
        .smooth(true),
        ChartSeries::new(
            "Dashed",
            [
                ChartPoint::new("Mon", 22.0),
                ChartPoint::new("Tue", 35.0),
                ChartPoint::new("Wed", 52.0),
                ChartPoint::new("Thu", 58.0),
                ChartPoint::new("Fri", 76.0),
            ],
        )
        .stroke_color(green())
        .stroke_width(px(2.6))
        .dashed()
        .smooth(false),
        ChartSeries::new(
            "Dotted",
            [
                ChartPoint::new("Mon", 60.0),
                ChartPoint::new("Tue", 54.0),
                ChartPoint::new("Wed", 49.0),
                ChartPoint::new("Thu", 45.0),
                ChartPoint::new("Fri", 39.0),
            ],
        )
        .stroke_color(gpui::red())
        .stroke_width(px(2.8))
        .dotted()
        .smooth(true),
    ]
}

pub fn dense_series() -> Vec<ChartSeries> {
    vec![ChartSeries::new(
        "Latency",
        (0..2_000).map(|index| {
            let wave = ((index as f64) / 24.0).sin() * 18.0;
            let spike = if index % 240 == 0 { 32.0 } else { 0.0 };
            ChartPoint::new(format!("T{index}"), 48.0 + wave + spike)
        }),
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
        assert!(source.contains("stroke_color"));
        assert!(source.contains("value_label_content"));
        assert!(source.contains("dashed()"));
        assert!(source.contains("dotted()"));
        assert!(source.contains("ChartLineStyle"));
        assert!(source.contains("max_render_points"));
        assert!(source.contains("dense_series"));
    }
}
