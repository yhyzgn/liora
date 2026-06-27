use gpui::{IntoElement, px};
use liora_components::{CandlestickChart, CandlestickPoint};

pub fn candlestick_chart_basic() -> impl IntoElement {
    CandlestickChart::new([
        CandlestickPoint::new("Mon", 102.0, 112.0, 98.0, 109.0).volume(12_400.0),
        CandlestickPoint::new("Tue", 109.0, 115.0, 104.0, 106.0).volume(15_800.0),
        CandlestickPoint::new("Wed", 106.0, 121.0, 105.0, 118.0).volume(18_600.0),
        CandlestickPoint::new("Thu", 118.0, 124.0, 111.0, 114.0).volume(16_100.0),
        CandlestickPoint::new("Fri", 114.0, 128.0, 113.0, 126.0).volume(21_900.0),
    ])
    .height(px(320.0))
    .show_legend(true)
    .tooltip_hit_radius(px(12.0))
}
