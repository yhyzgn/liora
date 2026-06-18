//! Steps with descriptions and semantic icons.

use liora_components::{StepItem, Steps};
use liora_icons_lucide::IconName;

pub fn steps_with_descriptions() -> Steps {
    Steps::new()
        .active(1)
        .step(
            StepItem::new("步骤 1")
                .description("这是一段描述性文字")
                .icon(IconName::User),
        )
        .step(
            StepItem::new("步骤 2")
                .description("这是一段描述性文字")
                .icon(IconName::Settings),
        )
        .step(
            StepItem::new("步骤 3")
                .description("这是一段描述性文字")
                .icon(IconName::Check),
        )
}
