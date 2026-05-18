use aura_components::{HorizontalList, Text};
use gpui::{AnyElement, App, Entity, div, prelude::*, px, rgb};

const STEPS: &[(&str, &str)] = &[("01", "Discover"), ("02", "Design"), ("03", "Build")];

pub fn horizontal_list_basic(cx: &mut App) -> Entity<HorizontalList> {
    HorizontalList::entity(STEPS.len(), cx, step_card)
}

fn step_card(index: usize) -> AnyElement {
    let (number, label) = STEPS[index];
    div()
        .w(px(132.0))
        .h(px(72.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xdbeafe))
        .bg(rgb(0xeff6ff))
        .p_3()
        .flex()
        .flex_col()
        .justify_between()
        .child(
            Text::new(number)
                .size(px(12.0))
                .text_color(rgb(0x2563eb).into()),
        )
        .child(Text::new(label).bold().text_color(rgb(0x1e3a8a).into()))
        .into_any_element()
}
