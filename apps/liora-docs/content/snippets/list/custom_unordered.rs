//! Unordered list with per-level and per-item custom markers.

use gpui::rgb;
use liora_components::{Card, List, ListItem, ListMarker};

pub fn custom_unordered_list() -> Card {
    Card::new(
        List::unordered()
            .unordered_markers([
                ListMarker::Check,
                ListMarker::Star,
                ListMarker::Text("◆".into()),
            ])
            .marker_colors([
                rgb(0x16a34a).into(),
                rgb(0xf59e0b).into(),
                rgb(0x7c3aed).into(),
            ])
            .item(
                ListItem::new("Design tokens applied")
                    .marker(ListMarker::Check)
                    .marker_color(rgb(0x2563eb).into()),
            )
            .item(ListItem::new("Theme-aware marker colors"))
            .item(
                ListItem::new("Nested custom levels")
                    .child(ListItem::new("Stars for highlighted children"))
                    .child(ListItem::new("Text markers for deep notes")),
            ),
    )
}
