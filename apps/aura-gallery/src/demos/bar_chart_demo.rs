use aura_components::layout_helpers::{page, row_md, section};
use aura_components::{
    BarChart, BarChartFill, BarChartValueColorRange, BarChartValueFillRange, ChartPoint,
    ChartSeries, ChartValueLabelContent, Space,
};
use gpui::{AnyView, App, Context, Render, Window, blue, green, prelude::*, px, rgb};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BarChartDemo).into()
}

struct BarChartDemo;

impl Render for BarChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "BarChart 柱状图",
            "使用 GPUI 原生矩形绘制分类统计，支持分组、堆叠和柱体 hover tooltip。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础分组 + hover tooltip",
                    "比较不同分类下的单项指标；鼠标悬停在柱体上会显示对应分类和值。",
                    BarChart::new(revenue_series())
                        .id("bar-chart-demo-basic")
                        .height(px(380.0))
                        .tooltip_hit_radius(px(10.0)),
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
                    "颜色、间距与标签内容",
                    "柱体颜色、组内间距、标签显示数量还是百分比都可配置。",
                    BarChart::new(custom_series())
                        .id("bar-chart-demo-custom")
                        .height(px(400.0))
                        .y_domain(0.0, 120.0)
                        .bar_gap_ratio(0.32)
                        .value_label_content(ChartValueLabelContent::ValueAndPercentage)
                        .percentage_decimals(1),
                ))
                .child(section(
                    "渐变柱体",
                    "普通柱状图也可以配置全局渐变、逐根柱渐变，以及按值区间切换渐变。",
                    BarChart::new(revenue_series())
                        .id("bar-chart-demo-gradient")
                        .height(px(360.0))
                        .bar_radius(px(6.0))
                        .bar_vertical_gradient(rgb(0x60a5fa).into(), rgb(0x2563eb).into())
                        .value_fill_ranges([
                            BarChartValueFillRange::new(
                                0.0,
                                60.0,
                                BarChartFill::vertical_gradient(rgb(0xbfdbfe).into(), rgb(0x3b82f6).into()),
                            ),
                            BarChartValueFillRange::new(
                                60.0,
                                100.0,
                                BarChartFill::vertical_gradient(rgb(0xfef08a).into(), rgb(0xf97316).into()),
                            ),
                        ]),
                ))
                .child(section(
                    "逐根柱渐变",
                    "每一根柱都可以指定不同的渐变填充，普通柱状图和迷你指标共用同一套填充能力。",
                    BarChart::new(revenue_series())
                        .id("bar-chart-demo-per-bar-gradient")
                        .height(px(340.0))
                        .bar_radius(px(8.0))
                        .bar_fills([
                            BarChartFill::vertical_gradient(rgb(0xdbeafe).into(), rgb(0x2563eb).into()),
                            BarChartFill::vertical_gradient(rgb(0xdcfce7).into(), rgb(0x16a34a).into()),
                            BarChartFill::vertical_gradient(rgb(0xffedd5).into(), rgb(0xea580c).into()),
                            BarChartFill::vertical_gradient(rgb(0xfce7f3).into(), rgb(0xdb2777).into()),
                        ]),
                ))
                .child(section(
                    "堆叠柱状图 + 分段 hover",
                    "在同一个分类柱中展示构成占比；tooltip 会命中具体堆叠分段而不是整根柱。",
                    BarChart::new(multi_series())
                        .id("bar-chart-demo-stacked")
                        .height(px(400.0))
                        .tooltip_hit_radius(px(8.0))
                        .stacked(),
                ))
                .child(section(
                    "独立柱状图 / 迷你指标",
                    "隐藏坐标轴、网格和图例，适合在卡片或看板中展示一组扁平化指标；固定柱宽时按内容宽度紧凑布局。",
                    row_md(vec![
                        BarChart::new(standalone_series())
                            .id("bar-chart-demo-standalone-compact")
                            .standalone()
                            .bar_width(px(8.0))
                            .bar_gap(px(4.0))
                            .bar_radius(px(5.0))
                            .value_color_ranges([
                                BarChartValueColorRange::new(0.0, 35.0, rgb(0x86efac).into()),
                                BarChartValueColorRange::new(35.0, 70.0, rgb(0x22c55e).into()),
                                BarChartValueColorRange::new(70.0, 100.0, rgb(0x16a34a).into()),
                            ])
                            .into_any_element(),
                        BarChart::new(standalone_series())
                            .id("bar-chart-demo-standalone-gradient")
                            .standalone()
                            .bar_width(px(10.0))
                            .bar_gap(px(5.0))
                            .bar_radius(px(8.0))
                            .bar_fills([
                                BarChartFill::vertical_gradient(rgb(0xc4b5fd).into(), rgb(0x7c3aed).into()),
                                BarChartFill::vertical_gradient(rgb(0xbae6fd).into(), rgb(0x0284c7).into()),
                                BarChartFill::vertical_gradient(rgb(0xfde68a).into(), rgb(0xd97706).into()),
                            ])
                            .into_any_element(),
                        BarChart::new(standalone_series())
                            .id("bar-chart-demo-standalone-wide")
                            .standalone()
                            .height(px(96.0))
                            .bar_width(px(14.0))
                            .bar_gap(px(8.0))
                            .bar_radius(px(3.0))
                            .show_tooltip(false)
                            .bar_vertical_gradient(rgb(0xfda4af).into(), rgb(0xe11d48).into())
                            .into_any_element(),
                    ]),
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

pub fn custom_series() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new(
            "Online",
            [
                ChartPoint::new("Jan", 42.0),
                ChartPoint::new("Feb", 58.0),
                ChartPoint::new("Mar", 64.0),
                ChartPoint::new("Apr", 72.0),
            ],
        )
        .fill_color(blue()),
        ChartSeries::new(
            "Retail",
            [
                ChartPoint::new("Jan", 28.0),
                ChartPoint::new("Feb", 34.0),
                ChartPoint::new("Mar", 39.0),
                ChartPoint::new("Apr", 45.0),
            ],
        )
        .fill_color(green()),
    ]
}

pub fn standalone_series() -> Vec<ChartSeries> {
    vec![ChartSeries::new(
        "Active",
        [
            ChartPoint::new("Mon", 18.0),
            ChartPoint::new("Tue", 42.0),
            ChartPoint::new("Wed", 33.0),
            ChartPoint::new("Thu", 76.0),
            ChartPoint::new("Fri", 61.0),
            ChartPoint::new("Sat", 88.0),
            ChartPoint::new("Sun", 54.0),
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
        assert!(source.contains("bar_gap_ratio"));
        assert!(source.contains("value_label_content"));
        assert!(source.contains("standalone()"));
        assert!(source.contains("value_color_ranges"));
        assert!(source.contains("bar_vertical_gradient"));
        assert!(source.contains("BarChartFill::vertical_gradient"));
        assert!(source.contains("value_fill_ranges"));
        assert!(source.contains("row_md"));
        assert!(source.contains("bar-chart-demo-standalone-gradient"));
    }
}
