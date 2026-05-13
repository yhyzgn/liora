//! Left and right positioned Tabs.
//!
//! `Tabs` is a stateful GPUI view component. In an application view, create each
//! value with `cx.new(|_| left_position_tabs())` or `cx.new(|_| right_position_tabs())`,
//! then compose the resulting `Entity<Tabs>` values in a `Space` or `Row`.

use aura_components::{TabPosition, Tabs, Text};

pub fn left_position_tabs() -> Tabs {
    Tabs::new("first")
        .id("docs-tabs-left")
        .position(TabPosition::Left)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
}

pub fn right_position_tabs() -> Tabs {
    Tabs::new("first")
        .id("docs-tabs-right")
        .position(TabPosition::Right)
        .pane("first", "用户管理", |_, _| Text::new("用户管理内容"))
        .pane("second", "配置管理", |_, _| Text::new("配置管理内容"))
}
