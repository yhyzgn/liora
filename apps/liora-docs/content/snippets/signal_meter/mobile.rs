use gpui::{IntoElement, px};
use liora_components::SignalMeter;

pub fn signal_meter_mobile() -> impl IntoElement {
    // 移动信号阶梯条，level 会按 max_level 自动截断。
    SignalMeter::new(3).max_level(4).height(px(36.0))
}
