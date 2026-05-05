use crate::{Dialog, Button};
use gpui::{
    prelude::*, App, Window, div, SharedString,
};
use std::sync::Arc;

pub struct MessageBox {
    title: SharedString,
    content: SharedString,
}

impl MessageBox {
    pub fn new(title: impl Into<SharedString>, content: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
        }
    }

    pub fn alert(self, cx: &mut App) {
        let content = self.content.clone();
        Dialog::new()
            .title(self.title)
            .content(move |_, _| {
                div().flex().flex_col().gap_4()
                    .child(content.clone())
                    .child(
                        div().flex().justify_end()
                            .child(Button::new("OK").primary().on_click(|_, _, cx| {
                                aura_core::popper::clear_portals(cx);
                            }))
                    )
            })
            .show(cx);
    }

    pub fn confirm(self, on_confirm: impl Fn(&mut Window, &mut App) + 'static, cx: &mut App) {
        let content = self.content.clone();
        let on_confirm = Arc::new(on_confirm);
        
        Dialog::new()
            .title(self.title)
            .content(move |_window, _cx| {
                let on_confirm = on_confirm.clone();
                div().flex().flex_col().gap_4()
                    .child(content.clone())
                    .child(
                        div().flex().justify_end().gap_2()
                            .child(Button::new("Cancel").on_click(|_, _, cx| {
                                aura_core::popper::clear_portals(cx);
                            }))
                            .child(Button::new("Confirm").primary().on_click(move |_, window, cx| {
                                on_confirm(window, cx);
                                aura_core::popper::clear_portals(cx);
                            }))
                    )
            })
            .show(cx);
    }
}

pub fn alert(title: impl Into<SharedString>, content: impl Into<SharedString>, cx: &mut App) {
    MessageBox::new(title, content).alert(cx);
}

pub fn confirm(title: impl Into<SharedString>, content: impl Into<SharedString>, on_confirm: impl Fn(&mut Window, &mut App) + 'static, cx: &mut App) {
    MessageBox::new(title, content).confirm(on_confirm, cx);
}
