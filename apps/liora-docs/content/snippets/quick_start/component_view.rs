//! A small stateful GPUI View composed with Liora components.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Button, Card, Input, Space, Switch, Text, Title, toast_success};

pub struct AppView {
    name: Entity<Input>,
    enabled: Entity<Switch>,
}

impl AppView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            // Keep stateful controls as Entity fields so focus/value survive re-rendering.
            name: cx.new(|cx| Input::new("Liora", cx).placeholder("Project name")),
            enabled: cx.new(|cx| Switch::new(true, cx)),
        }
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Card::new(
            Space::new()
                .vertical()
                .gap_lg()
                .child(Title::new("Hello Liora").h2())
                .child(Text::new("This is a native GPUI element tree."))
                .child(self.name.clone())
                .child(self.enabled.clone())
                .child(
                    Button::new("Save")
                        .primary()
                        .on_click(|_, _, _| toast_success!("Saved from {}", "Liora")),
                ),
        )
        .no_shadow()
    }
}

fn main() {}
