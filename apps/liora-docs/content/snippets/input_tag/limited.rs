use gpui::{App, AppContext, Entity};
use liora_components::InputTag;

pub fn limited_input_tag(cx: &mut App) -> Entity<InputTag> {
    cx.new(|cx| {
        InputTag::new(vec!["Design", "Docs"], cx)
            .placeholder("Max 4")
            .max_tags(4)
    })
}
