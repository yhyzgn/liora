//! Autocomplete without the suffix icon.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{Autocomplete, AutocompleteItem};

struct AutocompleteNoSuffixDemo {
    input: gpui::Entity<Autocomplete>,
}

impl AutocompleteNoSuffixDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let suggestions = vec![
            AutocompleteItem::labeled("rust", "Rust"),
            AutocompleteItem::labeled("gpui", "GPUI"),
            AutocompleteItem::labeled("liora", "Liora UI"),
        ];

        Self {
            input: cx.new(|cx| {
                Autocomplete::new(suggestions, cx)
                    .placeholder("No suffix icon")
                    .no_suffix_icon()
            }),
        }
    }
}

impl Render for AutocompleteNoSuffixDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.input.clone()
    }
}

fn main() {}
