use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Space;

use liora_components::layout_helpers::{page, section};

#[path = "../../../liora-docs/content/snippets/message/formatting.rs"]
mod message_formatting_example;
#[path = "../../../liora-docs/content/snippets/message/types.rs"]
mod message_types_example;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageDemo).into()
}

struct MessageDemo;

impl Render for MessageDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Message 全局提示",
            "应用级提示逐条独立计时 3 秒后自动移除；悬停不暂停，也没有关闭按钮。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Toast 快捷宏",
                    "四种宏共享 Message 层；Stack messages 一次加入三条，每条独立到期。",
                    message_types_example::message_types(),
                ))
                .child(section(
                    "模板格式化",
                    "宏支持 format! 风格的位置参数和命名参数。",
                    message_formatting_example::message_formatting(),
                )),
        )
    }
}
