//! Basic TodoList with stable IDs, descriptions, and initial states.

use gpui::{AppContext, Context, Render, Window};
use liora_components::{TodoItemData, TodoList};

struct BasicTodoList {
    todos: gpui::Entity<TodoList>,
}

impl BasicTodoList {
    fn new(cx: &mut Context<Self>) -> Self {
        let todos = cx.new(|cx| {
            TodoList::from_data(
                vec![
                    TodoItemData::new("brief", "Prepare the product brief")
                        .description("Confirm scope with the design team")
                        .checked(true),
                    TodoItemData::new("prototype", "Build the native prototype")
                        .description("Keep the first pass small and testable"),
                    TodoItemData::new("review", "Schedule a review").disabled(true),
                ],
                cx,
            )
        });
        Self { todos }
    }
}

impl Render for BasicTodoList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.todos.clone()
    }
}

fn main() {}
