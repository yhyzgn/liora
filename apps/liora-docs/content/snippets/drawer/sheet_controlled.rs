//! Controlled Drawer sheet-style close behavior example.

use liora_components::{Button, Drawer, Space, Text};

pub fn controlled_drawer_sheet_button() -> Button {
    Button::new("Open blocking review")
        .primary()
        .on_click(|_, _, cx| {
            Drawer::sheet()
                .id("blocking-review")
                .title("Blocking review")
                .width_lg()
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content_view(|_| {
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new(
                            "Close this panel from an action or Drawer::close_id.",
                        ))
                        .child(
                            Button::new("Close")
                                .on_click(|_, _, cx| Drawer::close_id("blocking-review", cx)),
                        )
                })
                .show(cx);
        })
}
