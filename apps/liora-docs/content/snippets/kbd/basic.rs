//! Kbd basic examples.

use gpui::IntoElement;
use liora_components::{Kbd, Space};

pub fn kbd_basic() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(Kbd::new("⌘K"))
        .child(Kbd::new("Ctrl"))
        .child(Kbd::new("Shift"))
        .child(Kbd::new("Enter"))
        .child(Kbd::new("Esc"))
}
