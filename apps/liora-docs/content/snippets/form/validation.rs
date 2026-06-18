//! FormItem required marker and error message.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Form, FormItem, Input, Textarea};

struct ValidationFormDemo {
    title: Entity<Input>,
    description: Entity<Textarea>,
}

impl ValidationFormDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            title: cx.new(|cx| Input::new("", cx).placeholder("请输入标题")),
            description: cx.new(|cx| Textarea::new("Draft", cx).rows(3).max_length(120)),
        }
    }
}

impl Render for ValidationFormDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Form::new()
            .child(
                FormItem::new()
                    .label("Title")
                    .required(true)
                    .error("Title is required")
                    .child(self.title.clone()),
            )
            .child(
                FormItem::new()
                    .label("Description")
                    .child(self.description.clone()),
            )
    }
}

fn main() {}
