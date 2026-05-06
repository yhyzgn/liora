use aura_components::Avatar;
use aura_core::Config;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Render, Window, div, prelude::*};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| AvatarDemo).into()
}

struct AvatarDemo;

impl Render for AvatarDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

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
                            .child("Avatar 头像"),
                    )
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child("用图标、图片或字符展示用户或事物。"),
                    ),
            )
            // Shapes
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("形状"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_4()
                            .child(Avatar::new())
                            .child(Avatar::new().square()),
                    ),
            )
            // Sizes
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("尺寸"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_4()
                            .child(Avatar::new().small())
                            .child(Avatar::new())
                            .child(Avatar::new().large()),
                    ),
            )
            // Icons
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(div().font_weight(gpui::FontWeight::BOLD).child("展示类型"))
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_4()
                            .child(Avatar::new().icon(IconName::User))
                            .child(Avatar::new().icon(IconName::Star))
                            .child(Avatar::new().src("https://github.com/zed-industries.png")),
                    ),
            )
    }
}
