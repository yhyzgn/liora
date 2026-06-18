use gpui::{IntoElement, px};
use liora_components::{AreaChart, ChartPoint, ChartSeries};

pub fn area_chart_stacked() -> impl IntoElement {
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
        ),
        ChartSeries::new(
            "Mobile",
            [
                ChartPoint::new("Mon", 18.0),
                ChartPoint::new("Tue", 25.0),
                ChartPoint::new("Wed", 32.0),
                ChartPoint::new("Thu", 39.0),
                ChartPoint::new("Fri", 48.0),
            ],
        ),
    ])
    .height(px(300.0))
    .stacked()
}
