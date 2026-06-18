//! Empty with a native footer action.

use gpui::IntoElement;
use liora_components::{Button, ButtonVariant, Card, Empty};

pub fn empty_with_extra_action() -> Card {
    Card::new(Empty::new().extra(|_, _| {
        Button::new("去添加")
            .variant(ButtonVariant::Primary)
            .into_any_element()
    }))
    .width_md()
}
