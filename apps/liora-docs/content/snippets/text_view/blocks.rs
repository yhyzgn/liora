use gpui::{IntoElement, px};
use liora_components::{TextView, TextViewBlock};

pub fn text_view_blocks() -> impl IntoElement {
    TextView::new([
        TextViewBlock::heading(2, "Application bootstrap"),
        TextViewBlock::paragraph(
            "Initialize Liora once, then compose native GPUI windows with reusable components.",
        ),
        TextViewBlock::quote("TextView is lightweight; use Docs for full documentation chrome."),
        TextViewBlock::unordered([
            "Native selectable text",
            "Theme-aware quote and code surfaces",
            "Copyable code blocks",
        ]),
        TextViewBlock::code("liora_components::init_liora(cx);", "rust"),
    ])
    .framed(true)
    .max_width(px(760.0))
}
