use aura_components::{Button, ButtonVariant, Card, Empty};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| EmptyDemo).into()
}

struct EmptyDemo;

impl Render for EmptyDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .p_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Empty 空状态"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("展示页面无数据时的占位图及提示。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(Card::new(Empty::new())),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自定义描述"),
                    )
                    .child(Card::new(Empty::new().description("自定义描述文字"))),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自定义图片"),
                    )
                    .child(Card::new(
                        Empty::new()
                            .image(
                                Icon::new(IconName::Search)
                                    .size(px(100.0))
                                    .color(theme.neutral.text_3),
                            )
                            .description("没有找到相关内容"),
                    )),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("底部操作按钮"),
                    )
                    .child(Card::new(Empty::new().extra(|_, _| {
                        Button::new("去添加")
                            .variant(ButtonVariant::Primary)
                            .into_any_element()
                    }))),
            )
    }
}
