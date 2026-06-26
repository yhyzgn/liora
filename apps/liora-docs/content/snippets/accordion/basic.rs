//! Basic single-open Accordion.

use liora_components::{Accordion, Text};

pub fn basic_accordion() -> Accordion {
    Accordion::new()
        .id("docs-accordion-basic")
        .default_open("account")
        .item_with_description(
            "account",
            "Account",
            "Sign-in, security, and notifications",
            |_, _| Text::new("Only one panel stays open in the default single mode."),
        )
        .item_with_description(
            "billing",
            "Billing",
            "Payment methods and invoices",
            |_, _| Text::new("Use Accordion for FAQ and settings sections."),
        )
}
