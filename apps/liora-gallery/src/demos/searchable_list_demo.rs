use gpui::{AnyElement, AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{SearchableList, SearchableListItem, Space};
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

fn list_card(title: &'static str, detail: &'static str, body: AnyElement) -> AnyElement {
    showcase_card(title, detail, body).into_any_element()
}

impl Render for SearchableListDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        page(
            "SearchableList 可搜索列表",
            "通用过滤列表底座，统一 value/label/description/group/disabled/selected 等选项能力，供 Combobox、命令面板和设置页复用。",
            Space::new().vertical().gap_xl().child(section(
                "SearchableList showcase",
                "过滤、分组、限制和空态统一使用卡片网格展示。",
                showcase_grid(vec![
                    list_card(
                        "All components",
                        "空查询展示全部分组和禁用项。",
                        SearchableList::new(component_items())
                            .selected("combobox")
                            .width(px(328.0))
                            .into_any_element(),
                    ),
                    list_card(
                        "Query: shell",
                        "可命中 value、label、description 或 group。",
                        SearchableList::new(component_items())
                            .query("shell")
                            .selected_values(vec!["status-bar", "dock-layout"])
                            .width(px(328.0))
                            .into_any_element(),
                    ),
                    list_card(
                        "Limited",
                        "只展示前 2 个匹配项。",
                        SearchableList::new(component_items())
                            .max_items(2)
                            .width(px(328.0))
                            .into_any_element(),
                    ),
                    list_card(
                        "Empty",
                        "无匹配项时渲染轻量空态。",
                        SearchableList::new(component_items())
                            .query("not-found")
                            .empty_text("No component found")
                            .background(theme.neutral.hover.opacity(0.5))
                            .width(px(328.0))
                            .into_any_element(),
                    ),
                ]),
            )),
        )
    }
}
