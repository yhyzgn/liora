use aura_components::{ChartLineStyle, ChartPoint, ChartSeries, LineChart};
use gpui::{IntoElement, blue, green, px, red};

pub fn line_chart_line_styles() -> impl IntoElement {
    // 每个序列都可以独立指定线条颜色、粗细、平滑和虚/实/点线样式。
    LineChart::new([
        ChartSeries::new(
            "Solid Smooth",
            [
                ChartPoint::new("Mon", 32.0),
                ChartPoint::new("Tue", 44.0),
                ChartPoint::new("Wed", 38.0),
                ChartPoint::new("Thu", 70.0),
            ],
        )
        .stroke_color(blue())
        .stroke_width(px(3.2))
        .line_style(ChartLineStyle::Solid)
        .smooth(true),
        ChartSeries::new(
            "Dashed",
            [
                ChartPoint::new("Mon", 22.0),
                ChartPoint::new("Tue", 35.0),
                ChartPoint::new("Wed", 52.0),
                ChartPoint::new("Thu", 58.0),
            ],
        )
        .stroke_color(green())
        .stroke_width(px(2.6))
        .dashed()
        .smooth(false),
        ChartSeries::new(
            "Dotted",
            [
                ChartPoint::new("Mon", 60.0),
                ChartPoint::new("Tue", 54.0),
                ChartPoint::new("Wed", 49.0),
                ChartPoint::new("Thu", 45.0),
            ],
        )
        .stroke_color(red())
        .stroke_width(px(2.8))
        .dotted()
        .smooth(true),
    ])
    .height(px(320.0))
    .y_domain(0.0, 100.0)
    .area_fill(false)
    .point_markers(false)
    // 纯展示型小图也可以关闭 hover tooltip。
    .show_tooltip(false)
}
