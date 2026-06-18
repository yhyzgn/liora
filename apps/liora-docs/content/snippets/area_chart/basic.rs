use gpui::{IntoElement, blue, px};
use liora_components::{AreaChart, ChartPoint, ChartSeries, ChartValueLabelContent};

pub fn area_chart_basic() -> impl IntoElement {
    AreaChart::new([ChartSeries::new(
        "Visitors",
        [
            ChartPoint::new("Mon", 24.0),
            ChartPoint::new("Tue", 32.0),
            ChartPoint::new("Wed", 45.0),
            ChartPoint::new("Thu", 52.0),
            ChartPoint::new("Fri", 61.0),
            ChartPoint::new("Sat", 72.0),
            ChartPoint::new("Sun", 68.0),
        ],
    )
    .stroke_color(blue())
    .fill_color(blue().opacity(0.35))
    .stroke_width(px(2.6))
    .smooth(true)])
    .height(px(260.0))
    .smooth(true)
    .value_label_content(ChartValueLabelContent::Percentage)
    .percentage_decimals(1)
}
