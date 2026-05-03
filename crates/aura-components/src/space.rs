use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window};

pub struct Space {
    w: f32,
    h: f32,
}

impl Space {
    pub fn horizontal(w: f32) -> Self { Self { w, h: 0.0 } }
    pub fn vertical(h: f32) -> Self   { Self { w: 0.0, h } }
}

impl RenderOnce for Space {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut div = gpui::div().flex_none();
        if self.w > 0.0 { div = div.w(px(self.w)); }
        if self.h > 0.0 { div = div.h(px(self.h)); }
        div
    }
}

impl IntoElement for Space {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
