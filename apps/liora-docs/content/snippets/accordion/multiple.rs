//! Multiple-open Accordion.

use liora_components::{Accordion, Text};

pub fn multiple_accordion() -> Accordion {
    Accordion::new()
        .id("docs-accordion-multiple")
        .multiple()
        .default_open("status")
        .default_open("deploy")
        .item("status", "Service status", |_, _| {
            Text::new("Multiple panels can stay expanded.")
        })
        .item("deploy", "Deploy checks", |_, _| {
            Text::new("Good for checklists and audits.")
        })
}
