//! Interactive QR image recognition using the Liora Upload file picker.

use gpui::{
    AppContext, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled, Window,
    div, px, rgb,
};
use liora_components::{QrCode, Space, Upload, UploadStatus};

struct QrCodeDecodeDemo {
    upload: Entity<Upload>,
    result: SharedString,
}

impl QrCodeDecodeDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let view = cx.entity().clone();
        Self {
            upload: cx.new(move |_cx| {
                Upload::new()
                    .accept(".png,.jpg,.jpeg,.gif,.bmp,.webp,image/*")
                    .limit(1)
                    .button_text("选择二维码图片")
                    .tip("会打开本地文件选择器，选择后自动识别二维码内容")
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
                                    .map(|file| file.status(UploadStatus::Success))
                                    .collect(),
                                cx,
                            );
                            view.update(cx, |this, cx| {
                                this.result = message.into();
                                cx.notify();
                            });
                        }
                    })
            }),
            result: "等待选择本地二维码图片".into(),
        }
    }
}

impl Render for QrCodeDecodeDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.upload.clone())
            .child(result_box(self.result.clone()))
    }
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

fn main() {}
