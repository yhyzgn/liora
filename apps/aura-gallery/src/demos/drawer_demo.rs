use aura_components::{Button, Drawer, DrawerPlacement, Space, Text};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DrawerDemo).into()
}

struct DrawerDemo;

impl Render for DrawerDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Drawer 抽屉",
            "屏幕边缘滑出的浮层面板。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Placements 四方向",
                    "屏幕边缘滑出的浮层面板。",
                    Space::new()
                        .gap_lg()
                        .wrap()
                        .child(Button::new("Right Drawer").primary().on_click(|_, _, cx| {
                            drawer("Right Drawer", DrawerPlacement::Right).show(cx);
                        }))
                        .child(Button::new("Left Drawer").on_click(|_, _, cx| {
                            drawer("Left Drawer", DrawerPlacement::Left).show(cx);
                        }))
                        .child(Button::new("Top Drawer").on_click(|_, _, cx| {
                            drawer("Top Drawer", DrawerPlacement::Top)
                                .height_sm()
                                .show(cx);
                        }))
                        .child(Button::new("Bottom Drawer").on_click(|_, _, cx| {
                            drawer("Bottom Drawer", DrawerPlacement::Bottom)
                                .height_sm()
                                .show(cx);
                        })),
                ))
                .child(section(
                    "Size 尺寸",
                    "可配置宽度或高度。",
                    Space::new()
                        .gap_lg()
                        .wrap()
                        .child(Button::new("Wide Drawer").on_click(|_, _, cx| {
                            drawer("Wide Drawer", DrawerPlacement::Right)
                                .width_lg()
                                .show(cx);
                        }))
                        .child(Button::new("Tall Top Drawer").on_click(|_, _, cx| {
                            drawer("Tall Top Drawer", DrawerPlacement::Top)
                                .height_lg()
                                .show(cx);
                        })),
                ))
                .child(section(
                    "Close strategy 关闭策略",
                    "可禁用遮罩和 ESC 关闭，改由业务按钮手动关闭。",
                    Space::new().gap_lg().wrap().child(
                        Button::new("Manual Close Only")
                            .warning()
                            .on_click(|_, _, cx| {
                                Drawer::new()
                                    .title("Manual close drawer")
                                    .close_on_click_outside(false)
                                    .close_on_escape(false)
                                    .content(|_, _| {
                                        Space::new()
                                            .vertical()
                                            .gap_lg()
                                            .child(Text::new("点击遮罩和按 ESC 都不会关闭。"))
                                            .child(
                                                Button::new("Close Drawer")
                                                    .primary()
                                                    .on_click(|_, _, cx| Drawer::close(cx)),
                                            )
                                    })
                                    .show(cx);
                            }),
                    ),
                )),
        )
    }
}

fn drawer(title: &'static str, placement: DrawerPlacement) -> Drawer {
    Drawer::new()
        .title(title)
        .placement(placement)
        .content(move |_, _| {
            Space::new()
                .vertical()
                .gap_lg()
                .child(Text::new(format!("This is a {:?} drawer.", placement)))
                .child(
                    Button::new("Close")
                        .primary()
                        .on_click(|_, _, cx| Drawer::close(cx)),
                )
        })
}
