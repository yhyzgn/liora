//! Default Empty state.

use liora_components::{Card, Empty};

pub fn default_empty() -> Card {
    Card::new(Empty::new()).width_md()
}
