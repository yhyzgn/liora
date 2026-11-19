//! Observe selection changes, including the native clear button.

use gpui::{Context, Entity, Render, SharedString, Window, prelude::*};
use liora_components::{SearchableListItem, Select, Space, Text};

struct SelectClearDemo {
    single: Entity<Select>,
    single_value: Option<SharedString>,
    select: Entity<Select>,
    values: Vec<SharedString>,
    changes: usize,
}

impl SelectClearDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let owner = cx.weak_entity();
        Self {
            single: cx.new(|cx| {
                Select::searchable(
                    vec![
                        SearchableListItem::labeled("gpui", "GPUI"),
                        SearchableListItem::labeled("liora", "Liora"),
                    ],
                    cx,
                )
                .placeholder("Choose framework")
                .on_values_change({
                    let owner = owner.clone();
                    move |values, _, cx| {
                        let _ = owner.update(cx, |this, cx| {
                            this.single_value = values.into_iter().next();
                            cx.notify();
                        });
                    }
                })
            }),
            single_value: None,
            select: cx.new(|cx| {
                Select::searchable(
                    vec![
                        SearchableListItem::new("Button"),
                        SearchableListItem::new("Select"),
                    ],
                    cx,
                )
                .multiple()
                .selected_values(vec!["Button", "Select"])
                .on_values_change(move |values, _, cx| {
                    let _ = owner.update(cx, |this, cx| {
                        this.values = values;
                        this.changes += 1;
                        cx.notify();
                    });
                })
            }),
            values: vec!["Button".into(), "Select".into()],
            changes: 0,
        }
    }
}

impl Render for SelectClearDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .child(self.single.clone())
            .child(Text::new(format!(
                "Single selected: [{}]",
                self.single_value.as_deref().unwrap_or("")
            )))
            .child(self.select.clone())
            .child(Text::new(format!(
                "Selected: {:?} · Changes: {}",
                self.values, self.changes
            )))
    }
}

fn main() {}
