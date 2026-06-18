//! Basic Form with labeled native fields.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Form, FormItem, Input, Select, Space, Switch};

struct BasicFormDemo {
    name: Entity<Input>,
    role: Entity<Select>,
    enabled: Entity<Switch>,
}

impl BasicFormDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            name: cx.new(|cx| Input::new("Liora", cx).placeholder("Name")),
            role: cx.new(|cx| Select::new(vec!["Admin", "Editor", "Viewer"], Some(0), cx)),
            enabled: cx.new(|cx| Switch::new(true, cx)),
        }
    }
}

impl Render for BasicFormDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Form::new()
            .child(
                FormItem::new()
                    .label("Name")
                    .required(true)
                    .child(self.name.clone()),
            )
            .child(FormItem::new().label("Role").child(self.role.clone()))
            .child(FormItem::new().label("Enabled").child(self.enabled.clone()))
    }
}

fn main() {}
