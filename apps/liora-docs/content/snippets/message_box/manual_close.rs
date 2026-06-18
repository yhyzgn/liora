//! MessageBox manual close policy.

use gpui::IntoElement;
use liora_components::{Button, MessageBox, Space};

pub fn message_box_manual_close() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("Manual Alert").warning().on_click(|_, _, cx| {
            MessageBox::new(
                "Manual Alert",
                "Only the OK button can close this message box.",
            )
            .close_on_click_outside(false)
            .close_on_escape(false)
            .alert(cx);
        }))
        .child(Button::new("Manual Confirm").danger().on_click(|_, _, cx| {
            MessageBox::new(
                "Manual Confirm",
                "Only Cancel or Confirm can close this message box.",
            )
            .close_on_click_outside(false)
            .close_on_escape(false)
            .confirm(|_, _| {}, cx);
        }))
}
