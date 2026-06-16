use aura_components::{BarChart, BarChartFill, BarChartValueFillRange, ChartPoint, ChartSeries};
use gpui::{IntoElement, px, rgb};

pub fn bar_chart_gradient() -> impl IntoElement {
    // 普通柱状图：支持统一渐变，也支持按数值区间替换为不同渐变。
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
    .bar_radius(px(6.0))
    .bar_vertical_gradient(rgb(0x60a5fa).into(), rgb(0x2563eb).into())
    .value_fill_ranges([
        BarChartValueFillRange::new(
            0.0,
            60.0,
            BarChartFill::vertical_gradient(rgb(0xbfdbfe).into(), rgb(0x3b82f6).into()),
        ),
        BarChartValueFillRange::new(
            60.0,
            100.0,
            BarChartFill::vertical_gradient(rgb(0xfef08a).into(), rgb(0xf97316).into()),
        ),
    ])
}
