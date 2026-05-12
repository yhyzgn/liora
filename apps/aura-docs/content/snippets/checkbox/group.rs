//! CheckboxGroup options.

use aura_components::CheckboxGroup;
use gpui::{AppContext, Context, Render, Window};

struct CheckboxGroupDemo {
    group: gpui::Entity<CheckboxGroup>,
}

impl CheckboxGroupDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // `selected` contains the indexes checked by default.
            group: cx.new(|cx| {
                CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx)
            }),
        }
    }
}

impl Render for CheckboxGroupDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.group.clone()
    }
}

fn main() {}
