use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Space, TabPosition, TabType, Tabs, Text};

use liora_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TabsDemo {
        basic: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-basic")
                .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
                .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
                .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
        }),
        stretch: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-stretch")
                .stretch(true)
                .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
                .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
                .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
        }),
        card: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-card")
                .type_(TabType::Card)
                .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
                .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
                .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
        }),
        border_card: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-border-card")
                .type_(TabType::BorderCard)
                .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
                .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
                .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
                .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
        }),
        left: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-left")
                .position(TabPosition::Left)
                .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
                .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
        }),
        right: cx.new(|_| {
            Tabs::new("first")
                .id("tabs-demo-right")
                .position(TabPosition::Right)
                .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
                .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
        }),
        editable: cx.new(|_| {
            Tabs::new("1")
                .id("tabs-demo-editable")
                .editable(true)
                .pane("1", "Tab 1", |_, _| Text::new("Content of Tab 1"))
                .pane("2", "Tab 2", |_, _| Text::new("Content of Tab 2"))
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
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tabs 标签页",
            "在同一区域展示多个面板，通过点击标签进行切换。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section("基础用法", "默认标签页。", self.basic.clone()))
                .child(section(
                    "自动均分并占满宽度",
                    "stretch 模式让标签均分宽度。",
                    self.stretch.clone(),
                ))
                .child(section(
                    "卡片样式",
                    "使用 Card 类型标签。",
                    self.card.clone(),
                ))
                .child(section(
                    "带边框卡片样式",
                    "带外边框的卡片标签页。",
                    self.border_card.clone(),
                ))
                .child(section(
                    "不同位置 (Left / Right)",
                    "标签可展示在左右两侧。",
                    Space::new()
                        .gap_lg()
                        .wrap()
                        .child(self.left.clone())
                        .child(self.right.clone()),
                ))
                .child(section(
                    "可编辑 (Add / Remove)",
                    "支持新增和移除标签。",
                    self.editable.clone(),
                )),
        )
    }
}
