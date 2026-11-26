use gpui::IntoElement;
use liora_components::{Button, HoverCard, Space, Text};

pub fn hover_card_basic() -> impl IntoElement {
    Space::new()
        .gap_lg()
        .wrap()
        .child(
            HoverCard::new(Button::new("Open profile").text())
                .id("hover-card-profile")
                .content(|_, _| {
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new("Neo").bold())
                        .child(Text::new("Maintainer · Native GPUI components"))
                }),
        )
        .child(
            HoverCard::new(Button::new("Preview link").text())
                .id("hover-card-link")
                .content(|_, _| {
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Text::new("Release notes").bold())
                        .child(Text::new("Show a concise summary before navigation."))
                }),
        )
}
