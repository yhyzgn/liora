use gpui::{IntoElement, px, rgb};
use liora_components::SignalMeter;

pub fn signal_meter_levels() -> impl IntoElement {
    // total_signals 指定总格数，level_colors 按每个等级分别指定颜色。
    SignalMeter::new(5)
        .total_signals(6)
        .level_colors([
            rgb(0xef4444).into(),
            rgb(0xf97316).into(),
            rgb(0xf59e0b).into(),
            rgb(0x84cc16).into(),
            rgb(0x22c55e).into(),
            rgb(0x16a34a).into(),
        ])
        .height(px(44.0))
        .bar_width(px(7.0))
        .gap(px(5.0))
}
