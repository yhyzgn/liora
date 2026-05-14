use aura_components::{ChartSeries, LineChart};
use gpui::{IntoElement, px};

pub fn line_chart_empty() -> impl IntoElement {
    LineChart::new(Vec::<ChartSeries>::new()).height(px(220.0))
}
