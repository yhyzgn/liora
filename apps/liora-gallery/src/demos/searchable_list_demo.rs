use gpui::{AnyElement, AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{SearchableListItem, SearchableListPanel, Space};
use liora_core::Config;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| SearchableListDemo {
        basic: cx.new(|cx| {
            SearchableListPanel::new(component_items(), cx)
                .selected("select-search")
                .item_height(px(48.0))
                .width(px(520.0))
                .placeholder("Search components...")
        }),
        filtered: cx.new(|cx| {
            SearchableListPanel::new(component_items(), cx)
                .selected_values(vec!["status-bar", "dock-layout"])
                .item_height(px(48.0))
                .width(px(520.0))
                .placeholder("Type shell, input, button...")
        }),
        limited: cx.new(|cx| {
            SearchableListPanel::new(component_items(), cx)
                .max_items(2)
                .item_height(px(48.0))
                .width(px(520.0))
                .placeholder("Search limited results...")
        }),
        empty: cx.new(|cx| {
            SearchableListPanel::new(component_items(), cx)
                .empty_text("No component found")
                .background(cx.global::<Config>().theme.neutral.hover.opacity(0.5))
                .item_height(px(48.0))
                .width(px(520.0))
                .placeholder("Try a missing keyword...")
        }),
    })
    .into()
}

struct SearchableListDemo {
    basic: gpui::Entity<SearchableListPanel>,
    filtered: gpui::Entity<SearchableListPanel>,
    limited: gpui::Entity<SearchableListPanel>,
    empty: gpui::Entity<SearchableListPanel>,
}

fn component_items() -> Vec<SearchableListItem> {
    vec![
        SearchableListItem::labeled("button", "Button")
            .description("Primary actions and loading states")
            .group("Basic"),
        SearchableListItem::labeled("input", "Input")
            .description("Text entry with prefix/suffix slots")
            .group("Basic"),
        SearchableListItem::labeled("select-search", "Select::searchable")
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
    showcase_card_wide(title, detail, body).into_any_element()
}

impl Render for SearchableListDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "SearchableList 可搜索列表",
            "通用过滤列表底座，统一 value/label/description/group/disabled/selected 等选项能力，供 Select::searchable、命令面板和设置页复用。",
            Space::new().vertical().gap_xl().child(section(
                "SearchableList showcase",
                "过滤、分组、限制和空态按场景纵向展示，避免多列表并排挤压导致阅读和交互混乱。",
                showcase_stack(vec![
                    list_card(
                        "All components",
                        "空查询展示全部分组和禁用项。",
                        self.basic.clone().into_any_element(),
                    ),
                    list_card(
                        "Query: shell",
                        "可命中 value、label、description 或 group。",
                        self.filtered.clone().into_any_element(),
                    ),
                    list_card(
                        "Limited",
                        "只展示前 2 个匹配项。",
                        self.limited.clone().into_any_element(),
                    ),
                    list_card(
                        "Empty",
                        "无匹配项时渲染轻量空态。",
                        self.empty.clone().into_any_element(),
                    ),
                ]),
            )),
        )
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn searchable_list_demo_uses_ordered_wide_cards() {
        let source = include_str!("searchable_list_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();

        assert!(source.contains("showcase_stack"));
        assert!(source.contains("showcase_card_wide"));
        assert!(source.contains("SearchableListPanel::new"));
        assert!(source.contains(r#".placeholder("Search components...")"#));
        assert!(source.contains(".item_height(px(48.0))"));
        assert!(!source.contains("showcase_grid"));
    }
}
