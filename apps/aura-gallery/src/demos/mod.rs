pub mod button_demo;
pub mod icon_demo;
pub mod link_demo;
pub mod typography_demo;
pub mod layout_demo;
pub mod container_demo;

use gpui::AnyElement;

pub struct DemoEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub render: fn() -> AnyElement,
}

pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry { name: "Button 按钮", description: "常用的操作按钮", render: button_demo::render },
        DemoEntry { name: "Link 链接", description: "文字超链接", render: link_demo::render },
        DemoEntry { name: "Typography 排版", description: "标题、段落、文本", render: typography_demo::render },
        DemoEntry { name: "Layout 布局", description: "分割线、间距、栅格", render: layout_demo::render },
        DemoEntry { name: "Container 容器", description: "页面框架布局", render: container_demo::render },
        DemoEntry { name: "Icon 图标", description: "基于 Lucide 的图标系统", render: icon_demo::render },
    ]
}
