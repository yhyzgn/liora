//! Spinner semantic color examples.

use gpui::{IntoElement, rgb};
use liora_components::{Space, Spinner};

pub fn spinner_colors() -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Spinner::new().color(rgb(0x2563eb).into()))
        .child(Spinner::new().color(rgb(0x16a34a).into()))
        .child(Spinner::new().color(rgb(0xf59e0b).into()))
        .child(Spinner::new().color(rgb(0xdc2626).into()))
}
