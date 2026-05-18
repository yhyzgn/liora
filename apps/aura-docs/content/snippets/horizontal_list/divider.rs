use aura_components::{HorizontalList, Text};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyElement, App, Entity, div, prelude::*, px, rgb};

const ITEMS: &[&str] = &["Input", "Validate", "Transform", "Export"];

pub fn horizontal_list_divider(cx: &mut App) -> Entity<HorizontalList> {
    cx.new(|_| {
        HorizontalList::new(ITEMS.len(), item_card)
            .height(px(84.0))
            .divider(|_| {
                Icon::new(IconName::ChevronRight)
                    .size(px(18.0))
                    .color(rgb(0x94a3b8).into())
                    .into_any_element()
            })
    })
}

fn item_card(index: usize) -> AnyElement {
    div()
        .w(px(144.0))
        .h(px(68.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .child(Text::new(ITEMS[index]).bold())
        .into_any_element()
}
