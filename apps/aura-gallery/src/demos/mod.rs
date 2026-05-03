pub mod button_demo;

use gpui::AnyElement;
use aura_theme::AuraTheme;
use crate::category::Category;

pub struct DemoEntry {
    pub name: &'static str,
    pub category: Category,
    pub description: &'static str,
    pub render: fn(&AuraTheme) -> AnyElement,
}

pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry {
            name: "Button 按钮",
            category: Category::Basic,
            description: "常用的操作按钮",
            render: button_demo::render,
        },
    ]
}
