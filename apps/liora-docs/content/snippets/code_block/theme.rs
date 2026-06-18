//! CodeBlock highlighter and theme selection.

use liora_components::{CodeBlock, CodeHighlighter, CodeTheme, Space};

fn themed_code_blocks() -> Space {
    Space::new()
        .vertical()
        .gap_md()
        // Default: follows the global Liora light/dark theme.
        .child(
            CodeBlock::new("cargo run -p liora-docs")
                .shell()
                .auto_theme(),
        )
        // Force a specific dark palette for Rust examples.
        .child(
            CodeBlock::new(r#"fn main() { println!(\"Liora\"); }"#)
                .rust()
                .github_dark_theme(),
        )
        // Keep advanced options explicit when docs need to show exact behavior.
        .child(
            CodeBlock::new("[package]\nname = \"liora\"")
                .toml()
                .highlighter(CodeHighlighter::Syntect)
                .theme(CodeTheme::Nord)
                .selectable(true),
        )
}

fn main() {
    let _ = themed_code_blocks();
}
