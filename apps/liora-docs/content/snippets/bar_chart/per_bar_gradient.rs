use gpui::{IntoElement, px, rgb};
use liora_components::{BarChart, BarChartFill, ChartPoint, ChartSeries};

pub fn bar_chart_per_bar_gradient() -> impl IntoElement {
    // 每根柱可独立指定渐变，适合做重点值、状态值或品牌化展示。
    BarChart::new([ChartSeries::new(
        "Revenue",
        [
            ChartPoint::new("Q1", 42.0),
            ChartPoint::new("Q2", 58.0),
            ChartPoint::new("Q3", 73.0),
            ChartPoint::new("Q4", 96.0),
        ],
    )])
    .height(px(300.0))
    .bar_radius(px(8.0))
    .bar_fills([
        BarChartFill::vertical_gradient(rgb(0xdbeafe).into(), rgb(0x2563eb).into()),
        BarChartFill::vertical_gradient(rgb(0xdcfce7).into(), rgb(0x16a34a).into()),
        BarChartFill::vertical_gradient(rgb(0xffedd5).into(), rgb(0xea580c).into()),
        BarChartFill::vertical_gradient(rgb(0xfce7f3).into(), rgb(0xdb2777).into()),
    ])
}
