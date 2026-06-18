//! Compose Liora components in a native GPUI element tree.

use liora_components::{Button, CodeBlock, Space, Title};

fn docs_landing() -> Space {
    Space::new()
        .vertical()
        .gap_lg()
        .child(Title::new("Liora UI").h2())
        .child(Button::new("Primary").primary())
        .child(CodeBlock::new("cargo run -p liora-docs").shell())
}

fn main() {
    let _ = docs_landing();
}
