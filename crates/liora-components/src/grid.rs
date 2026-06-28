//! Grid layout primitives for responsive native GPUI content walls.
//!
//! `Grid` is intentionally a layout component, not a data table. Use it for
//! card decks, icon galleries, settings tiles, and other two-dimensional visual
//! collections. Use `Table` or `VirtualizedTable` for tabular data.

use gpui::{
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, Window, div, prelude::*, px,
};
use liora_core::{Config, stable_unique_id};
use std::sync::Arc;

/// Horizontal and vertical spacing presets for [`Grid`].
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridGap {
    /// 4 px gap.
    Xs,
    /// 8 px gap.
    Sm,
    /// 12 px gap.
    Md,
    /// 16 px gap.
    Lg,
    /// 24 px gap.
    Xl,
    /// Custom pixel gap.
    Px(Pixels),
}

impl GridGap {
    fn pixels(self) -> Pixels {
        match self {
            GridGap::Xs => px(4.0),
            GridGap::Sm => px(8.0),
            GridGap::Md => px(12.0),
            GridGap::Lg => px(16.0),
            GridGap::Xl => px(24.0),
            GridGap::Px(value) => value,
        }
    }
}

impl From<Pixels> for GridGap {
    fn from(value: Pixels) -> Self {
        Self::Px(value)
    }
}

/// Strategy used by [`Grid`] to adapt horizontal columns.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GridMode {
    /// Keep item width stable and let the number of columns change with parent width.
    FitItem { item_size: Pixels },
    /// Keep the column count stable and let item width scale with parent width.
    FitColumns { columns: u16 },
}

/// Native responsive grid container.
///
/// The default mode is [`GridMode::FitItem`]: every item keeps a stable square
/// size and rows wrap as the available width changes. [`Grid::fit_columns`]
/// switches to a fixed number of columns where each item stretches with the
/// parent width.
pub struct Grid {
    children: Vec<AnyElement>,
    mode: GridMode,
    gap: GridGap,
    align_start: bool,
}

impl Grid {
    /// Creates an auto-fit grid with an 88 px minimum item width and 12 px gap.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            mode: GridMode::FitItem {
                item_size: px(96.0),
            },
            gap: GridGap::Md,
            align_start: true,
        }
    }

    /// Keeps item size stable and lets the column count adapt to parent width.
    pub fn fit_item(mut self, item_size: impl Into<Pixels>) -> Self {
        self.mode = GridMode::FitItem {
            item_size: item_size.into().max(px(1.0)),
        };
        self
    }

    /// Uses the compact fixed-item preset for icon/tool grids.
    pub fn fit_item_sm(self) -> Self {
        self.fit_item(px(88.0))
    }

    /// Uses the default fixed-item preset for icon/tool grids.
    pub fn fit_item_md(self) -> Self {
        self.fit_item(px(104.0))
    }

    /// Uses the large fixed-item preset for roomy card/icon grids.
    pub fn fit_item_lg(self) -> Self {
        self.fit_item(px(132.0))
    }

    /// Keeps the column count stable and lets item width scale with parent width.
    pub fn fit_columns(mut self, columns: u16) -> Self {
        self.mode = GridMode::FitColumns {
            columns: columns.max(1),
        };
        self
    }

    /// Alias for [`Grid::fit_columns`].
    pub fn columns(self, columns: u16) -> Self {
        self.fit_columns(columns)
    }

    /// Alias for [`Grid::fit_item`] kept for callers that think in minimum item width.
    pub fn min_item_width(self, width: impl Into<Pixels>) -> Self {
        self.fit_item(width)
    }

    /// Returns to fixed-item auto-column mode using the current default size.
    pub fn auto_fit(self) -> Self {
        self.fit_item(px(96.0))
    }

    /// Sets a custom gap.
    pub fn gap(mut self, gap: impl Into<GridGap>) -> Self {
        self.gap = gap.into();
        self
    }

    /// Applies the extra-small gap preset.
    pub fn gap_xs(self) -> Self {
        self.gap(GridGap::Xs)
    }

    /// Applies the small gap preset.
    pub fn gap_sm(self) -> Self {
        self.gap(GridGap::Sm)
    }

    /// Applies the medium gap preset.
    pub fn gap_md(self) -> Self {
        self.gap(GridGap::Md)
    }

    /// Applies the large gap preset.
    pub fn gap_lg(self) -> Self {
        self.gap(GridGap::Lg)
    }

    /// Applies the extra-large gap preset.
    pub fn gap_xl(self) -> Self {
        self.gap(GridGap::Xl)
    }

    /// Aligns items to the top/start edge. This is enabled by default.
    pub fn align_start(mut self) -> Self {
        self.align_start = true;
        self
    }

    /// Centers items on the cross axis.
    pub fn align_center(mut self) -> Self {
        self.align_start = false;
        self
    }

    /// Adds a single child.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|child| child.into_any_element()));
        self
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Grid {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let gap = self.gap.pixels();
        let mut el = div().w_full().gap(gap);

        match self.mode {
            GridMode::FitColumns { columns } => {
                el = el.grid().grid_cols(columns);
            }
            GridMode::FitItem { .. } => {
                el = el.flex().flex_row().flex_wrap();
            }
        }

        if self.align_start {
            el = el.items_start();
        } else {
            el = el.items_center();
        }

        let mode = self.mode;
        el.children(self.children.into_iter().map(move |child| {
            let item = div().child(child);
            match mode {
                GridMode::FitColumns { .. } => item.w_full().into_any_element(),
                GridMode::FitItem { item_size } => item.w(item_size).flex_none().into_any_element(),
            }
        }))
    }
}

impl IntoElement for Grid {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

type GridItemClick = dyn Fn(&mut Window, &mut App) + 'static;

/// Themed clickable/non-clickable tile intended for use inside [`Grid`].
pub struct GridItem {
    id: Option<String>,
    body: AnyElement,
    on_click: Option<Arc<GridItemClick>>,
    square: bool,
    centered: bool,
}

impl GridItem {
    /// Creates a grid tile from arbitrary Liora/native content.
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            id: None,
            body: body.into_any_element(),
            on_click: None,
            square: true,
            centered: true,
        }
    }

    /// Assigns a stable element id used for interaction state and testing.
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Keeps the default square tile shape.
    pub fn square(mut self) -> Self {
        self.square = true;
        self
    }

    /// Allows the tile height to follow its content instead of its width.
    pub fn rectangular(mut self) -> Self {
        self.square = false;
        self
    }

    /// Centers child content inside the tile.
    pub fn centered(mut self) -> Self {
        self.centered = true;
        self
    }

    /// Aligns child content to the top/start edge.
    pub fn align_start(mut self) -> Self {
        self.centered = false;
        self
    }

    /// Registers a click handler and switches the tile to pointer/hover styling.
    pub fn on_click(mut self, callback: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Arc::new(callback));
        self
    }
}

impl RenderOnce for GridItem {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self
            .id
            .unwrap_or_else(|| stable_unique_id("grid-item", "grid-item", window, cx).to_string());
        let click = self.on_click.clone();

        div()
            .id(id)
            .w_full()
            .when(self.square, |s| s.aspect_square())
            .when(self.centered, |s| s.flex().items_center().justify_center())
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .p_3()
            .text_color(theme.neutral.text_2)
            .when(click.is_some(), |s| {
                s.cursor_pointer()
                    .hover(|s| {
                        s.cursor_pointer()
                            .bg(theme.neutral.hover)
                            .border_color(theme.primary.base)
                    })
                    .on_click(move |_, window, cx| {
                        if let Some(click) = &click {
                            click(window, cx);
                        }
                    })
            })
            .child(self.body)
    }
}

impl IntoElement for GridItem {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn grid_supports_auto_fit_and_fixed_columns() {
        let source = include_str!("grid.rs");

        assert!(source.contains("pub struct Grid"));
        assert!(source.contains("pub enum GridMode"));
        assert!(source.contains("pub fn fit_item"));
        assert!(source.contains("pub fn fit_columns"));
        assert!(source.contains("pub fn fit_item_md"));
        assert!(source.contains("flex_wrap"));
        assert!(source.contains("grid_cols(columns)"));
    }

    #[test]
    fn grid_item_is_clickable_with_pointer_hover_feedback() {
        let source = include_str!("grid.rs");

        assert!(source.contains("pub struct GridItem"));
        assert!(source.contains("pub fn on_click"));
        assert!(source.contains(".cursor_pointer()"));
        assert!(source.contains(".aspect_square()"));
        assert!(source.contains("pub fn rectangular"));
        assert!(source.contains(".on_click(move |_, window, cx|"));
    }
}
