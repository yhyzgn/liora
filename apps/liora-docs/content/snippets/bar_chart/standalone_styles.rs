use gpui::{IntoElement, px, rgb};
use liora_components::{
    BarChart, BarChartFill, BarChartValueColorRange, ChartPoint, ChartSeries, Space,
};

fn mini_series() -> Vec<ChartSeries> {
    vec![ChartSeries::new(
        "Active",
        [
            ChartPoint::new("Mon", 18.0),
            ChartPoint::new("Tue", 42.0),
            ChartPoint::new("Wed", 33.0),
            ChartPoint::new("Thu", 76.0),
            ChartPoint::new("Fri", 61.0),
            ChartPoint::new("Sat", 88.0),
            ChartPoint::new("Sun", 54.0),
        ],
    )]
}

pub fn bar_chart_standalone_styles() -> impl IntoElement {
    // 固定柱宽/间距后，standalone 会按内容宽度紧凑展示，而不是强行撑满整行。
    Space::new()
        .wrap()
        .gap_lg()
        .child(
            BarChart::new(mini_series())
                .standalone()
                .bar_width(px(8.0))
                .bar_gap(px(4.0))
                .bar_radius(px(5.0))
                .value_color_ranges([
                    BarChartValueColorRange::new(0.0, 35.0, rgb(0x86efac).into()),
                    BarChartValueColorRange::new(35.0, 70.0, rgb(0x22c55e).into()),
                    BarChartValueColorRange::new(70.0, 100.0, rgb(0x16a34a).into()),
                ]),
        )
        .child(
            BarChart::new(mini_series())
                .standalone()
                .bar_width(px(10.0))
                .bar_gap(px(5.0))
                .bar_radius(px(8.0))
                .bar_fills([
                    BarChartFill::vertical_gradient(rgb(0xc4b5fd).into(), rgb(0x7c3aed).into()),
                    BarChartFill::vertical_gradient(rgb(0xbae6fd).into(), rgb(0x0284c7).into()),
                    BarChartFill::vertical_gradient(rgb(0xfde68a).into(), rgb(0xd97706).into()),
                ]),
        )
        .child(
            BarChart::new(mini_series())
                .standalone()
                .height(px(96.0))
                .bar_width(px(14.0))
                .bar_gap(px(8.0))
                .bar_radius(px(3.0))
                .bar_vertical_gradient(rgb(0xfda4af).into(), rgb(0xe11d48).into()),
        )
}
