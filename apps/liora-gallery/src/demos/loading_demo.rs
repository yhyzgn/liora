use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::Loading;

use liora_components::layout_helpers::{page, row};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| LoadingDemo).into()
}

struct LoadingDemo;

impl Render for LoadingDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Loading 加载",
            "加载数据时显示。",
            row(vec![Loading::new(), Loading::new().text("Loading...")]),
        )
    }
}
