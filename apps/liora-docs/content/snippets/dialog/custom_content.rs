//! Dialog content accepts any native Liora elements.

use liora_components::{Button, Dialog, Row, RowJustify, Space, Text};

pub fn custom_content_dialog_button() -> Button {
    Button::new("Form-like Content").on_click(|_, _, cx| {
        Dialog::new()
            .title("Edit profile")
            .content(|_, _| {
                Space::new()
                    .vertical()
                    .gap_md()
                    .child(Text::new("Name: Liora User"))
                    .child(Text::new("Role: Designer"))
                    .child(
                        Row::new()
                            .justify(RowJustify::End)
                            .child(Button::new("Cancel").on_click(|_, _, cx| Dialog::close(cx)))
                            .child(
                                Button::new("Save")
                                    .primary()
                                    .on_click(|_, _, cx| Dialog::close(cx)),
                            ),
                    )
            })
            .show(cx);
    })
}
