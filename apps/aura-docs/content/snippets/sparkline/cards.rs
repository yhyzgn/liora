use aura_components::{Card, Space, Sparkline, Text};
use gpui::{IntoElement, px, rgb};

pub fn sparkline_cards() -> impl IntoElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(
                Text::new("Revenue")
                    .size(px(12.0))
                    .text_color(rgb(0x64748b).into()),
            )
            .child(Text::new("$42.8k").size(px(24.0)).bold())
            .child(
                Sparkline::new([12.0, 15.0, 14.0, 18.0, 21.0, 19.0, 24.0, 28.0])
                    .height(px(64.0))
                    .area_fill(true),
            ),
    )
    .width(px(240.0))
}
