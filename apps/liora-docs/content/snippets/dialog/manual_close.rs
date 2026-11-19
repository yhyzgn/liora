//! Disable overlay and ESC dismissal; the title close control remains available.

use liora_components::{Button, Dialog, Row, RowJustify, Space, Text};

pub fn manual_close_dialog_button() -> Button {
    Button::new("Manual Close Only")
        .warning()
        .on_click(|_, _, cx| {
            Dialog::new()
                .title("Manual close dialog")
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content(|_, _| {
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(Text::new(
                            "遮罩和 ESC 不会关闭；标题关闭按钮或业务按钮仍可关闭。",
                        ))
                        .child(
                            Row::new().justify(RowJustify::End).child(
                                Button::new("I understand")
                                    .primary()
                                    .on_click(|_, _, cx| Dialog::close(cx)),
                            ),
                        )
                })
                .show(cx);
        })
}
