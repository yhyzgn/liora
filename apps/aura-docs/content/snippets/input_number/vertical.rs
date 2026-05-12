//! InputNumber with controls on the right side.

use aura_components::{InputNumber, InputNumberControlsPosition};
use gpui::{AppContext, Context, Render, Window};

struct InputNumberVerticalDemo {
    input: gpui::Entity<InputNumber>,
}

impl InputNumberVerticalDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| {
                InputNumber::new(5.0, cx)
                    .min(0.0)
                    .max(10.0)
                    .controls_position(InputNumberControlsPosition::Right)
            }),
        }
    }
}

impl Render for InputNumberVerticalDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
