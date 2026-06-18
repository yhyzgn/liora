//! Basic Select.

use gpui::{AppContext, Context, Render, Window};
use liora_components::Select;

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
                .close_on_click_outside(false)
                .close_on_escape(false)
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
