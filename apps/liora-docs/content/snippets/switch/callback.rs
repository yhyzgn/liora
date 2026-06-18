//! Switch change callback.

use gpui::{AppContext, Context, Entity, Render, Window};
use liora_components::{Space, Switch, toast_info, toast_success};

struct SwitchCallbackDemo {
    switch: Entity<Switch>,
}

impl SwitchCallbackDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            switch: cx.new(|cx| {
                Switch::new(false, cx).on_change(|checked, _window, _cx| {
                    if checked {
                        toast_success!("Switch is on");
                    } else {
                        toast_info!("Switch is off");
                    }
                })
            }),
        }
    }
}

impl Render for SwitchCallbackDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new().wrap().gap_sm().child(self.switch.clone())
    }
}

fn main() {}
