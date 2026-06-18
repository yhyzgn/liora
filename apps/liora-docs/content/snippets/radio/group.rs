//! RadioGroup options.

use gpui::{AppContext, Context, Render, Window};
use liora_components::RadioGroup;

struct RadioGroupDemo {
    group: gpui::Entity<RadioGroup>,
    disabled: gpui::Entity<RadioGroup>,
}

impl RadioGroupDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            group: cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
            disabled: cx
                .new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)),
        }
    }
}

impl Render for RadioGroupDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        liora_components::Space::new()
            .vertical()
            .gap_md()
            .child(self.group.clone())
            .child(self.disabled.clone())
    }
}

fn main() {}
