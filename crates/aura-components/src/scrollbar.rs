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
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;

        // Content area with overflow scroll
        let content = gpui::div()
            .flex_1().flex().flex_col()
            .id("scrollbar-content")
            .overflow_y_scroll()
            .children(self.children);

        // Visual scrollbar track (decorative)
        let track = gpui::div()
            .flex_none().w(px(6.0))
            .bg(theme.neutral.hover)
            .rounded(px(3.0))
            .mx(px(4.0)); // margin around track

        let mut outer = gpui::div().flex().flex_row().size_full();
        if let Some(h) = self.height { outer = outer.h(h); }
        outer.child(content).child(track)
    }
}

impl IntoElement for Scrollbar {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
