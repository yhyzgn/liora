use aura_components::layout_helpers::{page, row_md, section};
use aura_components::{Card, ChartLineStyle, ChartPoint, Space, Sparkline, Text};
use gpui::{AnyView, App, Context, Render, Window, prelude::*, px, rgb};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SparklineDemo).into()
}

struct SparklineDemo;

impl Render for SparklineDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Sparkline 迷你趋势图",
            "紧凑的原生 GPUI 趋势图，适合卡片指标、表格单元格和 Dashboard 列表内嵌展示。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "指标卡片",
                    "默认根据趋势方向自动选择正/负颜色，并显示最后一个点。",
                    row_md(vec![
                        metric_card(
                            "Revenue",
                            "$42.8k",
                            Sparkline::new(up_values()).width(px(180.0)),
                        )
                        .into_any_element(),
                        metric_card(
                            "Churn",
                            "2.4%",
                            Sparkline::new(down_values()).width(px(180.0)),
                        )
                        .into_any_element(),
                    ]),
                ))
                .child(section(
                    "区域填充与基线",
                    "支持区域填充、0 基线、自定义正负趋势颜色和固定 y 轴范围。",
                    row_md(vec![
                        Sparkline::new(mixed_values())
                            .id("sparkline-demo-area")
                            .width(px(280.0))
                            .height(px(96.0))
                            .area_fill(true)
                            .show_baseline(true)
                            .trend_colors(rgb(0x14b8a6).into(), rgb(0xf43f5e).into())
                            .fill_color(gpui::Hsla::from(rgb(0x14b8a6)).opacity(0.18))
                            .y_domain(-8.0, 12.0)
                            .into_any_element(),
                        Sparkline::from_points([
                            ChartPoint::new("Mon", 4.0),
                            ChartPoint::new("Tue", 6.0),
                            ChartPoint::new("Wed", 5.0),
                            ChartPoint::new("Thu", 9.0),
                            ChartPoint::new("Fri", 8.0),
                        ])
                        .id("sparkline-demo-points")
                        .width(px(280.0))
                        .height(px(96.0))
                        .color(rgb(0x6366f1).into())
                        .fill_color(gpui::Hsla::from(rgb(0x6366f1)).opacity(0.16))
                        .stroke_width(px(2.8))
                        .into_any_element(),
                    ]),
                ))
                .child(section(
                    "线型与紧凑模式",
                    "可切换实线、虚线、点线、平滑或折线，适合表格内嵌小图。",
                    row_md(vec![
                        Sparkline::new(up_values())
                            .id("sparkline-demo-dashed")
                            .width(px(180.0))
                            .height(px(48.0))
                            .color(rgb(0x2563eb).into())
                            .line_style(ChartLineStyle::Dashed)
                            .smooth(false)
                            .show_last_point(false)
                            .into_any_element(),
                        Sparkline::new(down_values())
                            .id("sparkline-demo-dotted")
                            .width(px(180.0))
                            .height(px(48.0))
                            .color(rgb(0xdc2626).into())
                            .dotted()
                            .show_last_point(false)
                            .into_any_element(),
                    ]),
                )),
        )
    }
}

fn metric_card(title: &str, value: &str, sparkline: Sparkline) -> impl IntoElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(
                Text::new(title.to_string())
                    .size(px(12.0))
                    .text_color(rgb(0x64748b).into()),
            )
            .child(Text::new(value.to_string()).size(px(24.0)).bold())
            .child(sparkline.height(px(64.0)).area_fill(true)),
    )
    .width(px(240.0))
}

fn up_values() -> [f64; 8] {
    [12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0]
}

fn down_values() -> [f64; 8] {
    [28.0, 24.0, 25.0, 22.0, 18.0, 17.0, 15.0, 12.0]
}

fn mixed_values() -> [f64; 9] {
    [-4.0, -1.0, 3.0, 7.0, 5.0, -2.0, 4.0, 10.0, 8.0]
}

#[cfg(test)]
mod tests {
    #[test]
    fn sparkline_demo_covers_core_options() {
        let source = include_str!("sparkline_demo.rs");
        assert!(source.contains("Sparkline::new"));
        assert!(source.contains("Sparkline::from_points"));
        assert!(source.contains(".area_fill(true)"));
        assert!(source.contains(".show_baseline(true)"));
        assert!(source.contains(".trend_colors("));
        assert!(source.contains("ChartLineStyle::Dashed"));
        assert!(source.contains(".dotted()"));
    }
}
