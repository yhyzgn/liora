use aura_components::{Splitter, Text, Title};
use aura_core::Config;
use gpui::{App, Context, Entity, IntoElement, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> Entity<SplitterDemo> {
    cx.new(|_| SplitterDemo)
}

pub struct SplitterDemo;
impl Render for SplitterDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div().flex().flex_col().gap_2()
            .child(Title::new("Splitter 分隔面板").h2())
            .child(Text::new("Left panel + divider + right panel:"))
            .child(
                div().h(px(200.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0))
                    .child(
                        Splitter::new()
                            .left(div().p_2().child(Text::new("Left panel")))
                            .right(div().p_2().child(Text::new("Right panel")))
                    )
            )
    }
}
