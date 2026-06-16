use aura_components::layout_helpers::{page, section};
use aura_components::{HeatBar, HeatBarColorRange, HeatBarItem, HeatBarLegend, Space};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, rgb};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| HeatBarDemo).into()
}
struct HeatBarDemo;
impl Render for HeatBarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "HeatBar 热力条",
            "用于按时间轴展示高密度事件、错误、告警等热力分布。",
            Space::new().vertical().gap_xl().child(section(
                "事件热力",
                "顶部可展示分类统计，主体用密集竖条表示不同时间点的强度。",
                HeatBar::new(sample_items())
                    .legends([
                        HeatBarLegend::new("错误", 3, rgb(0xef4444).into()),
                        HeatBarLegend::new("警告", 24, rgb(0xf59e0b).into()),
                    ])
                    .color_ranges([
                        HeatBarColorRange::new(0.0, 3.0, rgb(0x93c5fd).into()),
                        HeatBarColorRange::new(3.0, 7.0, rgb(0xf59e0b).into()),
                        HeatBarColorRange::above(7.0, rgb(0xef4444).into()),
                    ])
                    .max_value(10.0)
                    .x_labels(["00:00", "06:00", "12:00", "18:00", "24:00"]),
            )),
        )
    }
}
fn sample_items() -> Vec<HeatBarItem> {
    (0..72)
        .map(|i| {
            let value = ((i * 7 + 3) % 11) as f64;
            // Item color remains a fallback; color_ranges drives final severity mapping.
            HeatBarItem::new(format!("t{i}"), value, rgb(0x93c5fd).into())
        })
        .collect()
}
