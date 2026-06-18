//! Basic Collapse where multiple panels can stay open.

use liora_components::{Collapse, Text};

pub fn basic_collapse() -> Collapse {
    Collapse::new()
        .id("docs-collapse-basic")
        .item("consistency", "Consistency", |_, _| {
            Text::new("Consistent with real life: in line with process and intuition.")
        })
        .item("feedback", "Feedback", |_, _| {
            Text::new("Operation feedback: users clearly perceive style updates.")
        })
}
