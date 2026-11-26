//! Toast macros support standard format! arguments.
//! Initialize Liora, call message::render_messages(cx) from the root render, and mount liora_components::PortalLayer.

use gpui::IntoElement;
use liora_components::{Button, Space, toast_info, toast_success};

pub fn message_formatting() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Button::new("位置参数").on_click(|_, _, _| {
            let name = "Liora";
            let count = 4;
            toast_info!("{}, you have {} toast variants.", name, count);
        }))
        .child(Button::new("命名参数").primary().on_click(|_, _, _| {
            let component = "Message";
            let api = "toast_success!";
            toast_success!("{component} macro {api} works.");
        }))
}
