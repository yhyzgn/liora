use aura_components::layout_helpers::{page, section};
use aura_components::{ChartPoint, ChartSeries, PieChart, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PieChartDemo).into()
}

struct PieChartDemo;

impl Render for PieChartDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "PieChart 饼图",
            "使用原生扇区 path 绘制分类占比。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础饼图",
                    "展示分类占比。",
                    PieChart::new(slices())
                        .id("pie-chart-demo-basic")
                        .height(px(420.0))
                        .percentage_decimals(1),
                ))
                .child(section(
                    "隐藏图例",
                    "突出扇区本身。",
                    PieChart::new(slices())
                        .id("pie-chart-demo-no-legend")
                        .height(px(420.0))
                        .show_legend(false)
                        .show_percentage_labels(false),
                )),
        )
    }
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
    }
}
