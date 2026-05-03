use aura_components::{Container, Text, Title};
use aura_core::Config;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(ContainerDemo).into_any_element() }

struct ContainerDemo;
impl RenderOnce for ContainerDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let c = |s: &str| div().px_2().py_1().child(s.to_string());

        div().flex().flex_col().gap_4()
            .child(Title::new("Full layout").h2())
            .child(div().h(px(200.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0)).child(
                Container::new()
                    .header(c("Header"))
                    .aside(c("Aside"))
                    .child(div().p_2().child(Text::new("Main content area")))
                    .footer(c("Footer"))
            ))
            .child(Title::new("Aside right").h2())
            .child(div().h(px(200.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0)).child(
                Container::new()
                    .header(c("Header"))
                    .aside(c("Aside")).aside_right()
                    .child(div().p_2().child(Text::new("Main with right sidebar")))
                    .footer(c("Footer"))
            ))
            .child(Title::new("Header + Main only").h2())
            .child(div().h(px(150.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0)).child(
                Container::new()
                    .header(c("Header"))
                    .child(div().p_2().child(Text::new("Simple header + content")))
            ))
            .child(Title::new("Main + Footer only").h2())
            .child(div().h(px(150.0)).border_1().border_color(theme.neutral.border).rounded(px(4.0)).child(
                Container::new()
                    .child(div().p_2().child(Text::new("Content with footer")))
                    .footer(c("Footer"))
            ))
    }
}
