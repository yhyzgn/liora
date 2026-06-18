//! Rendering a fenced Markdown block as Liora's native CodeBlock.

use gpui::{AnyElement, IntoElement, SharedString};
use liora_components::CodeBlock;

fn render_code_block(language: Option<SharedString>, code: SharedString) -> AnyElement {
    let mut code_block = CodeBlock::new(code);
    if let Some(language) = language {
        // Language can come from the Markdown fence, e.g. ```rust.
        code_block = code_block.language(language.as_ref());
    }
    code_block.into_any_element()
}

fn main() {
    let _ = render_code_block(Some("rust".into()), "fn main() {}".into());
}
