//! Text markdown subset example.

use liora_components::Text;

pub fn markdown_document() -> Text {
    Text::markdown(
        "# Release notes\n\nLiora renders app documents as native GPUI elements.\n\n> Keep SDK docs close to product behavior.\n\n1. Parse a small Markdown subset\n2. Render reusable component blocks\n\n```rust\nText::markdown(markdown)\n```",
    )
    .framed(true)
    .max_width(gpui::px(760.0))
}
