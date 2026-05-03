use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window, DefiniteLength};

pub struct Scrollbar {
    height: Option<DefiniteLength>,
    children: Vec<gpui::AnyElement>,
}

impl Scrollbar {
    pub fn new() -> Self { Self { height: None, children: vec![] } }
    pub fn height(mut self, h: impl Into<DefiniteLength>) -> Self { self.height = Some(h.into()); self }
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.children.push(el.into_any_element()); self
    }
}

impl RenderOnce for Scrollbar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut div = gpui::div().flex().flex_col().id("scrollbar");
        if let Some(h) = self.height { div = div.h(h); } else { div = div.flex_1(); }
        div.overflow_y_scroll().children(self.children)
    }
}

impl IntoElement for Scrollbar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
