use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Input, Space, Text, write_text_to_clipboard};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| ClipboardDemo {
        source: cx.new(|cx| Input::new("Liora 原生剪贴板 🦀", cx).clearable(true)),
        feedback: "编辑文本后复制，再读取系统剪贴板验证。".into(),
        read_back: None,
    })
    .into()
}

struct ClipboardDemo {
    source: Entity<Input>,
    feedback: String,
    read_back: Option<String>,
}

impl Render for ClipboardDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Clipboard 剪贴板辅助",
            "原生文本剪贴板 helper，供复制按钮、代码块和菜单动作复用。",
            section(
                "复制与读取",
                "复制会替换系统剪贴板内容；也可以先在其他应用复制文字，再点击读取。",
                Card::new(
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(Text::new("待复制文本").bold())
                        .child(self.source.clone())
                        .child(
                            Space::new()
                                .wrap()
                                .gap_sm()
                                .child(Button::new("复制到系统剪贴板").primary().on_click(
                                    cx.listener(|this, _, _, cx| {
                                        let text = this.source.read(cx).value().to_string();
                                        write_text_to_clipboard(cx, text);
                                        this.feedback =
                                            "已发送复制请求，点击读取验证实际内容。".into();
                                        this.read_back = None;
                                        cx.notify();
                                    }),
                                ))
                                .child(Button::new("读取 / 粘贴剪贴板").on_click(cx.listener(
                                    |this, _, _, cx| {
                                        this.read_back =
                                            cx.read_from_clipboard().and_then(|item| item.text());
                                        this.feedback = match &this.read_back {
                                            Some(_) => "已读取系统剪贴板中的文本。",
                                            None => "剪贴板为空或不包含可读取的文本。",
                                        }
                                        .into();
                                        cx.notify();
                                    },
                                ))),
                        )
                        .child(Text::new(self.feedback.clone()).wrap())
                        .child(Text::new("最近一次读取结果（不会自动刷新）").bold())
                        .child(
                            Text::new(
                                self.read_back
                                    .clone()
                                    .unwrap_or_else(|| "尚无文本读取结果".into()),
                            )
                            .wrap(),
                        ),
                ),
            ),
        )
    }
}
