use crate::{Dialog, Button};
use gpui::{
    prelude::*, App, Window, div, SharedString,
};
use std::sync::Arc;

pub struct MessageBox {
    title: SharedString,
    content: SharedString,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl MessageBox {
    pub fn new(title: impl Into<SharedString>, content: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            content: content.into(),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    pub fn close_on_escape(mut self, c: bool) -> Self {
        self.close_on_escape = c;
        self
    }

    pub fn alert(self, cx: &mut App) {
        let content = self.content.clone();
        Dialog::new()
            .title(self.title)
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_, _| {
                div().flex().flex_col().gap_4()
                    .child(content.clone())
                    .child(
                        div().flex().justify_end()
                            .child(Button::new("OK").primary().on_click(|_, _, cx| {
                                Dialog::close(cx);
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
            .close_on_click_outside(self.close_on_click_outside)
            .close_on_escape(self.close_on_escape)
            .content(move |_window, _cx| {
                let on_confirm = on_confirm.clone();
                div().flex().flex_col().gap_4()
                    .child(content.clone())
                    .child(
                        div().flex().justify_end().gap_2()
                            .child(Button::new("Cancel").on_click(|_, _, cx| {
                                Dialog::close(cx);
                            }))
                            .child(Button::new("Confirm").primary().on_click(move |_, window, cx| {
                                on_confirm(window, cx);
                                Dialog::close(cx);
                            }))
                    )
            })
            .show(cx);
    }

    pub fn close(cx: &mut App) {
        Dialog::close(cx);
    }
}

pub fn close(cx: &mut App) {
    MessageBox::close(cx);
}

pub fn alert(title: impl Into<SharedString>, content: impl Into<SharedString>, cx: &mut App) {
    MessageBox::new(title, content).alert(cx);
}

pub fn confirm(title: impl Into<SharedString>, content: impl Into<SharedString>, on_confirm: impl Fn(&mut Window, &mut App) + 'static, cx: &mut App) {
    MessageBox::new(title, content).confirm(on_confirm, cx);
}
