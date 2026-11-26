//! Scrollbar containing card-like rows.

use gpui::{Context, IntoElement};
use liora_components::{Card, Scrollbar, Space, Tag, Text};

pub fn card_scrollbar(cx: &mut Context<Scrollbar>) -> Scrollbar {
    Scrollbar::new(cx, |_, _| {
        Space::new()
            .vertical()
            .gap_md()
            .children((1..=12).map(|i| {
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Text::new(format!("Workflow card #{:02}", i)).bold())
                        .child(
                            Text::new(
                                "Complex component trees can live inside a Scrollbar viewport.",
                            )
                            .wrap(),
                        )
                        .child(
                            Space::new()
                                .gap_xs()
                                .child(Tag::new("scroll").success())
                                .child(Tag::new("native").info()),
                        ),
                )
                .id(("workflow-card", i as usize))
                .no_shadow()
            }))
            .into_any_element()
    })
    .height(300.0)
}
