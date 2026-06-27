use gpui::{IntoElement, green, px, red};
use liora_components::{CandlestickChart, CandlestickPoint, ChartValueLabelContent};

pub fn candlestick_chart_dense() -> impl IntoElement {
    let points = (0..48).map(|index| {
        let base = 110.0 + (index as f64 * 0.36) + ((index % 7) as f64 - 3.0) * 1.6;
        let open = base + ((index % 5) as f64 - 2.0) * 0.9;
        let close = base + (((index + 2) % 5) as f64 - 2.0) * 1.2;
        let high = open.max(close) + 3.0 + (index % 4) as f64;
        let low = open.min(close) - 2.6 - (index % 3) as f64;
        CandlestickPoint::new(format!("D{}", index + 1), open, high, low, close)
    });

    CandlestickChart::new(points)
        .height(px(340.0))
        .up_color(green())
        .down_color(red())
        .max_render_points(28)
        .max_axis_labels(8)
        .max_value_labels(8)
        .show_value_labels(true)
        .value_label_content(ChartValueLabelContent::Value)
}
