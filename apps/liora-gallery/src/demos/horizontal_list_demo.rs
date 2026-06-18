use gpui::{
    AnyElement, AnyView, App, Context, Render, SharedString, Window, div, prelude::*, px, rgb,
};
use liora_components::layout_helpers::{page, section};
use liora_components::{HorizontalList, Space, Text, toast_success};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| HorizontalListDemo::new(cx)).into()
}

struct HorizontalListDemo {
    basic: gpui::Entity<HorizontalList>,
    divider: gpui::Entity<HorizontalList>,
    draggable: gpui::Entity<HorizontalList>,
}

impl HorizontalListDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|_| {
            HorizontalList::new(PROCESS_STEPS.len(), step_card)
                .height(px(88.0))
                .item_gap(px(12.0))
        });
        let divider = cx.new(|_| {
            HorizontalList::new(FLOW_ITEMS.len(), flow_card)
                .height(px(92.0))
                .item_gap(px(10.0))
                .divider(|_| {
                    Icon::new(IconName::ChevronRight)
                        .size(px(18.0))
                        .color(rgb(0x94a3b8).into())
                        .into_any_element()
                })
        });
        let draggable = cx.new(|_| {
            HorizontalList::new(KANBAN_ITEMS.len(), kanban_card)
                .height(px(118.0))
                .item_gap(px(12.0))
                .draggable(true)
                .on_reorder(|from, to, _, _| {
                    toast_success!("HorizontalList reordered: {} -> {}", from + 1, to + 1);
                })
        });

        Self {
            basic,
            divider,
            draggable,
        }
    }
}

impl Render for HorizontalListDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "HorizontalList 水平列表",
            "横向滚动列表，支持完全自定义 item、divider 和原生拖动排序。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础横向列表",
                    "适合展示流程节点、指标卡片、横向快捷入口。",
                    self.basic.clone(),
                ))
                .child(section(
                    "自定义 divider",
                    "默认 divider 是竖线，也可以改成箭头、图标或任意 GPUI 元素。",
                    self.divider.clone(),
                ))
                .child(section(
                    "拖动排序",
                    "开启 draggable 后，item 前端会显示 Grip 拖拽把手；按住把手拖到目标 item 上松开即可重排，并触发 on_reorder 回调。",
                    self.draggable.clone(),
                )),
        )
    }
}

const PROCESS_STEPS: &[(&str, &str)] = &[
    ("01", "Discover"),
    ("02", "Design"),
    ("03", "Build"),
    ("04", "Verify"),
    ("05", "Ship"),
    ("06", "Observe"),
];

const FLOW_ITEMS: &[(&str, &str)] = &[
    ("Input", "Collect data"),
    ("Validate", "Check rules"),
    ("Transform", "Normalize"),
    ("Export", "Send output"),
];

const KANBAN_ITEMS: &[(&str, &str)] = &[
    ("Inbox", "8 tasks"),
    ("Ready", "5 tasks"),
    ("Doing", "3 tasks"),
    ("Review", "2 tasks"),
    ("Done", "12 tasks"),
];

fn step_card(index: usize) -> AnyElement {
    let (number, label) = PROCESS_STEPS[index];
    div()
        .w(px(132.0))
        .h(px(72.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xdbeafe))
        .bg(rgb(0xeff6ff))
        .p_3()
        .flex()
        .flex_col()
        .justify_between()
        .child(
            Text::new(number)
                .size(px(12.0))
                .text_color(rgb(0x2563eb).into()),
        )
        .child(Text::new(label).bold().text_color(rgb(0x1e3a8a).into()))
        .into_any_element()
}

fn flow_card(index: usize) -> AnyElement {
    let (label, desc) = FLOW_ITEMS[index];
    div()
        .w(px(150.0))
        .h(px(76.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .flex()
        .flex_col()
        .gap_1()
        .child(Text::new(label).bold().text_color(rgb(0x0f172a).into()))
        .child(
            Text::new(desc)
                .size(px(12.0))
                .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}

fn kanban_card(index: usize) -> AnyElement {
    let (label, desc) = KANBAN_ITEMS[index];
    let accent = match index % 4 {
        0 => rgb(0x6366f1),
        1 => rgb(0x14b8a6),
        2 => rgb(0xf59e0b),
        _ => rgb(0xef4444),
    };
    div()
        .w(px(156.0))
        .h(px(96.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().w(px(28.0)).h(px(4.0)).rounded_full().bg(accent))
        .child(Text::new(SharedString::from(label)).bold())
        .child(
            Text::new(SharedString::from(desc))
                .size(px(12.0))
                .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    #[test]
    fn horizontal_list_demo_covers_custom_divider_and_drag() {
        let source = include_str!("horizontal_list_demo.rs");
        assert!(source.contains("HorizontalList::new"));
        assert!(source.contains("divider"));
        assert!(source.contains("draggable(true)"));
        assert!(source.contains("on_reorder"));
    }
}
