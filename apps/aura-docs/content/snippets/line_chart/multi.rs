use aura_components::{ChartPoint, ChartSeries, LineChart};
use gpui::{IntoElement, px};

pub fn line_chart_multi() -> impl IntoElement {
    LineChart::new([
        ChartSeries::new(
            "CPU",
            [
                ChartPoint::new("Mon", 25.0),
                ChartPoint::new("Tue", 38.0),
                ChartPoint::new("Wed", 42.0),
                ChartPoint::new("Thu", 58.0),
                ChartPoint::new("Fri", 49.0),
                ChartPoint::new("Sat", 72.0),
                ChartPoint::new("Sun", 61.0),
            ],
        ),
        ChartSeries::new(
            "Memory",
            [
                ChartPoint::new("Mon", 48.0),
                ChartPoint::new("Tue", 52.0),
                ChartPoint::new("Wed", 57.0),
                ChartPoint::new("Thu", 63.0),
                ChartPoint::new("Fri", 66.0),
                ChartPoint::new("Sat", 69.0),
                ChartPoint::new("Sun", 74.0),
            ],
        ),
    ])
    .height(px(300.0))
    .y_domain(0.0, 100.0)
}
