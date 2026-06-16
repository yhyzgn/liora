use aura_components::InputTag;
use gpui::{App, AppContext, Entity};

pub fn basic_input_tag(cx: &mut App) -> Entity<InputTag> {
    cx.new(|cx| InputTag::new(vec!["Rust", "GPUI", "Aura"], cx).placeholder("Add skill"))
}
