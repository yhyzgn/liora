//! Input prefix/suffix examples.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Input, Space};
use liora_icons_lucide::IconName;

struct InputAffixDemo {
    prepend: Entity<Input>,
    append: Entity<Input>,
    composite: Entity<Input>,
}

impl InputAffixDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            prepend: cx.new(|cx| Input::new("", cx).prepend_text("http://")),
            append: cx.new(|cx| Input::new("", cx).append_text(".com")),
            composite: cx.new(|cx| {
                Input::new("", cx)
                    .prepend_icon(IconName::User)
                    .append_text("Admin")
            }),
        }
    }
}

impl Render for InputAffixDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.prepend.clone())
            .child(self.append.clone())
            .child(self.composite.clone())
    }
}

fn main() {}
