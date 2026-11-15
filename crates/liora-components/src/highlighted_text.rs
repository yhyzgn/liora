//! Search-result text with selectable, character-level match emphasis.

use crate::{Paragraph, Text};
use gpui::{
    App, FontWeight, IntoElement, RenderOnce, SharedString, ViewElement as Component, Window,
};
use liora_core::Config;

/// Emphasizes selected Unicode character positions without splitting the visible text into elements.
pub struct HighlightedText {
    text: SharedString,
    indices: Vec<usize>,
    accent: bool,
    id: Option<SharedString>,
}

impl HighlightedText {
    /// Create a selectable text label with no matches.
    pub fn new(text: impl Into<SharedString>) -> Self {
        Self {
            text: text.into(),
            indices: Vec::new(),
            accent: false,
            id: None,
        }
    }

    /// Mark zero-based Unicode scalar positions; duplicates and out-of-range positions are ignored.
    pub fn indices(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        self.indices = indices.into_iter().collect();
        self.indices.sort_unstable();
        self.indices.dedup();
        self
    }

    /// Supply a stable ID when multiple rows may contain the same text.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Use the theme accent for matched text instead of only a stronger weight.
    pub fn accent(mut self) -> Self {
        self.accent = true;
        self
    }
}

fn for_each_segment<'a>(text: &'a str, indices: &[usize], mut emit: impl FnMut(&'a str, bool)) {
    let mut matches = indices.iter().copied().peekable();
    let mut start = 0;
    let mut active = false;
    for (index, (byte, _)) in text.char_indices().enumerate() {
        let matched = matches.peek().is_some_and(|next| *next == index);
        if matched {
            matches.next();
        }
        if matched != active {
            if start < byte {
                emit(&text[start..byte], active);
            }
            start = byte;
            active = matched;
        }
    }
    if start < text.len() {
        emit(&text[start..], active);
    }
}

impl RenderOnce for HighlightedText {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let color = cx.global::<Config>().theme.primary.base;
        let mut segments = Vec::new();
        for_each_segment(self.text.as_ref(), &self.indices, |text, matched| {
            let mut segment = Text::new(text.to_owned());
            if matched {
                segment = segment.weight(FontWeight::SEMIBOLD);
                if self.accent {
                    segment = segment.text_color(color);
                }
            }
            segments.push(segment);
        });
        Paragraph::new()
            .preserve_segment_styles()
            .id(self.id.unwrap_or(self.text))
            .children(segments)
    }
}

impl IntoElement for HighlightedText {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unicode_matches_keep_original_text_and_character_boundaries() {
        let mut segments = Vec::new();
        let indices = HighlightedText::new("A好🙂Z")
            .indices([3, 2, 2, 99])
            .indices;
        for_each_segment("A好🙂Z", &indices, |text, matched| {
            segments.push((text, matched))
        });
        assert_eq!(segments, [("A好", false), ("🙂Z", true)]);
        assert_eq!(
            segments.iter().map(|(text, _)| *text).collect::<String>(),
            "A好🙂Z"
        );
    }

    #[test]
    fn isolated_and_final_matches_only_style_the_selected_scalars() {
        let mut segments = Vec::new();
        for_each_segment("abcd", &[1, 3], |text, matched| {
            segments.push((text, matched))
        });
        assert_eq!(
            segments,
            [("a", false), ("b", true), ("c", false), ("d", true)]
        );

        segments.clear();
        for_each_segment("abcd", &[1], |text, matched| segments.push((text, matched)));
        assert_eq!(segments, [("a", false), ("b", true), ("cd", false)]);
    }

    #[test]
    fn empty_and_leading_matches_do_not_make_empty_spans() {
        let mut segments = Vec::new();
        for_each_segment("éX", &[0], |text, matched| segments.push((text, matched)));
        assert_eq!(segments, [("é", true), ("X", false)]);
        segments.clear();
        for_each_segment("", &[0], |text, matched| segments.push((text, matched)));
        assert!(segments.is_empty());
    }
}
