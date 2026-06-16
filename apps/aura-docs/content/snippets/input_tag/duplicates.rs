use aura_components::InputTag;
use gpui::{App, AppContext, Entity};

pub fn duplicate_input_tag(cx: &mut App) -> Entity<InputTag> {
    cx.new(|cx| {
        InputTag::new(vec!["blue", "blue"], cx)
            .allow_duplicates(true)
            .placeholder("Duplicates allowed")
    })
}
