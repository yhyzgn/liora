use gpui::{IntoElement, px, rgb};
use liora_components::{ChartLineStyle, Sparkline};

pub fn sparkline_styles() -> impl IntoElement {
    Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
        .width(px(220.0))
        .height(px(56.0))
        .color(rgb(0x2563eb).into())
        .line_style(ChartLineStyle::Dashed)
        .smooth(false)
        .show_last_point(false)
}
