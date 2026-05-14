use aura_components::{ChartPoint, ChartSeries, PieChart};
use gpui::{IntoElement, px};

pub fn pie_chart_basic() -> impl IntoElement {
    PieChart::new([
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]),
    ])
    .height(px(340.0))
    .percentage_decimals(1)
    .outside_label_threshold_degrees(30)
}
