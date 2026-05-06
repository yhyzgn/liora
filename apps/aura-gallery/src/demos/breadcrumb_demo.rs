use aura_components::{Breadcrumb, BreadcrumbItem};
use aura_core::Config;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BreadcrumbDemo).into()
}

struct BreadcrumbDemo;

impl Render for BreadcrumbDemo {
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
                            .child("Breadcrumb 面包屑"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("显示当前页面的路径，快速返回之前的页面。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        Breadcrumb::new()
                            .item(BreadcrumbItem::new("首页"))
                            .item(BreadcrumbItem::new("活动管理"))
                            .item(BreadcrumbItem::new("活动列表"))
                            .item(BreadcrumbItem::new("活动详情")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("图标类型"))
                    .child(
                        Breadcrumb::new()
                            .item(BreadcrumbItem::new("首页").icon(IconName::House))
                            .item(BreadcrumbItem::new("推广管理"))
                            .item(BreadcrumbItem::new("推广列表"))
                            .item(BreadcrumbItem::new("推广详情")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自定义分隔符 (String)"),
                    )
                    .child(
                        Breadcrumb::new()
                            .separator(">")
                            .item(BreadcrumbItem::new("首页"))
                            .item(BreadcrumbItem::new("推广管理"))
                            .item(BreadcrumbItem::new("推广列表"))
                            .item(BreadcrumbItem::new("推广详情")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自定义分隔符 (Icon)"),
                    )
                    .child(
                        Breadcrumb::new()
                            .separator_icon(IconName::ChevronRight)
                            .item(BreadcrumbItem::new("首页"))
                            .item(BreadcrumbItem::new("推广管理"))
                            .item(BreadcrumbItem::new("推广列表"))
                            .item(BreadcrumbItem::new("推广详情")),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("点击事件"))
                    .child(
                        Breadcrumb::new()
                            .item(
                                BreadcrumbItem::new("首页")
                                    .on_click(|_, _| println!("Home Clicked")),
                            )
                            .item(
                                BreadcrumbItem::new("推广管理")
                                    .on_click(|_, _| println!("Management Clicked")),
                            )
                            .item(BreadcrumbItem::new("推广列表")),
                    ),
            )
    }
}
