use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_grid};
use liora_components::{ScrollableMask, Space, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| ScrollableMaskDemo).into()
}

struct ScrollableMaskDemo;

impl Render for ScrollableMaskDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "ScrollableMask 滚动渐隐",
            "为滚动内容提供边缘渐隐提示，适合菜单、侧栏、日志和列表预览。",
            Space::new().vertical().gap_xl().child(section(
                "边缘提示",
                "内容超出容器时用渐隐边缘提示仍可继续滚动。",
                showcase_grid(vec![
                    ScrollableMask::new(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .children((1..=12).map(|i| Text::new(format!("Activity row {i}")))),
                    )
                    .height(px(156.0))
                    .into_any_element(),
                    ScrollableMask::new(
                        Space::new()
                            .vertical()
                            .gap_sm()
                            .children((1..=16).map(|i| Text::new(format!("Log line {i}")))),
                    )
                    .height(px(180.0))
                    .into_any_element(),
                ]),
            )),
        )
    }
}
