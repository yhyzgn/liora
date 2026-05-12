//! CheckboxGroup button style.

use aura_components::{CheckboxGroup, Space};
use gpui::{AppContext, Context, Render, Window};

fn city_checkbox_group(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(
        vec!["New York", "Washington", "Los Angeles", "Chicago"],
        vec![1],
        cx,
    )
    .button()
}

struct CheckboxButtonsDemo {
    large: gpui::Entity<CheckboxGroup>,
    default: gpui::Entity<CheckboxGroup>,
    small: gpui::Entity<CheckboxGroup>,
    stretch: gpui::Entity<CheckboxGroup>,
}

impl CheckboxButtonsDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            large: cx.new(|cx| city_checkbox_group(cx).large()),
            default: cx.new(city_checkbox_group),
            small: cx.new(|cx| city_checkbox_group(cx).small()),
            stretch: cx.new(|cx| city_checkbox_group(cx).stretch(true)),
        }
    }
}

impl Render for CheckboxButtonsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.large.clone())
            .child(self.default.clone())
            .child(self.small.clone())
            .child(self.stretch.clone())
    }
}

fn main() {}
