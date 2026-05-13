//! Border-card Tabs wrap the whole tab panel with a border.

use aura_components::{TabType, Tabs, Text};

pub fn border_card_tabs() -> Tabs {
    Tabs::new("first")
        .id("docs-tabs-border-card")
        .type_(TabType::BorderCard)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
        .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
        .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
}
