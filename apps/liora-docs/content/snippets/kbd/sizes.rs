//! Kbd size examples.

use gpui::IntoElement;
use liora_components::{Kbd, Space};

pub fn kbd_sizes() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Kbd::new("⌘").small())
        .child(Kbd::new("Tab"))
        .child(Kbd::new("Space").large())
}
