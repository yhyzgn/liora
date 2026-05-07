use aura_components::{TabPosition, TabType, Tabs};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TabsDemo {
        basic: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-basic")
                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
        }),
        stretch: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-stretch")
                .stretch(true)
                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
        }),
        card: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-card")
                .type_(TabType::Card)
                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
        }),
        border_card: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-border-card")
                .type_(TabType::BorderCard)
                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
                .pane("third", "角色管理", |_, _| div().child("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| div().child("定时任务内容"))
        }),
        left: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-left")
                .position(TabPosition::Left)
                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
        }),
        right: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-right")
                .position(TabPosition::Right)
                .pane("first", "用户管理", |_, _| div().child("用户管理内容"))
                .pane("second", "配置管理", |_, _| div().child("配置管理内容"))
        }),
        editable: cx.new(|_| {
            Tabs::new("1")
                .id("tabs-demo-editable")
                .editable(true)
                .pane("1", "Tab 1", |_, _| div().child("Content of Tab 1"))
                .pane("2", "Tab 2", |_, _| div().child("Content of Tab 2"))
                .on_tab_add(|_, _| println!("Add Tab Clicked"))
                .on_tab_remove(|name, _, _| println!("Remove Tab: {}", name))
        }),
    })
    .into()
}

struct TabsDemo {
    basic: Entity<Tabs>,
    stretch: Entity<Tabs>,
    card: Entity<Tabs>,
    border_card: Entity<Tabs>,
    left: Entity<Tabs>,
    right: Entity<Tabs>,
    editable: Entity<Tabs>,
}

impl Render for TabsDemo {
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
                            .child("Tabs 标签页"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("在同一区域展示多个面板，通过点击标签进行切换。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(self.basic.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("自动均分并占满宽度"),
                    )
                    .child(self.stretch.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("卡片样式"))
                    .child(self.card.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("带边框卡片样式"),
                    )
                    .child(self.border_card.clone()),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("不同位置 (Left / Right)"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .gap_8()
                            .child(self.left.clone())
                            .child(self.right.clone()),
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
                            .child("可编辑 (Add / Remove)"),
                    )
                    .child(self.editable.clone()),
            )
    }
}
