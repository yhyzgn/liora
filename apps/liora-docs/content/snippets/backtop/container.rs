//! Backtop must share the same ScrollHandle as the scroll container.
//!
//! Retain `Backtop` as `Entity<Backtop>` in the view and pass it with the same handle.

use gpui::{App, Entity, IntoElement, ScrollHandle};
use liora_components::{Backtop, Flex, Space, Text};
use liora_core::Config;

pub fn backtop_scroll_region(
    cx: &mut App,
    scroll_handle: ScrollHandle,
    backtop: Entity<Backtop>,
) -> impl IntoElement {
    let theme = &cx.global::<Config>().theme;

    Flex::new()
        .relative()
        .height_units(360.0)
        .overflow_hidden()
        .border()
        .border_color(theme.neutral.border)
        .rounded_md()
        .child(
            Flex::new()
                .size_full()
                .id("docs-backtop-scroll-view")
                .overflow_y_scroll()
                .track_scroll(&scroll_handle)
                .child(Space::new().vertical().gap_sm().children((0..20).map(|i| {
                    Flex::new()
                        .height_units(40.0)
                        .center()
                        .bg(theme.neutral.hover)
                        .child(Text::new(format!("Scroll Item {i}")))
                }))),
        )
        .child(backtop)
}
