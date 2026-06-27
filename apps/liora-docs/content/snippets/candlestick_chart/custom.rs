use gpui::{IntoElement, px, rgb};
use liora_components::{CandlestickChart, CandlestickPoint};

pub fn candlestick_chart_custom() -> impl IntoElement {
    CandlestickChart::new([
        CandlestickPoint::new("09:30", 88.0, 94.0, 85.0, 92.0),
        CandlestickPoint::new("10:00", 92.0, 96.0, 89.0, 90.0),
        CandlestickPoint::new("10:30", 90.0, 101.0, 88.0, 99.0),
        CandlestickPoint::new("11:00", 99.0, 103.0, 95.0, 97.0),
        CandlestickPoint::new("11:30", 97.0, 108.0, 96.0, 106.0),
    ])
    .height(px(300.0))
    .up_color(rgb(0x14b8a6).into())
    .down_color(rgb(0xf43f5e).into())
    .body_width(px(12.0))
    .wick_width(px(2.0))
    .max_axis_labels(5)
}
