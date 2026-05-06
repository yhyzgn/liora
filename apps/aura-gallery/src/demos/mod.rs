pub mod button_demo;
pub mod icon_demo;
pub mod link_demo;
pub mod typography_demo;
pub mod layout_demo;
pub mod container_demo;
pub mod scrollbar_demo;
pub mod splitter_demo;
pub mod form_demo;
pub mod tooltip_demo;
pub mod popover_demo;
pub mod popconfirm_demo;
pub mod dialog_demo;
pub mod drawer_demo;
pub mod message_demo;
pub mod notification_demo;
pub mod page_header_demo;
pub mod progress_demo;
pub mod skeleton_demo;
pub mod result_demo;
pub mod timeline_demo;
pub mod tree_demo;
pub mod descriptions_demo;
pub mod empty_demo;
pub mod alert_demo;
pub mod affix_demo;
pub mod backtop_demo;
pub mod anchor_demo;
pub mod breadcrumb_demo;
pub mod steps_demo;
pub mod menu_demo;
pub mod tabs_demo;
pub mod loading_demo;
pub mod card_demo;
pub mod collapse_demo;
pub mod dropdown_demo;
pub mod message_box_demo;

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
        DemoEntry { name: "Tooltip 文字提示", description: "简单的文字提示", render: |cx| tooltip_demo::render(cx).into() },
        DemoEntry { name: "Popover 气泡卡片", description: "点击弹出的卡片容器", render: |cx| popover_demo::render(cx).into() },
        DemoEntry { name: "Popconfirm 气泡确认框", description: "简单的确认操作", render: |cx| popconfirm_demo::render(cx).into() },
        DemoEntry { name: "Dialog 对话框", description: "模态对话框", render: |cx| dialog_demo::render(cx).into() },
        DemoEntry { name: "Drawer 抽屉", description: "屏幕边缘滑出的浮层面板", render: |cx| drawer_demo::render(cx).into() },
        DemoEntry { name: "Message 全局提示", description: "常用于操作后的反馈提示", render: |cx| message_demo::render(cx).into() },
        DemoEntry { name: "Notification 通知", description: "悬浮在页面角落的通知", render: |cx| notification_demo::render(cx).into() },
        DemoEntry { name: "PageHeader 页头", description: "标识页面内容并提供操作", render: |cx| page_header_demo::render(cx).into() },
        DemoEntry { name: "Progress 进度条", description: "展示操作进度", render: |cx| progress_demo::render(cx).into() },
        DemoEntry { name: "Skeleton 骨架屏", description: "数据加载时的占位展示", render: |cx| skeleton_demo::render(cx).into() },
        DemoEntry { name: "Result 结果页", description: "反馈操作结果", render: |cx| result_demo::render(cx).into() },
        DemoEntry { name: "Descriptions 描述列表", description: "展示多个字段", render: |cx| descriptions_demo::render(cx).into() },
        DemoEntry { name: "Timeline 时间线", description: "垂直展示一系列信息", render: |cx| timeline_demo::render(cx).into() },
        DemoEntry { name: "Tree 树形控件", description: "分层展示数据", render: |cx| tree_demo::render(cx).into() },
        DemoEntry { name: "Empty 空状态", description: "页面无数据时的占位提示", render: |cx| empty_demo::render(cx).into() },
        DemoEntry { name: "Alert 警告", description: "页面展示的重要提示信息", render: |cx| alert_demo::render(cx).into() },
        DemoEntry { name: "Affix 固钉", description: "将内容固定在特定可视区域", render: |cx| affix_demo::render(cx).into() },
        DemoEntry { name: "Backtop 回到顶部", description: "返回页面顶部的快捷按钮", render: |cx| backtop_demo::render(cx).into() },
        DemoEntry { name: "Anchor 锚点", description: "长页面快速跳转与滚动同步", render: |cx| anchor_demo::render(cx).into() },
        DemoEntry { name: "Breadcrumb 面包屑", description: "显示当前页面的路径", render: |cx| breadcrumb_demo::render(cx).into() },
        DemoEntry { name: "Steps 步骤条", description: "引导用户完成任务", render: |cx| steps_demo::render(cx).into() },
        DemoEntry { name: "Menu 导航菜单", description: "为网站提供导航轮廓", render: |cx| menu_demo::render(cx).into() },
        DemoEntry { name: "Tabs 标签页", description: "在同一区域展示多个面板", render: |cx| tabs_demo::render(cx).into() },
        DemoEntry { name: "Loading 加载", description: "加载数据时显示", render: |cx| loading_demo::render(cx).into() },
        DemoEntry { name: "MessageBox 弹窗消息", description: "简单的消息对话框", render: |cx| message_box_demo::render(cx).into() },
        DemoEntry { name: "Card 卡片", description: "内容聚合容器", render: |cx| card_demo::render(cx).into() },
        DemoEntry { name: "Collapse 折叠面板", description: "内容收纳容器", render: |cx| collapse_demo::render(cx).into() },
        DemoEntry { name: "Dropdown 下拉菜单", description: "操作列表容器", render: |cx| dropdown_demo::render(cx).into() },
        DemoEntry { name: "Link 链接", description: "文字超链接", render: |cx| link_demo::render(cx).into() },
        DemoEntry { name: "Typography 排版", description: "标题、段落、文本", render: |cx| typography_demo::render(cx).into() },
        DemoEntry { name: "Layout 布局", description: "分割线、间距、栅格", render: |cx| layout_demo::render(cx).into() },
        DemoEntry { name: "Container 容器", description: "页面框架布局", render: |cx| container_demo::render(cx).into() },
        DemoEntry { name: "Scrollbar 滚动条", description: "原生滚动容器", render: |cx| scrollbar_demo::render(cx).into() },
        DemoEntry { name: "Splitter 分隔面板", description: "左右面板分割", render: |cx| splitter_demo::render(cx).into() },
        DemoEntry { name: "Icon 图标", description: "基于 Lucide 的图标系统", render: |cx| icon_demo::render(cx).into() },
    ]
}
