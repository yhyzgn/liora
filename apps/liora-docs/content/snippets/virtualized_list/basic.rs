//! Basic `VirtualizedList` snippet used by Liora Docs.
//! The list renders native Liora/GPUI rows only when they enter the viewport.

use gpui::{App, Entity, div, prelude::*, px};
use liora_components::{Space, Tag, Text, VirtualizedList};

pub fn build_virtualized_list(cx: &mut App) -> Entity<VirtualizedList> {
    cx.new(|cx| {
        let mut list = VirtualizedList::new(1_000, cx, |index, _window, _cx| {
            div()
                .flex()
                .items_center()
                .justify_between()
                .p_3()
                .rounded(px(8.0))
                .border_1()
                .border_color(gpui::rgb(0xe5e7eb))
                .child(
                    Space::new()
                        .vertical()
                        .gap_xs()
                        .child(Text::new(format!("Row #{:04}", index + 1)).bold())
                        .child(Text::new("Rendered inside the native virtual viewport.")),
                )
                .child(Tag::new(if index % 2 == 0 { "even" } else { "odd" }).info())
                .into_any_element()
        });

        // Give the virtual viewport a bounded height and comfortable item gaps.
        list.set_height(Some(px(320.0)));
        list.set_item_spacing(px(12.0));
        list.measure_all_items_for_scrollbar();
        list
    })
}
