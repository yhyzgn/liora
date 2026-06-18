use gpui::{AnyElement, App, Entity, div, prelude::*, px, rgb};
use liora_components::{HorizontalList, Text, toast_success};

const LANES: &[(&str, &str)] = &[
    ("Inbox", "8 tasks"),
    ("Ready", "5 tasks"),
    ("Doing", "3 tasks"),
    ("Done", "12 tasks"),
];

pub fn horizontal_list_draggable(cx: &mut App) -> Entity<HorizontalList> {
    cx.new(|_| {
        HorizontalList::new(LANES.len(), lane_card)
            .height(px(112.0))
            .draggable(true)
            .on_reorder(|from, to, _, _| {
                toast_success!("HorizontalList reordered: {} -> {}", from + 1, to + 1);
            })
    })
}

fn lane_card(index: usize) -> AnyElement {
    let (label, desc) = LANES[index];
    div()
        .w(px(156.0))
        .h(px(92.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(Text::new(label).bold())
        .child(
            Text::new(desc)
                .size(px(12.0))
                .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}
