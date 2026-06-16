use aura_components::{AreaChart, ChartPoint, ChartSeries};
use gpui::{IntoElement, px};

pub fn area_chart_downsample() -> impl IntoElement {
    // 面积图同样支持 max_render_points，堆叠模式会按总量形状保留尖峰。
    let desktop = (0..1_800).map(|index| {
        let wave = ((index as f64) / 32.0).sin() * 14.0;
        ChartPoint::new(format!("T{index}"), 42.0 + wave)
    });
    let mobile = (0..1_800).map(|index| {
        let wave = ((index as f64) / 27.0).cos() * 10.0;
        let spike = if index % 360 == 0 { 24.0 } else { 0.0 };
        ChartPoint::new(format!("T{index}"), 28.0 + wave + spike)
    });

    AreaChart::new([
        ChartSeries::new("Desktop", desktop),
        ChartSeries::new("Mobile", mobile),
    ])
    .height(px(320.0))
    .stacked()
    .max_render_points(160)
    .max_axis_labels(8)
    .max_value_labels(12)
}
