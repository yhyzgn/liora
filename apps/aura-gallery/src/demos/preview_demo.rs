use aura_components::{Card, Image, Preview, Space, Text};
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, prelude::*};

use super::common::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| PreviewDemo).into()
}

struct PreviewDemo;

impl Render for PreviewDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";
        let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

        page(
            "Preview 预览",
            "为任意触发内容添加图片预览弹层，Image 的 preview 能力也基于它复用。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "图片触发",
                    "点击缩略图打开预览。",
                    row(vec![
                        Preview::new(remote)
                            .child(Image::new(remote).thumbnail().cover().preview(false)),
                        Preview::new(local.clone())
                            .child(Image::new(local.clone()).thumbnail().cover().preview(false)),
                    ]),
                ))
                .child(section(
                    "自定义触发器",
                    "Preview 可以包裹卡片、按钮或其他元素。",
                    row(vec![
                        Preview::new(local).child(
                            Card::new(
                                Space::new()
                                    .gap_md()
                                    .child(
                                        Icon::new(IconName::Image)
                                            .size_lg()
                                            .color(theme.primary.base),
                                    )
                                    .child(
                                        Space::new()
                                            .vertical()
                                            .gap_xs()
                                            .child(Text::new("点击查看大图").bold())
                                            .child(Text::new(
                                                "Preview 可以包裹卡片、按钮或其他元素。",
                                            )),
                                    ),
                            )
                            .no_shadow(),
                        ),
                    ]),
                )),
        )
    }
}
