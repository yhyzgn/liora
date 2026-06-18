//! Draggable `VirtualizedList` snippet used by Liora Docs.

use gpui::{App, Entity, div, prelude::*, px};
use liora_components::{Space, Tag, Text, VirtualizedList, toast_success};

pub fn build_draggable_virtualized_list(cx: &mut App) -> Entity<VirtualizedList> {
    cx.new(|cx| {
        let mut list = VirtualizedList::new(48, cx, |index, _window, _cx| {
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
                        .child(Text::new(format!("Task #{:02}", index + 1)).bold())
                        .child(Text::new("Drag this row to reorder visible data.")),
                )
                .child(Tag::new(if index % 2 == 0 { "ready" } else { "doing" }).info())
                .into_any_element()
        });

        list.set_height(Some(px(320.0)));
        list.set_item_spacing(px(12.0));
        list.measure_all_items_for_scrollbar();
        list.set_draggable(true);
        list.set_on_reorder(|from, to, _, _| {
            toast_success!("VirtualizedList reordered: {} -> {}", from + 1, to + 1);
        });
        list
    })
}
