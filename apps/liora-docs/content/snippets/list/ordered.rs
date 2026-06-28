//! Nested ordered list with default counter styles.

use liora_components::{Card, List, ListItem};

pub fn ordered_list() -> Card {
    Card::new(
        List::ordered()
            .item(
                ListItem::new("Install Liora")
                    .child(ListItem::new("Add the liora crate"))
                    .child(ListItem::new("Pin the matching official GPUI rev")),
            )
            .item(
                ListItem::new("Initialize the app")
                    .child(ListItem::new("Call init_liora(cx)"))
                    .child(ListItem::new("Render portals near the root")),
            )
            .item(ListItem::new("Compose components")),
    )
}
