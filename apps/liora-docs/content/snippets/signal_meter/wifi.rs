use gpui::{IntoElement, px, rgb};
use liora_components::SignalMeter;

pub fn signal_meter_wifi() -> impl IntoElement {
    // Wi-Fi 风格可以独立配置激活/未激活颜色和柱宽。
    SignalMeter::new(2)
        .wifi()
        .active_color(rgb(0x3b82f6).into())
        .inactive_color(rgb(0xdbeafe).into())
        .bar_width(px(8.0))
        .gap(px(5.0))
        .height(px(44.0))
}
