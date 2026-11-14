//! Basic left/right Splitter panel.

use gpui::px;
use liora_components::{Card, Splitter, Text};

pub fn basic_splitter() -> Splitter {
    Splitter::new()
        .height_md()
        .bordered()
        .min_size(px(120.0))
        .left(Card::new(Text::new("Left panel")).no_shadow())
        .right(Card::new(Text::new("Right panel")).no_shadow())
}
