use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Space, toast_error, toast_info, toast_success, toast_warning};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageDemo).into()
}

struct MessageDemo;

impl Render for MessageDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Message 全局提示",
            "常用于主动操作后的反馈提示。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "Toast 快捷宏",
                    "snake_case 宏会复用 Message 全局提示层，不需要在调用处传入 cx。",
                    Space::new()
                        .wrap()
                        .gap_md()
                        .child(Button::new("toast_info!").on_click(|_, _, _| {
                            toast_info!("This is an info toast");
                        }))
                        .child(Button::new("toast_success!").primary().on_click(|_, _, _| {
                            toast_success!("Congrats! Operation success.");
                        }))
                        .child(Button::new("toast_warning!").warning().on_click(|_, _, _| {
                            toast_warning!("Be careful! This is a warning.");
                        }))
                        .child(Button::new("toast_error!").danger().on_click(|_, _, _| {
                            toast_error!("Oops! Something went wrong.");
                        })),
                ))
                .child(section(
                    "模板格式化",
                    "宏支持 format! 风格的位置参数和命名参数。",
                    Space::new()
                        .wrap()
                        .gap_md()
                        .child(Button::new("位置参数").on_click(|_, _, _| {
                            let name = "Liora";
                            let count = 4;
                            toast_info!("{}, you have {} toast variants.", name, count);
                        }))
                        .child(Button::new("命名参数").primary().on_click(|_, _, _| {
                            let component = "Message";
                            let api = "toast_success!";
                            toast_success!("{component} macro {api} works.");
                        })),
                )),
        )
    }
}
