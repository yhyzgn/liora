//! Disabled Switch states.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Space, Switch};

struct SwitchDisabledDemo {
    off: Entity<Switch>,
    on: Entity<Switch>,
}

impl SwitchDisabledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            off: cx.new(|cx| Switch::new(false, cx).disabled(true)),
            on: cx.new(|cx| Switch::new(true, cx).disabled(true)),
        }
    }
}

impl Render for SwitchDisabledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .wrap()
            .gap_sm()
            .child(self.off.clone())
            .child(self.on.clone())
    }
}

fn main() {}
