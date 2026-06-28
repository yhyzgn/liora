//! Cross-block selection with Text and Paragraph.

use liora_components::{Paragraph, SelectableTextGroup, Text};
use liora_theme::Theme;

fn release_notes(theme: &Theme) -> SelectableTextGroup {
    // SelectableTextGroup renders all children as one selectable native text flow.
    SelectableTextGroup::new()
        .id("release-notes-selection")
        .separator("\n")
        .text(Text::new("Release notes").bold().text_color(theme.primary.base))
        .paragraph(
            Paragraph::new()
                .child(Text::new("Combine "))
                .child(Text::new("Text").code_style(theme))
                .child(Text::new(" and "))
                .child(Text::new("Paragraph").code_style(theme))
                .child(Text::new(" blocks without breaking mouse selection.")),
        )
        .paragraph(Paragraph::with_text(
            "Drag from the heading into this paragraph to copy one continuous range.",
        ))
}

fn main() {
    let theme = Theme::light();
    let _ = release_notes(&theme);
}
