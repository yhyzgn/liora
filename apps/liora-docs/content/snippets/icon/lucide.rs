//! Render a small set of Lucide icons with labels.

use gpui::{IntoElement, prelude::*};
use liora_components::{Space, Text};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn lucide_icon_row() -> impl IntoElement {
    Space::new().wrap().gap_md().children(
        [
            (IconName::House, "Home"),
            (IconName::User, "User"),
            (IconName::Search, "Search"),
            (IconName::Check, "Check"),
        ]
        .into_iter()
        .map(|(icon, label)| {
            Space::new()
                .vertical()
                .align_center()
                .gap_xs()
                .child(Icon::new(icon).size_lg())
                .child(Text::new(label).nowrap())
        }),
    )
}
