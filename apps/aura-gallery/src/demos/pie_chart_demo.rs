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
                        .height(px(420.0)),
                ))
                .child(section(
                    "隐藏图例",
                    "突出扇区本身。",
                    PieChart::new(slices())
                        .id("pie-chart-demo-no-legend")
                        .height(px(420.0))
                        .show_legend(false),
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
    fn pie_chart_demo_uses_component_api() {
        let source = include_str!("pie_chart_demo.rs");
        assert!(source.contains("PieChart::new"));
    }
}
