use aura_components::{Button, Dialog, Row, RowJustify, Space, Text};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DialogDemo).into()
}

struct DialogDemo;

impl Render for DialogDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Dialog 对话框",
            "模态对话框用于展示重要信息或承载需要确认的操作。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Basic 基础用法",
                    "保留当前页面状态时展示重要信息。",
                    Space::new()
                        .gap_lg()
                        .wrap()
                        .child(Button::new("Open Dialog").primary().on_click(|_, _, cx| {
                            Dialog::new()
                                .title("Tips")
                                .content(|_, _| dialog_body("This is a message from the dialog."))
                                .show(cx);
                        }))
                        .child(Button::new("Manual Close Only").warning().on_click(|_, _, cx| {
                            Dialog::new()
                                .title("Manual close dialog")
                                .close_on_click_outside(false)
                                .close_on_escape(false)
                                .content(|_, _| {
                                    Space::new()
                                        .vertical()
                                        .gap_lg()
                                        .child(Text::new("点击遮罩和按 ESC 都不会关闭，只能点击按钮手动关闭。"))
                                        .child(
                                            Row::new().justify(RowJustify::End).child(
                                                Button::new("I understand")
                                                    .primary()
                                                    .on_click(|_, _, cx| Dialog::close(cx)),
                                            ),
                                        )
                                })
                                .show(cx);
                        })),
                ))
                .child(section(
                    "Content 内容",
                    "内容区域可放置任意组件。",
                    Space::new().gap_lg().wrap().child(Button::new("Form-like Content").on_click(
                        |_, _, cx| {
                            Dialog::new()
                                .title("Edit profile")
                                .content(|_, _| {
                                    Space::new()
                                        .vertical()
                                        .gap_md()
                                        .child(Text::new("Name: Aura User"))
                                        .child(Text::new("Role: Designer"))
                                        .child(
                                            Row::new()
                                                .justify(RowJustify::End)
                                                .child(Button::new("Cancel").on_click(|_, _, cx| {
                                                    Dialog::close(cx)
                                                }))
                                                .child(Button::new("Save").primary().on_click(
                                                    |_, _, cx| Dialog::close(cx),
                                                )),
                                        )
                                })
                                .show(cx);
                        },
                    )),
                )),
        )
    }
}

fn dialog_body(message: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_lg()
        .child(Text::new(message))
        .child(
            Row::new().justify(RowJustify::End).child(
                Button::new("Close")
                    .primary()
                    .on_click(|_, _, cx| Dialog::close(cx)),
            ),
        )
}
