use aura_components::{HeatBar, HeatBarColorRange, HeatBarItem, HeatBarLegend};
use gpui::{IntoElement, rgb};

pub fn heat_bar_events() -> impl IntoElement {
    // 时间轴密集竖条：value 负责高度，color_ranges 负责严重等级映射。
    let items = (0..48).map(|index| {
        let value = ((index * 7 + 3) % 11) as f64;
        HeatBarItem::new(format!("t{index}"), value, rgb(0x93c5fd).into())
    });

    HeatBar::new(items)
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
        .x_labels(["00:00", "06:00", "12:00", "18:00", "24:00"])
}
