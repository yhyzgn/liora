use gpui::{IntoElement, blue, green, px};
use liora_components::{BarChart, ChartPoint, ChartSeries, ChartValueLabelContent};

pub fn bar_chart_custom() -> impl IntoElement {
    BarChart::new([
        ChartSeries::new(
            "Online",
            [
                ChartPoint::new("Jan", 42.0),
                ChartPoint::new("Feb", 58.0),
                ChartPoint::new("Mar", 64.0),
                ChartPoint::new("Apr", 72.0),
            ],
        )
        .fill_color(blue()),
        ChartSeries::new(
            "Retail",
            [
                ChartPoint::new("Jan", 28.0),
                ChartPoint::new("Feb", 34.0),
                ChartPoint::new("Mar", 39.0),
                ChartPoint::new("Apr", 45.0),
            ],
        )
        .fill_color(green()),
    ])
    .height(px(340.0))
    .y_domain(0.0, 120.0)
    .bar_gap_ratio(0.32)
    .value_label_content(ChartValueLabelContent::ValueAndPercentage)
    .percentage_decimals(1)
}
