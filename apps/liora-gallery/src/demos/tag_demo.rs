use gpui::{AnyView, App, Context, Entity, Pixels, Render, Window, prelude::*};
use liora_components::{Button, Card, Input, Space, Tag, TagFlow};

use liora_components::layout_helpers::{page, row_md, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TagDemo {
        tags: vec!["Tag 1".into(), "Tag 2".into(), "Tag 3".into()],
        input: cx.new(|cx| Input::new("", cx).width_sm()),
        is_input_visible: false,
    })
    .into()
}

struct TagDemo {
    tags: Vec<String>,
    input: Entity<Input>,
    is_input_visible: bool,
}

impl Render for TagDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Tag 标签",
            "用于标记和选择。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "动态添加和移除",
                    "通过输入框新增标签，点击关闭图标移除标签。",
                    Card::new(
                        Space::new()
                            .gap_md()
                            .wrap()
                            .children(self.tags.iter().enumerate().map(|(idx, label)| {
                                let label_clone = label.clone();
                                Tag::new(label_clone).closable(true).on_close({
                                    let view = cx.entity().clone();
                                    move |_, cx| {
                                        view.update(cx, |view: &mut Self, cx| {
                                            if idx < view.tags.len() {
                                                view.tags.remove(idx);
                                            }
                                            cx.notify();
                                        });
                                    }
                                })
                            }))
                            .child({
                                let view_handle = cx.entity().clone();
                                if self.is_input_visible {
                                    cx.update_entity(&self.input, |input, cx| {
                                        input.set_placeholder("Tag Name", cx);
                                        input.set_width_sm(cx);
                                        input.set_on_enter(
                                            {
                                                let view_handle = view_handle.clone();
                                                move |input, value, _, cx| {
                                                    if !value.trim().is_empty() {
                                                        input.set_value("", cx);
                                                        view_handle.update(
                                                            cx,
                                                            |view: &mut Self, cx| {
                                                                view.tags
                                                                    .push(value.trim().to_string());
                                                                view.is_input_visible = false;
                                                                cx.notify();
                                                            },
                                                        );
                                                    } else {
                                                        view_handle.update(cx, |view, cx| {
                                                            view.is_input_visible = false;
                                                            cx.notify();
                                                        });
                                                    }
                                                }
                                            },
                                            cx,
                                        );
                                    });

                                    self.input.clone().into_any_element()
                                } else {
                                    Button::new("+ New Tag")
                                        .small()
                                        .on_click(move |_, window, cx| {
                                            view_handle.update(cx, |view, cx| {
                                                view.is_input_visible = true;
                                                cx.focus_view(&view.input, window);
                                                cx.notify();
                                            });
                                        })
                                        .into_any_element()
                                }
                            }),
                    ),
                ))
                .child(section(
                    "基础用法",
                    "展示不同语义类型。",
                    row_md(vec![
                        Tag::new("Tag 1").into_any_element(),
                        Tag::new("Tag 2").success().into_any_element(),
                        Tag::new("Tag 3").warning().into_any_element(),
                        Tag::new("Tag 4").danger().into_any_element(),
                    ]),
                ))
                .child(section(
                    "可移除标签",
                    "closable 标签会展示关闭图标。",
                    row_md(vec![
                        Tag::new("Tag 1").closable(true).into_any_element(),
                        Tag::new("Tag 2")
                            .success()
                            .closable(true)
                            .into_any_element(),
                        Tag::new("Tag 3")
                            .warning()
                            .closable(true)
                            .into_any_element(),
                        Tag::new("Tag 4").danger().closable(true).into_any_element(),
                    ]),
                ))
                .child(section(
                    "不同主题",
                    "Dark 与 Plain 展示不同视觉效果。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(row_md(vec![
                            Tag::new("Dark").dark().into_any_element(),
                            Tag::new("Success").success().dark().into_any_element(),
                            Tag::new("Warning").warning().dark().into_any_element(),
                            Tag::new("Danger").danger().dark().into_any_element(),
                        ]))
                        .child(row_md(vec![
                            Tag::new("Plain").plain().into_any_element(),
                            Tag::new("Success").success().plain().into_any_element(),
                            Tag::new("Warning").warning().plain().into_any_element(),
                            Tag::new("Danger").danger().plain().into_any_element(),
                        ])),
                ))
                .child(section(
                    "不同尺寸",
                    "提供默认、大号和小号尺寸。",
                    row_md(vec![
                        Tag::new("Default").into_any_element(),
                        Tag::new("Large").large().into_any_element(),
                        Tag::new("Small").small().into_any_element(),
                    ]),
                ))
                .child(section(
                    "圆角按钮",
                    "round 模式展示胶囊形标签。",
                    row_md(vec![
                        Tag::new("Tag 1").round(true).into_any_element(),
                        Tag::new("Tag 2").success().round(true).into_any_element(),
                        Tag::new("Tag 3").warning().round(true).into_any_element(),
                        Tag::new("Tag 4").danger().round(true).into_any_element(),
                    ]),
                ))
                .child(section(
                    "流式布局",
                    "TagFlow 会自动换行，适合标签云、筛选条件和多选结果展示。",
                    Card::new(
                        TagFlow::new([
                            Tag::new("Design").round(true),
                            Tag::new("GPUI").success().round(true),
                            Tag::new("Animation").warning().round(true),
                            Tag::new("Native Rust").danger().round(true),
                            Tag::new("Charts").round(true),
                            Tag::new("Docs").success().round(true),
                            Tag::new("Installer").warning().round(true),
                            Tag::new("Tray").round(true),
                        ])
                        .gap(Pixels::from(10.0_f32))
                        .max_rows(2)
                        .estimated_items_per_row(4)
                        .overflow_indicator("更多"),
                    ),
                )),
        )
    }
}
