//! Streaming Markdown rendered as native GPUI selectable text, code, tables and controls.
//! No HTML execution, implicit image fetching, or application-specific demo directives.

mod assets;
mod diagram;
mod math;
mod model;
mod render;
mod rich_engine;
mod rich_native;
mod table_editor;

use gpui::{Context, Entity, IntoElement, Render, ScrollHandle, SharedString, Window, prelude::*};
pub use model::{
    MarkdownDocument, MarkdownFootnote, MarkdownFootnoteReference, MarkdownHeading,
    MarkdownMetadata, MarkdownMetadataDiagnostic, MarkdownMetadataField, MarkdownTable,
    MarkdownTableAlignment, MarkdownTableCell, MarkdownTableEdit, MarkdownTableEditError,
    MarkdownTableRow, MetadataBlockKind, is_safe_markdown_link,
};
use std::{collections::HashMap, ops::Range, path::PathBuf, rc::Rc, sync::Arc};

/// Host-approved image content. Remote images must be fetched and decoded by the
/// host, which owns redirect, network permission, size and decoding limits.
#[derive(Clone)]
pub enum MarkdownImage {
    /// Explicitly approved local path, never interpreted as a network URL.
    File(PathBuf),
    /// Already decoded content, including images fetched by the host network layer.
    Render(Arc<gpui::RenderImage>),
}

type ImageResolver = Rc<dyn Fn(&str) -> Option<MarkdownImage>>;

/// Content or image policy changed; hosts can invalidate measured item height.
/// Subscribe and remeasure after layout (for example with Transcript::notify_item_changed).
#[derive(Clone, Copy, Debug)]
pub struct MarkdownChanged {
    /// Completed parse generation. Policy-only changes retain the same generation.
    pub generation: u64,
}
/// Host-owned task-list source mutation request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownTaskToggled {
    /// Stable block identity for the current parsed document.
    pub block_id: u64,
    /// Source range of the Markdown task marker.
    pub source_range: Range<usize>,
    /// Requested checked state.
    pub checked: bool,
}

/// Document-local fragment activation request.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownNavigationRequested {
    /// Heading ID without the leading `#`.
    pub fragment: SharedString,
}

impl gpui::EventEmitter<MarkdownTaskToggled> for MarkdownView {}
impl gpui::EventEmitter<MarkdownNavigationRequested> for MarkdownView {}

impl gpui::EventEmitter<MarkdownChanged> for MarkdownView {}

/// Retain as `Entity<MarkdownView>` so document IDs, selection and parse cache survive renders.
pub struct MarkdownView {
    document: MarkdownDocument,
    plain_text: SharedString,
    image_resolver: Option<ImageResolver>,
    resolved_images: HashMap<SharedString, Option<MarkdownImage>>,
    selection_id: SharedString,
    continuous_selection: bool,
    show_toc: bool,
    navigation_anchor: Option<Entity<crate::Anchor>>,
    assets: assets::AssetCache,
}

impl MarkdownView {
    /// Construct inside `cx.new(|cx| MarkdownView::new(source, cx))`.
    pub fn new(source: impl Into<String>, _cx: &mut Context<Self>) -> Self {
        Self::from_document(MarkdownDocument::new(source))
    }

    /// Uses an existing parsed document without parsing again.
    pub fn from_document(document: MarkdownDocument) -> Self {
        Self {
            plain_text: document.plain_text().into(),
            document,
            continuous_selection: false,
            show_toc: false,
            navigation_anchor: None,
            assets: assets::AssetCache::default(),
            selection_id: liora_core::unique_id("markdown-selection"),
            image_resolver: None,
            resolved_images: HashMap::new(),
        }
    }

    /// Last parsed document; source includes pending streaming updates.
    pub fn document(&self) -> &MarkdownDocument {
        &self.document
    }
    /// Opts into a native contents list without changing existing document layout by default.
    pub fn set_show_toc(&mut self, enabled: bool, cx: &mut Context<Self>) {
        if self.show_toc != enabled {
            self.show_toc = enabled;
            cx.notify();
        }
    }
    /// Enables measured heading and footnote scrolling within a host-tracked viewport.
    pub fn set_navigation_scroll(&mut self, handle: ScrollHandle, cx: &mut Context<Self>) {
        self.navigation_anchor = Some(cx.new(|_| crate::Anchor::new(handle)));
        cx.notify();
    }

    /// Switches to one continuous native text range for cross-block selection/copy.
    /// Rich layout mode keeps each block's typography while sharing one selection scope.
    pub fn set_continuous_selection(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.continuous_selection = enabled;
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }

    /// Whether the flattened continuous text projection is displayed.
    pub fn continuous_selection(&self) -> bool {
        self.continuous_selection
    }

    /// Replace source. Parsing is deferred until the next render.
    pub fn set_source(&mut self, source: impl Into<String>, cx: &mut Context<Self>) {
        self.document.set_source(source);
        // Raster keys include source and theme; prose edits must not rerender unchanged assets.
        cx.notify();
    }

    /// Append without parsing. GPUI coalesces notifications; one render parses the batch once.
    pub fn append(&mut self, chunk: &str, cx: &mut Context<Self>) {
        self.document.append(chunk);
        cx.notify();
    }

    /// Explicitly grants image-loading policy to the host. The input is an untrusted
    /// Markdown destination; return an approved local file or decoded image, or None.
    /// Called once per distinct destination until the policy changes. Default: deny all.
    /// Do not blindly return the input for model-generated Markdown.
    pub fn set_image_resolver(
        &mut self,
        resolver: impl Fn(&str) -> Option<MarkdownImage> + 'static,
        cx: &mut Context<Self>,
    ) {
        self.image_resolver = Some(Rc::new(resolver));
        self.resolved_images.clear();
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }

    /// Revokes image access and forgets previously approved destinations.
    pub fn clear_image_resolver(&mut self, cx: &mut Context<Self>) {
        self.image_resolver = None;
        self.resolved_images.clear();
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }
}

impl Render for MarkdownView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        if self.document.flush() {
            self.plain_text = self.document.plain_text().into();
            cx.emit(MarkdownChanged {
                generation: self.document.parse_count(),
            });
            // Bound the policy cache to destinations still present in the parsed document.
            let mut destinations = std::collections::HashSet::new();
            render::image_destinations(&self.document.blocks, &mut destinations);
            self.resolved_images
                .retain(|destination, _| destinations.contains(destination));
        }
        let task_target = cx.weak_entity();
        let task_toggle: render::TaskToggleHandler =
            Rc::new(move |block_id, source_range, checked, cx| {
                let _ = task_target.update(cx, |_, cx| {
                    cx.emit(MarkdownTaskToggled {
                        block_id,
                        source_range,
                        checked,
                    });
                });
            });
        let navigation_target = cx.weak_entity();
        let anchor = self.navigation_anchor.clone();
        let navigation: render::NavigationHandler = Rc::new(move |fragment, cx| {
            if let Some(anchor) = &anchor {
                let _ = anchor.update(cx, |anchor, _| anchor.scroll_to_target(&fragment));
            }
            let _ = navigation_target.update(cx, |_, cx| {
                cx.emit(MarkdownNavigationRequested { fragment });
            });
        });
        let weak = cx.weak_entity();
        let toggle = gpui::div()
            .id(SharedString::from(format!(
                "{}-toggle-focus",
                self.selection_id
            )))
            .focusable()
            .tab_index(0)
            .on_key_down(cx.listener(|view, event: &gpui::KeyDownEvent, _, cx| {
                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                    view.set_continuous_selection(!view.continuous_selection, cx);
                    cx.stop_propagation();
                }
            }))
            .child(
                crate::Button::new(if self.continuous_selection {
                    "Show rich document"
                } else {
                    "Select across blocks"
                })
                .id(SharedString::from(format!("{}-toggle", self.selection_id)))
                .text()
                .small()
                .on_click(move |_, _, cx| {
                    let _ = weak.update(cx, |view, cx| {
                        view.set_continuous_selection(!view.continuous_selection, cx)
                    });
                }),
            );
        let mut toc = gpui::div().flex().flex_col().gap_1().min_w_0();
        if self.show_toc && !self.continuous_selection {
            let headings = self.document.headings();
            if !headings.is_empty() {
                toc = toc.child(crate::Text::new("Contents"));
            }
            for heading in headings {
                let fragment = heading.id.clone();
                let keyboard_fragment = fragment.clone();
                let click_navigation = navigation.clone();
                let keyboard_navigation = navigation.clone();
                toc = toc.child(
                    gpui::div()
                        .id(SharedString::from(format!(
                            "{}-toc-{}-focus",
                            self.selection_id, heading.id
                        )))
                        .focusable()
                        .tab_index(0)
                        .on_key_down(move |event: &gpui::KeyDownEvent, _, cx| {
                            if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                                keyboard_navigation(keyboard_fragment.clone(), cx);
                                cx.stop_propagation();
                            }
                        })
                        .child(
                            crate::Button::new(heading.title)
                                .id(SharedString::from(format!(
                                    "{}-toc-{}",
                                    self.selection_id, heading.id
                                )))
                                .text()
                                .small()
                                .on_click(move |_, _, cx| {
                                    click_navigation(fragment.clone(), cx);
                                }),
                        ),
                );
            }
        }
        let content = if self.continuous_selection {
            let mut options = crate::SelectableTextOptions::new(
                self.selection_id.clone(),
                self.plain_text.clone(),
            );
            options.text_color = cx.global::<liora_core::Config>().theme.neutral.text_1;
            options.font_family = liora_core::ui_font_family(cx);
            crate::SelectableText::view(options, window, cx)
        } else {
            render::document(
                &self.document.blocks,
                self.selection_id.clone(),
                self.image_resolver.as_ref(),
                &mut self.resolved_images,
                Some(&task_toggle),
                Some(&navigation),
                self.navigation_anchor.as_ref(),
                &self.assets,
                window,
                cx,
            )
        };
        gpui::div()
            .flex()
            .flex_col()
            .gap_2()
            .w_full()
            .min_w_0()
            .child(toggle)
            .child(toc)
            .child(content)
    }
}

/// Presentation mode for [`MarkdownEditor`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkdownEditMode {
    /// Native multiline editing of the canonical Markdown source.
    Source,
    /// Native rendered projection of the canonical source.
    #[default]
    Rich,
}

/// Change notification emitted after the source editor changes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MarkdownEditorChanged {
    /// Current canonical Markdown source.
    pub source: SharedString,
}

pub use rich_native::MarkdownEditor;
