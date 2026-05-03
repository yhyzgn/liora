use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window};

pub struct Splitter {
    left: Option<gpui::AnyElement>,
    right: Option<gpui::AnyElement>,
}

impl Splitter {
    pub fn new() -> Self { Self { left: None, right: None } }
    pub fn left(mut self, el: impl IntoElement) -> Self { self.left = Some(el.into_any_element()); self }
    pub fn right(mut self, el: impl IntoElement) -> Self { self.right = Some(el.into_any_element()); self }
}

impl RenderOnce for Splitter {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let left = self.left.unwrap_or_else(|| gpui::div().into_any_element());
        let right = self.right.unwrap_or_else(|| gpui::div().into_any_element());

        gpui::div().flex().flex_row().size_full()
            .child(gpui::div().flex_none().w(px(300.0)).h_full().child(left))
            .child(gpui::div().flex_none().w(px(4.0)).h_full().bg(theme.neutral.border))
            .child(gpui::div().flex_1().h_full().child(right))
    }
}

impl IntoElement for Splitter {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
