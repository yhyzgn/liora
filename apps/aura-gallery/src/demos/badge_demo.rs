use aura_components::{Avatar, Badge, BadgeType, Button};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| BadgeDemo).into()
}

struct BadgeDemo;

impl Render for BadgeDemo {
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
                            .child("Badge 徽章"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("按钮和图标右上角的提示信息。"),
                    ),
            )
            // Basic Usage
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_8()
                            .child(Badge::new(Button::new("Messages")).value("5"))
                            .child(
                                Badge::new(Button::new("Updates"))
                                    .value("10")
                                    .badge_type(BadgeType::Primary),
                            )
                            .child(
                                Badge::new(Button::new("Alerts"))
                                    .value("2")
                                    .badge_type(BadgeType::Warning),
                            ),
                    ),
            )
            // Max Value
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("最大值"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_8()
                            .child(Badge::new(Button::new("Messages")).value("200").max(99))
                            .child(Badge::new(Button::new("Updates")).value("50").max(10)),
                    ),
            )
            // Dot
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("小红点"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_8()
                            .child(Badge::new(div().child("Query")).is_dot(true))
                            .child(Badge::new(Avatar::new()).is_dot(true)),
                    ),
            )
    }
}
