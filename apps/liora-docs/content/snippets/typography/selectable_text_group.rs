//! Cross-block selection without flattening individual Text, Title or Paragraph layouts.

use liora_components::{Paragraph, SelectableTextGroup, Text, Title};
use liora_theme::Theme;

fn release_notes(theme: &Theme) -> SelectableTextGroup {
    // Each child keeps its native layout; one selection range spans the group.
    SelectableTextGroup::new()
        .id("release-notes-selection")
        .title(Title::new("Release notes").h2())
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
