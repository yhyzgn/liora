//! Stretch Tabs split the tab bar width evenly.

use aura_components::{Tabs, Text};

pub fn stretch_tabs() -> Tabs {
    Tabs::new("first")
        .id("docs-tabs-stretch")
        .stretch(true)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
        .pane("third", "角色管理", |_, _| Text::new("角色管理内容"))
        .pane("fourth", "定时任务", |_, _| Text::new("定时任务内容"))
}
