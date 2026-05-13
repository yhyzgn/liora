//! Basic Segmented control with a change callback.

use aura_components::{Segmented, SegmentedOption, toast_info};

pub fn basic_segmented() -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("Daily", "daily"),
        SegmentedOption::new("Weekly", "weekly"),
        SegmentedOption::new("Monthly", "monthly"),
        SegmentedOption::new("Quarterly", "quarterly"),
        SegmentedOption::new("Yearly", "yearly"),
    ])
    .id("docs-segmented-basic")
    .on_change(|value, _, _| toast_info!("Selected: {}", value))
}
