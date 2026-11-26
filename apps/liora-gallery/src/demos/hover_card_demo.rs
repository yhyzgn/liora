use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Space;
use liora_components::layout_helpers::{page, section};

#[path = "../../../liora-docs/content/snippets/hover_card/basic.rs"]
mod hover_card_basic_example;

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
                "悬停后可移入卡片阅读；Tab 聚焦按钮，Enter/Space 打开，Escape 关闭。悬停不抢走编辑焦点。",
                hover_card_basic_example::hover_card_basic(),
            )),
        )
    }
}
