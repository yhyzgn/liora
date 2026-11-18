//! Accessibility metadata groups a retained native Input without replacing its keyboard behavior.

use gpui::{App, Context, Entity, Render, Role, Subscription, Window, prelude::*};
use liora_components::{Accessible, Input, Space, Text};

fn accessible_input(cx: &mut App) -> Entity<AccessibleDemo> {
    cx.new(AccessibleDemo::new)
}

struct AccessibleDemo {
    input: Entity<Input>,
    _subscription: Subscription,
}

impl AccessibleDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let input = cx.new(|cx| Input::new("", cx).placeholder("Type a project name"));
        let subscription = cx.observe(&input, |_, _, cx| cx.notify());
        Self {
            input,
            _subscription: subscription,
        }
    }
}

impl Render for AccessibleDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new("Project name"))
            .child(
                Accessible::new("layout.project-name", self.input.clone())
                    .role(Role::Group)
                    .label("Project name")
                    .description(
                        "Enter the project name; the native input retains keyboard editing.",
                    ),
            )
            .child(Text::new(format!(
                "Project value: {}",
                self.input.read(cx).value()
            )))
    }
}

fn main() {}
