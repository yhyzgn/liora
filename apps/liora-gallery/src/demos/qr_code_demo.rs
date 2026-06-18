use gpui::{
    App, Context, Entity, IntoElement, Render, SharedString, Window, div, prelude::*, px, rgb,
};
use liora_components::{
    Button, Input, QrCode, QrFinderStyle, QrGradientDirection, QrModuleStyle, Space, Text, Title,
    Upload, toast_error, toast_success,
};

pub fn render(cx: &mut App) -> Entity<QrCodeDemo> {
    cx.new(|cx| QrCodeDemo::new(cx))
}

pub struct QrCodeDemo {
    generator_input: Entity<Input>,
    upload: Entity<Upload>,
    generated_value: SharedString,
    decode_result: SharedString,
}

impl QrCodeDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let view = cx.entity().clone();
        Self {
            generator_input: cx.new(|cx| {
                Input::new("https://github.com/yhyzgn/liora", cx)
                    .placeholder("输入要编码的字符串")
                    .clearable(true)
                    .width(px(420.0))
            }),
            upload: cx.new({
                let view = view.clone();
                move |_cx| {
                    Upload::new()
                        .accept(".png,.jpg,.jpeg,.gif,.bmp,.webp,image/*")
                        .limit(1)
                        .button_text("选择二维码图片")
                        .tip("选择本地图片后会自动识别二维码内容")
                        .width_lg()
                        .on_select({
                            let view = view.clone();
                            move |upload, cx| {
                                let Some(path) = upload.selected_paths().first().cloned() else {
                                    return;
                                };
                                let message = match QrCode::decode_file(&path) {
                                    Ok(items) if !items.is_empty() => {
                                        let content = items[0].content.to_string();
                                        format!("识别结果：{}", content)
                                    }
                                    Ok(_) => "未识别到二维码".to_string(),
                                    Err(err) => {
                                        let message = format!("识别失败：{err:?}");
                                        message
                                    }
                                };
                                upload.set_files(
                                    upload
                                        .files_ref()
                                        .iter()
                                        .cloned()
                                        .map(|file| {
                                            file.status(liora_components::UploadStatus::Success)
                                        })
                                        .collect(),
                                    cx,
                                );
                                view.update(cx, |this, cx| {
                                    this.decode_result = message.into();
                                    cx.notify();
                                });
                            }
                        })
                }
            }),
            generated_value: "https://github.com/yhyzgn/liora".into(),
            decode_result: "点击选择本地二维码图片后会自动识别".into(),
        }
    }

    fn input_value(input: &Entity<Input>, cx: &App) -> SharedString {
        input.read(cx).value()
    }
}

impl Render for QrCodeDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let view = cx.entity().clone();
        let generate_input = self.generator_input.clone();

        Space::new()
            .vertical()
            .gap_xl()
            .child(section(
                "输入字符串并生成二维码",
                Space::new()
                    .vertical()
                    .gap_md()
                    .child(
                        Space::new()
                            .wrap()
                            .gap_md()
                            .child(self.generator_input.clone())
                            .child(Button::new("生成二维码").primary().on_click({
                                let view = view.clone();
                                move |_, _, cx| {
                                    let value = QrCodeDemo::input_value(&generate_input, cx);
                                    let trimmed = value.trim();
                                    if trimmed.is_empty() {
                                        toast_error!("二维码内容不能为空");
                                        return;
                                    }

                                    view.update(cx, |this, cx| {
                                        this.generated_value = trimmed.to_string().into();
                                        cx.notify();
                                    });
                                    toast_success!("二维码已生成");
                                }
                            })),
                    )
                    .child(QrCode::new(self.generated_value.clone()).show_text(true)),
            ))
            .child(section(
                "颜色、模块风格和 Logo",
                row(vec![
                    QrCode::new("Liora primary QR")
                        .size(px(160.0))
                        .colors(rgb(0x2563eb).into(), rgb(0xeff6ff).into())
                        .module_style(QrModuleStyle::Square)
                        .finder_style(QrFinderStyle::Rounded),
                    QrCode::new("Gradient diagonal QR")
                        .size(px(180.0))
                        .dot_modules()
                        .circle_finders()
                        .foreground_gradient(
                            vec![
                                rgb(0x7c3aed).into(),
                                rgb(0x06b6d4).into(),
                                rgb(0x22c55e).into(),
                            ],
                            QrGradientDirection::ToBottomRight,
                        )
                        .background(rgb(0xf8fafc).into()),
                    QrCode::new("Gradient left QR")
                        .size(px(180.0))
                        .rounded_modules()
                        .rounded_finders()
                        .foreground_gradient(
                            vec![rgb(0xf97316).into(), rgb(0xec4899).into()],
                            QrGradientDirection::ToLeft,
                        )
                        .background(rgb(0xfffbeb).into()),
                    QrCode::new("Rounded modules with custom logo")
                        .size(px(180.0))
                        .high_recovery()
                        .rounded_modules()
                        .circle_finders()
                        .logo_text("二维")
                        .logo_background(rgb(0x22c55e).into())
                        .logo_color(gpui::white())
                        .colors(rgb(0x0f172a).into(), rgb(0xf8fafc).into()),
                ]),
            ))
            .child(section(
                "打开本地图片并识别二维码",
                Space::new()
                    .vertical()
                    .gap_md()
                    .child(self.upload.clone())
                    .child(Text::new(
                        "使用 Upload 打开本地图片；选择后会自动调用 QrCode::decode_file。",
                    ))
                    .child(result_box(self.decode_result.clone())),
            ))
    }
}

fn section(title: &str, content: impl IntoElement) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Title::new(title.to_string()).h3())
        .child(content)
}

fn row(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_lg().children(children)
}

fn result_box(content: SharedString) -> impl IntoElement {
    div()
        .w(px(420.0))
        .min_h(px(72.0))
        .px_3()
        .py_2()
        .rounded_md()
        .bg(rgb(0xf8fafc))
        .text_sm()
        .text_color(rgb(0x334155))
        .child("识别结果")
        .child(div().mt_1().text_color(rgb(0x0f172a)).child(content))
}
