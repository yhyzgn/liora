use gpui::{IntoElement, px, rgb};
use liora_components::{SignalLevelColor, SignalMeter};

pub fn signal_meter_threshold_colors() -> impl IntoElement {
    // 到达某个等级后，所有激活格统一切换成该等级对应颜色。
    SignalMeter::new(4)
        .total_signals(5)
        .threshold_colors([
            SignalLevelColor::new(2, rgb(0xef4444).into()),
            SignalLevelColor::new(3, rgb(0xeab308).into()),
            SignalLevelColor::new(4, rgb(0xf97316).into()),
            SignalLevelColor::new(5, rgb(0x22c55e).into()),
        ])
        .inactive_color(rgb(0xf1f5f9).into())
        .height(px(42.0))
        .bar_width(px(7.0))
        .gap(px(5.0))
}
