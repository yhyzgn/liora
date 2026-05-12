//! Basic Rate.

use aura_components::Rate;
use gpui::{AppContext, Context, Render, Window};

struct RateBasicDemo {
    rate: gpui::Entity<Rate>,
}

impl RateBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            rate: cx.new(|cx| Rate::new(3.0, cx)),
        }
    }
}

impl Render for RateBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.rate.clone()
    }
}

fn main() {}
