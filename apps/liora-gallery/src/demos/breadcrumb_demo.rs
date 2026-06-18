use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Breadcrumb, BreadcrumbItem, Space};
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BreadcrumbDemo).into()
}

struct BreadcrumbDemo;

impl Render for BreadcrumbDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Breadcrumb 面包屑",
            "显示当前页面的路径，快速返回之前的页面。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "默认使用斜杠分隔各级路径。",
                    Breadcrumb::new()
                        .item(BreadcrumbItem::new("首页"))
                        .item(BreadcrumbItem::new("活动管理"))
                        .item(BreadcrumbItem::new("活动列表"))
                        .item(BreadcrumbItem::new("活动详情")),
                ))
                .child(section(
                    "图标类型",
                    "首项可以携带图标来强化入口语义。",
                    Breadcrumb::new()
                        .item(BreadcrumbItem::new("首页").icon(IconName::House))
                        .item(BreadcrumbItem::new("推广管理"))
                        .item(BreadcrumbItem::new("推广列表"))
                        .item(BreadcrumbItem::new("推广详情")),
                ))
                .child(section(
                    "自定义分隔符 (String)",
                    "使用文本作为路径层级之间的分隔符。",
                    Breadcrumb::new()
                        .separator(">")
                        .item(BreadcrumbItem::new("首页"))
                        .item(BreadcrumbItem::new("推广管理"))
                        .item(BreadcrumbItem::new("推广列表"))
                        .item(BreadcrumbItem::new("推广详情")),
                ))
                .child(section(
                    "自定义分隔符 (Icon)",
                    "使用图标作为路径层级之间的分隔符。",
                    Breadcrumb::new()
                        .separator_icon(IconName::ChevronRight)
                        .item(BreadcrumbItem::new("首页"))
                        .item(BreadcrumbItem::new("推广管理"))
                        .item(BreadcrumbItem::new("推广列表"))
                        .item(BreadcrumbItem::new("推广详情")),
                ))
                .child(section(
                    "点击事件",
                    "前置路径项可以响应点击并执行跳转逻辑。",
                    Breadcrumb::new()
                        .item(BreadcrumbItem::new("首页").on_click(|_, _| println!("Home Clicked")))
                        .item(
                            BreadcrumbItem::new("推广管理")
                                .on_click(|_, _| println!("Management Clicked")),
                        )
                        .item(BreadcrumbItem::new("推广列表")),
                )),
        )
    }
}
