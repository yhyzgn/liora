use aura_components::{Card, Image, ImageFit, ImageRadius};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ImageDemo).into()
}

struct ImageDemo;

impl Render for ImageDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let sample =
            "https://images.unsplash.com/photo-1500530855697-b586d89ba3ee?w=640&h=420&fit=crop";

        div()
            .flex()
            .flex_col()
            .gap_8()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_lg()
                            .font_weight(gpui::FontWeight::BOLD)
                            .child("Image 图片"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("图片容器，支持不同填充方式、圆角、边框、占位与加载失败状态。"),
                    ),
            )
            .child(section(
                "基础用法",
                div()
                    .flex()
                    .gap_4()
                    .flex_wrap()
                    .child(
                        Image::new(sample)
                            .size(px(180.0), px(120.0))
                            .cover()
                            .preview(true),
                    )
                    .child(Image::new(sample).size(px(180.0), px(120.0)).contain()),
            ))
            .child(section(
                "填充方式",
                div().flex().gap_4().flex_wrap().children(
                    [
                        ("Fill", ImageFit::Fill),
                        ("Contain", ImageFit::Contain),
                        ("Cover", ImageFit::Cover),
                        ("ScaleDown", ImageFit::ScaleDown),
                    ]
                    .into_iter()
                    .map(|(label, fit)| {
                        Card::new(
                            div()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(Image::new(sample).size(px(132.0), px(88.0)).fit(fit))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.neutral.text_3)
                                        .child(label),
                                ),
                        )
                        .no_shadow()
                    }),
                ),
            ))
            .child(section(
                "形状与状态",
                div()
                    .flex()
                    .gap_4()
                    .flex_wrap()
                    .child(Image::new(sample).square(px(96.0)).cover().round())
                    .child(
                        Image::new(sample)
                            .size(px(150.0), px(96.0))
                            .cover()
                            .radius(ImageRadius::Large)
                            .shadow(true),
                    )
                    .child(
                        Image::new("aura://missing-image.png")
                            .size(px(150.0), px(96.0))
                            .alt("加载失败")
                            .fallback({
                                let theme = theme.clone();
                                move || {
                                    div()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .justify_center()
                                        .gap_2()
                                        .size_full()
                                        .text_color(theme.neutral.text_3)
                                        .child(
                                            Icon::new(IconName::ImageOff)
                                                .size(px(24.0))
                                                .color(theme.neutral.icon),
                                        )
                                        .child(div().text_xs().child("加载失败"))
                                }
                            }),
                    )
                    .child(Image::empty().size(px(150.0), px(96.0))),
            ))
    }
}

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
}
