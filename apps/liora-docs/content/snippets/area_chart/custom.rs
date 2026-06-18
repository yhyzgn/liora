use gpui::{IntoElement, blue, green, px};
use liora_components::{AreaChart, ChartPoint, ChartSeries, ChartValueLabelContent};

pub fn area_chart_custom() -> impl IntoElement {
    AreaChart::new([
        ChartSeries::new(
            "Desktop",
            [
                ChartPoint::new("Mon", 28.0),
                ChartPoint::new("Tue", 34.0),
                ChartPoint::new("Wed", 38.0),
                ChartPoint::new("Thu", 44.0),
                ChartPoint::new("Fri", 50.0),
            ],
        )
        .stroke_color(blue())
        .fill_color(blue().opacity(0.36))
        .stroke_width(px(3.0))
        .smooth(true),
        ChartSeries::new(
            "Mobile",
            [
                ChartPoint::new("Mon", 18.0),
                ChartPoint::new("Tue", 25.0),
                ChartPoint::new("Wed", 32.0),
                ChartPoint::new("Thu", 39.0),
                ChartPoint::new("Fri", 48.0),
            ],
        )
        .stroke_color(green())
        .fill_color(green().opacity(0.24))
        .stroke_width(px(2.2))
        .smooth(false),
    ])
    .height(px(340.0))
    .y_domain(0.0, 100.0)
    .smooth(true)
    .value_label_content(ChartValueLabelContent::ValueAndPercentage)
    .percentage_decimals(1)
}
