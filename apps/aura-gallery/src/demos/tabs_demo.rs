use aura_components::{Tabs, TabPosition, TabType};
use aura_core::{Config};
use gpui::{prelude::*, App, Context, Render, Window, div, AnyView, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TabsDemo).into()
}

struct TabsDemo;

impl Render for TabsDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        
        div().flex().flex_col().gap_8().p_4()
            .child(
                div().flex().flex_col().gap_2()
                    .child(div().text_lg().font_weight(gpui::FontWeight::BOLD).child("Tabs 标签页"))
                    .child(div().text_sm().text_color(theme.neutral.text_3).child("在同一区域展示多个面板，通过点击标签进行切换。"))
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        cx.new(|_| {
                            Tabs::new("first")
                                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
                        })
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("卡片样式"))
                    .child(
                        cx.new(|_| {
                            Tabs::new("first")
                                .type_(TabType::Card)
                                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
                        })
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("带边框卡片样式"))
                    .child(
                        cx.new(|_| {
                            Tabs::new("first")
                                .type_(TabType::BorderCard)
                                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
                        })
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("不同位置 (Left / Right)"))
                    .child(
                        div().flex().flex_row().gap_8()
                            .child(
                                cx.new(|_| {
                                    Tabs::new("first")
                                        .position(TabPosition::Left)
                                        .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                                        .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                                })
                            )
                            .child(
                                cx.new(|_| {
                                    Tabs::new("first")
                                        .position(TabPosition::Right)
                                        .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                                        .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                                })
                            )
                    )
            )
            .child(
                div().flex().flex_col().gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("可编辑 (Add / Remove)"))
                    .child(
                        cx.new(|_| {
                            Tabs::new("1")
                                .editable(true)
                                .pane("1", "Tab 1", |_, _| div().child("Content of Tab 1"))
                                .pane("2", "Tab 2", |_, _| div().child("Content of Tab 2"))
                                .on_tab_add(|_, _| println!("Add Tab Clicked"))
                                .on_tab_remove(|name, _, _| println!("Remove Tab: {}", name))
                        })
                    )
            )
    }
}
