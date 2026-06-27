use gpui::{AnyElement, AnyView, App, Context, Render, Window, div, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{SearchableList, SearchableListItem, Space, Text};
use liora_core::Config;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SearchableListDemo).into()
}

struct SearchableListDemo;

fn component_items() -> Vec<SearchableListItem> {
    vec![
        SearchableListItem::labeled("button", "Button")
            .description("Primary actions and loading states")
            .group("Basic"),
        SearchableListItem::labeled("input", "Input")
            .description("Text entry with prefix/suffix slots")
            .group("Basic"),
        SearchableListItem::labeled("combobox", "Combobox")
            .description("Searchable select with grouped options")
            .group("Input"),
        SearchableListItem::labeled("status-bar", "StatusBar")
            .description("Desktop shell status strip")
            .group("Shell"),
        SearchableListItem::labeled("dock-layout", "DockLayout")
            .description("Panel docking and split regions")
            .group("Shell")
            .disabled(true),
    ]
}

fn demo_card(title: &'static str, detail: &'static str, body: AnyElement) -> impl IntoElement {
    div()
        .w(px(360.0))
        .flex()
        .flex_col()
        .gap_3()
        .rounded_lg()
        .border_1()
        .p_4()
        .child(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new(title).bold())
                .child(Text::new(detail).sm().wrap()),
        )
        .child(body)
}

impl Render for SearchableListDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        page(
            "SearchableList 可搜索列表",
            "通用过滤列表底座，统一 value/label/description/group/disabled/selected 等选项能力，供 Combobox、命令面板和设置页复用。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "过滤与分组",
                    "同一组数据可以按查询词过滤，并保留分组标题与描述信息。",
                    Space::new()
                        .wrap()
                        .gap_lg()
                        .child(demo_card(
                            "All components",
                            "空查询展示全部分组和禁用项。",
                            SearchableList::new(component_items())
                                .selected("combobox")
                                .width(px(328.0))
                                .into_any_element(),
                        ))
                        .child(demo_card(
                            "Query: shell",
                            "可命中 value、label、description 或 group。",
                            SearchableList::new(component_items())
                                .query("shell")
                                .selected_values(vec!["status-bar", "dock-layout"])
                                .width(px(328.0))
                                .into_any_element(),
                        )),
                ))
                .child(section(
                    "空态与限制",
                    "用 max_items 控制首屏数量，用 empty_text 给业务语境。",
                    Space::new()
                        .wrap()
                        .gap_lg()
                        .child(demo_card(
                            "Limited",
                            "只展示前 2 个匹配项。",
                            SearchableList::new(component_items())
                                .max_items(2)
                                .width(px(328.0))
                                .into_any_element(),
                        ))
                        .child(demo_card(
                            "Empty",
                            "无匹配项时渲染轻量空态。",
                            SearchableList::new(component_items())
                                .query("not-found")
                                .empty_text("No component found")
                                .background(theme.neutral.hover.opacity(0.5))
                                .width(px(328.0))
                                .into_any_element(),
                        )),
                )),
        )
    }
}
