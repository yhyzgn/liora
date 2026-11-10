//! Business views retain task entities while filtering, sorting, and composing list chrome.

use gpui::{AppContext, Context, Render, Window, prelude::*};
use liora_components::{
    Button, Space, Text, TodoItemData, TodoList, TodoListChanged, TodoListMutation,
};

struct TodoChanges {
    todos: gpui::Entity<TodoList>,
}

impl TodoChanges {
    fn new(cx: &mut Context<Self>) -> Self {
        let todos = cx.new(|cx| {
            TodoList::from_data(vec![TodoItemData::new("ship", "Ship the release")], cx)
                .on_change(|change: TodoListChanged, _cx| {
                    println!("{} changed: {}", change.id, change.checked);
                })
                .on_mutation(|mutation: TodoListMutation, _cx| {
                    println!("{} mutation: {:?}", mutation.id, mutation.kind);
                })
                .header_renderer(|context| {
                    Text::new(format!(
                        "{} shown / {} total · {} done",
                        context.visible, context.total, context.checked
                    ))
                    .bold()
                    .into_any_element()
                })
                .empty_renderer(|_| Text::new("No matching release tasks").into_any_element())
                .contextual_footer_renderer(|context| {
                    Text::new(format!(
                        "{} tasks remaining",
                        context.total - context.checked
                    ))
                    .into_any_element()
                })
        });

        todos.update(cx, |todos, cx| {
            todos.push_data(
                TodoItemData::new("docs", "Refresh the docs")
                    .description("Add the migration notes"),
                cx,
            );
            todos.set_title("docs", "Refresh the docs and examples", cx);
            todos.set_description(
                "docs",
                Some("Include icon, image, and emoji renderer examples".into()),
                cx,
            );
            todos.set_checked("ship", true, cx);
            // Host-owned assignment IDs affect display only, not access control.
            let assigned_ids = std::collections::HashSet::from(["docs".to_owned()]);
            todos.set_filter(
                move |item| assigned_ids.contains(item.id().as_ref()) && !item.is_checked(),
                cx,
            );
            todos.set_sort(|left, right| left.title().cmp(&right.title()), cx);
        });
        Self { todos }
    }
}

impl Render for TodoChanges {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let all = self.todos.clone();
        let open = self.todos.clone();
        let insertion = self.todos.clone();
        Space::new()
            .vertical()
            .child(
                Space::new()
                    .child(Button::new("All tasks").on_click(move |_, _, cx| {
                        all.update(cx, |todos, cx| todos.clear_filter(cx));
                    }))
                    .child(Button::new("Open tasks").on_click(move |_, _, cx| {
                        open.update(cx, |todos, cx| {
                            todos.set_filter(|item| !item.is_checked(), cx)
                        });
                    }))
                    .child(Button::new("Insertion order").on_click(move |_, _, cx| {
                        insertion.update(cx, |todos, cx| todos.clear_sort(cx));
                    })),
            )
            .child(self.todos.clone())
    }
}

fn main() {}
