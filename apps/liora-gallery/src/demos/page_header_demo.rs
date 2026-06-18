use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Button, ButtonVariant, Card, PageHeader, Row, Space, Text};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PageHeaderDemo).into()
}

struct PageHeaderDemo;

impl Render for PageHeaderDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "PageHeader 页头",
            "位于页面顶部，用于标识页面内容并提供相关的操作。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "只有标题与返回入口。",
                    PageHeader::new("详情页面").on_back(|_, _| println!("Back Clicked")),
                ))
                .child(section(
                    "有副标题及操作区",
                    "标题右侧可以放置操作按钮。",
                    PageHeader::new("详情页面")
                        .sub_title("子标题")
                        .on_back(|_, _| println!("Back Clicked"))
                        .extra(|_, _| {
                            Space::new()
                                .gap_sm()
                                .child(Button::new("编辑"))
                                .child(Button::new("主要操作").variant(ButtonVariant::Primary))
                                .into_any_element()
                        }),
                ))
                .child(section(
                    "完整案例",
                    "包含页头内容与页脚区域。",
                    Card::new(
                        PageHeader::new("详情页面")
                            .sub_title("子标题")
                            .on_back(|_, _| println!("Back Clicked"))
                            .extra(|_, _| {
                                Space::new()
                                    .gap_sm()
                                    .child(Button::new("刷新"))
                                    .child(Button::new("提交").variant(ButtonVariant::Primary))
                                    .into_any_element()
                            })
                            .content(|_, _| {
                                Row::new()
                                    .child(
                                        Space::new()
                                            .vertical()
                                            .gap_xs()
                                            .child(Text::new("创建人"))
                                            .child(Text::new("张三").bold()),
                                    )
                                    .child(
                                        Space::new()
                                            .vertical()
                                            .gap_xs()
                                            .child(Text::new("创建时间"))
                                            .child(Text::new("2026-05-06").bold()),
                                    )
                                    .into_any_element()
                            })
                            .footer(|_, _| Text::new("页脚内容区域").into_any_element()),
                    )
                    .no_shadow(),
                )),
        )
    }
}
