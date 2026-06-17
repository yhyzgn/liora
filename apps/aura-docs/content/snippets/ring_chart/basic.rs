use aura_components::{ChartPoint, ChartSeries, ChartValueLabelPlacement, RingChart};
use gpui::{IntoElement, px};

pub fn ring_chart_basic() -> impl IntoElement {
    RingChart::new([
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]),
    ])
    .height(px(340.0))
    .percentage_decimals(1)
    .outside_label_threshold_degrees(30)
    .value_label_placement(ChartValueLabelPlacement::OutsideAligned)
    // RingChart 的 tooltip 只命中圆环分段，不命中中间空洞。
    .tooltip_hit_radius(px(10.0))
}
