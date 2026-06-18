//! Basic horizontal Steps with automatic status derived from `active`.

use liora_components::{StepItem, Steps};

pub fn basic_steps() -> Steps {
    Steps::new()
        .active(1)
        .step(StepItem::new("步骤 1"))
        .step(StepItem::new("步骤 2"))
        .step(StepItem::new("步骤 3"))
}
