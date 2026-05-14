use aura_components::{ChartPoint, ChartSeries, LineChart};
use gpui::{IntoElement, px};

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
    )])
    .height(px(360.0))
    .smooth(true)
    .area_fill(true)
}
