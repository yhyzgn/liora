//! Basic Popconfirm actions.

use gpui::IntoElement;
use liora_components::{Button, Popconfirm, Space, toast_success, toast_warning};

pub fn basic_popconfirm() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(
            Popconfirm::new(Button::new("Delete").danger())
                .id("docs-popconfirm-delete")
                .title("Are you sure to delete this task?")
                .on_confirm(|_, _| toast_success!("Deleted"))
                .on_cancel(|_, _| toast_warning!("Cancelled")),
        )
        .child(
            Popconfirm::new(Button::new("Archive"))
                .id("docs-popconfirm-archive")
                .title("Archive this item?")
                .confirm_text("Yes")
                .cancel_text("No"),
        )
}
