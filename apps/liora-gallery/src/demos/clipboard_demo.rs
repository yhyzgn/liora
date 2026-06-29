use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_grid};
use liora_components::{Space, Tag, Text, clipboard_text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ClipboardDemo).into()
}

struct ClipboardDemo;

impl Render for ClipboardDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let item = clipboard_text("Liora");
        page(
            "Clipboard 剪贴板辅助",
            "原生文本剪贴板 helper，供复制按钮、代码块和菜单动作复用。",
            Space::new().vertical().gap_xl().child(section(
                "文本复制 payload",
                "事件处理里可把 helper 生成的 ClipboardItem 写入 GPUI clipboard。",
                showcase_grid(vec![
                    Space::new()
                        .vertical()
                        .gap_sm()
                        .child(Tag::new("Helper").info())
                        .child(Text::new(format!("Prepared item: {item:?}")).wrap())
                        .into_any_element(),
                ]),
            )),
        )
    }
}
