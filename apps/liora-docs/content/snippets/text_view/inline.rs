use gpui::{IntoElement, px};
use liora_components::{TextView, TextViewBlock};

pub fn text_view_inline() -> impl IntoElement {
    TextView::new([
        TextViewBlock::heading(3, "Inline help"),
        TextViewBlock::paragraph(
            "Use max_width for readable line length and selectable(false) for decorative guidance.",
        ),
        TextViewBlock::Divider,
        TextViewBlock::ordered([
            "Keep content concise",
            "Use real components",
            "Avoid browser runtime",
        ]),
    ])
    .selectable(false)
    .max_width(px(680.0))
}
