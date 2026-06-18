//! Explicit language selection for syntax highlighting.

use liora_components::CodeBlock;

fn rust_code_block() -> CodeBlock {
    CodeBlock::new(r#"fn main() { println!(\"Liora\"); }"#).language("rust")
}

fn main() {
    let _ = rust_code_block();
}
