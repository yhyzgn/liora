use aura_components::{Button, NotificationType, Space, show_notification};
use aura_core::Config;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| NotificationDemo).into()
}

struct NotificationDemo;

impl Render for NotificationDemo {
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
                            .child("Notification 通知"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("悬浮出现在页面右上角，用于全局展示通知信息。"),
                    ),
            )
            .child(
                Space::new()
                    .gap(px(16.0))
                    .child(
                        Button::new("Success Notification")
                            .primary()
                            .on_click(|_, _, cx| {
                                show_notification(
                                    "Success",
                                    Some("This is a success description".into()),
                                    NotificationType::Success,
                                    cx,
                                );
                            }),
                    )
                    .child(Button::new("Info Notification").on_click(|_, _, cx| {
                        show_notification(
                            "Notification Title",
                            Some("This is the content of the notification".into()),
                            NotificationType::Info,
                            cx,
                        );
                    }))
                    .child(
                        Button::new("Warning Notification")
                            .warning()
                            .on_click(|_, _, cx| {
                                show_notification("Warning", None, NotificationType::Warning, cx);
                            }),
                    )
                    .child(
                        Button::new("Error Notification")
                            .danger()
                            .on_click(|_, _, cx| {
                                show_notification(
                                    "Error",
                                    Some("Detailed error message goes here".into()),
                                    NotificationType::Error,
                                    cx,
                                );
                            }),
                    ),
            )
    }
}
