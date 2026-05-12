//! Basic Select.

use aura_components::Select;
use gpui::{AppContext, Context, Render, Window};

struct SelectBasicDemo {
    select: gpui::Entity<Select>,
}

impl SelectBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // `selected_idx` is zero-based; Some(1) selects Banana initially.
            select: cx.new(|cx| {
                Select::new(
                    vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"],
                    Some(1),
                    cx,
                )
            }),
        }
    }
}

impl Render for SelectBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.select.clone()
    }
}

fn main() {}
