use aura_components::{Autocomplete, AutocompleteItem, Card, Space, Text};
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AutocompleteDemo::new(cx)).into()
}

struct AutocompleteDemo {
    basic: Entity<Autocomplete>,
    custom: Entity<Autocomplete>,
    no_suffix: Entity<Autocomplete>,
    disabled: Entity<Autocomplete>,
}

impl AutocompleteDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let suggestions = vec![
            AutocompleteItem::labeled("rust", "Rust"),
            AutocompleteItem::labeled("gpui", "GPUI"),
            AutocompleteItem::labeled("aura", "Aura UI"),
            AutocompleteItem::labeled("element-plus", "Element Plus"),
            AutocompleteItem::labeled("autocomplete", "Autocomplete"),
        ];

        Self {
            basic: cx.new({
                let suggestions = suggestions.clone();
                move |cx| Autocomplete::new(suggestions, cx).placeholder("Search component")
            }),
            custom: cx.new({
                let suggestions = vec![
                    AutocompleteItem::labeled("/dashboard", "Dashboard"),
                    AutocompleteItem::labeled("/settings", "Settings"),
                    AutocompleteItem::labeled("/profile", "Profile"),
                    AutocompleteItem::labeled("/billing", "Billing"),
                ];
                move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("Jump to route")
                        .width_lg()
                        .max_suggestions(4)
                        .suffix_icon(IconName::Command)
                }
            }),
            no_suffix: cx.new({
                let suggestions = suggestions.clone();
                move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("No suffix icon")
                        .no_suffix_icon()
                }
            }),
            disabled: cx.new({
                let suggestions = suggestions.clone();
                move |cx| {
                    Autocomplete::new(suggestions, cx)
                        .placeholder("Disabled")
                        .disabled(true)
                }
            }),
        }
    }
}

impl Render for AutocompleteDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Autocomplete 自动补全",
            "输入时展示匹配建议，点击选项回填输入框。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础用法",
                    "Try: rust, gpui, aura。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.basic.clone())
                        .child(Text::new("Try: rust, gpui, aura")),
                ))
                .child(section(
                    "自定义建议",
                    "Value and label can be different, useful for routes or commands.",
                    Card::new(
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(self.custom.clone())
                            .child(Text::new("Value and label can be different, useful for routes or commands.")),
                    )
                    .no_shadow(),
                ))
                .child(section(
                    "无右侧图标",
                    "Suffix icon can be disabled; clear icon still appears only after typing.",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.no_suffix.clone())
                        .child(Text::new("Suffix icon can be disabled; clear icon still appears only after typing.")),
                ))
                .child(section("禁用状态", "禁用后不可输入或展开建议。", self.disabled.clone())),
        )
    }
}
