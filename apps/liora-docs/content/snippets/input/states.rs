//! Input clearable and disabled states.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Input, Space};

struct InputStatesDemo {
    clearable: Entity<Input>,
    disabled: Entity<Input>,
}

impl InputStatesDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            clearable: cx.new(|cx| Input::new("Clear me", cx).clearable(true)),
            disabled: cx.new(|cx| Input::new("Disabled", cx).disabled(true)),
        }
    }
}

impl Render for InputStatesDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.clearable.clone())
            .child(self.disabled.clone())
    }
}

fn main() {}
