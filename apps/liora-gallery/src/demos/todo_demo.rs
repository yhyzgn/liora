use std::collections::HashMap;

use gpui::{
    AnyElement, AnyView, App, Context, Entity, MouseButton, Render, SharedString, Window,
    prelude::*, px, rgb,
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

struct TodoEditorState {
    title: Entity<Input>,
    description: Entity<Input>,
    is_new: bool,
}

struct DynamicTodoDemo {
    todos: Entity<TodoList>,
    editors: HashMap<String, TodoEditorState>,
    image_src: SharedString,
    next_id: usize,
}

impl DynamicTodoDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let owner = cx.entity().clone();
        let todos = cx.new(|cx| {
            TodoList::new(cx).footer_renderer({
                let owner = owner.clone();
                move || todo_add_button(owner.clone())
            })
        });
        let mut demo = Self {
            todos,
            editors: HashMap::new(),
            image_src: format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR")).into(),
            next_id: 1,
        };
        let item = demo.create_item(
            TodoItemData::new("first", "Try inline editing")
                .description("Use the row actions to edit, or the + button below to add a task."),
            false,
            cx,
        );
        demo.todos.update(cx, |todos, cx| todos.push(item, cx));
        demo
    }

    fn create_item(
        &mut self,
        data: TodoItemData,
        is_new: bool,
        cx: &mut Context<Self>,
    ) -> Entity<TodoItem> {
        let id = data.id.clone();
        let title_value = data.title.clone();
        let description_value = data.description.clone().unwrap_or_default();
        let title_input = cx.new(|cx| {
            Input::new(title_value.clone(), cx)
                .placeholder("Task title")
                .clearable(true)
        });
        let description_input = cx.new(|cx| {
            Input::new(description_value.clone(), cx)
                .placeholder("Description (optional)")
                .clearable(true)
        });
        self.editors.insert(
            id.to_string(),
            TodoEditorState {
                title: title_input.clone(),
                description: description_input.clone(),
                is_new,
            },
        );

        let owner = cx.entity().clone();
        let editor_owner = owner.clone();
        let editor_id = id.clone();
        let action_owner = owner.clone();
        let action_id = id.clone();
        let image_src = self.image_src.clone();
        cx.new(|cx| {
            TodoItem::from_data(data, cx)
                .leading_renderer(move |_| {
                    Image::new(image_src.clone())
                        .thumbnail_sm()
                        .round()
                        .into_any_element()
                })
                .check_renderer(|context| {
                    Text::new(if context.checked { "✅" } else { "🟡" }).into_any_element()
                })
                .content_renderer(dynamic_content)
                .editor_renderer(move |_| {
                    inline_todo_editor(
                        editor_owner.clone(),
                        editor_id.clone(),
                        title_input.clone(),
                        description_input.clone(),
                    )
                })
                .trailing_renderer(move |context| {
                    todo_item_actions(action_owner.clone(), action_id.clone(), context.editing)
                })
        })
    }

    fn begin_new(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let id: SharedString = format!("dynamic-{}", self.next_id).into();
        self.next_id += 1;
        let item = self.create_item(TodoItemData::new(id.clone(), ""), true, cx);
        self.todos.update(cx, |todos, cx| {
            todos.push(item, cx);
            todos.set_editing(id.as_ref(), true, cx);
        });
        self.focus_title(&id, window, cx);
        cx.notify();
    }

    fn begin_edit(&mut self, id: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        let Some(item) = self.todos.read(cx).item(id.as_ref(), cx) else {
            return;
        };
        let (title, description) = {
            let item = item.read(cx);
            (item.title(), item.description_value().unwrap_or_default())
        };
        if let Some(editor) = self.editors.get(id.as_ref()) {
            editor
                .title
                .update(cx, |input, cx| input.set_value(title, cx));
            editor
                .description
                .update(cx, |input, cx| input.set_value(description, cx));
        }
        self.todos
            .update(cx, |todos, cx| todos.set_editing(id.as_ref(), true, cx));
        self.focus_title(&id, window, cx);
        cx.notify();
    }

    fn focus_title(&self, id: &str, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(editor) = self.editors.get(id) {
            cx.focus_view(&editor.title, window);
        }
    }

    fn save_edit(&mut self, id: SharedString, cx: &mut Context<Self>) {
        let Some(editor) = self.editors.get(id.as_ref()) else {
            return;
        };
        let title = editor.title.read(cx).value();
        if title.trim().is_empty() {
            return;
        }
        let description = editor.description.read(cx).value();
        self.todos.update(cx, |todos, cx| {
            todos.set_title(id.as_ref(), title.trim().to_owned(), cx);
            let description = description.trim();
            todos.set_description(
                id.as_ref(),
                (!description.is_empty()).then(|| description.to_owned().into()),
                cx,
            );
            todos.set_editing(id.as_ref(), false, cx);
        });
        if let Some(editor) = self.editors.get_mut(id.as_ref()) {
            editor.is_new = false;
        }
        cx.notify();
    }

    fn cancel_edit(&mut self, id: SharedString, cx: &mut Context<Self>) {
        let is_new = self
            .editors
            .get(id.as_ref())
            .is_some_and(|editor| editor.is_new);
        if is_new {
            self.todos.update(cx, |todos, cx| {
                todos.remove(id.as_ref(), cx);
            });
            self.editors.remove(id.as_ref());
        } else {
            self.todos
                .update(cx, |todos, cx| todos.set_editing(id.as_ref(), false, cx));
        }
        cx.notify();
    }

    fn remove_task(&mut self, id: SharedString, cx: &mut Context<Self>) {
        self.todos.update(cx, |todos, cx| {
            todos.remove(id.as_ref(), cx);
        });
        self.editors.remove(id.as_ref());
        cx.notify();
    }
}

impl Render for DynamicTodoDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Card::new(self.todos.clone()).no_shadow()
    }
}

fn todo_add_button(owner: Entity<DynamicTodoDemo>) -> AnyElement {
    Button::new("Add task")
        .small()
        .primary()
        .icon_start(IconName::Plus)
        .on_click(move |_, window, cx| {
            cx.stop_propagation();
            owner.update(cx, |demo, cx| demo.begin_new(window, cx));
        })
        .into_any_element()
}

fn inline_todo_editor(
    owner: Entity<DynamicTodoDemo>,
    id: SharedString,
    title: Entity<Input>,
    description: Entity<Input>,
) -> AnyElement {
    let save_owner = owner.clone();
    let save_id = id.clone();
    let cancel_owner = owner;
    let cancel_id = id;
    Flex::new()
        .column()
        .gap_px(6.0)
        .flex_1()
        .child(title)
        .child(description)
        .child(
            Space::new()
                .gap_xs()
                .child(
                    Button::new("Save")
                        .small()
                        .success()
                        .icon_start(IconName::Check)
                        .on_click(move |_, _, cx| {
                            cx.stop_propagation();
                            save_owner.update(cx, |demo, cx| demo.save_edit(save_id.clone(), cx));
                        }),
                )
                .child(
                    Button::new("Cancel")
                        .small()
                        .icon_start(IconName::X)
                        .on_click(move |_, _, cx| {
                            cx.stop_propagation();
                            cancel_owner
                                .update(cx, |demo, cx| demo.cancel_edit(cancel_id.clone(), cx));
                        }),
                ),
        )
        .into_any_element()
}

fn todo_item_actions(
    owner: Entity<DynamicTodoDemo>,
    id: SharedString,
    editing: bool,
) -> AnyElement {
    if editing {
        return Space::new().into_any_element();
    }
    let edit_owner = owner.clone();
    let edit_id = id.clone();
    let delete_owner = owner;
    let delete_id = id;
    Space::new()
        .gap_xs()
        .child(stop_row_toggle(
            Button::new("")
                .small()
                .icon_only(IconName::Pencil)
                .on_click(move |_, window, cx| {
                    cx.stop_propagation();
                    edit_owner.update(cx, |demo, cx| demo.begin_edit(edit_id.clone(), window, cx));
                }),
        ))
        .child(stop_row_toggle(
            Button::new("")
                .small()
                .danger()
                .icon_only(IconName::Trash2)
                .on_click(move |_, _, cx| {
                    cx.stop_propagation();
                    delete_owner.update(cx, |demo, cx| demo.remove_task(delete_id.clone(), cx));
                }),
        ))
        .into_any_element()
}

fn stop_row_toggle(button: Button) -> AnyElement {
    gpui::div()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .on_mouse_up(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .child(button)
        .into_any_element()
}

fn dynamic_content(context: TodoItemRenderContext) -> AnyElement {
    Flex::new()
        .column()
        .gap_px(4.0)
        .flex_1()
        .child(
            Text::new(context.title.clone())
                .bold()
                .when(context.checked, |text| text.strikethrough()),
        )
        .when_some(context.description, |content, description| {
            content.child(Text::new(description).sm().text_color(rgb(0x64748b).into()))
        })
        .into_any_element()
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
