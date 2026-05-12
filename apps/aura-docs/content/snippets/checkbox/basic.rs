//! Basic Checkbox states.

use aura_components::{Checkbox, Space};
use gpui::{AppContext, Context, Render, Window};

struct CheckboxBasicDemo {
    checked: gpui::Entity<Checkbox>,
    unchecked: gpui::Entity<Checkbox>,
    labeled: gpui::Entity<Checkbox>,
    disabled: gpui::Entity<Checkbox>,
    disabled_checked: gpui::Entity<Checkbox>,
}

impl CheckboxBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            checked: cx.new(|cx| Checkbox::new(true, cx)),
            unchecked: cx.new(|cx| Checkbox::new(false, cx)),
            labeled: cx.new(|cx| Checkbox::new(false, cx).label("Label")),
            disabled: cx.new(|cx| Checkbox::new(false, cx).disabled(true)),
            disabled_checked: cx.new(|cx| Checkbox::new(true, cx).disabled(true)),
        }
    }
}

impl Render for CheckboxBasicDemo {
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
