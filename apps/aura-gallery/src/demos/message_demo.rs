use aura_components::{Button, MessageType, show_message};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use super::common::{page, row};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageDemo).into()
}

struct MessageDemo;

impl Render for MessageDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Message 全局提示",
            "常用于主动操作后的反馈提示。",
            row(vec![
                Button::new("Info Message").on_click(|_, _, cx| {
                    show_message("This is an info message", MessageType::Info, cx);
                }),
                Button::new("Success Message")
                    .primary()
                    .on_click(|_, _, cx| {
                        show_message("Congrats! Operation success.", MessageType::Success, cx);
                    }),
                Button::new("Warning Message")
                    .warning()
                    .on_click(|_, _, cx| {
                        show_message("Be careful! This is a warning.", MessageType::Warning, cx);
                    }),
                Button::new("Error Message").danger().on_click(|_, _, cx| {
                    show_message("Oops! Something went wrong.", MessageType::Error, cx);
                }),
            ]),
        )
    }
}
