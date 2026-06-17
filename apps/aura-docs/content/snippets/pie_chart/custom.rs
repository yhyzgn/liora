use aura_components::{
    ChartPoint, ChartSeries, ChartValueLabelContent, ChartValueLabelPlacement, PieChart,
};
use gpui::{IntoElement, blue, green, px, red, yellow};

pub fn pie_chart_custom() -> impl IntoElement {
    PieChart::new([
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]).fill_color(blue()),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]).fill_color(green()),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]).fill_color(yellow()),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]).fill_color(red()),
    ])
    .height(px(360.0))
    .value_label_content(ChartValueLabelContent::Percentage)
    .value_label_placement(ChartValueLabelPlacement::OutsideFree)
    .percentage_decimals(2)
    .outside_label_threshold_degrees(120)
    .show_tooltip(false)
}
