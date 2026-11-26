use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_stack};
use liora_components::{Card, Divider, Flex, Scrollbar, Space, Tag, Text};
use liora_core::Config;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| ScrollbarDemo {
        basic: cx.new(basic_scrollbar),
        cards: cx.new(card_scrollbar),
        article: cx.new(article_scrollbar),
        compact: cx.new(compact_scrollbar),
    })
    .into()
}

struct ScrollbarDemo {
    basic: Entity<Scrollbar>,
    cards: Entity<Scrollbar>,
    article: Entity<Scrollbar>,
    compact: Entity<Scrollbar>,
}

impl Render for ScrollbarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Scrollbar 滚动条",
            "自举原生 GPUI 滚动容器：支持固定视口、长内容、卡片流和精确可拖拽滚动条。hover 滚动条会加宽，整条轨道都可命中。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "滚动场景",
                    "不同内容密度下 thumb 高度和位置应保持稳定；拖拽、滚轮和 hover 加宽都应自然。",
                    showcase_stack(vec![
                        showcase_card_wide(
                            "基础长列表",
                            "短视口 + 多行文本，适合菜单、日志和设置项列表。",
                            self.basic.clone(),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "卡片流",
                            "滚动内容内部可以继续组合 Card、Tag、Flex 等复杂组件。",
                            self.cards.clone(),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "长段落阅读",
                            "正文内容可换行、可选择，同时保持滚动条坐标准确。",
                            self.article.clone(),
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "紧凑高度",
                            "小高度视口用于弹窗、下拉面板等空间受限区域。",
                            self.compact.clone(),
                        )
                        .into_any_element(),
                    ]),
                ))
                .child(Divider::new())
                .child(Text::new("提示：将鼠标移到右侧滚动条轨道上，thumb 会变宽；也可以直接在轨道区域按住拖动。").wrap()),
        )
    }
}

fn basic_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, cx| {
        let theme = cx.global::<Config>().theme.clone();
        Flex::new()
            .column()
            .gap_md()
            .padding_md()
            .children((1..=32).map(move |i| {
                Flex::new()
                    .row()
                    .align_center()
                    .justify_between()
                    .padding_sm()
                    .rounded_units(4.0)
                    .bg(if i % 2 == 0 {
                        theme.neutral.hover
                    } else {
                        theme.neutral.card
                    })
                    .child(Text::new(format!("Scrollable line {:02}", i)))
                    .child(Tag::new(if i % 3 == 0 { "active" } else { "idle" }).info())
            }))
            .into_any_element()
    })
    .height(220.0)
}

fn card_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        Flex::new()
            .column()
            .gap_md()
            .padding_md()
            .children((1..=12).map(move |i| {
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Text::new(format!("Workflow card #{:02}", i)).bold())
                        .child(Text::new("Cards inside Scrollbar keep their own padding, border and theme-aware surfaces.").wrap())
                        .child(
                            Space::new()
                                .gap_xs()
                                .wrap()
                                .child(Tag::new("scroll").success())
                                .child(Tag::new("native").info())
                                .child(Tag::new("gpui").warning()),
                        ),
                )
                .id(("workflow-card", i as usize))
                .no_shadow()
            }))
            .into_any_element()
    })
    .height(300.0)
}

fn article_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        Flex::new()
            .column()
            .gap_lg()
            .padding_md()
            .children((1..=8).map(|i| {
                Space::new()
                    .vertical()
                    .gap_xs()
                    .child(Text::new(format!("Section {}", i)).bold())
                    .child(Text::new("Liora Scrollbar wraps native GPUI content without WebView or DOM. Long paragraphs can wrap naturally, remain selectable, and still report a stable scroll extent for the draggable thumb.").wrap())
            }))
            .into_any_element()
    })
    .height(260.0)
}

fn compact_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        Flex::new()
            .column()
            .gap_sm()
            .padding_sm()
            .children((1..=18).map(|i| Text::new(format!("Compact item {}", i))))
            .into_any_element()
    })
    .height(120.0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn scrollbar_demo_covers_multiple_content_shapes() {
        let source = include_str!("scrollbar_demo.rs");

        assert!(source.contains("basic: Entity<Scrollbar>"));
        assert!(source.contains("cards: Entity<Scrollbar>"));
        assert!(source.contains("article: Entity<Scrollbar>"));
        assert!(source.contains("compact: Entity<Scrollbar>"));
        assert!(source.contains("hover 滚动条会加宽"));
        assert!(source.contains("card_scrollbar"));
        assert!(source.contains("article_scrollbar"));
    }
}
