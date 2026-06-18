//! Open a basic modal dialog from a button click.

use gpui::IntoElement;
use liora_components::{Button, Dialog, Row, RowJustify, Space, Text};

pub fn open_dialog_button() -> Button {
    Button::new("Open Dialog").primary().on_click(|_, _, cx| {
        Dialog::new()
            .title("Tips")
            .content(|_, _| dialog_body("This is a message from the dialog."))
            .show(cx);
    })
}

fn dialog_body(message: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_lg()
        .child(Text::new(message))
        .child(
            Row::new().justify(RowJustify::End).child(
                Button::new("Close")
                    .primary()
                    .on_click(|_, _, cx| Dialog::close(cx)),
            ),
        )
}
