//! InputNumber precision and fractional step.

use gpui::{AppContext, Context, Render, Window};
use liora_components::InputNumber;

struct InputNumberPrecisionDemo {
    input: gpui::Entity<InputNumber>,
}

impl InputNumberPrecisionDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Keep two decimal places and move by one cent per step.
            input: cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01)),
        }
    }
}

impl Render for InputNumberPrecisionDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
