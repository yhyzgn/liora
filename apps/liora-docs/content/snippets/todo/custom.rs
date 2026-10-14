//! Custom checkbox, content, leading, and trailing todo slots.

use gpui::{AnyElement, AppContext, Context, Render, Window, prelude::*, px, rgb};
use liora_components::{Tag, TagType, Text, TodoItem, TodoItemRenderContext, TodoList};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

fn custom_check(context: TodoItemRenderContext) -> AnyElement {
    let mark = if context.checked { "✓" } else { "○" };
    Text::new(mark)
        .bold()
        .text_color(rgb(0x16a34a).into())
        .into_any_element()
}

fn custom_content(context: TodoItemRenderContext) -> AnyElement {
    let title = Text::new(context.title.clone());
    let title = if context.checked {
        title.strikethrough().text_color(rgb(0x64748b).into())
    } else {
        title.bold()
    };
    title.into_any_element()
}

struct CustomTodoList {
    todos: gpui::Entity<TodoList>,
}

impl CustomTodoList {
    fn new(cx: &mut Context<Self>) -> Self {
        let todos = cx.new(|cx| {
            let mut list = TodoList::new(cx).gap(px(8.0));
            let item = cx.new(|cx| {
                TodoItem::new("docs", "Refresh the docs", cx)
                    .check_renderer(custom_check)
                    .leading_renderer(|_| {
                        Icon::new(IconName::BookOpen)
                            .size(px(16.0))
                            .into_any_element()
                    })
                    .content_renderer(custom_content)
                    .trailing_renderer(|_| {
                        Tag::new("Docs").tag_type(TagType::Info).into_any_element()
                    })
            });
            list.push(item, cx);
            list
        });
        Self { todos }
    }
}

impl Render for CustomTodoList {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        self.todos.clone()
    }
}

fn main() {}
