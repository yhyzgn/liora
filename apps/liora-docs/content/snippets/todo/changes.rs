//! Observe completion changes from a TodoList.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{TodoItemData, TodoList, TodoListChanged};

struct TodoChanges {
    todos: gpui::Entity<TodoList>,
}

impl TodoChanges {
    fn new(cx: &mut Context<Self>) -> Self {
        let todos = cx.new(|cx| {
            TodoList::from_data(vec![TodoItemData::new("ship", "Ship the release")], cx).on_change(
                |change: TodoListChanged, _cx| {
                    println!("{} changed: {}", change.id, change.checked);
                },
            )
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
