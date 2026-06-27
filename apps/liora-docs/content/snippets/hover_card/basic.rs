use gpui::IntoElement;
use liora_components::{HoverCard, Space, Text};

pub fn hover_card_basic() -> impl IntoElement {
    HoverCard::new(Text::new("Hover target").underline()).content(|_, _| {
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new("Preview card").bold())
            .child(Text::new("Use this for profile or link previews."))
    })
}
