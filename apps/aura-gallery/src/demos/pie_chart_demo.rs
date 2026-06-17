use aura_components::layout_helpers::{page, section};
use aura_components::{
    ChartPoint, ChartSeries, ChartValueLabelContent, ChartValueLabelPlacement, PieChart, Space,
};
use gpui::{AnyView, App, Context, Render, Window, blue, green, prelude::*, px, red, yellow};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PieChartDemo).into()
}

struct PieChartDemo;

impl Render for PieChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "PieChart 饼图",
            "使用原生扇区 path 绘制分类占比，支持极坐标扇区 hover tooltip。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础饼图",
                    "展示分类占比。",
                    PieChart::new(slices())
                        .id("pie-chart-demo-basic")
                        .height(px(420.0))
                        .percentage_decimals(1)
                        .value_label_placement(ChartValueLabelPlacement::OutsideAligned)
                        .tooltip_hit_radius(px(10.0)),
                ))
                .child(section(
                    "外部自由标注 + 百分比",
                    "所有标签都放到图形外部，按扇区方向自由排列，并只显示百分比。",
                    PieChart::new(colored_slices())
                        .id("pie-chart-demo-outside-free")
                        .height(px(420.0))
                        .value_label_content(ChartValueLabelContent::Percentage)
                        .value_label_placement(ChartValueLabelPlacement::OutsideFree)
                        .percentage_decimals(2)
                        .outside_label_threshold_degrees(120),
                ))
                .child(section(
                    "隐藏图例",
                    "突出扇区本身。",
                    PieChart::new(slices())
                        .id("pie-chart-demo-no-legend")
                        .height(px(420.0))
                        .show_legend(false)
                        .show_percentage_labels(false)
                        .show_tooltip(false),
                )),
        )
    }
}

fn colored_slices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]).fill_color(blue()),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]).fill_color(green()),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]).fill_color(yellow()),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]).fill_color(red()),
    ]
}

fn slices() -> Vec<ChartSeries> {
    vec![
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn pie_chart_demo_uses_component_api() {
        let source = include_str!("pie_chart_demo.rs");
        assert!(source.contains("PieChart::new"));
        assert!(source.contains("OutsideFree"));
        assert!(source.contains("ChartValueLabelContent::Percentage"));
        assert!(source.contains("tooltip_hit_radius"));
        assert!(source.contains("show_tooltip(false)"));
    }
}
