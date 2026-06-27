//! Controlled Sheet close behavior example.

use gpui::IntoElement;
use liora_components::{Button, Sheet, Space, Text};
use liora_icons_lucide::IconName;

pub fn sheet_controlled() -> impl IntoElement {
    Button::new("Open blocking review")
        .primary()
        .icon_start(IconName::ShieldCheck)
        .on_click(|_, _, cx| {
            Sheet::new()
                .id("blocking-review")
                .title("Blocking review")
                .width_lg()
                .close_on_click_outside(false)
                .close_on_escape(false)
                .content(|_| {
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new("Explicit close only").bold())
                        .child(Text::new(
                            "Close this sheet from an action or Sheet::close_id.",
                        ))
                })
                .show(cx);
        })
}
