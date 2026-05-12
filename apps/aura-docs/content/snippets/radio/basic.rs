//! Basic Radio states.

use aura_components::{Radio, Space};
use gpui::{AppContext, Context, Render, Window};

struct RadioBasicDemo {
    checked: gpui::Entity<Radio>,
    unchecked: gpui::Entity<Radio>,
    labeled: gpui::Entity<Radio>,
    disabled: gpui::Entity<Radio>,
    disabled_checked: gpui::Entity<Radio>,
}

impl RadioBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            checked: cx.new(|cx| Radio::new(true, cx)),
            unchecked: cx.new(|cx| Radio::new(false, cx)),
            labeled: cx.new(|cx| Radio::new(false, cx).label("Label")),
            disabled: cx.new(|cx| Radio::new(false, cx).disabled(true)),
            disabled_checked: cx.new(|cx| Radio::new(true, cx).disabled(true)),
        }
    }
}

impl Render for RadioBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .wrap()
            .gap_lg()
            .child(self.checked.clone())
            .child(self.unchecked.clone())
            .child(self.labeled.clone())
            .child(self.disabled.clone())
            .child(self.disabled_checked.clone())
    }
}

fn main() {}
