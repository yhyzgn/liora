//! Basic CodeBlock with copy support.

use liora_components::CodeBlock;

fn basic_code_block() -> CodeBlock {
    CodeBlock::new("cargo run -p liora-docs")
        .shell()
        .copyable(true)
}

fn main() {
    let _ = basic_code_block();
}
