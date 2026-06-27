use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Combobox, SearchableListItem, Space, Text, combobox_create_footer};
use liora_core::Config;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| ComboboxDemo::new(cx)).into()
}

struct ComboboxDemo {
    basic: gpui::Entity<Combobox>,
    grouped: gpui::Entity<Combobox>,
    multiple: gpui::Entity<Combobox>,
    footer: gpui::Entity<Combobox>,
}

impl ComboboxDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx
                .new(|cx| Combobox::new(framework_items(), cx).placeholder("Choose framework")),
            grouped: cx.new(|cx| {
                Combobox::new(component_items(), cx)
                    .placeholder("Search components")
                    .width(px(340.0))
            }),
            multiple: cx.new(|cx| {
                Combobox::new(component_items(), cx)
                    .multiple()
                    .selected_values(vec!["button", "combobox"])
                    .placeholder("Pick multiple components")
                    .width(px(340.0))
            }),
            footer: cx.new(|cx| {
                Combobox::new(component_items(), cx)
                    .placeholder("Create or select")
                    .width(px(340.0))
                    .footer(|_, _| combobox_create_footer("Create component").into_any_element())
            }),
        }
    }
}

fn framework_items() -> Vec<SearchableListItem> {
    vec![
        SearchableListItem::labeled("gpui", "GPUI").description("Native Rust UI runtime"),
        SearchableListItem::labeled("liora", "Liora").description("Component SDK on official GPUI"),
        SearchableListItem::labeled("iced", "Iced").description("Cross-platform Rust GUI"),
        SearchableListItem::labeled("egui", "egui").description("Immediate mode GUI"),
    ]
}

fn component_items() -> Vec<SearchableListItem> {
    vec![
        SearchableListItem::labeled("button", "Button").group("Basic"),
        SearchableListItem::labeled("input", "Input").group("Basic"),
        SearchableListItem::labeled("select", "Select").group("Input"),
        SearchableListItem::labeled("combobox", "Combobox").group("Input"),
        SearchableListItem::labeled("sidebar", "Sidebar").group("Shell"),
        SearchableListItem::labeled("status-bar", "StatusBar").group("Shell"),
        SearchableListItem::labeled("dock-layout", "DockLayout")
            .group("Shell")
            .disabled(true),
    ]
}

fn demo_panel(
    title: &'static str,
    detail: &'static str,
    child: impl IntoElement,
) -> impl IntoElement {
    div()
        .w(px(400.0))
        .min_h(px(132.0))
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
        .child(child)
}

impl Render for ComboboxDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        page(
            "Combobox 组合选择器",
            "可搜索的单选/多选下拉，支持分组选项、禁用项、空态和 footer 扩展。它复用 SearchableList，避免每个选择类控件重复实现过滤逻辑。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础与分组",
                    "基础用法适合短选项，分组用法适合组件、项目、成员等较长列表。",
                    Space::new()
                        .wrap()
                        .gap_lg()
                        .child(demo_panel(
                            "Single select",
                            "点击输入框后搜索并选择一个框架。",
                            self.basic.clone(),
                        ))
                        .child(demo_panel(
                            "Grouped options",
                            "按组件类型分组，禁用项保持可见但不可选。",
                            self.grouped.clone(),
                        )),
                ))
                .child(section(
                    "多选与 footer",
                    "多选会保留多个值，footer 可承载创建、管理或高级筛选入口。",
                    Space::new()
                        .wrap()
                        .gap_lg()
                        .child(demo_panel(
                            "Multiple",
                            "再次点击已选项可取消选择。",
                            self.multiple.clone(),
                        ))
                        .child(demo_panel(
                            "Footer action",
                            "Footer slot 使用 Liora Button，可放新增操作。",
                            self.footer.clone(),
                        )),
                ))
                .child(section(
                    "组合说明",
                    "Combobox 不替代 Select：固定少量选项继续用 Select；需要输入过滤、分组或创建入口时使用 Combobox。",
                    Space::new()
                        .gap_sm()
                        .wrap()
                        .child(Button::new("Searchable").primary().icon_start(IconName::Search))
                        .child(Button::new("Grouped").icon_start(IconName::Layers))
                        .child(Button::new("Theme aware").icon_start(IconName::Palette))
                        .child(Text::new("Popup surfaces use current light/dark theme tokens.").sm().text_color(theme.neutral.text_3)),
                )),
        )
    }
}
