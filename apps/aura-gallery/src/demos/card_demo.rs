use aura_components::{Button, Card};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CardDemo).into()
}

struct CardDemo;

impl Render for CardDemo {
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
                            .child("Card 卡片"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("将信息聚合在卡片容器中展示。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(div().w(px(300.0)).child(
                        Card::new("Standard card content goes here.").title("Standard Card"),
                    ))
                    .child(
                        div().w(px(300.0)).child(
                            Card::new("This card will change shadow on hover.")
                                .title("Hoverable Card")
                                .hoverable(),
                        ),
                    ),
            )
            .child(
                div().flex().flex_col().gap_2().child(
                    div()
                        .text_lg()
                        .font_weight(gpui::FontWeight::BOLD)
                        .child("底部操作"),
                ),
            )
            .child(
                div().w(px(400.0)).child(
                    Card::new("Card body with a custom footer.")
                        .title("Card with Footer")
                        .footer(
                            div()
                                .flex()
                                .justify_end()
                                .gap_2()
                                .child(Button::new("Cancel").small())
                                .child(Button::new("Save").primary().small()),
                        ),
                ),
            )
    }
}
