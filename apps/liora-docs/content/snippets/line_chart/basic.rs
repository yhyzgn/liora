use gpui::{IntoElement, blue, px};
use liora_components::{ChartPoint, ChartSeries, ChartValueLabelContent, LineChart};

pub fn line_chart_basic() -> impl IntoElement {
    LineChart::new([ChartSeries::new(
        "CPU Usage",
        [
            ChartPoint::new("10:00", 24.0),
            ChartPoint::new("10:05", 36.0),
            ChartPoint::new("10:10", 32.0),
            ChartPoint::new("10:15", 52.0),
            ChartPoint::new("10:20", 46.0),
            ChartPoint::new("10:25", 64.0),
        ],
    )
    .stroke_color(blue())
    .fill_color(blue().opacity(0.28))
    .stroke_width(px(3.0))
    .smooth(true)])
    .height(px(360.0))
    .smooth(true)
    .area_fill(true)
    // 默认开启 hover tooltip；可通过 tooltip_hit_radius 调整命中距离。
    .tooltip_hit_radius(px(16.0))
    .value_label_content(ChartValueLabelContent::ValueAndPercentage)
    .percentage_decimals(1)
}
