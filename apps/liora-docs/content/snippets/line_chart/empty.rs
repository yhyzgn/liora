use gpui::{IntoElement, px};
use liora_components::{ChartSeries, LineChart};

pub fn line_chart_empty() -> impl IntoElement {
    LineChart::new(Vec::<ChartSeries>::new()).height(px(220.0))
}
