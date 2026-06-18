use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, ButtonVariant, Card, Empty};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::{page, row_md, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| EmptyDemo).into()
}

struct EmptyDemo;

impl Render for EmptyDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Empty 空状态",
            "展示页面无数据时的占位图及提示。",
            row_md(vec![
                section("基础用法", "默认空状态。", Card::new(Empty::new())).into_any_element(),
                section(
                    "自定义描述",
                    "覆盖默认描述文案。",
                    Card::new(Empty::new().description("自定义描述文字")),
                )
                .into_any_element(),
                section(
                    "自定义图片",
                    "使用图标作为空状态图片。",
                    Card::new(
                        Empty::new()
                            .image(Icon::new(IconName::Search))
                            .description("没有找到相关内容"),
                    ),
                )
                .into_any_element(),
                section(
                    "底部操作按钮",
                    "在空状态底部追加操作。",
                    Card::new(Empty::new().extra(|_, _| {
                        Button::new("去添加")
                            .variant(ButtonVariant::Primary)
                            .into_any_element()
                    })),
                )
                .into_any_element(),
            ]),
        )
    }
}
