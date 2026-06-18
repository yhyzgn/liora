//! Notification feedback types.

use gpui::IntoElement;
use liora_components::{Button, NotificationType, Space, show_notification};

pub fn notification_types() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
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
                Some("This is the content".into()),
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
        )
}
