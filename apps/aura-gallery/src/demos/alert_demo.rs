use aura_components::{Alert, AlertType};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AlertDemo).into()
}

struct AlertDemo;

impl Render for AlertDemo {
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
                            .child("Alert 警告"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于页面中展示重要的提示信息。"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(Alert::new("Info Alert").alert_type(AlertType::Info))
                    .child(Alert::new("Success Alert").alert_type(AlertType::Success))
                    .child(Alert::new("Warning Alert").alert_type(AlertType::Warning))
                    .child(Alert::new("Error Alert").alert_type(AlertType::Error)),
            )
            .child(
                div().flex().flex_col().gap_2().child(
                    div()
                        .text_lg()
                        .font_weight(gpui::FontWeight::BOLD)
                        .child("辅助性文字"),
                ),
            )
            .child(
                Alert::new("Warning")
                    .alert_type(AlertType::Warning)
                    .description("More detailed description of the warning."),
            )
    }
}
