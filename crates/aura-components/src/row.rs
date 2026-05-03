use gpui::{prelude::*, px, IntoElement, App, Component, RenderOnce, Window};

pub struct Row {
    justify: Option<RowJustify>,
    align: Option<RowAlign>,
    children: Vec<gpui::AnyElement>,
}

#[derive(Clone, Copy)]
pub enum RowJustify { Start, Center, End, SpaceBetween, SpaceAround }
#[derive(Clone, Copy)]
pub enum RowAlign { Top, Middle, Bottom }

impl Row {
    pub fn new() -> Self { Self { justify: None, align: None, children: vec![] } }
    pub fn justify(mut self, j: RowJustify) -> Self { self.justify = Some(j); self }
    pub fn align(mut self, a: RowAlign) -> Self { self.align = Some(a); self }
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element()); self
    }
}

impl RenderOnce for Row {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut row = gpui::div().flex().flex_row().flex_wrap().gap_2();
        match self.justify {
            Some(RowJustify::Start) => { row = row.justify_start(); }
            Some(RowJustify::Center) => { row = row.justify_center(); }
            Some(RowJustify::End) => { row = row.justify_end(); }
            Some(RowJustify::SpaceBetween) => { row = row.justify_between(); }
            Some(RowJustify::SpaceAround) => { row = row.justify_around(); }
            None => {}
        }
        match self.align {
            Some(RowAlign::Top) => { row = row.items_start(); }
            Some(RowAlign::Middle) => { row = row.items_center(); }
            Some(RowAlign::Bottom) => { row = row.items_end(); }
            None => {}
        }
        row.children(self.children)
    }
}

impl IntoElement for Row {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element { Component::new(self) }
}
