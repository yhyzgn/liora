use gpui::{IntoElement, blue, green, px, red, yellow};
use liora_components::{
    ChartPoint, ChartSeries, ChartValueLabelContent, RingChart, RingExternalLegendOptions,
};

pub fn ring_chart_external() -> impl IntoElement {
    RingChart::new([
        ChartSeries::new("Desktop", [ChartPoint::new("Desktop", 62.0)]).fill_color(blue()),
        ChartSeries::new("Mobile", [ChartPoint::new("Mobile", 24.0)]).fill_color(green()),
        ChartSeries::new("Tablet", [ChartPoint::new("Tablet", 9.0)]).fill_color(yellow()),
        ChartSeries::new("Other", [ChartPoint::new("Other", 5.0)]).fill_color(red()),
    ])
    .height(px(340.0))
    .inner_ratio(0.6)
    .external_legend(
        RingExternalLegendOptions::new()
            .vertical()
            .right()
            .max_items(3)
            .content(ChartValueLabelContent::ValueOverTotalAndPercentage)
            .percentage_decimals(1),
    )
    .show_tooltip(false)
}
