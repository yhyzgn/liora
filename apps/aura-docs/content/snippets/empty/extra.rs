//! Empty with a native footer action.

use aura_components::{Button, ButtonVariant, Card, Empty};
use gpui::IntoElement;

pub fn empty_with_extra_action() -> Card {
    Card::new(Empty::new().extra(|_, _| {
        Button::new("去添加")
            .variant(ButtonVariant::Primary)
            .into_any_element()
    }))
    .width_md()
}
