//! Streaming Markdown rendered as native GPUI selectable text, code, tables and controls.
//! No HTML execution, implicit image fetching, or application-specific demo directives.

mod model;
mod render;

use gpui::{Context, IntoElement, Render, SharedString, Window};
pub use model::{MarkdownDocument, is_safe_markdown_link};
use std::{collections::HashMap, path::PathBuf, rc::Rc, sync::Arc};

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

impl gpui::EventEmitter<MarkdownChanged> for MarkdownView {}

/// Retain as `Entity<MarkdownView>` so document IDs, selection and parse cache survive renders.
pub struct MarkdownView {
    document: MarkdownDocument,
    image_resolver: Option<ImageResolver>,
    resolved_images: HashMap<SharedString, Option<MarkdownImage>>,
    plain_text: SharedString,
    continuous_selection: bool,
    selection_id: SharedString,
}

impl MarkdownView {
    /// Construct inside `cx.new(|cx| MarkdownView::new(source, cx))`.
    pub fn new(source: impl Into<String>, _cx: &mut Context<Self>) -> Self {
        Self::from_document(MarkdownDocument::new(source))
    }

    /// Uses an existing parsed document without parsing again.
    pub fn from_document(document: MarkdownDocument) -> Self {
        let plain_text = document.plain_text().into();
        Self {
            document,
            plain_text,
            continuous_selection: false,
            selection_id: liora_core::unique_id("markdown-selection"),
            image_resolver: None,
            resolved_images: HashMap::new(),
        }
    }

    /// Last parsed document; source includes pending streaming updates.
    pub fn document(&self) -> &MarkdownDocument {
        &self.document
    }

    /// Switches to one continuous native text range for cross-block selection/copy.
    /// Rich layout mode selects within individual paragraphs/code blocks.
    pub fn set_continuous_selection(&mut self, enabled: bool, cx: &mut Context<Self>) {
        self.continuous_selection = enabled;
        cx.emit(MarkdownChanged {
            generation: self.document.parse_count(),
        });
        cx.notify();
    }

    /// Whether the continuous text projection is displayed instead of rich blocks.
    pub fn continuous_selection(&self) -> bool {
        self.continuous_selection
    }

    /// Replace source. Parsing is deferred until the next render.
    pub fn set_source(&mut self, source: impl Into<String>, cx: &mut Context<Self>) {
        self.document.set_source(source);
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
        use gpui::prelude::*;
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
                self.image_resolver.as_ref(),
                &mut self.resolved_images,
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
            .child(content)
    }
}
