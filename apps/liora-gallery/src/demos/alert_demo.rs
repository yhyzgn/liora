use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Alert, AlertType, Button, Space, Title};

use liora_components::layout_helpers::page;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| AlertDemo {
        closable: cx.new(|_| ClosableAlertDemo { visible: [true; 2] }),
    })
    .into()
}

struct AlertDemo {
    closable: Entity<ClosableAlertDemo>,
}

impl Render for AlertDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Alert 警告",
            "用于页面中展示重要的提示信息。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Alert::new("Info Alert").alert_type(AlertType::Info))
                        .child(Alert::new("Success Alert").alert_type(AlertType::Success))
                        .child(Alert::new("Warning Alert").alert_type(AlertType::Warning))
                        .child(Alert::new("Error Alert").alert_type(AlertType::Error)),
                )
                .child(Title::new("辅助性文字").h3())
                .child(
                    Alert::new("Warning")
                        .alert_type(AlertType::Warning)
                        .description("More detailed description of the warning."),
                )
                .child(Title::new("关闭与恢复").h3())
                .child(self.closable.clone()),
        )
    }
}

pub fn render_closable(cx: &mut App) -> AnyView {
    cx.new(|_| ClosableAlertDemo { visible: [true; 2] }).into()
}

struct ClosableAlertDemo {
    visible: [bool; 2],
}

impl Render for ClosableAlertDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        gpui::div()
            .flex()
            .flex_col()
            .gap_3()
            .children(self.visible.iter().enumerate().map(|(index, visible)| {
                let view = cx.entity().clone();
                if *visible {
                    Alert::new("Dismissible warning")
                        .id(("dismissible-alert", index))
                        .alert_type(AlertType::Warning)
                        .description("Close this alert, then restore it using the button.")
                        .closable(true)
                        .on_close(move |_, cx| {
                            view.update(cx, |this, cx| {
                                this.visible[index] = false;
                                cx.notify();
                            });
                        })
                        .into_any_element()
                } else {
                    Button::new(format!("Restore alert {}", index + 1))
                        .on_click(move |_, _, cx| {
                            view.update(cx, |this, cx| {
                                this.visible[index] = true;
                                cx.notify();
                            });
                        })
                        .into_any_element()
                }
            }))
    }
}
