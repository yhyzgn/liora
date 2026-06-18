use gpui::{IntoElement, blue, green, px};
use liora_components::{ChartPoint, ChartSeries, ChartValueLabelContent, LineChart};

pub fn line_chart_custom() -> impl IntoElement {
    LineChart::new([
        ChartSeries::new(
            "CPU",
            [
                ChartPoint::new("Mon", 25.0),
                ChartPoint::new("Tue", 38.0),
                ChartPoint::new("Wed", 42.0),
                ChartPoint::new("Thu", 58.0),
                ChartPoint::new("Fri", 49.0),
            ],
        )
        .stroke_color(blue())
        .fill_color(blue().opacity(0.22))
        .stroke_width(px(3.2))
        .smooth(true),
        ChartSeries::new(
            "Memory",
            [
                ChartPoint::new("Mon", 48.0),
                ChartPoint::new("Tue", 52.0),
                ChartPoint::new("Wed", 57.0),
                ChartPoint::new("Thu", 63.0),
                ChartPoint::new("Fri", 66.0),
            ],
        )
        .stroke_color(green())
        .fill_color(green().opacity(0.18))
        .stroke_width(px(2.4))
        .smooth(false),
    ])
    .height(px(380.0))
    .y_domain(0.0, 100.0)
    .area_fill(true)
    .value_label_content(ChartValueLabelContent::Percentage)
    .percentage_decimals(1)
}
