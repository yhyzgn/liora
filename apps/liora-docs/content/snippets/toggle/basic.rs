use gpui::IntoElement;
use liora_components::{Space, Toggle, ToggleGroup, ToggleOption};

pub fn toggle_basic() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Toggle::new("Bold", true))
        .child(
            ToggleGroup::new([
                ToggleOption::new("preview", "Preview"),
                ToggleOption::new("code", "Code"),
                ToggleOption::new("split", "Split"),
            ])
            .selected("preview"),
        )
}
