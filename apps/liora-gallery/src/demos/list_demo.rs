use gpui::{AnyView, App, Context, Render, Window, prelude::*, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Card, List, ListItem, ListMarker, OrderedCounterStyle, OrderedMarker, Space, Text,
};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ListDemo).into()
}

struct ListDemo;

impl Render for ListDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "List 列表",
            "有序 / 无序内容列表，支持嵌套层级、每级 marker 策略和单项覆盖。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "无序列表",
                    "默认每一级使用不同的指示器：disc、circle、square、dash。",
                    Card::new(unordered_list()),
                ))
                .child(section(
                    "自定义无序 marker",
                    "每一级 marker 可统一配置，单个 item 也可以覆盖。",
                    Card::new(custom_unordered_list()),
                ))
                .child(section(
                    "有序列表",
                    "默认逐级使用 decimal、lower-alpha、lower-roman 和 leading-zero。",
                    Card::new(ordered_list()),
                ))
                .child(section(
                    "自定义有序格式",
                    "每一级可以指定 counter 类型和 pattern，pattern 中的 {n} 会替换为计数值。",
                    custom_ordered_list(),
                )),
        )
    }
}

fn unordered_list() -> List {
    List::unordered()
        .item(
            ListItem::new("Prepare component API")
                .child(ListItem::new("Define row spacing and marker width"))
                .child(
                    ListItem::new("Document nested behavior")
                        .child(ListItem::new("Explain default level markers"))
                        .child(ListItem::new("Show item-level overrides")),
                ),
        )
        .item(ListItem::new("Build Gallery examples"))
        .item(ListItem::new("Sync Docs snippets"))
}

fn custom_unordered_list() -> List {
    List::unordered()
        .unordered_markers([
            ListMarker::Check,
            ListMarker::Star,
            ListMarker::Text("◆".into()),
        ])
        .marker_colors([
            rgb(0x16a34a).into(),
            rgb(0xf59e0b).into(),
            rgb(0x7c3aed).into(),
        ])
        .item(
            ListItem::new("Design tokens applied")
                .marker(ListMarker::Check)
                .marker_color(rgb(0x2563eb).into()),
        )
        .item(ListItem::new("Theme-aware marker colors"))
        .item(
            ListItem::new("Nested custom levels")
                .child(ListItem::new("Stars for highlighted children"))
                .child(ListItem::new("Text markers for deep notes")),
        )
}

fn ordered_list() -> List {
    List::ordered()
        .item(
            ListItem::new("Install Liora")
                .child(ListItem::new("Add the liora crate"))
                .child(ListItem::new("Pin the matching official GPUI rev")),
        )
        .item(
            ListItem::new("Initialize the app")
                .child(ListItem::new("Call init_liora(cx)"))
                .child(ListItem::new("Render portals near the root")),
        )
        .item(ListItem::new("Compose components"))
}

fn custom_ordered_list() -> Space {
    Space::new()
        .vertical()
        .gap_lg()
        .child(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Text::new("Phase workflow").bold())
                    .child(
                        List::ordered()
                            .start(3)
                            .marker_width(gpui::px(78.0))
                            .indent(4)
                            .marker_colors([
                                rgb(0x2563eb).into(),
                                rgb(0x16a34a).into(),
                                rgb(0xf97316).into(),
                            ])
                            .ordered_markers([
                                OrderedMarker::new(OrderedCounterStyle::UpperAlpha, "Phase {n}"),
                                OrderedMarker::new(OrderedCounterStyle::DecimalLeadingZero, "{n}/"),
                                OrderedMarker::new(OrderedCounterStyle::LowerRoman, "({n})"),
                            ])
                            .item(ListItem::element(Text::new("Audit current UI").bold()))
                            .item(
                                ListItem::new("Implement component")
                                    .child(ListItem::new("Write SDK API"))
                                    .child(ListItem::new("Add Gallery and Docs")),
                            )
                            .item(ListItem::new("Verify and publish notes")),
                    ),
            )
            .no_shadow(),
        )
        .child(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Text::new("Document outline").bold())
                    .child(
                        List::ordered()
                            .marker_width(gpui::px(54.0))
                            .marker_colors([
                                rgb(0x7c3aed).into(),
                                rgb(0x0891b2).into(),
                                rgb(0xdb2777).into(),
                            ])
                            .ordered_markers([
                                OrderedMarker::new(OrderedCounterStyle::Decimal, "§ {n}"),
                                OrderedMarker::new(OrderedCounterStyle::UpperRoman, "{n}."),
                                OrderedMarker::new(OrderedCounterStyle::LowerAlpha, "{n})"),
                            ])
                            .item(
                                ListItem::new("Quick Start")
                                    .child(ListItem::new("Install dependencies"))
                                    .child(ListItem::new("Open the first window")),
                            )
                            .item(ListItem::new("Component catalog"))
                            .item(ListItem::new("Release checklist")),
                    ),
            )
            .no_shadow(),
        )
        .child(
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Text::new("Task runbook").bold())
                    .child(
                        List::ordered()
                            .marker_width(gpui::px(64.0))
                            .ordered_markers([
                                OrderedMarker::new(
                                    OrderedCounterStyle::DecimalLeadingZero,
                                    "T-{n}",
                                ),
                                OrderedMarker::new(OrderedCounterStyle::LowerAlpha, "[{n}]"),
                                OrderedMarker::new(OrderedCounterStyle::LowerRoman, "step {n}"),
                            ])
                            .item(
                                ListItem::new("Prepare")
                                    .child(ListItem::new("Lock requirements"))
                                    .child(ListItem::new("Create regression checks")),
                            )
                            .item(ListItem::new("Implement"))
                            .item(ListItem::new("Verify")),
                    ),
            )
            .no_shadow(),
        )
}

#[cfg(test)]
mod tests {
    #[test]
    fn list_demo_covers_ordered_unordered_and_marker_customization() {
        let source = include_str!("list_demo.rs");
        assert!(source.contains("List::unordered()"));
        assert!(source.contains("List::ordered()"));
        assert!(source.contains("unordered_markers"));
        assert!(source.contains("ordered_markers"));
        assert!(source.contains("OrderedCounterStyle::UpperAlpha"));
        assert!(source.contains("marker_colors"));
        assert!(source.contains("marker_color"));
        assert!(source.contains("ListMarker::Check"));
    }
}
