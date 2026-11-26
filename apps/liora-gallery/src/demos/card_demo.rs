use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Space;

use liora_components::layout_helpers::{page, section};

#[path = "../../../liora-docs/content/snippets/card/basic.rs"]
mod card_basic_example;
#[path = "../../../liora-docs/content/snippets/card/footer.rs"]
mod card_footer_example;
#[path = "../../../liora-docs/content/snippets/card/layout.rs"]
mod card_layout_example;

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
                    card_basic_example::basic_cards(),
                ))
                .child(section(
                    "底部操作",
                    "Cancel / Save 通过消息展示演示请求，不会保存或撤销数据。",
                    card_footer_example::card_with_footer(),
                ))
                .child(section(
                    "窄容器与自定义布局",
                    "320 px 宽、220 px 高的原生滚动容器；完整宽度、禁止压缩、无默认阴影。",
                    card_layout_example::card_layout(),
                )),
        )
    }
}
