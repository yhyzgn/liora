use aura_components::{Card, Image, Preview};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*, px};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PreviewDemo).into()
}

struct PreviewDemo;

impl Render for PreviewDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";
        let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

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
                            .child("Preview 预览"),
                    )
                    .child(div().text_sm().text_color(theme.neutral.text_3).child(
                        "为任意触发内容添加图片预览弹层，Image 的 preview 能力也基于它复用。",
                    )),
            )
            .child(section(
                "图片触发",
                div()
                    .flex()
                    .gap_4()
                    .flex_wrap()
                    .child(
                        Preview::new(remote).child(
                            Image::new(remote)
                                .size(px(180.0), px(120.0))
                                .cover()
                                .preview(false),
                        ),
                    )
                    .child(
                        Preview::new(local.clone()).child(
                            Image::new(local.clone())
                                .size(px(180.0), px(120.0))
                                .cover()
                                .preview(false),
                        ),
                    ),
            ))
            .child(section(
                "自定义触发器",
                div().flex().gap_4().flex_wrap().child(
                    Preview::new(local).child(
                        Card::new(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    Icon::new(IconName::Image)
                                        .size(px(24.0))
                                        .color(theme.primary.base),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(
                                            div()
                                                .font_weight(gpui::FontWeight::BOLD)
                                                .child("点击查看大图"),
                                        )
                                        .child(
                                            div()
                                                .text_sm()
                                                .text_color(theme.neutral.text_3)
                                                .child("Preview 可以包裹卡片、按钮或其他元素。"),
                                        ),
                                ),
                        )
                        .no_shadow(),
                    ),
                ),
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
