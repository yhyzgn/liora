//! Rich inline text rendered by Liora Paragraph.

use liora_components::{Paragraph, Text};
use liora_theme::Theme;

fn rich_paragraph(theme: &Theme) -> Paragraph {
    Paragraph::new()
        .child(Text::new("Normal "))
        .child(Text::new("Bold").bold())
        .child(Text::new(" code ").code_style(theme))
}

fn main() {
    let theme = Theme::light();
    let _ = rich_paragraph(&theme);
}
