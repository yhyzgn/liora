pub mod button_demo;
pub mod icon_demo;
pub mod link_demo;
pub mod typography_demo;
pub mod layout_demo;
pub mod container_demo;
pub mod scrollbar_demo;
pub mod splitter_demo;
pub mod form_demo;

use gpui::{AnyView, App};

pub struct DemoEntry {
    pub name: &'static str,
    pub description: &'static str,
    pub render: fn(&mut App) -> AnyView,
}

pub fn registry() -> Vec<DemoEntry> {
    vec![
        DemoEntry { name: "Button 按钮", description: "常用的操作按钮", render: |cx| button_demo::render(cx).into() },
        DemoEntry { name: "Form 表单", description: "输入框、单选、多选、开关", render: |cx| form_demo::render(cx).into() },
        DemoEntry { name: "Link 链接", description: "文字超链接", render: |cx| link_demo::render(cx).into() },
        DemoEntry { name: "Typography 排版", description: "标题、段落、文本", render: |cx| typography_demo::render(cx).into() },
        DemoEntry { name: "Layout 布局", description: "分割线、间距、栅格", render: |cx| layout_demo::render(cx).into() },
        DemoEntry { name: "Container 容器", description: "页面框架布局", render: |cx| container_demo::render(cx).into() },
        DemoEntry { name: "Scrollbar 滚动条", description: "原生滚动容器", render: |cx| scrollbar_demo::render(cx).into() },
        DemoEntry { name: "Splitter 分隔面板", description: "左右面板分割", render: |cx| splitter_demo::render(cx).into() },
        DemoEntry { name: "Icon 图标", description: "基于 Lucide 的图标系统", render: |cx| icon_demo::render(cx).into() },
    ]
}
