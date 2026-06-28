//! Basic selectable Text example.

use liora_components::{Space, Text};
use liora_theme::Theme;

fn text_examples(theme: &Theme) -> Space {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Text::new("Drag to select this default Text value."))
        .child(
            Text::new("Primary bold text")
                .text_color(theme.primary.base)
                .bold(),
        )
        .child(Text::new("Inline code").code_style(theme))
        .child(Text::new("Decorative label").selectable(false))
}

fn main() {
    let theme = Theme::light();
    let _ = text_examples(&theme);
}
