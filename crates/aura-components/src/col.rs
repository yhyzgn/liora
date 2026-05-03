use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window};

pub struct Col {
    span: u8,
    children: Vec<gpui::AnyElement>,
}

impl Col {
    pub fn new(span: u8) -> Self { Self { span: span.min(24), children: vec![] } }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl RenderOnce for Col {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let flex = self.span as f32 / 24.0;
        gpui::div().flex_none().w(px(flex * 100.0)) // simplified percentage
            .children(self.children)
    }
}

impl IntoElement for Col {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
