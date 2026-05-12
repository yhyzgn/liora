//! Basic InputNumber with min/max bounds.

use aura_components::InputNumber;
use gpui::{AppContext, Context, Render, Window};

struct InputNumberBasicDemo {
    input: gpui::Entity<InputNumber>,
}

impl InputNumberBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // The value is clamped between 0 and 10 when controls are used.
            input: cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(10.0)),
        }
    }
}

impl Render for InputNumberBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
