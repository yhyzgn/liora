use aura_components::Sparkline;
use gpui::{Hsla, IntoElement, px, rgb};

pub fn sparkline_area() -> impl IntoElement {
    Sparkline::new([-4.0, -1.0, 3.0, 7.0, 5.0, -2.0, 4.0, 10.0, 8.0])
        .width(px(280.0))
        .height(px(96.0))
        .area_fill(true)
        .show_baseline(true)
        .trend_colors(rgb(0x14b8a6).into(), rgb(0xf43f5e).into())
        .fill_color(Hsla::from(rgb(0x14b8a6)).opacity(0.18))
        .y_domain(-8.0, 12.0)
}
