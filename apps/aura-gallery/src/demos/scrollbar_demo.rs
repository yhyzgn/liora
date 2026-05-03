use aura_components::{Scrollbar, Text, Title};
use aura_core::Config;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(ScrollbarDemo).into_any_element() }

struct ScrollbarDemo;
impl RenderOnce for ScrollbarDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let items: Vec<String> = (1..=5).map(|i| format!("Line {}", i)).collect();

        div().flex().flex_col().gap_2()
            .child(Title::new("Scrollbar 滚动条").h2())
            .child(Text::new("Fixed height with native scroll:"))
            .child(
                div().h(px(200.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0))
                    .child(Scrollbar::new().height(px(200.0))
                        .child(div().flex().flex_col().p_2().children(
                            items.iter().map(|s| div().py_1().child(s.clone()))
                        ))
                    )
            )
    }
}
