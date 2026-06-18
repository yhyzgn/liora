//! Circular progress examples rendered with native GPUI paths.

use gpui::{IntoElement, px};
use liora_components::{Progress, ProgressStatus, Space};

pub fn circle_progress() -> impl IntoElement {
    Space::new().gap_lg().wrap().children(vec![
        Progress::new(32.0).circle(),
        Progress::new(58.0).circle().status(ProgressStatus::Warning),
        Progress::new(76.0)
            .circle()
            .status(ProgressStatus::Exception),
        Progress::new(100.0)
            .circle()
            .circle_size(px(132.0))
            .status(ProgressStatus::Success),
    ])
}
