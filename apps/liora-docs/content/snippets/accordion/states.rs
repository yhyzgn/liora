//! Disabled and borderless Accordion states.

use liora_components::{Accordion, Text};

pub fn stateful_accordion() -> Accordion {
    Accordion::new()
        .id("docs-accordion-states")
        .large()
        .bordered(false)
        .default_open("enabled")
        .item("enabled", "Enabled item", |_, _| {
            Text::new("Large borderless rows work well in docs.")
        })
        .disabled_item("locked", "Disabled item", |_, _| {
            Text::new("Disabled panels do not toggle.")
        })
}
