use gpui::{App, AppContext, Entity};
use liora_components::InputTag;

pub fn basic_input_tag(cx: &mut App) -> Entity<InputTag> {
    cx.new(|cx| InputTag::new(vec!["Rust", "GPUI", "Liora"], cx).placeholder("Add skill"))
}
