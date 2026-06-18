//! RadioGroup button style.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{RadioGroup, Space};

fn city_radio_group(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(
        vec!["New York", "Washington", "Los Angeles", "Chicago"],
        1,
        cx,
    )
    .button()
}

struct RadioButtonsDemo {
    large: gpui::Entity<RadioGroup>,
    default: gpui::Entity<RadioGroup>,
    small: gpui::Entity<RadioGroup>,
    stretch: gpui::Entity<RadioGroup>,
}

impl RadioButtonsDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            large: cx.new(|cx| city_radio_group(cx).large()),
            default: cx.new(city_radio_group),
            small: cx.new(|cx| city_radio_group(cx).small()),
            stretch: cx.new(|cx| city_radio_group(cx).stretch(true)),
        }
    }
}

impl Render for RadioButtonsDemo {
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
