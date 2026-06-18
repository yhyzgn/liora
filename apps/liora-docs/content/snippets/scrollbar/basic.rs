//! Scrollbar with a fixed-height scroll viewport.
//!
//! `Scrollbar::new` is called from the component's own GPUI context when you
//! create the stateful scrollbar entity with `cx.new(|cx| basic_scrollbar(cx))`.

use gpui::{Context, IntoElement};
use liora_components::{Scrollbar, Space, Text};

pub fn basic_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        let items = (1..=20).map(|i| Text::new(format!("Scrollable line {}", i)));
        Space::new()
            .vertical()
            .gap_lg()
            .children(items)
            .into_any_element()
    })
    .height(150.0)
}
