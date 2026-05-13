//! Empty with business-specific description text.

use aura_components::{Card, Empty};

pub fn empty_with_description() -> Card {
    Card::new(Empty::new().description("自定义描述文字")).width_md()
}
