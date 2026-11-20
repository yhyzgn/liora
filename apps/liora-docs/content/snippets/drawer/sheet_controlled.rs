//! Controlled Drawer sheet-style close behavior example.

use liora_components::{Button, Drawer, Space, Text};

pub fn controlled_drawer_sheet_button() -> Button {
    Button::new("Open blocking review")
        .primary()
        .on_click(|_, _, cx| {
            const REVIEW_ID: &str = "blocking-review";
            Drawer::sheet()
                .id(REVIEW_ID)
                .title("Blocking review")
                .width_lg()
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content_view(|_| {
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new("Explicit close only").bold())
                        .child(Text::new("Cancel or apply to close this panel."))
                        .child(
                            Space::new()
                                .gap_sm()
                                .child(
                                    Button::new("Cancel")
                                        .on_click(|_, _, cx| Drawer::close_id(REVIEW_ID, cx)),
                                )
                                .child(
                                    Button::new("Apply")
                                        .primary()
                                        .on_click(|_, _, cx| Drawer::close_id(REVIEW_ID, cx)),
                                ),
                        )
                })
                .show(cx);
        })
}
