use gpui::{IntoElement, px, rgb};
use liora_components::{BarChart, BarChartValueColorRange, ChartPoint, ChartSeries};

pub fn bar_chart_standalone() -> impl IntoElement {
    // 迷你柱状图：隐藏坐标轴/网格/图例，适合嵌入卡片指标。
    BarChart::new([ChartSeries::new(
        "Active",
        [
            ChartPoint::new("Mon", 18.0),
            ChartPoint::new("Tue", 42.0),
            ChartPoint::new("Wed", 33.0),
            ChartPoint::new("Thu", 76.0),
            ChartPoint::new("Fri", 61.0),
        ],
    )])
    .standalone()
    .bar_width(px(8.0))
    .bar_gap(px(7.0))
    .bar_radius(px(5.0))
    .value_color_ranges([
        BarChartValueColorRange::new(0.0, 35.0, rgb(0x86efac).into()),
        BarChartValueColorRange::new(35.0, 70.0, rgb(0x22c55e).into()),
        BarChartValueColorRange::new(70.0, 100.0, rgb(0x16a34a).into()),
    ])
}
