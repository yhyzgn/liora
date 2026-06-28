use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::{Divider, Link, Paragraph, Space, Text, TextBlock, Title};

use liora_components::layout_helpers::{page, row, section, showcase_card_wide, showcase_stack};

pub fn render(cx: &mut App) -> Entity<TypographyDemo> {
    cx.new(|_| TypographyDemo)
}

pub struct TypographyDemo;

impl Render for TypographyDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<liora_core::Config>().theme.clone();

        page(
            "Typography 排版",
            "标题、文本、链接和段落组合。Text、Title、Paragraph 默认支持鼠标拖拽自由选中文字。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Title 标题",
                    "内置 h1 到 h6 标题层级；默认可用鼠标拖拽选择标题文字。",
                    Space::new().vertical().gap_sm().children(vec![
                        Title::new("h1. Liora UI Title").h1().into_any_element(),
                        Title::new("h2. Liora UI Title").h2().into_any_element(),
                        Title::new("h3. Liora UI Title").h3().into_any_element(),
                        Title::new("h4. Liora UI Title").h4().into_any_element(),
                        Title::new("h5. Liora UI Title").h5().into_any_element(),
                        Title::new("h6. Liora UI Title").h6().into_any_element(),
                    ]),
                ))
                .child(Divider::new())
                .child(section(
                    "Text 文本",
                    "展示基础文字颜色和大小；默认可拖拽选择，装饰性文字可调用 selectable(false) 关闭。",
                    Space::new().vertical().gap_sm().children(vec![
                        Text::new("Default text color and size").into_any_element(),
                        Text::new("Primary color text")
                            .text_color(theme.primary.base)
                            .into_any_element(),
                        Text::new("Success color text")
                            .text_color(theme.success.base)
                            .into_any_element(),
                        Text::new("Warning color text")
                            .text_color(theme.warning.base)
                            .into_any_element(),
                        Text::new("Danger color text")
                            .text_color(theme.danger.base)
                            .into_any_element(),
                        Text::new("Drag across this sentence with the mouse: Text is selectable by default and supports normal copy shortcuts.")
                            .wrap()
                            .into_any_element(),
                        Text::new("Decorative label with selection disabled")
                            .selectable(false)
                            .text_color(theme.neutral.text_3)
                            .into_any_element(),
                        Text::new("Auto wrap: this is a deliberately long Text component sample. It should wrap automatically within the available container instead of overflowing horizontally.")
                            .wrap()
                            .into_any_element(),
                    ]),
                ))

                .child(Divider::new())
                .child(section(
                    "Text 文档模式",
                    "About、Help、Release notes 等轻量文档也统一由 Text 承载，不再使用单独 TextView 控件。",
                    showcase_stack(vec![
                        showcase_card_wide(
                            "结构化文档块",
                            "直接用 block model 组合标题、段落、引用、列表和代码块。",
                            Text::document([
                                TextBlock::heading(2, "Application bootstrap"),
                                TextBlock::paragraph("Initialize Liora once, then compose native GPUI windows with reusable Liora components."),
                                TextBlock::quote("Text document mode is lightweight; use Docs for full documentation chrome."),
                                TextBlock::unordered([
                                    "Native selectable text",
                                    "Theme-aware quote and code surfaces",
                                    "Copyable code blocks",
                                ]),
                                TextBlock::code("liora_components::init_liora(cx);", "rust"),
                            ])
                            .framed(true)
                        )
                        .into_any_element(),
                        showcase_card_wide(
                            "快速 Markdown 子集",
                            "适合应用内 help/about，不支持完整 CommonMark，也不会引入 WebView。",
                            Text::markdown("# Release notes\n\nLiora renders documents as native GPUI elements.\n\n> Keep SDK docs close to product behavior.\n\n1. Parse a small Markdown subset\n2. Render reusable component blocks\n\n```rust\nText::markdown(markdown)\n```")
                                .framed(true)
                        )
                        .into_any_element(),
                    ]),
                ))
                .child(Divider::new())
                .child(section(
                    "Link 链接",
                    "展示不同状态的文字链接。",
                    row(vec![
                        Link::new("Default Link"),
                        Link::new("Primary Link").primary(),
                        Link::new("Success Link").success(),
                        Link::new("Warning Link").warning(),
                        Link::new("Danger Link").danger(),
                        Link::new("No Underline").underline(false),
                        Link::new("Disabled").disabled(true),
                    ]),
                ))
                .child(Divider::new())
                .child(section(
                    "Paragraph 段落",
                    "支持组合不同 Text 样式，并把整段作为一个连续可选择的文本流。",
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(
                            Paragraph::new()
                                .child(Text::new("Liora UI is a professional "))
                                .child(
                                    Text::new("desktop UI library")
                                        .bold()
                                        .text_color(theme.primary.base),
                                )
                                .child(Text::new(" for Rust, built on top of ").text_color(theme.info.base))
                                .child(Text::new("GPUI").italic().bg(theme.neutral.hover))
                                .child(Text::new(". It provides a comprehensive set of components inspired by "))
                                .child(Text::new("Element Plus").underline())
                                .child(Text::new(", designed to help developers build beautiful and performant native applications. The long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long long Text test wrapping.")),
                        )
                        .child(Paragraph::with_text(
                            "Or use the shorthand with_text for simple cases.",
                        )),
                )),
        )
    }
}
