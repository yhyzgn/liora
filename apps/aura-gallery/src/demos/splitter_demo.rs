use aura_components::{Card, Splitter, Text};
use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> Entity<SplitterDemo> {
    cx.new(|_| SplitterDemo)
}

pub struct SplitterDemo;
impl Render for SplitterDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Splitter 分隔面板",
            "左右面板分割。",
            section(
                "基础用法",
                "Left panel + divider + right panel.",
                Splitter::new()
                    .height_md()
                    .bordered()
                    .left(Card::new(Text::new("Left panel")).no_shadow())
                    .right(Card::new(Text::new("Right panel")).no_shadow()),
            ),
        )
    }
}
