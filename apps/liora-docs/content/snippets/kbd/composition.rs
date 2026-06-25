//! Kbd composition examples.

use gpui::{IntoElement, rgb};
use liora_components::{Kbd, Space};

pub fn kbd_composition() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Kbd::new("Esc").color(rgb(0xdc2626).into()))
        .child(
            Kbd::new("⌘S")
                .bg(rgb(0xdcfce7).into())
                .color(rgb(0x166534).into()),
        )
        .child(
            Space::new()
                .gap_xs()
                .child(Kbd::new("⌘"))
                .child(Kbd::new("K")),
        )
}
