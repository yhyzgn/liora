use aura_components::Collapse;
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| CollapseDemo {
        basic: cx.new(|_| {
            Collapse::new()
                .item("item1", "Consistency", |_, _| {
                    div().child("Consistent with real life: in line with the process and intuition of real life.")
                })
                .item("item2", "Feedback", |_, _| {
                    div().child("Operation feedback: enable the users to clearly perceive their operations by style updates.")
                })
        }),
        accordion: cx.new(|_| {
            Collapse::new()
                .accordion()
                .item("item1", "Consistency", |_, _| {
                    div().child("Consistent with real life: in line with the process and intuition of real life.")
                })
                .item("item2", "Feedback", |_, _| {
                    div().child("Operation feedback: enable the users to clearly perceive their operations by style updates.")
                })
        }),
    })
    .into()
}

struct CollapseDemo {
    basic: Entity<Collapse>,
    accordion: Entity<Collapse>,
}

impl Render for CollapseDemo {
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
                            .child("Collapse 折叠面板"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("通过折叠面板收纳内容区域。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(self.basic.clone())
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("手风琴模式 (Accordion)"),
                    )
                    .child(self.accordion.clone()),
            )
    }
}
