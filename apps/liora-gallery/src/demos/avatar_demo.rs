use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::{Avatar, Space};
use liora_icons_lucide::IconName;

use liora_components::layout_helpers::{page, row, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AvatarDemo).into()
}

struct AvatarDemo;

impl Render for AvatarDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Avatar 头像",
            "用图标、图片或字符展示用户或事物。",
            Space::new()
                .vertical()
                .gap_lg()
                .child(section(
                    "形状",
                    "头像支持圆形和方形两种形状。",
                    row(vec![
                        Avatar::new().into_any_element(),
                        Avatar::new().square().into_any_element(),
                    ]),
                ))
                .child(section(
                    "尺寸",
                    "提供小号、默认和大号三种尺寸。",
                    row(vec![
                        Avatar::new().small().into_any_element(),
                        Avatar::new().into_any_element(),
                        Avatar::new().large().into_any_element(),
                    ]),
                ))
                .child(section(
                    "展示类型",
                    "可以展示默认图标、自定义图标或远程图片。",
                    row(vec![
                        Avatar::new().icon(IconName::User).into_any_element(),
                        Avatar::new().icon(IconName::Star).into_any_element(),
                        Avatar::new()
                            .src("https://github.com/zed-industries.png")
                            .into_any_element(),
                    ]),
                )),
        )
    }
}
