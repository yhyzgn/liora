use gpui::{AnyView, App, Context, Render, Window, blue, green, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{AreaChart, ChartPoint, ChartSeries, ChartValueLabelContent, Space};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AreaChartDemo).into()
}

struct AreaChartDemo;

impl Render for AreaChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "AreaChart 面积图",
            "使用填充路径展示趋势规模，支持叠加、堆叠面积，以及叠加模式下的 hover tooltip。",
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
                    "多序列叠加 + hover tooltip",
                    "多条半透明面积用于对比趋势；叠加模式下鼠标靠近点位会显示最近点。",
                    AreaChart::new(multi_series())
                        .id("area-chart-demo-overlay")
                        .height(px(400.0))
                        .y_domain(0.0, 100.0)
                        .tooltip_hit_radius(px(18.0)),
                ))
                .child(section(
                    "平滑、颜色与百分比标签",
                    "序列颜色、填充透明度、线条粗细和平滑策略都可单独配置。",
                    AreaChart::new(custom_series())
                        .id("area-chart-demo-custom")
                        .height(px(400.0))
                        .y_domain(0.0, 100.0)
                        .smooth(true)
                        .value_label_content(ChartValueLabelContent::ValueAndPercentage)
                        .percentage_decimals(1),
                ))
                .child(section(
                    "堆叠面积",
                    "展示多个渠道共同组成的总量变化。",
                    AreaChart::new(multi_series())
                        .id("area-chart-demo-stacked")
                        .height(px(400.0))
                        .stacked(),
                ))
                .child(section(
                    "大数据降采样",
                    "长序列面积图可限制绘制点数；堆叠模式按总量形状保留尖峰。",
                    AreaChart::new(dense_series())
                        .id("area-chart-demo-downsample")
                        .height(px(400.0))
                        .stacked()
                        .max_render_points(160),
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

pub fn custom_series() -> Vec<ChartSeries> {
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
        )
        .stroke_color(blue())
        .fill_color(blue().opacity(0.36))
        .stroke_width(px(3.0))
        .smooth(true),
        ChartSeries::new(
            "Mobile",
            [
                ChartPoint::new("Mon", 18.0),
                ChartPoint::new("Tue", 25.0),
                ChartPoint::new("Wed", 32.0),
                ChartPoint::new("Thu", 39.0),
                ChartPoint::new("Fri", 48.0),
            ],
        )
        .stroke_color(green())
        .fill_color(green().opacity(0.24))
        .stroke_width(px(2.2))
        .smooth(false),
    ]
}

pub fn dense_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "Desktop",
            (0..1_800).map(|index| {
                let wave = ((index as f64) / 32.0).sin() * 14.0;
                ChartPoint::new(format!("T{index}"), 42.0 + wave)
            }),
        ),
        ChartSeries::new(
            "Mobile",
            (0..1_800).map(|index| {
                let wave = ((index as f64) / 27.0).cos() * 10.0;
                let spike = if index % 360 == 0 { 24.0 } else { 0.0 };
                ChartPoint::new(format!("T{index}"), 28.0 + wave + spike)
            }),
        ),
    ]
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
        assert!(source.contains("stroke_color"));
        assert!(source.contains("value_label_content"));
        assert!(source.contains("max_render_points"));
        assert!(source.contains("dense_series"));
    }
}
