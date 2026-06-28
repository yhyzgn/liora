//! Text document inline example.

use liora_components::{Text, TextBlock};

pub fn inline_document() -> Text {
    Text::document([
        TextBlock::heading(3, "Inline help"),
        TextBlock::paragraph(
            "Use max_width for readable line length and selectable(false) for decorative guidance.",
        ),
        TextBlock::Divider,
        TextBlock::ordered([
            "Keep content concise",
            "Use real components",
            "Avoid browser runtime",
        ]),
    ])
    .selectable(false)
    .max_width(gpui::px(680.0))
}
