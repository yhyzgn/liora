//! Basic Autocomplete.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{Autocomplete, AutocompleteItem};

struct AutocompleteBasicDemo {
    input: gpui::Entity<Autocomplete>,
}

impl AutocompleteBasicDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let suggestions = vec![
            AutocompleteItem::labeled("rust", "Rust"),
            AutocompleteItem::labeled("gpui", "GPUI"),
            AutocompleteItem::labeled("liora", "Liora UI"),
        ];

        Self {
            input: cx.new(|cx| Autocomplete::new(suggestions, cx).placeholder("Search component")),
        }
    }
}

impl Render for AutocompleteBasicDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
