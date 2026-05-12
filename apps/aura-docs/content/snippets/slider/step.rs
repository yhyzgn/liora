//! Slider with discrete steps.

use aura_components::Slider;
use gpui::{AppContext, Context, Render, Window};

struct SliderStepDemo {
    slider: gpui::Entity<Slider>,
}

impl SliderStepDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Values snap to multiples of 10.
            slider: cx.new(|cx| Slider::new(20.0, cx).step(10.0)),
        }
    }
}

impl Render for SliderStepDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.slider.clone()
    }
}

fn main() {}
