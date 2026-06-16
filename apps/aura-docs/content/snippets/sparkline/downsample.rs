use aura_components::Sparkline;
use gpui::{IntoElement, px, rgb};

pub fn sparkline_downsample() -> impl IntoElement {
    // Sparkline 也可限制绘制点数，适合表格或列表里批量展示长趋势。
    let dense = (0..1_200).map(|index| {
        let wave = ((index as f64) / 18.0).sin() * 8.0;
        let spike = if index % 180 == 0 { 16.0 } else { 0.0 };
        40.0 + wave + spike
    });

    Sparkline::new(dense)
        .width(px(280.0))
        .height(px(72.0))
        .color(rgb(0x7c3aed).into())
        .area_fill(true)
        .max_render_points(96)
}
