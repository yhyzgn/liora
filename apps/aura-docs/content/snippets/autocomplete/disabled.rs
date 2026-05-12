//! Disabled Autocomplete.

use aura_components::{Autocomplete, AutocompleteItem};
use gpui::{AppContext, Context, Render, Window};

struct AutocompleteDisabledDemo {
    input: gpui::Entity<Autocomplete>,
}

impl AutocompleteDisabledDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let suggestions = vec![
            AutocompleteItem::labeled("rust", "Rust"),
            AutocompleteItem::labeled("gpui", "GPUI"),
            AutocompleteItem::labeled("aura", "Aura UI"),
        ];

        Self {
            input: cx.new(|cx| {
                Autocomplete::new(suggestions, cx)
                    .placeholder("Disabled")
                    .disabled(true)
            }),
        }
    }
}

impl Render for AutocompleteDisabledDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
