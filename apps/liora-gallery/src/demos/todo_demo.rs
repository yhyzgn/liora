use gpui::{AnyElement, AnyView, App, Context, Render, Window, prelude::*, px, rgb};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Card, Flex, Space, Tag, TagType, Text, TodoItem, TodoItemData, TodoItemRenderContext, TodoList,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(TodoDemo::new).into()
}

struct TodoDemo {
    basic: gpui::Entity<TodoList>,
    styled: gpui::Entity<TodoList>,
}

impl TodoDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|cx| {
            TodoList::from_data(
                vec![
                    TodoItemData::new("brief", "Prepare the product brief")
                        .description("Confirm scope with the design team")
                        .checked(true),
                    TodoItemData::new("prototype", "Build the native prototype")
                        .description("Keep the first pass small and testable"),
                    TodoItemData::new("review", "Schedule a review")
                        .description("This task is currently unavailable")
                        .disabled(true),
                ],
                cx,
            )
            .gap(px(8.0))
        });

        let styled = cx.new(|cx| {
            let mut list = TodoList::new(cx).gap(px(10.0));
            let items = [
                ("ship", "Ship the release", "Release", IconName::Rocket),
                ("docs", "Refresh the docs", "Docs", IconName::BookOpen),
                ("qa", "Run the smoke tests", "QA", IconName::TestTube),
            ];
            for (id, title, tag, icon) in items {
                let item = cx.new(|cx| {
                    TodoItem::new(id, title, cx)
                        .check_renderer(|context| custom_check(context))
                        .leading_renderer(move |_| {
                            Icon::new(icon)
                                .size(px(16.0))
                                .color(rgb(0x64748b).into())
                                .into_any_element()
                        })
                        .content_renderer(move |context| custom_content(context))
                        .trailing_renderer(move |_| {
                            Tag::new(tag).tag_type(TagType::Info).into_any_element()
                        })
                });
                list.push(item, cx);
            }
            list
        });

        Self { basic, styled }
    }
}

fn custom_check(context: TodoItemRenderContext) -> AnyElement {
    let color = if context.checked {
        rgb(0x16a34a)
    } else {
        rgb(0x94a3b8)
    };
    let mark = if context.checked { "✓" } else { "○" };
    Text::new(mark)
        .text_color(color.into())
        .bold()
        .into_any_element()
}

fn custom_content(context: TodoItemRenderContext) -> AnyElement {
    let title = Text::new(context.title.clone()).bold();
    let title = if context.checked {
        title.text_color(rgb(0x64748b).into()).strikethrough()
    } else {
        title.text_color(rgb(0x0f172a).into())
    };
    Flex::new()
        .column()
        .gap_px(4.0)
        .flex_1()
        .child(title)
        .when_some(context.description, |content, description| {
            content.child(Text::new(description).sm().text_color(rgb(0x64748b).into()))
        })
        .into_any_element()
}

impl Render for TodoDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "TodoList 待办列表",
            "专用待办控件复用 Checkbox 的交互语义，同时开放勾选、文本和尾部内容的自定义渲染。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础待办",
                    "数据构造器支持稳定 ID、描述、完成状态和禁用状态。",
                    Card::new(self.basic.clone()),
                ))
                .child(section(
                    "自定义视觉",
                    "leading_renderer、check_renderer、content_renderer 和 trailing_renderer 可以组合出图标、删除线和状态标签。",
                    Card::new(self.styled.clone()),
                )),
        )
    }
}
