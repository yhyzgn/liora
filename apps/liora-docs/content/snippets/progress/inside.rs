//! Render percentages inside a thicker progress track.

use gpui::IntoElement;
use liora_components::{Progress, ProgressStatus, Space};

pub fn inside_progress() -> impl IntoElement {
    Space::new().vertical().gap_md().children(vec![
        Progress::new(15.0).thick().text_inside(true),
        Progress::new(70.0).thick().text_inside(true),
        Progress::new(70.0).thick().text_inside_centered(),
        Progress::new(100.0)
            .thick()
            .text_inside(true)
            .status(ProgressStatus::Success),
    ])
}
