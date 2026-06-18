use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, Card, Row, Space};

use liora_components::layout_helpers::{page, row_md, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| CardDemo).into()
}

struct CardDemo;

impl Render for CardDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Card 卡片",
            "将信息聚合在卡片容器中展示。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础卡片",
                    "常规卡片和可悬停卡片。",
                    row_md(vec![
                        Card::new("Standard card content goes here.")
                            .title("Standard Card")
                            .width_md()
                            .into_any_element(),
                        Card::new("This card will change shadow on hover.")
                            .title("Hoverable Card")
                            .hoverable()
                            .width_md()
                            .into_any_element(),
                    ]),
                ))
                .child(section(
                    "底部操作",
                    "footer 区域可承载操作按钮。",
                    Card::new("Card body with a custom footer.")
                        .title("Card with Footer")
                        .width_lg()
                        .footer(
                            Row::new()
                                .justify(liora_components::RowJustify::End)
                                .child(Button::new("Cancel").small())
                                .child(Button::new("Save").primary().small()),
                        ),
                )),
        )
    }
}
