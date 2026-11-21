//! Two sibling alerts with stable IDs and parent-owned dismissal and restoration.

use gpui::{Context, Render, Window, prelude::*};
use liora_components::{Alert, AlertType, Button};

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

fn main() {
    let _ = std::any::TypeId::of::<ClosableAlertDemo>();
}
