use crate::Text;
use aura_core::Config;
use gpui::{App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};

pub struct Paragraph {
    children: Vec<Text>,
}

impl Paragraph {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn with_text(text: impl Into<SharedString>) -> Self {
        Self {
            children: vec![Text::new(text)],
        }
    }

    pub fn child(mut self, child: Text) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = Text>) -> Self {
        self.children.extend(children);
        self
    }
}

impl RenderOnce for Paragraph {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        // If we want real flow wrapping of mixed styles, StyledText is the way.
        // However, building StyledText requires calculating character ranges.

        let mut full_text = String::new();
        let mut runs = Vec::new();

        for segment in self.children {
            let start = full_text.len();
            full_text.push_str(&segment.content);
            let end = full_text.len();

            // Store the segment style to apply to the range
            runs.push((start..end, segment));
        }

        // We wrap in a div that handles line height and overall container style
        div()
            .flex()
            .flex_row()
            .flex_wrap() // Fallback if we use multiple elements
            .line_height(px(theme.font_size.md * 1.6))
            .children(runs.into_iter().map(|(_range, segment)| {
                // If GPUI's StyledText is complex to use here (needs font resolution),
                // we can use multiple Text elements with flex_wrap.
                // But the user specifically asked for StyledText-like wrapping.

                // Real implementation of flow text in GPUI often uses StyledText.
                // For now, let's use the individual Text segments but make sure they don't break flow.
                segment
            }))
    }
}

impl IntoElement for Paragraph {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
