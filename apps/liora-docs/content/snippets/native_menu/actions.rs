//! Menu action catalog example.

use liora_components::{MenuAction, Tag, Text, Space};
use gpui::IntoElement;

pub fn native_menu_actions() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .children(MenuAction::catalog().into_iter().map(|action| {
            let info = action.info();
            Space::new()
                .vertical()
                .gap_xs()
                .child(
                    Space::new()
                        .gap_sm()
                        .wrap()
                        .child(Text::new(info.name).bold())
                        .child(Tag::new(info.id).info().round(true))
                        .child(if info.handled_by_liora {
                            Tag::new("Liora handles").success().round(true)
                        } else {
                            Tag::new("App dispatch").warning().round(true)
                        }),
                )
                .child(Text::new(info.description).sm().wrap())
                .child(Text::new(info.effect).xs().wrap())
        }))
}
