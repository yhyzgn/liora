use aura_components::{Divider, Space, Text, Title};
use aura_core::Config;
use gpui::{AnyElement, App, Component, IntoElement, RenderOnce, Window, div, prelude::*, px};

pub fn render() -> AnyElement { Component::new(LayoutDemo).into_any_element() }

struct LayoutDemo;
impl RenderOnce for LayoutDemo {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        let mut page = div().flex().flex_col().gap_3();

        page = page.child(Title::new("Divider 分割线").h2());
        page = page.child(Text::new("Horizontal (default)"));
        page = page.child(Divider::new());
        page = page.child(Text::new("With label"));
        page = page.child(Divider::new().label("Center Text"));
        page = page.child(Text::new("Vertical"));
        page = page.child(
            div().flex().flex_row().h(px(60.0)).gap_4()
                .child(Text::new("Left"))
                .child(Divider::new().vertical())
                .child(Text::new("Right"))
        );

        page = page.child(Title::new("Space 间距").h2());
        page = page.child(Text::new("Horizontal gap 16px:"));
        page = page.child(
            div().flex().flex_row().items_center()
                .child(div().px_2().py_1().bg(theme.primary.base).rounded(px(4.0)).child("A"))
                .child(Space::horizontal(16.0))
                .child(div().px_2().py_1().bg(theme.primary.base).rounded(px(4.0)).child("B"))
        );
        page = page.child(Text::new("Vertical gap 8px:"));
        page = page.child(
            div().flex().flex_col()
                .child(div().px_2().py_1().bg(theme.success.base).rounded(px(4.0)).child("Top"))
                .child(Space::vertical(8.0))
                .child(div().px_2().py_1().bg(theme.success.base).rounded(px(4.0)).child("Bottom"))
        );

        page
    }
}
