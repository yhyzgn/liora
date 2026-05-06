use aura_components::{Menu, MenuMode};
use aura_core::Config;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| MenuDemo).into()
}

struct MenuDemo;

impl Render for MenuDemo {
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
                            .child("Menu 导航菜单"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("为网站提供导航轮廓。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("水平模式"))
                    .child(cx.new(|_| {
                        Menu::new()
                            .mode(MenuMode::Horizontal)
                            .default_active("1")
                            .item("1", "处理中心", Some(IconName::List))
                            .submenu("2", "我的工作台", Some(IconName::Briefcase), |s| {
                                s.item("2-1", "选项1", None)
                                    .item("2-2", "选项2", None)
                                    .item("2-3", "选项3", None)
                            })
                            .item("3", "消息中心", Some(IconName::Bell))
                            .item("4", "订单管理", Some(IconName::FileText))
                    })),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_8()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .w(px(240.0))
                            .child(div().font_weight(gpui::FontWeight::BOLD).child("垂直模式"))
                            .child(cx.new(|_| {
                                Menu::new()
                                    .mode(MenuMode::Vertical)
                                    .default_active("1")
                                    .item("1", "导航一", Some(IconName::House))
                                    .submenu("2", "导航二", Some(IconName::Settings), |s| {
                                        s.item("2-1", "选项1", None)
                                            .item("2-2", "选项2", None)
                                            .group("分组一", |g| {
                                                g.item("2-3", "选项3", None)
                                                    .item("2-4", "选项4", None)
                                            })
                                    })
                                    .item("3", "导航三", Some(IconName::MessageSquare))
                            })),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_4()
                            .w(px(64.0))
                            .child(div().font_weight(gpui::FontWeight::BOLD).child("折叠"))
                            .child(cx.new(|_| {
                                Menu::new()
                                    .mode(MenuMode::Vertical)
                                    .collapse(true)
                                    .default_active("1")
                                    .item("1", "导航一", Some(IconName::House))
                                    .submenu("2", "导航二", Some(IconName::Settings), |s| {
                                        s.item("2-1", "选项1", None).item("2-2", "选项2", None)
                                    })
                                    .item("3", "导航三", Some(IconName::MessageSquare))
                            })),
                    ),
            )
    }
}
