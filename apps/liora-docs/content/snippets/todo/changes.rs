//! Observe completion changes from a TodoList.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{TodoItemData, TodoList, TodoListChanged, TodoListMutation};

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
            todos.remove("ship", cx);
        });
        Self { todos }
    }
}

impl Render for TodoChanges {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.todos.clone()
    }
}

fn main() {}
