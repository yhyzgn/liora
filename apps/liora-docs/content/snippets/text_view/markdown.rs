use gpui::{IntoElement, px};
use liora_components::TextView;

pub fn text_view_markdown() -> impl IntoElement {
    TextView::from_plain_markdown(
        "# Release notes\n\nLiora renders app documents as native GPUI elements.\n\n> Keep SDK docs close to product behavior.\n\n1. Parse a small Markdown subset\n2. Render reusable component blocks\n\n```rust\nTextView::from_plain_markdown(markdown)\n```",
    )
    .framed(true)
    .max_width(px(760.0))
}
