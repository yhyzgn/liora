//! Vertical Steps for long process descriptions.

use liora_components::{StepItem, Steps, StepsDirection};

pub fn vertical_steps() -> Steps {
    Steps::new()
        .active(1)
        .direction(StepsDirection::Vertical)
        .step(StepItem::new("步骤 1").description("这是一段很长很长很长的描述性文字"))
        .step(StepItem::new("步骤 2"))
        .step(StepItem::new("步骤 3"))
}
