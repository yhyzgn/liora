use gpui::{IntoElement, px, rgb};
use liora_components::Sparkline;

pub fn sparkline_basic() -> impl IntoElement {
    Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
        .width(px(220.0))
        .height(px(64.0))
        .color(rgb(0x2563eb).into())
        .stroke_width(px(2.4))
}
