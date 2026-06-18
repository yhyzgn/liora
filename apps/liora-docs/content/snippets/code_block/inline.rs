//! Inline CodeBlock for compact command fragments.

use liora_components::CodeBlock;

fn inline_code() -> CodeBlock {
    // Inline mode disables the header/copy chrome and flows like text.
    CodeBlock::new("cargo check").shell().inline()
}

fn main() {
    let _ = inline_code();
}
