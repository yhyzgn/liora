//! Inline Form for compact filter bars.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Button, Form, FormItem, Input, Select};

struct InlineFormDemo {
    keyword: Entity<Input>,
    status: Entity<Select>,
}

impl InlineFormDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            keyword: cx.new(|cx| Input::new("", cx).placeholder("Search keyword")),
            status: cx.new(|cx| Select::new(vec!["All", "Open", "Closed"], Some(0), cx)),
        }
    }
}

impl Render for InlineFormDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Form::new()
            .inline(true)
            .child(FormItem::new().label("Keyword").child(self.keyword.clone()))
            .child(FormItem::new().label("Status").child(self.status.clone()))
            .child(FormItem::new().child(Button::new("Search").primary()))
    }
}

fn main() {}
