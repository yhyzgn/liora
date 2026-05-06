use aura_components::{Loading, Space};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| LoadingDemo).into()
}

struct LoadingDemo;

impl Render for LoadingDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Loading 加载"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("加载数据时显示。"),
                    ),
            )
            .child(
                Space::new()
                    .gap(px(32.0))
                    .child(Loading::new())
                    .child(Loading::new().text("Loading...")),
            )
    }
}
