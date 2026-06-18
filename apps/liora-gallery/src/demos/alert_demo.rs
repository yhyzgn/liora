use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Alert, AlertType, Space, Title};

use liora_components::layout_helpers::page;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AlertDemo).into()
}

struct AlertDemo;

impl Render for AlertDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Alert 警告",
            "用于页面中展示重要的提示信息。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Alert::new("Info Alert").alert_type(AlertType::Info))
                        .child(Alert::new("Success Alert").alert_type(AlertType::Success))
                        .child(Alert::new("Warning Alert").alert_type(AlertType::Warning))
                        .child(Alert::new("Error Alert").alert_type(AlertType::Error)),
                )
                .child(Title::new("辅助性文字").h3())
                .child(
                    Alert::new("Warning")
                        .alert_type(AlertType::Warning)
                        .description("More detailed description of the warning."),
                ),
        )
    }
}
