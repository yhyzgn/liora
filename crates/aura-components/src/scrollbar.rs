use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window};

pub struct Scrollbar {
    height: Option<f32>,
    children: Vec<gpui::AnyElement>,
}

impl Scrollbar {
    pub fn new() -> Self { Self { height: None, children: vec![] } }
    pub fn height(mut self, h: f32) -> Self { self.height = Some(h); self }
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.children.push(el.into_any_element()); self
    }
}

impl RenderOnce for Scrollbar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let child_count = self.children.len().max(1);
        let thumb_ratio = (5.0 / child_count as f32).clamp(0.05, 1.0);
        let thumb_h = (150.0 * thumb_ratio).max(20.0);
        let h = self.height.unwrap_or(200.0);

        gpui::div()
            .flex().flex_row().h(px(h))
            .child(
                gpui::div().flex_1()
                    .id("scrollbar-content")
                    .overflow_y_scroll()
                    .children(self.children)
            )
            .child(
                gpui::div().flex_none().w(px(14.0)).h_full()
                    .flex().flex_col().justify_start().items_center()
                    .px(px(4.0)).py(px(2.0))
                    .child(
                        gpui::div().flex_none().w(px(6.0)).h(px(thumb_h))
                            .bg(theme.neutral.border).rounded(px(3.0)).cursor_pointer()
                    )
            )
    }
}

impl IntoElement for Scrollbar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
