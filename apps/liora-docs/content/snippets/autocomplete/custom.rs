//! Autocomplete with custom labels and suffix icon.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{Autocomplete, AutocompleteItem};
use liora_icons_lucide::IconName;

struct AutocompleteCustomDemo {
    input: gpui::Entity<Autocomplete>,
}

impl AutocompleteCustomDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let routes = vec![
            AutocompleteItem::labeled("/dashboard", "Dashboard"),
            AutocompleteItem::labeled("/settings", "Settings"),
            AutocompleteItem::labeled("/profile", "Profile"),
            AutocompleteItem::labeled("/billing", "Billing"),
        ];

        Self {
            input: cx.new(|cx| {
                Autocomplete::new(routes, cx)
                    .placeholder("Jump to route")
                    .width_lg()
                    .max_suggestions(4)
                    .suffix_icon(IconName::Command)
                    .close_on_click_outside(false)
                    .close_on_escape(false)
            }),
        }
    }
}

impl Render for AutocompleteCustomDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
