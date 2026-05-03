use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window};
use crate::Row;

pub struct Col {
    span: u8,
    children: Vec<gpui::AnyElement>,
}

impl Col {
    pub fn new(span: u8) -> Self { Self { span: span.min(24), children: vec![] } }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
    /// Add a nested row.
    pub fn row(mut self, row: Row) -> Self {
        self.children.push(row.into_any_element()); self
    }
    /// Add multiple nested rows.
    pub fn rows(mut self, rows: Vec<Row>) -> Self {
        self.children.extend(rows.into_iter().map(|r| r.into_any_element())); self
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
