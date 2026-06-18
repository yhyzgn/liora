//! Disable overlay and ESC close when business flow must decide dismissal.

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
                            "点击遮罩和按 ESC 都不会关闭，只能点击按钮手动关闭。",
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
