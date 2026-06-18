use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::{Divider, Link, Paragraph, Space, Text, Title};

use liora_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> Entity<TypographyDemo> {
    cx.new(|_| TypographyDemo)
}

pub struct TypographyDemo;

impl Render for TypographyDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<liora_core::Config>().theme.clone();

        page(
            "Typography 排版",
            "标题、文本、链接和段落组合。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "Title 标题",
                    "内置 h1 到 h6 标题层级。",
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
                    "展示基础文字颜色和大小。",
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
                        Text::new("Auto wrap: this is a deliberately long Text component sample. It should wrap automatically within the available container instead of overflowing horizontally.")
                            .wrap()
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
                    "支持组合不同 Text 样式。",
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
