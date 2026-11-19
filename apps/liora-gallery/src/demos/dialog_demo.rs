use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Dialog, Row, RowJustify, Space, Text};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DialogDemo { child_closes: 0 }).into()
}

struct DialogDemo {
    child_closes: usize,
}

impl Render for DialogDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let owner = cx.weak_entity();
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
                                        .child(Text::new("遮罩和 ESC 不会关闭；标题关闭按钮或业务按钮仍可关闭。"))
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
                                        .child(Text::new("Name: Liora User"))
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
                ))
                .child(section(
                    "Nested 嵌套对话框",
                    "子对话框禁用遮罩关闭且屏蔽穿透；业务 Close 不计入 built-in 关闭次数。",
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(Text::new(format!("Child built-in closes: {}", self.child_closes)))
                        .child(Button::new("Open Parent Dialog").on_click(move |_, window, cx| {
                            let owner = owner.clone();
                            Dialog::new()
                                .title("Parent dialog")
                                .content(move |_, cx| {
                                    let child_owner = owner.clone();
                                    Space::new()
                                        .vertical()
                                        .gap_lg()
                                        .when_some(owner.upgrade(), |s, owner| {
                                            s.child(Text::new(format!(
                                                "Child built-in closes: {}",
                                                owner.read(cx).child_closes
                                            )))
                                        })
                                        .child(Button::new("Open Child Dialog").on_click(move |_, window, cx| {
                                            Dialog::new()
                                                .title("Child dialog")
                                                .close_on_click_outside(false)
                                                .on_close({
                                                    let owner = child_owner.clone();
                                                    move |_, cx| {
                                                        let _ = owner.update(cx, |this, cx| {
                                                            this.child_closes += 1;
                                                            cx.notify();
                                                        });
                                                    }
                                                })
                                                .content(|_, _| {
                                                    dialog_body("Outside clicks preserve both dialogs; business Close preserves the callback count.")
                                                })
                                                .show_in_window(window, cx);
                                        }))
                                })
                                .show_in_window(window, cx);
                        })),
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
