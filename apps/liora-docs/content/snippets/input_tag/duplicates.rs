use gpui::{App, AppContext, Entity};
use liora_components::InputTag;

pub fn duplicate_input_tag(cx: &mut App) -> Entity<InputTag> {
    cx.new(|cx| {
        InputTag::new(vec!["blue", "blue"], cx)
            .allow_duplicates(true)
            .placeholder("Duplicates allowed")
    })
}
