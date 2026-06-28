//! Nested unordered list with default level markers.

use liora_components::{Card, List, ListItem};

pub fn unordered_list() -> Card {
    Card::new(
        List::unordered()
            .item(
                ListItem::new("Prepare component API")
                    .child(ListItem::new("Define row spacing and marker width"))
                    .child(
                        ListItem::new("Document nested behavior")
                            .child(ListItem::new("Explain default level markers"))
                            .child(ListItem::new("Show item-level overrides")),
                    ),
            )
            .item(ListItem::new("Build Gallery examples"))
            .item(ListItem::new("Sync Docs snippets")),
    )
}
