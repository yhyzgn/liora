use gpui::{IntoElement, blue, green, px, red, yellow};
use liora_components::{
    ChartPoint, ChartSeries, ChartValueLabelContent, ChartValueLabelPlacement, RingChart,
};

pub fn ring_chart_custom() -> impl IntoElement {
    RingChart::new([
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]).fill_color(blue()),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]).fill_color(green()),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]).fill_color(yellow()),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]).fill_color(red()),
    ])
    .height(px(360.0))
    .inner_ratio(0.48)
    .value_label_content(ChartValueLabelContent::ValueOverTotalAndPercentage)
    .value_label_placement(ChartValueLabelPlacement::OutsideAligned)
    .percentage_decimals(1)
    .outside_label_threshold_degrees(120)
}
