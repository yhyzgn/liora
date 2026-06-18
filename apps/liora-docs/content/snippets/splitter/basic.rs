//! Basic left/right Splitter panel.

use liora_components::{Card, Splitter, Text};

pub fn basic_splitter() -> Splitter {
    Splitter::new()
        .height_md()
        .bordered()
        .left(Card::new(Text::new("Left panel")).no_shadow())
        .right(Card::new(Text::new("Right panel")).no_shadow())
}
