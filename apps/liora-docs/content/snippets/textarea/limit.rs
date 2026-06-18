//! Textarea with a character limit indicator.

use gpui::{AppContext, Context, Render, Window};
use liora_components::Textarea;

struct TextareaLimitDemo {
    textarea: gpui::Entity<Textarea>,
}

impl TextareaLimitDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            textarea: cx.new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2)),
        }
    }
}

impl Render for TextareaLimitDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.textarea.clone()
    }
}

fn main() {}
