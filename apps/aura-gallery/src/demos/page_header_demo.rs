use aura_components::{Button, ButtonVariant, PageHeader, Space};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PageHeaderDemo).into()
}

struct PageHeaderDemo;

impl Render for PageHeaderDemo {
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
                            .child("PageHeader 页头"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("位于页面顶部，用于标识页面内容并提供相关的操作。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(PageHeader::new("详情页面").on_back(|_, _| println!("Back Clicked"))),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("有副标题及操作区"),
                    )
                    .child(
                        PageHeader::new("详情页面")
                            .sub_title("子标题")
                            .on_back(|_, _| println!("Back Clicked"))
                            .extra(|_, _| {
                                Space::new()
                                    .gap(px(8.0))
                                    .child(Button::new("编辑"))
                                    .child(Button::new("主要操作").variant(ButtonVariant::Primary))
                                    .into_any_element()
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("完整案例"))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .border_1()
                            .border_color(theme.neutral.border)
                            .rounded(px(theme.radius.md))
                            .overflow_hidden()
                            .child(
                                PageHeader::new("详情页面")
                                    .sub_title("子标题")
                                    .on_back(|_, _| println!("Back Clicked"))
                                    .extra(|_, _| {
                                        Space::new()
                                            .gap(px(8.0))
                                            .child(Button::new("刷新"))
                                            .child(
                                                Button::new("提交").variant(ButtonVariant::Primary),
                                            )
                                            .into_any_element()
                                    })
                                    .content(|_, cx| {
                                        let theme = &cx.global::<Config>().theme;
                                        div()
                                            .flex()
                                            .flex_row()
                                            .gap_10()
                                            .text_sm()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_color(theme.neutral.text_3)
                                                            .child("创建人"),
                                                    )
                                                    .child(div().child("张三")),
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_col()
                                                    .gap_1()
                                                    .child(
                                                        div()
                                                            .text_color(theme.neutral.text_3)
                                                            .child("创建时间"),
                                                    )
                                                    .child(div().child("2026-05-06")),
                                            )
                                            .into_any_element()
                                    })
                                    .footer(|_, _| {
                                        div().text_sm().child("页脚内容区域").into_any_element()
                                    }),
                            ),
                    ),
            )
    }
}
