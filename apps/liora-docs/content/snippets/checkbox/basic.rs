//! Basic Checkbox states.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{Checkbox, Space};

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
            checked: cx.new(|cx| Checkbox::new(true, cx).aria_label("Receive updates")),
            unchecked: cx.new(|cx| Checkbox::new(false, cx).aria_label("Enable reminders")),
            labeled: cx.new(|cx| Checkbox::new(false, cx).label("Label")),
            disabled: cx.new(|cx| Checkbox::new(false, cx).aria_label("Managed reminders").disabled(true)),
            disabled_checked: cx.new(|cx| Checkbox::new(true, cx).aria_label("Managed updates").disabled(true)),
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
