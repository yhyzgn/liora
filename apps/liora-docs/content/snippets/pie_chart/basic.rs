use gpui::{IntoElement, px};
use liora_components::{ChartPoint, ChartSeries, ChartValueLabelPlacement, PieChart};

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
    .value_label_placement(ChartValueLabelPlacement::OutsideAligned)
    // 默认开启扇区 hover tooltip；命中半径用于更容易触达边界。
    .tooltip_hit_radius(px(10.0))
}
