//! Nested Anchor links for second-level sections.

use gpui::ScrollHandle;
use liora_components::{Anchor, AnchorLink};

pub fn nested_anchor(scroll_handle: ScrollHandle) -> Anchor {
    Anchor::new(scroll_handle).offset_sm().link(
        AnchorLink::new("API", "api")
            .child(AnchorLink::new("Attributes", "attributes"))
            .child(AnchorLink::new("Events", "events")),
    )
}
