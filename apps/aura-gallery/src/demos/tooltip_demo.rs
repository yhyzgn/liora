use aura_components::{Button, Space, Tooltip};
use aura_core::{Config, Placement};
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TooltipDemo).into()
}

struct TooltipDemo;

impl Render for TooltipDemo {
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
                            .child("Tooltip 基础用法"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("鼠标悬停在按钮上显示提示信息"),
                    ),
            )
            .child(
                Space::new()
                    .gap(px(16.0))
                    .child(
                        Tooltip::new(Button::new("Top Top Top Top Top Top"))
                            .content("Prompt info")
                            .placement(Placement::Top),
                    )
                    .child(
                        Tooltip::new(Button::new("Bottom"))
                            .content("Prompt info")
                            .placement(Placement::Bottom),
                    )
                    .child(
                        Tooltip::new(Button::new("Left"))
                            .content("Prompt info")
                            .placement(Placement::Left),
                    )
                    .child(
                        Tooltip::new(Button::new("Right"))
                            .content("Prompt info")
                            .placement(Placement::Right),
                    ),
            )
            .child(
                div().flex().flex_col().gap_2().child(
                    div()
                        .text_lg()
                        .font_weight(gpui::FontWeight::BOLD)
                        .child("更多方位"),
                ),
            )
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .child(
                        Tooltip::new(Button::new("Top Start"))
                            .content("Top Start")
                            .placement(Placement::TopStart),
                    )
                    .child(
                        Tooltip::new(Button::new("Top End"))
                            .content("Top End")
                            .placement(Placement::TopEnd),
                    )
                    .child(
                        Tooltip::new(Button::new("Bottom Start"))
                            .content("Bottom Start")
                            .placement(Placement::BottomStart),
                    )
                    .child(
                        Tooltip::new(Button::new("Bottom End"))
                            .content("Bottom End")
                            .placement(Placement::BottomEnd),
                    ),
            )
    }
}
