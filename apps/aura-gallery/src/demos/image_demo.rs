use aura_components::{Card, Image, ImageFit, ImageRadius, ImageRoundOptions, Space, Text};
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ImageDemo).into()
}

struct ImageDemo;

impl Render for ImageDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";
        let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

        page(
            "Image 图片",
            "图片容器，支持不同填充方式、圆角、边框、占位与加载失败状态。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础用法",
                    "展示远程、 本地与 contain 填充图片。",
                    row(vec![
                        Image::new(remote).thumbnail().cover().preview(true),
                        Image::new(local.clone()).thumbnail().cover(),
                        Image::new(local.clone()).thumbnail().contain(),
                    ]),
                ))
                .child(section(
                    "填充方式",
                    "对比不同 object-fit 效果。",
                    row([
                        ("Fill", ImageFit::Fill),
                        ("Contain", ImageFit::Contain),
                        ("Cover", ImageFit::Cover),
                        ("ScaleDown", ImageFit::ScaleDown),
                    ]
                    .into_iter()
                    .map(|(label, fit)| {
                        Card::new(
                            Space::new()
                                .vertical()
                                .gap_sm()
                                .child(Image::new(local.clone()).thumbnail_sm().fit(fit))
                                .child(Text::new(label).nowrap()),
                        )
                        .no_shadow()
                    })
                    .collect::<Vec<_>>()),
                ))
                .child(section(
                    "形状与状态",
                    "展示圆形裁剪、圆角边界、ring sleeve、阴影与空/失败状态。",
                    row(vec![
                        labeled_image(
                            Image::new(local.clone()).square_lg().cover().round(),
                            "Circle",
                        ),
                        labeled_image(
                            Image::new(local.clone())
                                .thumbnail_sm()
                                .cover()
                                .round_options(ImageRoundOptions::without_square_crop()),
                            "Round bounds",
                        ),
                        labeled_image(
                            Image::new(local.clone()).square_lg().cover().round_sleeve(),
                            "Ring sleeve",
                        ),
                        labeled_image(
                            Image::new(local.clone())
                                .thumbnail()
                                .cover()
                                .radius(ImageRadius::Large)
                                .shadow(true),
                            "Large shadow",
                        ),
                        labeled_image(
                            Image::new("aura://missing-image.png")
                                .thumbnail()
                                .alt("加载失败"),
                            "Fallback",
                        ),
                        labeled_image(Image::empty().thumbnail(), "Empty"),
                    ]),
                )),
        )
    }
}

fn labeled_image(image: Image, label: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_sm()
        .child(image)
        .child(Text::new(label).nowrap())
}
