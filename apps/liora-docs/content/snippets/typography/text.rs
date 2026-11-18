//! Basic selectable Text example.

use gpui::{ParentElement, div, prelude::*};
use liora_components::{Space, Text};
use liora_theme::Theme;

fn text_examples(theme: &Theme) -> Space {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Drag to select this default Text value."))
        .child(
            div().group("text-example-hover").child(
                Text::new("Hover for primary color; drag to select and copy.")
                    .id("text-example-hover-selection")
                    .group_hover_primary("text-example-hover"),
            ),
        )
        .child(
            Text::new("Primary bold text")
                .text_color(theme.primary.base)
                .bold(),
        )
        .child(Text::new("Inline code").code_style(theme))
        .child(Text::new("Decorative label").selectable(false))
        .child(
            Text::new("Nonselectable oblique text")
                .selectable(false)
                .font_style(gpui::FontStyle::Oblique),
        )
}

fn main() {
    let theme = Theme::light();
    let _ = text_examples(&theme);
}
