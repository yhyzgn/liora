use aura_components::{Button, NotificationType, show_notification};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use super::common::{page, row};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| NotificationDemo).into()
}

struct NotificationDemo;

impl Render for NotificationDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Notification 通知",
            "悬浮出现在页面右上角，用于全局展示通知信息。",
            row(vec![
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
                Button::new("Info Notification").on_click(|_, _, cx| {
                    show_notification(
                        "Notification Title",
                        Some("This is the content of the notification".into()),
                        NotificationType::Info,
                        cx,
                    );
                }),
                Button::new("Warning Notification")
                    .warning()
                    .on_click(|_, _, cx| {
                        show_notification("Warning", None, NotificationType::Warning, cx);
                    }),
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
            ]),
        )
    }
}
