//! Segmented with one disabled option and an explicit initial value.

use aura_components::{Segmented, SegmentedOption, toast_info};

pub fn disabled_segmented() -> Segmented {
    Segmented::new(vec![
        SegmentedOption::new("Map", "Map"),
        SegmentedOption::new("Transit", "Transit"),
        SegmentedOption::new("Satellite", "Satellite").disabled(true),
    ])
    .id("docs-segmented-disabled")
    .value("Map")
    .on_change(|value, _, _| toast_info!("Selected: {}", value))
}
