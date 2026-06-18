//! Basic Slider.

use gpui::{AppContext, Context, Render, Window};
use liora_components::Slider;

struct SliderBasicDemo {
    slider: gpui::Entity<Slider>,
}

impl SliderBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            slider: cx.new(|cx| Slider::new(50.0, cx)),
        }
    }
}

impl Render for SliderBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.slider.clone()
    }
}

fn main() {}
