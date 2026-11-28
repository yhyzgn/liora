//! Row module.
//!
//! This public module implements the Liora grid row helper for row/column layouts. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::Col;
use gpui::{App, IntoElement, RenderOnce, ViewElement as Component, Window, prelude::*};

/// Data model used by row rendering.
pub struct Row {
    justify: Option<RowJustify>,
    align: Option<RowAlign>,
    children: Vec<gpui::AnyElement>,
}

#[derive(Clone, Copy)]
/// Options that control row justify behavior.
pub enum RowJustify {
    /// Packs row children at the start edge.
    Start,
    /// Centers row children on the main axis.
    Center,
    /// Packs row children at the end edge.
    End,
    /// Distributes row children with space between items.
    SpaceBetween,
    /// Distributes row children with space around items.
    SpaceAround,
}
#[derive(Clone, Copy)]
/// Options that control row align behavior.
pub enum RowAlign {
    /// Places the overlay above the anchor.
    Top,
    /// Aligns row children to the vertical center.
    Middle,
    /// Places the overlay below the anchor.
    Bottom,
}

impl Row {
    /// Creates `Row` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            justify: None,
            align: None,
            children: vec![],
        }
    }
    /// Sets main-axis distribution for child content.
    pub fn justify(mut self, j: RowJustify) -> Self {
        self.justify = Some(j);
        self
    }
    /// Sets cross-axis alignment for child content.
    pub fn align(mut self, a: RowAlign) -> Self {
        self.align = Some(a);
        self
    }
    /// Adds a child element to the component body.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
    /// Shorthand for adding a column.
    pub fn column(mut self, col: Col) -> Self {
        self.children.push(col.into_any_element());
        self
    }
}

impl RenderOnce for Row {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let mut row = gpui::div().flex().flex_row().flex_wrap();
        match self.justify {
            Some(RowJustify::Start) => {
                row = row.justify_start();
            }
            Some(RowJustify::Center) => {
                row = row.justify_center();
            }
            Some(RowJustify::End) => {
                row = row.justify_end();
            }
            Some(RowJustify::SpaceBetween) => {
                row = row.justify_between();
            }
            Some(RowJustify::SpaceAround) => {
                row = row.justify_around();
            }
            None => {}
        }
        match self.align {
            Some(RowAlign::Top) => {
                row = row.items_start();
            }
            Some(RowAlign::Middle) => {
                row = row.items_center();
            }
            Some(RowAlign::Bottom) => {
                row = row.items_end();
            }
            None => {}
        }
        row.children(self.children)
    }
}

impl IntoElement for Row {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{Bounds, Pixels, TestAppContext, canvas, div, point, px, size};
    use std::{cell::RefCell, rc::Rc};

    #[gpui::test]
    fn twenty_four_column_rows_keep_spans_on_the_same_line(cx: &mut TestAppContext) {
        for width in [240.0, 241.0, 800.0] {
            for spans in [
                &[24][..],
                &[12, 12][..],
                &[8, 8, 8][..],
                &[6, 6, 6, 6][..],
                &[1; 24][..],
                &[4, 20][..],
                &[2, 22][..],
                &[4, 4][..],
                &[5, 7][..],
                &[0, 24][..],
                &[5, 7, 12][..],
                &[12, 12, 6][..],
            ] {
                let bounds = Rc::new(RefCell::new(Vec::<Bounds<Pixels>>::new()));
                cx.add_empty_window().draw(
                    point(px(0.0), px(0.0)),
                    size(px(width), px(100.0)),
                    |_, _| {
                        let mut row = Row::new();
                        for &span in spans {
                            let bounds = bounds.clone();
                            row = row.column(
                                Col::new(span).child(
                                    canvas(
                                        move |rect, _, _| bounds.borrow_mut().push(rect),
                                        |_, _, _, _| {},
                                    )
                                    .w_full()
                                    .h(px(20.0)),
                                ),
                            );
                        }
                        div().w(px(width)).child(row)
                    },
                );

                let bounds = bounds.borrow();
                assert_eq!(bounds.len(), spans.len());
                let mut used = 0;
                let mut line = 0;
                for (&span, rect) in spans.iter().zip(bounds.iter()) {
                    if used + span > 24 {
                        used = 0;
                        line += 1;
                    }
                    let expected_left = width * used as f32 / 24.0;
                    let expected_width = width * span as f32 / 24.0;
                    assert!(
                        (f32::from(rect.left()) - expected_left).abs() <= 1.0,
                        "{spans:?} at {width}px: {rect:?}"
                    );
                    assert_eq!(rect.top(), px(line as f32 * 20.0), "{spans:?} at {width}px");
                    assert!((f32::from(rect.size.width) - expected_width).abs() <= 1.0);
                    used += span;
                }
            }
        }
    }
}
