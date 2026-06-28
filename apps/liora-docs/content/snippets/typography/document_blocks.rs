//! Text document block example.

use liora_components::{Text, TextBlock};

pub fn document_blocks() -> Text {
    Text::document([
        TextBlock::heading(2, "Application bootstrap"),
        TextBlock::paragraph(
            "Initialize Liora once, then compose native GPUI windows with reusable components.",
        ),
        TextBlock::quote(
            "Text document mode is lightweight; use Docs for full documentation chrome.",
        ),
        TextBlock::unordered([
            "Native selectable text",
            "Theme-aware quote and code surfaces",
            "Copyable code blocks",
        ]),
        TextBlock::code("liora_components::init_liora(cx);", "rust"),
    ])
    .framed(true)
    .max_width(gpui::px(760.0))
}
