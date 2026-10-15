use gpui::{
    AnyElement, AnyView, App, Context, Entity, Render, SharedString, Window, prelude::*, px, rgb,
};
use liora_components::layout_helpers::{page, section};
use liora_components::{
    Avatar, Button, Card, Flex, Image, Input, Progress, ProgressStatus, Space, Tag, TagType, Text,
    TodoItem, TodoItemData, TodoItemRenderContext, TodoList,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(TodoDemo::new).into()
}

struct TodoDemo {
    basic: gpui::Entity<TodoList>,
    styled: gpui::Entity<TodoList>,
    project: gpui::Entity<TodoList>,
    focus: gpui::Entity<TodoList>,
    dynamic: Entity<DynamicTodoDemo>,
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
                (
                    "ship",
                    "Ship the release",
                    "Release",
                    IconName::Rocket,
                    true,
                ),
                (
                    "docs",
                    "Refresh the docs",
                    "Docs",
                    IconName::BookOpen,
                    false,
                ),
                ("qa", "Run the smoke tests", "QA", IconName::TestTube, false),
            ];
            for (id, title, tag, icon, checked) in items {
                let item = cx.new(|cx| {
                    TodoItem::from_data(
                        TodoItemData::new(id, title)
                            .description("Every slot is replaceable without leaving native GPUI")
                            .checked(checked),
                        cx,
                    )
                    .check_renderer(custom_check)
                    .leading_renderer(move |context| leading_icon(context, icon))
                    .content_renderer(custom_content)
                    .trailing_renderer(move |context| status_tag(context, tag, TagType::Info))
                });
                list.push(item, cx);
            }
            list
        });

        let project = cx.new(|cx| {
            let mut list = TodoList::new(cx).gap(px(12.0));
            for (id, title, description, icon, progress, tag, checked, disabled) in [
                (
                    "audit",
                    "Audit design tokens",
                    "Compare the palette, type scale, and radius across every surface.",
                    IconName::Palette,
                    100.0,
                    "DONE",
                    true,
                    false,
                ),
                (
                    "contrast",
                    "Fix dark-mode contrast",
                    "Two warning states still need a readable foreground color.",
                    IconName::Moon,
                    72.0,
                    "P1",
                    false,
                    false,
                ),
                (
                    "release",
                    "Publish v0.4.0 release notes",
                    "Overdue · owner: Maya · include the migration and risk notes.",
                    IconName::Megaphone,
                    48.0,
                    "OVERDUE",
                    false,
                    false,
                ),
                (
                    "locked",
                    "Approve compliance checklist",
                    "Locked by the security team until the audit window opens.",
                    IconName::Lock,
                    0.0,
                    "LOCKED",
                    false,
                    true,
                ),
            ] {
                let item = cx.new(|cx| {
                    TodoItem::from_data(
                        TodoItemData::new(id, title)
                            .description(description)
                            .checked(checked)
                            .disabled(disabled),
                        cx,
                    )
                    .leading_renderer(move |context| project_leading(context, icon))
                    .check_renderer(project_check)
                    .content_renderer(move |context| project_content(context, progress, tag))
                    .trailing_renderer(move |context| project_trailing(context, tag))
                });
                list.push(item, cx);
            }
            list
        });

        let focus = cx.new(|cx| {
            let mut list = TodoList::new(cx).gap(px(9.0));
            for (id, title, description, icon, tag) in [
                (
                    "deep-work",
                    "Deep work block",
                    "25 minutes · uninterrupted focus",
                    IconName::Flame,
                    "25:00",
                ),
                (
                    "standup",
                    "Write async standup",
                    "Yesterday / today / blockers · 3 bullet points",
                    IconName::MessageCircle,
                    "09:30",
                ),
                (
                    "retro",
                    "Capture one retro win",
                    "Make the learning visible before closing the sprint.",
                    IconName::Sparkles,
                    "FRI",
                ),
            ] {
                let item = cx.new(|cx| {
                    TodoItem::from_data(TodoItemData::new(id, title).description(description), cx)
                        .leading_renderer(move |context| focus_leading(context, icon))
                        .check_renderer(focus_check)
                        .content_renderer(focus_content)
                        .trailing_renderer(move |_| {
                            Tag::new(tag).round(true).dark().into_any_element()
                        })
                });
                list.push(item, cx);
            }
            list
        });

        let dynamic = cx.new(DynamicTodoDemo::new);

        Self {
            basic,
            styled,
            project,
            focus,
            dynamic,
        }
    }
}

struct DynamicTodoDemo {
    todos: Entity<TodoList>,
    title_input: Entity<Input>,
    description_input: Entity<Input>,
    image_src: SharedString,
    next_id: usize,
    selected_id: Option<SharedString>,
}

impl DynamicTodoDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let todos = cx.new(|cx| {
            TodoList::from_data(
                vec![
                    TodoItemData::new("first", "Try adding a task")
                        .description("Select a task, edit it, or remove it"),
                ],
                cx,
            )
        });
        Self {
            todos,
            title_input: cx.new(|cx| Input::new("", cx).placeholder("Task title")),
            description_input: cx
                .new(|cx| Input::new("", cx).placeholder("Description (optional)")),
            image_src: format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR")).into(),
            next_id: 1,
            selected_id: None,
        }
    }

    fn add_task(&mut self, cx: &mut Context<Self>) {
        let title = self.title_input.read(cx).value();
        if title.trim().is_empty() {
            return;
        }
        let description = self.description_input.read(cx).value();
        let image_src = self.image_src.clone();
        let id: SharedString = format!("dynamic-{}", self.next_id).into();
        self.next_id += 1;
        let item = cx.new(|cx| {
            TodoItem::from_data(
                TodoItemData::new(id.clone(), title.clone())
                    .description(description.clone())
                    .checked(false),
                cx,
            )
            .leading_renderer(move |_| {
                Image::new(image_src.clone())
                    .thumbnail_sm()
                    .round()
                    .into_any_element()
            })
            .check_renderer(|context| {
                Text::new(if context.checked { "✅" } else { "🟡" }).into_any_element()
            })
            .trailing_renderer(|_| Tag::new("NEW").small().round(true).into_any_element())
        });
        self.todos.update(cx, |todos, cx| todos.push(item, cx));
        self.selected_id = Some(id);
        self.title_input
            .update(cx, |input, cx| input.set_value("", cx));
        self.description_input
            .update(cx, |input, cx| input.set_value("", cx));
        cx.notify();
    }

    fn edit_selected(&mut self, cx: &mut Context<Self>) {
        let Some(id) = self.selected_id.clone() else {
            return;
        };
        let title = self.title_input.read(cx).value();
        let description = self.description_input.read(cx).value();
        self.todos.update(cx, |todos, cx| {
            if !title.trim().is_empty() {
                todos.set_title(id.as_ref(), title, cx);
            }
            todos.set_description(id.as_ref(), Some(description), cx);
        });
        cx.notify();
    }

    fn remove_selected(&mut self, cx: &mut Context<Self>) {
        let Some(id) = self.selected_id.take() else {
            return;
        };
        self.todos.update(cx, |todos, cx| {
            todos.remove(id.as_ref(), cx);
        });
        cx.notify();
    }
}

impl Render for DynamicTodoDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        let add_view = view.clone();
        let edit_view = view.clone();
        let remove_view = view.clone();
        let selected = self
            .selected_id
            .as_ref()
            .map(|id| id.to_string())
            .unwrap_or_else(|| "none".to_owned());
        Space::new()
            .vertical()
            .gap_md()
            .child(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(self.title_input.clone())
                    .child(self.description_input.clone())
                    .child(
                        Space::new()
                            .wrap()
                            .gap_sm()
                            .child(Button::new("Add task").primary().on_click(move |_, _, cx| {
                                add_view.update(cx, |this, cx| this.add_task(cx));
                            }))
                            .child(Button::new("Edit selected").on_click(move |_, _, cx| {
                                edit_view.update(cx, |this, cx| this.edit_selected(cx));
                            }))
                            .child(Button::new("Delete selected").danger().on_click(
                                move |_, _, cx| {
                                    remove_view.update(cx, |this, cx| this.remove_selected(cx));
                                },
                            )),
                    )
                    .child(Text::new(format!("Selected: {selected}")).xs()),
            )
            .child(Card::new(self.todos.clone()).no_shadow())
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

fn leading_icon(context: TodoItemRenderContext, icon: IconName) -> AnyElement {
    let color = if context.checked {
        rgb(0x22c55e)
    } else {
        rgb(0x6366f1)
    };
    Icon::new(icon)
        .size(px(18.0))
        .color(color.into())
        .into_any_element()
}

fn status_tag(context: TodoItemRenderContext, label: &str, kind: TagType) -> AnyElement {
    let label = if context.checked { "DONE" } else { label };
    Tag::new(label.to_owned())
        .tag_type(kind)
        .small()
        .round(true)
        .into_any_element()
}

fn project_leading(context: TodoItemRenderContext, icon: IconName) -> AnyElement {
    let background = if context.disabled {
        rgb(0xe2e8f0)
    } else if context.checked {
        rgb(0xdcfce7)
    } else {
        rgb(0xede9fe)
    };
    Flex::new()
        .width_px(30.0)
        .height_px(30.0)
        .flex_none()
        .rounded_pill()
        .bg(background.into())
        .align_center()
        .justify_center()
        .child(Icon::new(icon).size(px(15.0)).into_any_element())
        .into_any_element()
}

fn project_check(context: TodoItemRenderContext) -> AnyElement {
    let (background, border) = if context.disabled {
        (rgb(0xf1f5f9), rgb(0xcbd5e1))
    } else if context.checked {
        (rgb(0x22c55e), rgb(0x16a34a))
    } else {
        (rgb(0xfffbeb), rgb(0xf59e0b))
    };
    let marker = if context.disabled {
        Icon::new(IconName::Lock)
            .size(px(13.0))
            .color(border.into())
            .into_any_element()
    } else {
        Text::new(if context.checked { "★" } else { "☆" })
            .text_color(border.into())
            .bold()
            .into_any_element()
    };
    Flex::new()
        .width_px(22.0)
        .height_px(22.0)
        .flex_none()
        .rounded_pill()
        .bg(background.into())
        .border()
        .border_color(border.into())
        .align_center()
        .justify_center()
        .child(marker)
        .into_any_element()
}

fn project_content(context: TodoItemRenderContext, progress_value: f32, label: &str) -> AnyElement {
    let title_color = if context.disabled {
        rgb(0x94a3b8)
    } else if context.checked {
        rgb(0x64748b)
    } else {
        rgb(0x0f172a)
    };
    let bar_color = if label == "OVERDUE" {
        rgb(0xef4444)
    } else if context.checked {
        rgb(0x22c55e)
    } else {
        rgb(0x6366f1)
    };
    let progress = if label == "OVERDUE" {
        Progress::new(progress_value)
            .status(ProgressStatus::Exception)
            .show_text(false)
            .animated(false)
    } else if context.checked {
        Progress::new(progress_value)
            .status(ProgressStatus::Success)
            .show_text(false)
            .animated(false)
    } else {
        Progress::new(progress_value)
            .color(bar_color.into())
            .show_text(false)
            .animated(false)
    };
    Flex::new()
        .column()
        .gap_px(5.0)
        .flex_1()
        .child(
            Text::new(context.title.clone())
                .bold()
                .text_color(title_color.into())
                .when(context.checked, |text| text.strikethrough()),
        )
        .when_some(context.description, |content, description| {
            content.child(Text::new(description).sm().text_color(rgb(0x64748b).into()))
        })
        .child(
            Flex::new()
                .row()
                .align_center()
                .gap_px(8.0)
                .child(progress)
                .child(Text::new(format!("{progress_value:.0}%")).sm().bold()),
        )
        .into_any_element()
}

fn project_trailing(context: TodoItemRenderContext, label: &str) -> AnyElement {
    let kind = if context.disabled {
        TagType::Info
    } else if context.checked {
        TagType::Success
    } else if label == "OVERDUE" {
        TagType::Danger
    } else {
        TagType::Warning
    };
    status_tag(context, label, kind)
}

fn focus_leading(context: TodoItemRenderContext, icon: IconName) -> AnyElement {
    let bg = if context.checked {
        rgb(0x14532d)
    } else {
        rgb(0x312e81)
    };
    Flex::new()
        .width_px(32.0)
        .height_px(32.0)
        .flex_none()
        .rounded_pill()
        .bg(bg.into())
        .align_center()
        .justify_center()
        .child(Icon::new(icon).size(px(16.0)).color(rgb(0xfef3c7).into()))
        .into_any_element()
}

fn focus_check(context: TodoItemRenderContext) -> AnyElement {
    let mark = if context.checked { "✓" } else { "·" };
    Flex::new()
        .width_px(24.0)
        .height_px(24.0)
        .flex_none()
        .rounded_px(8.0)
        .border()
        .border_color(rgb(0xfbbf24).into())
        .bg(if context.checked {
            rgb(0xf59e0b).into()
        } else {
            rgb(0x451a03).into()
        })
        .align_center()
        .justify_center()
        .child(Text::new(mark).text_color(rgb(0xfffbeb).into()).bold())
        .into_any_element()
}

fn focus_content(context: TodoItemRenderContext) -> AnyElement {
    Flex::new()
        .column()
        .gap_px(3.0)
        .flex_1()
        .child(
            Text::new(context.title.clone())
                .bold()
                .text_color(rgb(0x0f172a).into())
                .when(context.checked, |text| text.strikethrough()),
        )
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
                ))
                .child(section(
                    "项目交付看板",
                    "优先级、负责人语义、截止状态、锁定任务和进度条都可以在 renderer slot 中组合出来。",
                    Card::new(self.project.clone()),
                ))
                .child(section(
                    "专注任务",
                    "使用高对比浅色卡片、圆角图标、星形勾选和时间胶囊，演示另一套完全不同的视觉语言。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(
                            Flex::new()
                                .row()
                                .align_center()
                                .gap_px(8.0)
                                .child(Avatar::new().small().icon(IconName::User))
                                .child(
                                    Flex::new()
                                        .flex_1()
                                        .min_w_0()
                                        .child(Text::new("Maya 的本周专注清单").bold().nowrap()),
                                )
                                .child(
                                    Flex::new()
                                        .width_px(220.0)
                                        .flex_none()
                                        .child(Progress::new(66.0).status(ProgressStatus::Success)),
                                ),
                        )
                        .child(Card::new(self.focus.clone())),
                ))
                .child(section(
                    "动态增删改与富元素",
                    "输入标题和描述即可新增任务；选中最近新增的任务后可以编辑或删除。动态任务同时演示图片、emoji 和标签 renderer。",
                    Card::new(self.dynamic.clone()),
                )),
        )
    }
}
