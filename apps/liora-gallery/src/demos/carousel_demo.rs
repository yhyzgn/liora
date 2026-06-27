use gpui::{AnyView, App, Context, Render, Window, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{Card, Carousel, CarouselIndicatorPosition, CarouselItem, Flex, Space};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CarouselDemo::default()).into()
}

#[derive(Default)]
struct CarouselDemo {
    basic_index: usize,
    autoplay_index: usize,
}

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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        let basic_index = self.basic_index;
        let autoplay_index = self.autoplay_index;
        page(
            "Carousel 走马灯",
            "轮播展示宣传卡片、功能亮点或图片内容，支持方向、指示器和自动播放配置。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础轮播",
                    "默认水平轮播，指示器位于内容内部。",
                    Card::new(
                        Carousel::new(items())
                            .active_index(basic_index)
                            .height(px(240.0))
                            .on_change({
                                let view = view.clone();
                                move |index, _, cx| {
                                    view.update(cx, |demo, cx| {
                                        demo.basic_index = index;
                                        cx.notify();
                                    });
                                }
                            }),
                    ),
                ))
                .child(section(
                    "外置指示器与自动播放配置",
                    "autoplay、interval 与 pause_on_hover 作为配置展示，后续可由上层定时状态驱动 active_index。",
                    Card::new(
                        Carousel::new(items())
                            .active_index(autoplay_index)
                            .autoplay(true)
                            .interval_ms(1800)
                            .pause_on_hover(true)
                            .indicator_position(CarouselIndicatorPosition::Outside)
                            .height(px(220.0))
                            .on_change({
                                let view = view.clone();
                                move |index, _, cx| {
                                    view.update(cx, |demo, cx| {
                                        demo.autoplay_index = index;
                                        cx.notify();
                                    });
                                }
                            }),
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
                                Flex::new()
                                    .row()
                                    .center()
                                    .rounded_pill()
                                    .bg(rgb(0xfff7ed).into())
                                    .padding_x_units(12.0)
                                    .padding_y_px(4.0)
                                    .text_sm()
                                    .text_color(rgb(0x9a3412).into())
                                    .child("Composable slot"),
                            ),
                    ]).show_arrows(false).hide_indicators()),
                )),
        )
    }
}
