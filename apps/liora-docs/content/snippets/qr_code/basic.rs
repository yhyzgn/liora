//! Interactive QR code generation from an input string.

use gpui::{AppContext, Context, Entity, Render, SharedString, Window, px};
use liora_components::{Button, Input, QrCode, Space, toast_error, toast_success};

struct QrCodeGenerateDemo {
    input: Entity<Input>,
    generated: SharedString,
}

impl QrCodeGenerateDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| {
                Input::new("https://github.com/yhyzgn/liora", cx)
                    .placeholder("输入要编码的字符串")
                    .clearable(true)
                    .width(px(420.0))
            }),
            generated: "https://github.com/yhyzgn/liora".into(),
        }
    }
}

impl Render for QrCodeGenerateDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl gpui::IntoElement {
        let view = cx.entity().clone();
        let input = self.input.clone();

        Space::new()
            .vertical()
            .gap_md()
            .child(
                Space::new()
                    .wrap()
                    .gap_md()
                    .child(self.input.clone())
                    .child(
                        Button::new("生成二维码")
                            .primary()
                            .on_click(move |_, _, cx| {
                                let value = input.read(cx).value();
                                let trimmed = value.trim();
                                if trimmed.is_empty() {
                                    toast_error!("二维码内容不能为空");
                                    return;
                                }
                                view.update(cx, |this, cx| {
                                    this.generated = trimmed.to_string().into();
                                    cx.notify();
                                });
                                toast_success!("二维码已生成");
                            }),
                    ),
            )
            .child(QrCode::new(self.generated.clone()).show_text(true))
    }
}

fn main() {}
