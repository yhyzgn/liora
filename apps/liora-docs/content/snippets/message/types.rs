//! Toast variants backed by the app-wide Message layer.
//! Initialize Liora, call message::render_messages(cx) from the root render, and mount liora_components::PortalLayer.

use gpui::IntoElement;
use liora_components::{Button, Space, toast_error, toast_info, toast_success, toast_warning};

pub fn message_types() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("toast_info!").on_click(|_, _, _| {
            toast_info!("This is an info toast");
        }))
        .child(Button::new("toast_success!").primary().on_click(|_, _, _| {
            toast_success!("Operation completed");
        }))
        .child(Button::new("toast_warning!").warning().on_click(|_, _, _| {
            toast_warning!("Please check the input");
        }))
        .child(Button::new("toast_error!").danger().on_click(|_, _, _| {
            toast_error!("Operation failed");
        }))
        .child(Button::new("Stack messages").on_click(|_, _, _| {
            toast_info!("Queue alpha");
            toast_success!("Queue beta");
            toast_warning!("Queue gamma");
        }))
}
