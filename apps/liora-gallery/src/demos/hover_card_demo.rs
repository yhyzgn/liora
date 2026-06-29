use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_grid};
use liora_components::{HoverCard, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| HoverCardDemo).into()
}

struct HoverCardDemo;

impl Render for HoverCardDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "HoverCard 悬停卡片",
            "基于 Popover 的轻量预览 facade，适合 profile、链接预览和上下文摘要。",
            Space::new().vertical().gap_xl().child(section(
                "基础预览",
                "悬停或触发目标后展示更丰富的说明内容。",
                showcase_grid(vec![
                    HoverCard::new(Text::new("Open profile").underline())
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Neo").bold())
                                .child(Text::new("Maintainer · Native GPUI components"))
                        })
                        .into_any_element(),
                    HoverCard::new(Text::new("Preview link").underline())
                        .content(|_, _| {
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Text::new("Release notes").bold())
                                .child(Text::new("Show a concise summary before navigation."))
                        })
                        .into_any_element(),
                ]),
            )),
        )
    }
}
