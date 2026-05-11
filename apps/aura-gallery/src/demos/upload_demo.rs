use aura_components::{Space, Text, Upload, UploadFile, UploadStatus};
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| UploadDemo::new(cx)).into()
}

struct UploadDemo {
    basic: Entity<Upload>,
    drag: Entity<Upload>,
    picture: Entity<Upload>,
    limited: Entity<Upload>,
    disabled: Entity<Upload>,
}

impl UploadDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            basic: cx.new(|_| {
                Upload::new()
                    .button_text("选择文件")
                    .accept(".pdf,.fig,.txt")
                    .max_size(5 * 1024 * 1024)
                    .tip("点击选择文件会打开系统文件选择器；仅接受 pdf/fig/txt，单文件 ≤ 5MB。")
                    .width_lg()
                    .add_file(
                        UploadFile::new("spec", "产品需求说明.pdf")
                            .size(428_000)
                            .status(UploadStatus::Success),
                    )
                    .add_file(
                        UploadFile::new("draft", "设计稿-v2.fig")
                            .size(2_480_000)
                            .status(UploadStatus::Uploading)
                            .progress(68),
                    )
            }),
            drag: cx.new(|_| {
                Upload::new()
                    .drag(true)
                    .multiple(true)
                    .accept(".png,.jpg,.jpeg,.pdf")
                    .max_size(2 * 1024 * 1024)
                    .button_text("拖拽文件到这里上传")
                    .tip("点击拖拽区域会打开系统文件选择器；真实拖放接入可由宿主扩展。")
                    .width_lg()
                    .add_file(
                        UploadFile::new("error", "合同扫描件.jpg")
                            .size(820_000)
                            .status(UploadStatus::Error)
                            .description("网络中断"),
                    )
            }),
            picture: cx.new(|_| {
                Upload::new()
                    .picture_card()
                    .button_text("上传图片")
                    .multiple(true)
                    .accept("image/*")
                    .max_size(2 * 1024 * 1024)
                    .width_lg()
                    .files([
                        UploadFile::new("cover", "cover.png")
                            .size(512_000)
                            .status(UploadStatus::Success),
                        UploadFile::new("banner", "banner.jpg")
                            .size(1_240_000)
                            .status(UploadStatus::Uploading)
                            .progress(42),
                    ])
            }),
            limited: cx.new(|_| {
                Upload::new()
                    .limit(1)
                    .accept(".zip,.txt")
                    .max_size(10 * 1024 * 1024)
                    .button_text("达到数量限制")
                    .tip("limit=1 时已有文件，入口自动禁用；移除后可再次点击打开文件选择器。")
                    .width_lg()
                    .add_file(
                        UploadFile::new("only", "唯一附件.zip")
                            .size(5_120_000)
                            .status(UploadStatus::Ready),
                    )
            }),
            disabled: cx.new(|_| {
                Upload::new()
                    .disabled(true)
                    .button_text("禁用上传")
                    .tip("禁用状态下入口不可用。")
                    .width_lg()
            }),
        }
    }
}

impl Render for UploadDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Upload 上传",
            "用于呈现上传入口和文件列表，支持系统文件选择器、类型/大小/数量限制、拖拽样式、图片卡片列表、进度、状态和移除回调。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "基础文件列表",
                    "展示普通上传入口与文件状态。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.basic.clone())
                        .child(Text::new("点击“选择文件”会打开系统文件选择器；点击垃圾桶图标可从组件内部移除文件。")),
                ))
                .child(section("拖拽上传样式", "使用拖拽区域样式呈现上传入口。", self.drag.clone()))
                .child(section("图片卡片列表", "以图片卡片样式展示文件。", self.picture.clone()))
                .child(section("数量限制", "达到 limit 后入口自动禁用。", self.limited.clone()))
                .child(section("禁用状态", "整体上传入口不可操作。", self.disabled.clone())),
        )
    }
}
