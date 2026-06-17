use aura_components::{BarChart, ChartPoint, ChartSeries, ChartValueLabelContent};
use gpui::{IntoElement, green, px};

pub fn bar_chart_basic() -> impl IntoElement {
    BarChart::new([ChartSeries::new(
        "Revenue",
        [
            ChartPoint::new("Q1", 42.0),
            ChartPoint::new("Q2", 58.0),
            ChartPoint::new("Q3", 73.0),
            ChartPoint::new("Q4", 96.0),
        ],
    )
    .fill_color(green())])
    .height(px(260.0))
    // 默认开启柱体 hover tooltip；可调小命中距离让 tooltip 只在靠近柱体时出现。
    .tooltip_hit_radius(px(10.0))
    .value_label_content(ChartValueLabelContent::ValueAndPercentage)
    .percentage_decimals(1)
    .bar_gap_ratio(0.28)
}
