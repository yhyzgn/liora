use aura_components::{Button, Input, Tag};
use aura_core::Config;
use gpui::{AnyView, App, Context, Entity, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| TagDemo {
        tags: vec!["Tag 1".into(), "Tag 2".into(), "Tag 3".into()],
        input: cx.new(|cx| Input::new("", cx)),
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
        let theme = &cx.global::<Config>().theme;

        div()
            .flex()
            .flex_col()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Tag 标签"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用于标记和选择。"),
                    ),
            )
            // Dynamic Tags
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("动态添加和移除"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .children(self.tags.iter().enumerate().map(|(idx, label)| {
                                let label_clone = label.clone();
                                Tag::new(label_clone).closable(true).on_close({
                                    let view = cx.entity().clone();
                                    move |_, cx| {
                                        view.update(cx, |view: &mut Self, cx| {
                                            view.tags.remove(idx);
                                            cx.notify();
                                        });
                                    }
                                })
                            }))
                            .child({
                                let view_handle = cx.entity().clone();
                                if self.is_input_visible {
                                    // Set up the input for this frame
                                    cx.update_entity(&self.input, |input, cx| {
                                        input.set_placeholder("Tag Name", cx);
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

                                    div().w_24().child(self.input.clone()).into_any_element()
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
            )
            // Basic Tags
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("基础用法"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(Tag::new("Tag 1"))
                            .child(Tag::new("Tag 2").success())
                            .child(Tag::new("Tag 3").warning())
                            .child(Tag::new("Tag 4").danger()),
                    ),
            )
            // Closable Tags
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("可移除标签"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(Tag::new("Tag 1").closable(true))
                            .child(Tag::new("Tag 2").success().closable(true))
                            .child(Tag::new("Tag 3").warning().closable(true))
                            .child(Tag::new("Tag 4").danger().closable(true)),
                    ),
            )
            // Effects
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("不同主题"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(Tag::new("Dark").dark())
                            .child(Tag::new("Success").success().dark())
                            .child(Tag::new("Warning").warning().dark())
                            .child(Tag::new("Danger").danger().dark()),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(Tag::new("Plain").plain())
                            .child(Tag::new("Success").success().plain())
                            .child(Tag::new("Warning").warning().plain())
                            .child(Tag::new("Danger").danger().plain()),
                    ),
            )
            // Sizes
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("不同尺寸"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(Tag::new("Default"))
                            .child(Tag::new("Large").large())
                            .child(Tag::new("Small").small()),
                    ),
            )
            // Round Tags
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("圆角按钮"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(Tag::new("Tag 1").round(true))
                            .child(Tag::new("Tag 2").success().round(true))
                            .child(Tag::new("Tag 3").warning().round(true))
                            .child(Tag::new("Tag 4").danger().round(true)),
                    ),
            )
    }
}
