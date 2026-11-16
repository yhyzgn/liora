//! Native GPUI PDF reader with bounded pure-Rust CPU decoding.
//!
//! [`PdfDocument::from_bytes`] rasterizes PDF pages through Hayro; call it on a
//! background worker. Hosts may instead supply [`PdfPage`] images. File access,
//! search indexing, selection and annotation persistence remain host-owned.

mod backend;
mod page;
use crate::{Button, FindBar, FindBarEvent};
pub use backend::{PdfDecodeError, PdfDecoder};
use gpui::{
    AnyElement, App, Context, Entity, EventEmitter, FocusHandle, Focusable, IntoElement, ObjectFit,
    Render, RenderImage, ScrollHandle, SharedString, Window, canvas, div, img, prelude::*, px,
};
use liora_core::{Config, ui_font_family, ui_font_weight};
use std::{collections::BTreeMap, sync::Arc};

/// Intrinsic page dimensions supplied by the PDF backend.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PdfPageSize {
    /// Page width in backend points.
    pub width: f32,
    /// Page height in backend points.
    pub height: f32,
}

impl PdfPageSize {
    /// Creates a page size, clamping invalid dimensions to one point.
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: if width.is_finite() {
                width.max(1.0)
            } else {
                1.0
            },
            height: if height.is_finite() {
                height.max(1.0)
            } else {
                1.0
            },
        }
    }
}

/// A host-rendered PDF page and optional thumbnail.
pub struct PdfPage {
    /// One-based page number; `PdfDocument::new` normalizes it to display order.
    pub number: usize,
    /// Intrinsic page size.
    pub size: PdfPageSize,
    /// Full-resolution page image supplied by the host.
    pub image: Option<Arc<RenderImage>>,
    /// Optional lower-resolution image used by the sidebar.
    pub thumbnail: Option<Arc<RenderImage>>,
}

impl PdfPage {
    /// Creates an unloaded page placeholder.
    pub fn new(number: usize, width: f32, height: f32) -> Self {
        Self {
            number: number.max(1),
            size: PdfPageSize::new(width, height),
            image: None,
            thumbnail: None,
        }
    }

    /// Supplies the full-resolution page image.
    pub fn image(mut self, image: Arc<RenderImage>) -> Self {
        self.image = Some(image);
        self
    }

    /// Supplies a sidebar thumbnail.
    pub fn thumbnail(mut self, thumbnail: Arc<RenderImage>) -> Self {
        self.thumbnail = Some(thumbnail);
        self
    }
}

/// Document metadata and host-rendered pages consumed by [`PdfReader`].
pub struct PdfDocument {
    /// User-facing document title.
    pub title: SharedString,
    /// Pages in display order.
    pub pages: Vec<PdfPage>,
}

impl PdfDocument {
    /// Decodes PDF bytes into bounded page images using the pure-Rust Hayro backend.
    ///
    /// This is CPU work: run it on a background executor, then pass the result to
    /// [`PdfReader::set_document`]. Rejects invalid/encrypted documents and input,
    /// geometry or eager image budgets exceeded; never executes PDF scripts.
    pub fn from_bytes(
        title: impl Into<SharedString>,
        bytes: Vec<u8>,
    ) -> Result<Self, PdfDecodeError> {
        backend::decode(title.into(), bytes)
    }

    /// Synchronously decodes, publishing a first-page preview before completion.
    ///
    /// The host must dispatch decoding to a background worker and marshal the
    /// preview back to its UI thread; `on_preview` runs on the calling thread.
    /// After all pages pass preflight, calls `on_preview` once with full page
    /// metadata and only the first page's image and thumbnail populated. Those
    /// images share their storage with the final document. Continue handling the
    /// returned result: rendering later pages can still fail after the preview.
    pub fn from_bytes_with_preview(
        title: impl Into<SharedString>,
        bytes: Vec<u8>,
        on_preview: impl FnOnce(Self),
    ) -> Result<Self, PdfDecodeError> {
        backend::decode_with_preview(title.into(), bytes, on_preview)
    }

    /// Creates a document from host-provided pages, numbering them in display order.
    pub fn new(title: impl Into<SharedString>, mut pages: Vec<PdfPage>) -> Self {
        for (index, page) in pages.iter_mut().enumerate() {
            page.number = index + 1;
        }
        Self {
            title: title.into(),
            pages,
        }
    }

    /// Creates unloaded pages for a backend that will render lazily.
    pub fn with_page_count(
        title: impl Into<SharedString>,
        page_count: usize,
        page_size: PdfPageSize,
    ) -> Self {
        Self::new(
            title,
            (1..=page_count)
                .map(|number| PdfPage::new(number, page_size.width, page_size.height))
                .collect(),
        )
    }

    /// Returns the number of pages.
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

/// Page arrangement used by the reader canvas.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PdfPageLayout {
    /// Render only the current page.
    #[default]
    SinglePage,
    /// Render all document pages in a vertically scrollable canvas.
    Continuous,
}

/// Page scaling policy.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PdfZoom {
    /// Fit the page inside the reader's bounded canvas.
    #[default]
    FitPage,
    /// Fit the page to the reader's content width.
    FitWidth,
    /// Use an explicit percentage of intrinsic size.
    Percent(u16),
}

/// Rotation applied in clockwise quarter turns.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PdfRotation {
    /// No rotation.
    #[default]
    Zero,
    /// Rotate 90 degrees clockwise.
    Quarter,
    /// Rotate 180 degrees.
    Half,
    /// Rotate 270 degrees clockwise.
    ThreeQuarter,
}

impl PdfRotation {
    fn next(self) -> Self {
        match self {
            Self::Zero => Self::Quarter,
            Self::Quarter => Self::Half,
            Self::Half => Self::ThreeQuarter,
            Self::ThreeQuarter => Self::Zero,
        }
    }

    fn is_quarter_turn(self) -> bool {
        matches!(self, Self::Quarter | Self::ThreeQuarter)
    }
}

/// Tool selection forwarded to a host annotation or selection backend.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PdfTool {
    /// Request text selection through the host backend.
    #[default]
    Select,
    /// Request page panning through the host backend.
    Hand,
    /// Request highlight-tool handling from the host backend.
    Highlight,
    /// Request underline-tool handling from the host backend.
    Underline,
    /// Request strike-through-tool handling from the host backend.
    StrikeThrough,
    /// Request note-tool handling from the host backend.
    Note,
    /// Request text-box-tool handling from the host backend.
    TextBox,
    /// Request freehand-pen-tool handling from the host backend.
    Pen,
    /// Request rectangle-tool handling from the host backend.
    Rectangle,
    /// Request ellipse-tool handling from the host backend.
    Ellipse,
    /// Request arrow-tool handling from the host backend.
    Arrow,
}

impl PdfTool {
    fn next(self) -> Self {
        match self {
            Self::Select => Self::Hand,
            Self::Hand => Self::Highlight,
            Self::Highlight => Self::Underline,
            Self::Underline => Self::StrikeThrough,
            Self::StrikeThrough => Self::Note,
            Self::Note => Self::TextBox,
            Self::TextBox => Self::Pen,
            Self::Pen => Self::Rectangle,
            Self::Rectangle => Self::Ellipse,
            Self::Ellipse => Self::Arrow,
            Self::Arrow => Self::Select,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Select => "Select",
            Self::Hand => "Hand",
            Self::Highlight => "Highlight",
            Self::Underline => "Underline",
            Self::StrikeThrough => "Strike-through",
            Self::Note => "Note",
            Self::TextBox => "Text box",
            Self::Pen => "Pen",
            Self::Rectangle => "Rectangle",
            Self::Ellipse => "Ellipse",
            Self::Arrow => "Arrow",
        }
    }
}

/// Events emitted after a reader interaction changes host-visible state.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PdfReaderEvent {
    /// Visible page images needed by a lazy decoding host.
    PagesRequested {
        /// One-based visible page numbers.
        pages: Vec<usize>,
    },
    /// Current page changed to a one-based page number.
    PageChanged {
        /// Current one-based page number.
        page: usize,
    },
    /// Zoom policy changed.
    ZoomChanged {
        /// New zoom policy.
        zoom: PdfZoom,
    },
    /// Rotation changed.
    RotationChanged {
        /// New clockwise rotation.
        rotation: PdfRotation,
    },
    /// Page arrangement changed.
    LayoutChanged {
        /// New page arrangement.
        layout: PdfPageLayout,
    },
    /// Annotation/selection tool changed.
    ToolChanged {
        /// Newly selected host-directed tool.
        tool: PdfTool,
    },
    /// Host should search the PDF text index.
    SearchRequested {
        /// Query to search.
        query: SharedString,
    },
    /// Host should reveal the previous search match.
    SearchPrevious,
    /// Host should reveal the next search match.
    SearchNext,
    /// Host should close the find UI.
    SearchClosed,
}

/// Native GPUI PDF reader control with host-rendered page images.
///
/// Fills its host's bounded size. The default [`PdfZoom::FitPage`] preserves
/// aspect ratio inside the remaining viewport after the toolbar and sidebar;
/// short, wide hosts therefore leave horizontal space around portrait pages.
pub struct PdfReader {
    id: SharedString,
    document: PdfDocument,
    current_page: usize,
    zoom: PdfZoom,
    rotation: PdfRotation,
    layout: PdfPageLayout,
    tool: PdfTool,
    sidebar_visible: bool,
    find_visible: bool,
    restore_focus: bool,
    find_bar: Entity<FindBar>,
    _find_subscription: gpui::Subscription,
    focus_handle: FocusHandle,
    scroll_handle: ScrollHandle,
    viewport: (f32, f32),
    rotated_images: BTreeMap<usize, Arc<RenderImage>>,
    requested_pages: Vec<usize>,
}

impl EventEmitter<PdfReaderEvent> for PdfReader {}

impl PdfReader {
    /// Creates a reader with page one selected and the sidebar visible.
    pub fn new(document: PdfDocument, cx: &mut Context<Self>) -> Self {
        let find_bar = cx.new(FindBar::new);
        let find_subscription =
            cx.subscribe(
                &find_bar,
                |reader, _, event: &FindBarEvent, cx| match event {
                    FindBarEvent::QueryChanged { query } => {
                        cx.emit(PdfReaderEvent::SearchRequested {
                            query: query.clone(),
                        });
                    }
                    FindBarEvent::Previous => cx.emit(PdfReaderEvent::SearchPrevious),
                    FindBarEvent::Next => cx.emit(PdfReaderEvent::SearchNext),
                    FindBarEvent::Closed => {
                        reader.find_visible = false;
                        reader.restore_focus = true;
                        cx.emit(PdfReaderEvent::SearchClosed);
                        cx.notify();
                    }
                },
            );
        Self {
            id: liora_core::unique_id("pdf-reader"),
            document,
            current_page: 1,
            zoom: PdfZoom::FitPage,
            rotation: PdfRotation::Zero,
            layout: PdfPageLayout::SinglePage,
            tool: PdfTool::Select,
            sidebar_visible: true,
            find_visible: false,
            restore_focus: false,
            find_bar,
            focus_handle: cx.focus_handle(),
            scroll_handle: ScrollHandle::new(),
            viewport: (1.0, 1.0),
            rotated_images: BTreeMap::new(),
            requested_pages: Vec::new(),
            _find_subscription: find_subscription,
        }
    }

    /// Returns the current document metadata and pages.
    pub fn document(&self) -> &PdfDocument {
        &self.document
    }

    /// Returns the embedded native find bar for host-side focus and inspection.
    pub fn find_bar(&self) -> Entity<FindBar> {
        self.find_bar.clone()
    }

    /// Sets the embedded find query through the native input.
    pub fn set_find_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.find_bar
            .update(cx, |find_bar, cx| find_bar.set_query(query, cx));
    }

    /// Displays host-provided search match information in the embedded find bar.
    pub fn set_find_match_status(
        &mut self,
        status: impl Into<SharedString>,
        cx: &mut Context<Self>,
    ) {
        self.find_bar
            .update(cx, |find_bar, cx| find_bar.set_match_status(status, cx));
    }

    /// Replaces the document and clamps the current page.
    pub fn set_document(&mut self, document: PdfDocument, cx: &mut Context<Self>) {
        self.document = document;
        self.rotated_images.clear();
        self.scroll_handle.set_offset(gpui::point(px(0.0), px(0.0)));
        self.requested_pages.clear();
        self.current_page = self
            .current_page
            .clamp(1, self.document.page_count().max(1));
        self.scroll_handle
            .scroll_to_top_of_item(if self.layout == PdfPageLayout::Continuous {
                self.current_page - 1
            } else {
                0
            });
        cx.notify();
    }

    /// Replaces one page's full-resolution image and notifies the reader.
    pub fn set_page_image(
        &mut self,
        page: usize,
        image: Arc<RenderImage>,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(index) = page.checked_sub(1) else {
            return false;
        };
        let Some(page) = self.document.pages.get_mut(index) else {
            return false;
        };
        if image.frame_count() == 0 {
            return false;
        }
        self.rotated_images.remove(&index);
        page.image = Some(image);
        cx.notify();
        true
    }

    /// Installs one decoded page without resetting navigation or zoom.
    pub fn set_decoded_page(&mut self, page: PdfPage, cx: &mut Context<Self>) -> bool {
        let Some(index) = page.number.checked_sub(1) else {
            return false;
        };
        let Some(slot) = self.document.pages.get_mut(index) else {
            return false;
        };
        *slot = page;
        self.rotated_images.remove(&index);
        cx.notify();
        true
    }

    /// Returns the one-based current page.
    pub fn current_page(&self) -> usize {
        self.current_page
    }

    /// Selects a page and emits [`PdfReaderEvent::PageChanged`].
    pub fn set_page(&mut self, page: usize, cx: &mut Context<Self>) {
        let page = page.clamp(1, self.document.page_count().max(1));
        if page != self.current_page {
            self.current_page = page;
            self.scroll_handle
                .scroll_to_top_of_item(if self.layout == PdfPageLayout::Continuous {
                    page - 1
                } else {
                    0
                });
            cx.emit(PdfReaderEvent::PageChanged { page });
            cx.notify();
        }
    }

    /// Advances to the next page.
    pub fn next_page(&mut self, cx: &mut Context<Self>) {
        self.set_page(self.current_page.saturating_add(1), cx);
    }

    /// Moves to the previous page.
    pub fn previous_page(&mut self, cx: &mut Context<Self>) {
        self.set_page(self.current_page.saturating_sub(1), cx);
    }

    /// Returns the current zoom policy.
    pub fn zoom(&self) -> PdfZoom {
        self.zoom
    }

    /// Sets zoom and emits an event when the policy changes.
    pub fn set_zoom(&mut self, zoom: PdfZoom, cx: &mut Context<Self>) {
        let zoom = match zoom {
            PdfZoom::Percent(percent) => PdfZoom::Percent(percent.clamp(25, 400)),
            fit => fit,
        };
        if self.zoom != zoom {
            self.zoom = zoom;
            cx.emit(PdfReaderEvent::ZoomChanged { zoom });
            cx.notify();
        }
    }

    /// Increases manual zoom by ten percent.
    pub fn zoom_in(&mut self, cx: &mut Context<Self>) {
        let next = match self.zoom {
            PdfZoom::Percent(percent) => PdfZoom::Percent(percent.saturating_add(10).min(400)),
            _ => PdfZoom::Percent(110),
        };
        self.set_zoom(next, cx);
    }

    /// Decreases manual zoom by ten percent, never below twenty-five percent.
    pub fn zoom_out(&mut self, cx: &mut Context<Self>) {
        let next = match self.zoom {
            PdfZoom::Percent(percent) => PdfZoom::Percent(percent.saturating_sub(10).max(25)),
            _ => PdfZoom::Percent(90),
        };
        self.set_zoom(next, cx);
    }

    /// Returns the current rotation.
    pub fn rotation(&self) -> PdfRotation {
        self.rotation
    }

    /// Rotates clockwise by ninety degrees.
    pub fn rotate(&mut self, cx: &mut Context<Self>) {
        self.rotation = self.rotation.next();
        self.rotated_images.clear();
        cx.emit(PdfReaderEvent::RotationChanged {
            rotation: self.rotation,
        });
        cx.notify();
    }

    /// Returns the current page arrangement.
    pub fn layout(&self) -> PdfPageLayout {
        self.layout
    }

    /// Sets single-page or continuous layout.
    pub fn set_layout(&mut self, layout: PdfPageLayout, cx: &mut Context<Self>) {
        if self.layout != layout {
            self.layout = layout;
            self.scroll_handle
                .scroll_to_top_of_item(if layout == PdfPageLayout::Continuous {
                    self.current_page - 1
                } else {
                    0
                });
            cx.emit(PdfReaderEvent::LayoutChanged { layout });
            cx.notify();
        }
    }

    /// Returns the selected host tool.
    pub fn tool(&self) -> PdfTool {
        self.tool
    }

    /// Selects a host tool and emits an event.
    pub fn set_tool(&mut self, tool: PdfTool, cx: &mut Context<Self>) {
        if self.tool != tool {
            self.tool = tool;
            cx.emit(PdfReaderEvent::ToolChanged { tool });
            cx.notify();
        }
    }

    /// Shows or hides the page thumbnail sidebar.
    pub fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.sidebar_visible = !self.sidebar_visible;
        cx.notify();
    }

    /// Shows or hides the host-connected find bar.
    pub fn toggle_find(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.find_visible = !self.find_visible;
        if self.find_visible {
            self.find_bar.update(cx, |bar, cx| bar.focus(window, cx));
        } else {
            window.focus(&self.focus_handle, cx);
        }
        cx.notify();
    }

    fn key_down(
        &mut self,
        event: &gpui::KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.focus_handle.is_focused(window) || event.keystroke.modifiers.modified() {
            return;
        }
        match event.keystroke.key.as_str() {
            "pageup" | "left" => self.previous_page(cx),
            "pagedown" | "right" => self.next_page(cx),
            _ => return,
        }
        cx.stop_propagation();
    }

    fn render_toolbar(&self, cx: &mut Context<Self>) -> AnyElement {
        let entity = cx.entity().clone();
        let page_count = self.document.page_count();
        let page = self.current_page;
        let zoom = self.zoom;
        let zoom_label: SharedString = match zoom {
            PdfZoom::FitPage => "Fit page".into(),
            PdfZoom::FitWidth => "Fit width".into(),
            PdfZoom::Percent(percent) => format!("{percent}%").into(),
        };
        let layout = self.layout;
        let tool = self.tool;
        div()
            .debug_selector(|| "pdf-reader-toolbar".into())
            .flex_none()
            .flex()
            .flex_wrap()
            .items_center()
            .gap_1()
            .w_full()
            .p_2()
            .border_b_1()
            .border_color(cx.global::<Config>().theme.neutral.border)
            .child(Button::new("Pages").small().text().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |reader, cx| reader.toggle_sidebar(cx));
                }
            }))
            .child(
                Button::new("Previous")
                    .small()
                    .disabled(page <= 1)
                    .on_click({
                        let entity = entity.clone();
                        move |_, _, cx| {
                            let _ = entity.update(cx, |reader, cx| reader.previous_page(cx));
                        }
                    }),
            )
            .child(
                Button::new(format!("Page {page} / {page_count}"))
                    .small()
                    .text(),
            )
            .child(
                Button::new("Next")
                    .small()
                    .disabled(page >= page_count.max(1))
                    .on_click({
                        let entity = entity.clone();
                        move |_, _, cx| {
                            let _ = entity.update(cx, |reader, cx| reader.next_page(cx));
                        }
                    }),
            )
            .child(Button::new("−").small().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |reader, cx| reader.zoom_out(cx));
                }
            }))
            .child(Button::new(zoom_label).small().text().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |reader, cx| {
                        reader.set_zoom(
                            if reader.zoom == PdfZoom::FitPage {
                                PdfZoom::FitWidth
                            } else {
                                PdfZoom::FitPage
                            },
                            cx,
                        )
                    });
                }
            }))
            .child(Button::new("+").small().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |reader, cx| reader.zoom_in(cx));
                }
            }))
            .child(Button::new("Rotate").small().text().on_click({
                let entity = entity.clone();
                move |_, _, cx| {
                    let _ = entity.update(cx, |reader, cx| reader.rotate(cx));
                }
            }))
            .child(
                Button::new(if layout == PdfPageLayout::SinglePage {
                    "Continuous"
                } else {
                    "Single page"
                })
                .small()
                .text()
                .on_click({
                    let entity = entity.clone();
                    move |_, _, cx| {
                        let _ = entity.update(cx, |reader, cx| {
                            reader.set_layout(
                                if reader.layout == PdfPageLayout::SinglePage {
                                    PdfPageLayout::Continuous
                                } else {
                                    PdfPageLayout::SinglePage
                                },
                                cx,
                            );
                        });
                    }
                }),
            )
            .child(
                Button::new(format!("Tool: {}", tool.label()))
                    .small()
                    .text()
                    .on_click(move |_, _, cx| {
                        let _ =
                            entity.update(cx, |reader, cx| reader.set_tool(reader.tool.next(), cx));
                    }),
            )
            .child(Button::new("Find").small().text().on_click({
                let entity = cx.entity().clone();
                move |_, window, cx| {
                    entity.update(cx, |reader, cx| reader.toggle_find(window, cx));
                }
            }))
            .into_any_element()
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> AnyElement {
        let entity = cx.entity().clone();
        let mut thumbnails = Vec::with_capacity(self.document.pages.len());
        for (index, page) in self.document.pages.iter().enumerate() {
            let page_number = index + 1;
            let active = page_number == self.current_page;
            let entity = entity.clone();
            let child = if let Some(image) = page.thumbnail.clone().or_else(|| page.image.clone()) {
                img(image)
                    .w(px(110.0))
                    .h(px(150.0))
                    .object_fit(ObjectFit::Contain)
                    .into_any_element()
            } else {
                div()
                    .w(px(110.0))
                    .h(px(150.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(format!("Page {page_number}"))
                    .into_any_element()
            };
            thumbnails.push(
                div()
                    .id(format!("pdf-thumbnail-{page_number}"))
                    .focusable()
                    .tab_index(0)
                    .aria_label(format!("Page {page_number}"))
                    .on_key_down({
                        let entity = entity.clone();
                        move |event, _, cx| {
                            if matches!(event.keystroke.key.as_str(), "enter" | "space")
                                && !event.keystroke.modifiers.modified()
                            {
                                entity.update(cx, |reader, cx| reader.set_page(page_number, cx));
                                cx.stop_propagation();
                            }
                        }
                    })
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .p_1()
                    .border_1()
                    .border_color(if active {
                        cx.global::<Config>().theme.primary.base
                    } else {
                        cx.global::<Config>().theme.neutral.border
                    })
                    .cursor_pointer()
                    .on_click(move |_, _, cx| {
                        let _ = entity.update(cx, |reader, cx| reader.set_page(page_number, cx));
                    })
                    .child(child)
                    .child(format!("Page {page_number}"))
                    .into_any_element(),
            );
        }
        div()
            .id("pdf-thumbnails")
            .debug_selector(|| "pdf-reader-sidebar".into())
            .flex()
            .flex_col()
            .flex_none()
            .w(px(140.0))
            .h_full()
            .min_h(px(0.0))
            .overflow_y_scroll()
            .gap(px(8.0))
            .p(px(8.0))
            .border_color(cx.global::<Config>().theme.neutral.border)
            .children(thumbnails)
            .into_any_element()
    }

    fn render_page(&mut self, index: usize, cx: &mut Context<Self>) -> AnyElement {
        let theme = &cx.global::<Config>().theme;
        let page = &self.document.pages[index];
        let (width, height) = page::dimensions(page.size, self.rotation, self.zoom, self.viewport);
        let image = page.image.as_ref().and_then(|image| {
            if self.rotation == PdfRotation::Zero {
                return Some(image.clone());
            }
            if let Some(cached) = self.rotated_images.get(&index) {
                return Some(cached.clone());
            }
            let rotated = page::rotate_image(image, self.rotation)?;
            self.rotated_images.insert(index, rotated.clone());
            Some(rotated)
        });
        let content = if let Some(image) = image {
            img(image)
                .w(px(width))
                .h(px(height))
                .object_fit(ObjectFit::Contain)
                .into_any_element()
        } else {
            div()
                .w(px(width))
                .h(px(height))
                .flex()
                .items_center()
                .justify_center()
                .text_color(theme.neutral.text_2)
                .child(format!("Page {}: no page image", index + 1))
                .into_any_element()
        };
        div()
            .id(format!("pdf-page-{}", index + 1))
            .debug_selector(|| format!("pdf-reader-page-row-{}", index + 1))
            .flex()
            .flex_none()
            .w_full()
            .min_w(px(width + 26.0))
            .justify_center()
            .p_3()
            .child(
                div()
                    .debug_selector(|| format!("pdf-reader-page-{}", index + 1))
                    .flex()
                    .flex_col()
                    .flex_none()
                    .bg(theme.neutral.card)
                    .border_1()
                    .border_color(theme.neutral.border)
                    .child(content),
            )
            .into_any_element()
    }
}

impl Focusable for PdfReader {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for PdfReader {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.restore_focus {
            self.restore_focus = false;
            window.focus(&self.focus_handle, cx);
        }
        let config = cx.global::<Config>();
        let mut root = div()
            .id(self.id.clone())
            .debug_selector(|| "pdf-reader-root".into())
            .track_focus(&self.focus_handle)
            .tab_index(0)
            .on_key_down(cx.listener(Self::key_down))
            .flex()
            .flex_col()
            .size_full()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .when_some(ui_font_family(cx), |view, font| view.font_family(font))
            .when_some(ui_font_weight(cx), |view, weight| view.font_weight(weight))
            .text_color(config.theme.neutral.text_1)
            .bg(config.theme.neutral.body)
            .child(self.render_toolbar(cx));
        if self.find_visible {
            root = root.child(self.find_bar.clone());
        }
        let content = if self.document.pages.is_empty() {
            vec![div().p_4().child("No PDF pages").into_any_element()]
        } else if self.layout == PdfPageLayout::Continuous {
            (0..self.document.page_count())
                .map(|index| self.render_page(index, cx))
                .collect()
        } else {
            vec![self.render_page(self.current_page - 1, cx)]
        };
        let pages = div()
            .id("pdf-pages")
            .debug_selector(|| "pdf-reader-pages".into())
            .flex_1()
            .w_full()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .flex()
            .flex_col()
            .overflow_scroll()
            .track_scroll(&self.scroll_handle)
            .on_mouse_down(
                gpui::MouseButton::Left,
                cx.listener(|reader, _, window, cx| window.focus(&reader.focus_handle, cx)),
            )
            .children(content);
        let owner = cx.entity().downgrade();
        let scroll = self.scroll_handle.clone();
        let viewport = div()
            .relative()
            .debug_selector(|| "pdf-reader-viewport".into())
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .overflow_hidden()
            .child(pages)
            .child(
                canvas(
                    move |bounds, window, cx| {
                        let measured =
                            (f32::from(bounds.size.width), f32::from(bounds.size.height));
                        let top_page = scroll.top_item() + 1;
                        window.defer(cx, move |_, cx| {
                            let _ = owner.update(cx, |reader, cx| {
                                if (reader.viewport.0 - measured.0).abs() >= 0.5
                                    || (reader.viewport.1 - measured.1).abs() >= 0.5
                                {
                                    reader.viewport = measured;
                                    cx.notify();
                                }
                                if reader.layout == PdfPageLayout::Continuous {
                                    let page = top_page.min(reader.document.page_count()).max(1);
                                    if page != reader.current_page {
                                        reader.current_page = page;
                                        cx.emit(PdfReaderEvent::PageChanged { page });
                                        cx.notify();
                                    }
                                }
                                let count = reader.document.page_count();
                                let visible = if count == 0 {
                                    Vec::new()
                                } else if reader.layout == PdfPageLayout::SinglePage {
                                    vec![reader.current_page]
                                } else {
                                    let first = reader.scroll_handle.top_item().min(count - 1);
                                    let last = reader.scroll_handle.bottom_item().min(count - 1);
                                    (first..=last).map(|index| index + 1).collect()
                                };
                                if visible != reader.requested_pages {
                                    reader.requested_pages = visible.clone();
                                    cx.emit(PdfReaderEvent::PagesRequested { pages: visible });
                                }
                            });
                        });
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            );
        let mut body = div().flex().flex_1().min_h(px(0.0)).min_w(px(0.0));
        if self.sidebar_visible {
            body = body.child(self.render_sidebar(cx));
        }
        root.child(body.child(viewport))
    }
}

#[cfg(test)]
mod tests {
    use super::{PdfDocument, PdfPage, PdfPageSize, PdfRotation, PdfTool};

    #[test]
    fn document_and_page_boundaries_are_safe() {
        let document = PdfDocument::with_page_count("test", 3, PdfPageSize::new(612.0, 792.0));
        assert_eq!(document.page_count(), 3);
        assert_eq!(PdfPage::new(0, 0.0, -1.0).number, 1);
        assert_eq!(PdfPageSize::new(0.0, -1.0), PdfPageSize::new(1.0, 1.0));
        assert_eq!(
            PdfPageSize::new(f32::NAN, f32::INFINITY),
            PdfPageSize::new(1.0, 1.0)
        );
        let document = PdfDocument::new(
            "out of order",
            vec![PdfPage::new(42, 10.0, 20.0), PdfPage::new(42, 10.0, 20.0)],
        );
        assert_eq!(
            document
                .pages
                .iter()
                .map(|page| page.number)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
    }

    #[test]
    fn rotation_and_tool_cycles_wrap_without_duplicates() {
        assert_eq!(PdfRotation::Zero.next(), PdfRotation::Quarter);
        assert_eq!(PdfRotation::ThreeQuarter.next(), PdfRotation::Zero);
        let mut tool = PdfTool::Select;
        for _ in 0..11 {
            tool = tool.next();
        }
        assert_eq!(tool, PdfTool::Select);
    }
}
