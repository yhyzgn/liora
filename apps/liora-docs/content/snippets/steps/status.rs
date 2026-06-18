//! Explicit StepItem status overrides.

use liora_components::{StepItem, StepStatus, Steps};

pub fn status_steps() -> Steps {
    Steps::new()
        .active(1)
        .step(StepItem::new("已完成").status(StepStatus::Finish))
        .step(StepItem::new("发生错误").status(StepStatus::Error))
        .step(StepItem::new("等待中"))
}
