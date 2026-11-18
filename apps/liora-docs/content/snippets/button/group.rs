//! Stateful grouped actions, matching the Gallery and Docs live demo.

use gpui::{App, AppContext, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::{Button, ButtonGroup, Space, Text};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

struct ButtonGroupDemo {
    last_action: &'static str,
}

fn button_groups(cx: &mut App) -> Entity<ButtonGroupDemo> {
    cx.new(|_| ButtonGroupDemo {
        last_action: "None",
    })
}

impl Render for ButtonGroupDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_sm()
            .child(
                ButtonGroup::new()
                    .button(
                        Button::new("Previous")
                            .icon_start(Icon::new(IconName::ArrowLeft))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.last_action = "Previous";
                                cx.notify();
                            })),
                    )
                    .button(
                        Button::new("Next")
                            .icon_end(Icon::new(IconName::ArrowRight))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.last_action = "Next";
                                cx.notify();
                            })),
                    ),
            )
            .child(
                ButtonGroup::new()
                    .button(
                        Button::new("Edit")
                            .icon_start(Icon::new(IconName::Pencil))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.last_action = "Edit";
                                cx.notify();
                            })),
                    )
                    .button(
                        Button::new("Share")
                            .icon_start(Icon::new(IconName::Share2))
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.last_action = "Share";
                                cx.notify();
                            })),
                    )
                    .button(
                        Button::new("Delete")
                            .icon_start(Icon::new(IconName::Trash2))
                            .disabled(true)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.last_action = "Delete";
                                cx.notify();
                            })),
                    ),
            )
            .child(Text::new(format!("Last action: {}", self.last_action)))
    }
}

fn main() {}
