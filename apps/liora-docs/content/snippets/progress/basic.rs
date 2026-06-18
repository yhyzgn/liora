//! Basic line progress examples.

use gpui::IntoElement;
use liora_components::{Progress, ProgressStatus, Space};

pub fn basic_progress() -> impl IntoElement {
    Space::new().vertical().gap_md().children(vec![
        Progress::new(0.0),
        Progress::new(30.0),
        Progress::new(50.0),
        Progress::new(100.0).status(ProgressStatus::Success),
    ])
}
