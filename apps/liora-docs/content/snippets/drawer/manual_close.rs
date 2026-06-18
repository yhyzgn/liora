//! Disable overlay and ESC dismissal and close from content explicitly.

use liora_components::{Button, Drawer, Space, Text};

pub fn manual_close_drawer_button() -> Button {
    Button::new("Manual Close Only")
        .warning()
        .on_click(|_, _, cx| {
            Drawer::new()
                .title("Manual close drawer")
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content(|_, _| {
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(Text::new("点击遮罩和按 ESC 都不会关闭。"))
                        .child(
                            Button::new("Close Drawer")
                                .primary()
                                .on_click(|_, _, cx| Drawer::close(cx)),
                        )
                })
                .show(cx);
        })
}
