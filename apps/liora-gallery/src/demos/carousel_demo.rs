use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Card, Carousel, CarouselIndicatorPosition, CarouselItem, Space};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CarouselDemo).into()
}

struct CarouselDemo;

fn items() -> Vec<CarouselItem> {
    vec![
        CarouselItem::new("Native Rust UI")
            .description("Pure GPUI rendering with Liora components, no WebView runtime.")
            .accent(rgb(0x2563eb).into()),
        CarouselItem::new("Charts & Dashboards")
            .description("Line, bar, ring, heat and compact metrics share native chart primitives.")
            .accent(rgb(0x16a34a).into()),
        CarouselItem::new("Packaging Ready")
            .description("Gallery and Docs can be packaged as native desktop applications.")
            .accent(rgb(0x9333ea).into()),
    ]
}

impl Render for CarouselDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Carousel 走马灯",
            "轮播展示宣传卡片、功能亮点或图片内容，支持方向、指示器和自动播放配置。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础轮播",
                    "默认水平轮播，指示器位于内容内部。",
                    Card::new(Carousel::new(items()).active_index(0).height(px(240.0))),
                ))
                .child(section(
                    "外置指示器与自动播放配置",
                    "autoplay、interval 与 pause_on_hover 作为配置展示，后续可由上层定时状态驱动 active_index。",
                    Card::new(
                        Carousel::new(items())
                            .active_index(1)
                            .autoplay(true)
                            .interval_ms(1800)
                            .pause_on_hover(true)
                            .indicator_position(CarouselIndicatorPosition::Outside)
                            .height(px(220.0)),
                    ),
                ))
                .child(section(
                    "自定义内容",
                    "CarouselItem 可以携带自定义原生元素作为内容区域的一部分。",
                    Card::new(Carousel::new(vec![
                        CarouselItem::new("Custom native content")
                            .description("The body below is just another GPUI element.")
                            .accent(rgb(0xf97316).into())
                            .content(
                                div()
                                    .flex()
                                    .rounded_full()
                                    .bg(gpui::white().opacity(0.72))
                                    .px_3()
                                    .py_1()
                                    .text_sm()
                                    .text_color(rgb(0x9a3412))
                                    .child("Composable slot"),
                            ),
                    ]).show_arrows(false).hide_indicators()),
                )),
        )
    }
}
