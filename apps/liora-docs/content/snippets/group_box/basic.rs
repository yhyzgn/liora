use gpui::IntoElement;
use liora_components::{GroupBox, Space, Text};

pub fn group_box_basic() -> impl IntoElement {
    GroupBox::new(
        "Editor",
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new("Tab size: 4"))
            .child(Text::new("Soft tabs enabled")),
    )
    .description("Project-level editor preferences.")
}
