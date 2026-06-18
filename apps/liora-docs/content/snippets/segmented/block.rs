//! Block Segmented fills the available width.

use liora_components::{Segmented, SegmentedOption, toast_info};

pub fn block_segmented() -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("123", "123"),
        SegmentedOption::new("456", "456"),
        SegmentedOption::new("long-text-option", "long"),
    ])
    .id("docs-segmented-block")
    .block(true)
    .on_change(|value, _, _| toast_info!("Selected: {}", value))
}
