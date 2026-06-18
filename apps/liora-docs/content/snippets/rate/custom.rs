//! Rate with a custom maximum.

use gpui::{AppContext, Context, Render, Window};
use liora_components::Rate;

struct RateCustomDemo {
    rate: gpui::Entity<Rate>,
}

impl RateCustomDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Use ten stars when the domain needs more granularity.
            rate: cx.new(|cx| Rate::new(4.0, cx).max(10)),
        }
    }
}

impl Render for RateCustomDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.rate.clone()
    }
}

fn main() {}
