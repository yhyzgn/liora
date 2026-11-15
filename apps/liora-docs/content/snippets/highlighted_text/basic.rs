//! Character-position matching for native selectable search results.
use liora_components::HighlightedText;

pub fn highlighted_text_basic() -> HighlightedText {
    HighlightedText::new("搜索 Liora 🙂 文档")
        .indices([0, 1, 3, 4, 5, 6, 7])
        .accent()
}
