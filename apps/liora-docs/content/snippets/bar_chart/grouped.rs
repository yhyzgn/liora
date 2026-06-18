use gpui::{IntoElement, px};
use liora_components::{BarChart, ChartPoint, ChartSeries};

pub fn bar_chart_grouped() -> impl IntoElement {
    BarChart::new([
        ChartSeries::new(
            "Online",
            [
                ChartPoint::new("Jan", 42.0),
                ChartPoint::new("Feb", 58.0),
                ChartPoint::new("Mar", 64.0),
                ChartPoint::new("Apr", 72.0),
            ],
        ),
        ChartSeries::new(
            "Retail",
            [
                ChartPoint::new("Jan", 28.0),
                ChartPoint::new("Feb", 34.0),
                ChartPoint::new("Mar", 39.0),
                ChartPoint::new("Apr", 45.0),
            ],
        ),
    ])
    .height(px(300.0))
    .y_domain(0.0, 120.0)
    .tooltip_hit_radius(px(12.0))
}
