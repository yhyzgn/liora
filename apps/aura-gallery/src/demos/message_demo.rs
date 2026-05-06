use aura_components::{Button, MessageType, Space, show_message};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MessageDemo).into()
}

struct MessageDemo;

impl Render for MessageDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Message 全局提示"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("常用于主动操作后的反馈提示。"),
                    ),
            )
            .child(
                Space::new()
                    .gap(px(16.0))
                    .child(Button::new("Info Message").on_click(|_, _, cx| {
                        show_message("This is an info message", MessageType::Info, cx);
                    }))
                    .child(
                        Button::new("Success Message")
                            .primary()
                            .on_click(|_, _, cx| {
                                show_message(
                                    "Congrats! Operation success.",
                                    MessageType::Success,
                                    cx,
                                );
                            }),
                    )
                    .child(
                        Button::new("Warning Message")
                            .warning()
                            .on_click(|_, _, cx| {
                                show_message(
                                    "Be careful! This is a warning.",
                                    MessageType::Warning,
                                    cx,
                                );
                            }),
                    )
                    .child(Button::new("Error Message").danger().on_click(|_, _, cx| {
                        show_message("Oops! Something went wrong.", MessageType::Error, cx);
                    })),
            )
    }
}
