//! Basic Textarea.

use aura_components::Textarea;
use gpui::{AppContext, Context, Render, Window};

struct TextareaBasicDemo {
    textarea: gpui::Entity<Textarea>,
}

impl TextareaBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            textarea: cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3)),
        }
    }
}

impl Render for TextareaBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.textarea.clone()
    }
}

fn main() {}
