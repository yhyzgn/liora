use gpui::{IntoElement, px};
use liora_components::{ChartPoint, ChartSeries, LineChart};

pub fn line_chart_downsample() -> impl IntoElement {
    // 大数据量趋势图默认会做 min/max bucket 降采样。
    // max_render_points 控制最终参与绘制的点数上限，仍保留首尾和局部峰谷。
    let dense = (0..2_000).map(|index| {
        let wave = ((index as f64) / 24.0).sin() * 18.0;
        let spike = if index % 240 == 0 { 32.0 } else { 0.0 };
        ChartPoint::new(format!("T{index}"), 48.0 + wave + spike)
    });

    LineChart::new([ChartSeries::new("Latency", dense)])
        .height(px(320.0))
        .y_domain(0.0, 100.0)
        .point_markers(false)
        .area_fill(true)
        .max_render_points(180)
}
