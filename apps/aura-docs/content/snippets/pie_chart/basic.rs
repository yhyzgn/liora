use aura_components::{ChartPoint, ChartSeries, PieChart};
use gpui::{IntoElement, px};

pub fn pie_chart_basic() -> impl IntoElement {
    PieChart::new([
        ChartSeries::new("A", [ChartPoint::new("A", 30.0)]),
        ChartSeries::new("B", [ChartPoint::new("B", 20.0)]),
        ChartSeries::new("C", [ChartPoint::new("C", 50.0)]),
    ])
    .height(px(340.0))
}
