//! MessageBox helper APIs.

use gpui::IntoElement;
use liora_components::{Button, Space, alert, confirm, toast_success};

pub fn message_box_basic() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Open Alert").on_click(|_, _, cx| {
            alert("Alert Title", "This is an alert message.", cx);
        }))
        .child(Button::new("Open Confirm").primary().on_click(|_, _, cx| {
            confirm(
                "Confirm Title",
                "Are you sure you want to proceed?",
                |_, _| toast_success!("Confirmed"),
                cx,
            );
        }))
}
