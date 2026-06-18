//! Accordion Collapse keeps only one panel open at a time.

use liora_components::{Collapse, Text};

pub fn accordion_collapse() -> Collapse {
    Collapse::new()
        .id("docs-collapse-accordion")
        .accordion()
        .item("consistency", "Consistency", |_, _| {
            Text::new("Consistent with real life: in line with process and intuition.")
        })
        .item("feedback", "Feedback", |_, _| {
            Text::new("Operation feedback: users clearly perceive style updates.")
        })
}
