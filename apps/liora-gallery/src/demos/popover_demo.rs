use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Popover, Space, Text};
use liora_core::Placement;

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PopoverDemo).into()
}

struct PopoverDemo;

impl Render for PopoverDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Popover 气泡卡片",
            "点击触发元素显示卡片内容。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Basic 基础用法",
                    "点击触发元素显示卡片内容。",
                    Space::new()
                        .gap_lg()
                        .wrap()
                        .child(card_popover("Bottom Center", Placement::Bottom))
                        .child(card_popover("Top Center", Placement::Top))
                        .child(card_popover("Left Center", Placement::Left))
                        .child(card_popover("Right Center", Placement::Right)),
                ))
                .child(section(
                    "Placements 十二方向",
                    "覆盖 Top/Bottom/Left/Right 及 Start/End 对齐。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(simple_popover("TopStart", Placement::TopStart))
                                .child(simple_popover("Top", Placement::Top))
                                .child(simple_popover("TopEnd", Placement::TopEnd)),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(simple_popover("LeftStart", Placement::LeftStart))
                                .child(simple_popover("RightStart", Placement::RightStart)),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(simple_popover("Left", Placement::Left))
                                .child(simple_popover("Right", Placement::Right)),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(simple_popover("LeftEnd", Placement::LeftEnd))
                                .child(simple_popover("RightEnd", Placement::RightEnd)),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(simple_popover("BottomStart", Placement::BottomStart))
                                .child(simple_popover("Bottom", Placement::Bottom))
                                .child(simple_popover("BottomEnd", Placement::BottomEnd)),
                        ),
                ))
                .child(section(
                    "Close strategy 关闭策略",
                    "支持禁用点击空白处关闭，只能通过内容按钮手动关闭。",
                    Space::new()
                        .gap_lg()
                        .wrap()
                        .child(
                            Popover::new(Button::new("Manual Close Only").warning())
                                .id("popover-demo-manual-close")
                                .placement(Placement::Bottom)
                                .close_on_click_outside(false)
                                .content(|_, _| {
                                    Space::new()
                                        .vertical()
                                        .gap_md()
                                        .child(Text::new("Manual close").bold())
                                        .child(Text::new("点击空白处不会关闭；点击按钮手动关闭。"))
                                        .child(
                                            Button::new("Close Popover")
                                                .primary()
                                                .small()
                                                .on_click(|_, _, cx| {
                                                    liora_core::clear_popover(
                                                        &"popover-demo-manual-close".into(),
                                                        cx,
                                                    );
                                                }),
                                        )
                                }),
                        )
                        .child(
                            Popover::new(Button::new("Custom Offset"))
                                .id("popover-demo-custom-offset")
                                .placement(Placement::Bottom)
                                .offset_lg()
                                .content(|_, _| Text::new("Offset = 20px")),
                        ),
                )),
        )
    }
}

fn simple_popover(label: &'static str, placement: Placement) -> Popover {
    Popover::new(Button::new(label).small())
        .id(format!("popover-demo-simple-{}", label))
        .placement(placement)
        .content(move |_, _| Text::new(format!("Placement: {:?}", placement)))
}

fn card_popover(label: &'static str, placement: Placement) -> Popover {
    Popover::new(Button::new(label).primary())
        .id(format!("popover-demo-card-{}", label))
        .placement(placement)
        .content(|_, _| {
            Space::new()
                .vertical()
                .gap_sm()
                .child(Text::new("Title").bold())
                .child(Text::new("This is the popover content."))
                .child(Button::new("Confirm").primary().small())
        })
}
