use aura_components::InputTag;
use gpui::{App, AppContext, Entity};

pub fn limited_input_tag(cx: &mut App) -> Entity<InputTag> {
    cx.new(|cx| {
        InputTag::new(vec!["Design", "Docs"], cx)
            .placeholder("Max 4")
            .max_tags(4)
    })
}
