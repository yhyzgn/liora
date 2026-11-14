use super::{
    ImageResolver, MarkdownImage, is_safe_markdown_link,
    model::{Block, Kind},
};
use crate::{Button, SelectableText, SelectableTextOptions};
use gpui::{
    AnyElement, App, ClipboardItem, Font, FontStyle, FontWeight, ObjectFit, SharedString,
    StrikethroughStyle, TextRun, UnderlineStyle, Window, div, img, prelude::*, px,
};
use liora_core::Config;
use pulldown_cmark::Alignment;
use std::collections::{HashMap, HashSet};

type ImageCache = HashMap<SharedString, Option<MarkdownImage>>;

struct DocumentFonts {
    ui: Option<SharedString>,
    code: SharedString,
}

pub(super) fn image_destinations(blocks: &[Block], destinations: &mut HashSet<SharedString>) {
    for block in blocks {
        for image in &block.images {
            destinations.insert(image.destination.clone());
        }
        image_destinations(&block.children, destinations);
    }
}

pub(super) fn document(
    blocks: &[Block],
    resolver: Option<&ImageResolver>,
    images: &mut ImageCache,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let fonts = DocumentFonts {
        ui: liora_core::ui_font_family(cx),
        code: liora_core::code_font_family(cx),
    };
    let mut root = div().flex().flex_col().gap_3().w_full().min_w_0();
    for block in blocks {
        root = root.child(render_block(
            block,
            resolver,
            images,
            Alignment::None,
            &[],
            &fonts,
            window,
            cx,
        ));
    }
    root.into_any_element()
}

fn render_block(
    block: &Block,
    resolver: Option<&ImageResolver>,
    images: &mut ImageCache,
    alignment: Alignment,
    columns: &[Alignment],
    fonts: &DocumentFonts,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let id = SharedString::from(format!("markdown-block-{}", block.id));
    if let Kind::Code(language) = &block.kind {
        // CodeBlock's deferred skeleton has a different height. Render selectable code
        // immediately so a streamed row never grows later without MarkdownChanged.
        let mut options = SelectableTextOptions::new(
            SharedString::from(format!("{id}-code")),
            block.text.clone(),
        );
        options.text_color = theme.neutral.text_1;
        options.font_family = Some(fonts.code.clone());
        options.runs = vec![TextRun {
            len: block.text.len(),
            font: gpui::font(fonts.code.clone()),
            color: theme.neutral.text_1,
            background_color: None,
            underline: None,
            strikethrough: None,
        }];
        let code = block.text.clone();
        return div()
            .id(id.clone())
            .flex()
            .flex_col()
            .gap_2()
            .p_3()
            .w_full()
            .min_w_0()
            .bg(theme.neutral.hover)
            .rounded_md()
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(crate::Text::new(language.clone()))
                    .child(
                        Button::new("Copy code")
                            .id(SharedString::from(format!("{id}-copy")))
                            .text()
                            .small()
                            .on_click(move |_, _, cx| {
                                cx.write_to_clipboard(ClipboardItem::new_string(code.to_string()))
                            }),
                    ),
            )
            .child(SelectableText::view(options, window, cx))
            .into_any_element();
    }
    let mut root = div()
        .id(id.clone())
        .flex()
        .flex_col()
        .gap_2()
        .min_w_0()
        .w_full();
    match &block.kind {
        Kind::Quote => {
            root = root.border_l_2().border_color(theme.neutral.border).pl_3();
        }
        Kind::Rule => {
            return root
                .border_b_1()
                .border_color(theme.neutral.border)
                .into_any_element();
        }
        Kind::Row(header) => {
            root = root.flex_row().items_stretch();
            if *header {
                root = root.bg(theme.neutral.hover);
            }
        }
        Kind::Cell => {
            root = root
                .flex_1()
                .p_2()
                .border_b_1()
                .border_color(theme.neutral.border);
        }
        Kind::Table(_) => {
            root = root
                .border_1()
                .border_color(theme.neutral.border)
                .rounded_md();
        }
        _ => {}
    }
    if !block.text.is_empty() {
        let font_size = match block.kind {
            Kind::Heading(level) => px(match level {
                1 => 28.0,
                2 => 24.0,
                3 => 20.0,
                _ => 16.0,
            }),
            _ => px(theme.font_size.md),
        };
        let mut options = SelectableTextOptions::new(
            SharedString::from(format!("{id}-text")),
            block.text.clone(),
        );
        options.font_size = font_size;
        options.line_height = font_size * 1.6;
        options.text_color = theme.neutral.text_1;
        options.font_family = fonts.ui.clone();
        options.fill_width = true;
        options.text_align = match alignment {
            Alignment::Center => gpui::TextAlign::Center,
            Alignment::Right => gpui::TextAlign::Right,
            Alignment::Left | Alignment::None => gpui::TextAlign::Left,
        };
        options.runs = block
            .spans
            .iter()
            .map(|span| {
                let style = &span.style;
                let mut font = Font::default();
                if let Some(family) = &fonts.ui {
                    font.family = family.clone();
                }
                if style.code {
                    font.family = fonts.code.clone();
                }
                if style.strong || matches!(block.kind, Kind::Heading(_)) {
                    font.weight = FontWeight::BOLD;
                }
                if style.emphasis {
                    font.style = FontStyle::Italic;
                }
                let color = if style.link.as_deref().is_some_and(is_safe_markdown_link) {
                    theme.primary.base
                } else {
                    theme.neutral.text_1
                };
                TextRun {
                    len: span.range.len(),
                    font,
                    color,
                    background_color: style.code.then_some(theme.neutral.hover),
                    underline: style
                        .link
                        .as_deref()
                        .is_some_and(is_safe_markdown_link)
                        .then_some(UnderlineStyle {
                            thickness: px(1.0),
                            color: Some(color),
                            wavy: false,
                        }),
                    strikethrough: style.strike.then_some(StrikethroughStyle {
                        thickness: px(1.0),
                        color: Some(color),
                    }),
                }
            })
            .collect();
        root = root.child(SelectableText::view(options, window, cx));
        let mut links = HashSet::new();
        for span in &block.spans {
            if let Some(destination) = &span.style.link {
                if is_safe_markdown_link(destination) && links.insert(destination.clone()) {
                    let destination = destination.clone();
                    let keyboard_destination = destination.clone();
                    root = root.child(
                        div()
                            .id(SharedString::from(format!(
                                "{id}-link-{}-focus",
                                span.range.start
                            )))
                            .focusable()
                            .tab_index(0)
                            .on_key_down(move |event: &gpui::KeyDownEvent, _, cx| {
                                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                                    if is_safe_markdown_link(&keyboard_destination) {
                                        cx.open_url(keyboard_destination.as_ref());
                                    }
                                    cx.stop_propagation();
                                }
                            })
                            .child(
                                Button::new(format!("Open {destination}"))
                                    .id(SharedString::from(format!(
                                        "{id}-link-{}",
                                        span.range.start
                                    )))
                                    .text()
                                    .small()
                                    .on_click(move |_, _, cx| {
                                        if is_safe_markdown_link(&destination) {
                                            cx.open_url(destination.as_ref());
                                        }
                                    }),
                            ),
                    );
                }
            }
        }
    }
    for (index, occurrence) in block.images.iter().enumerate() {
        let approved = images
            .entry(occurrence.destination.clone())
            .or_insert_with(|| {
                resolver.and_then(|resolve| resolve(occurrence.destination.as_ref()))
            });
        let alt = &block.text[occurrence.range.clone()];
        let mut occurrence_view = div()
            .id(SharedString::from(format!("{id}-image-{index}")))
            .flex()
            .flex_col()
            .gap_1()
            .w_full()
            .min_w_0();
        if let Some(source) = approved {
            let native = match source {
                MarkdownImage::File(path) => img(path.clone()),
                MarkdownImage::Render(image) => img(image.clone()),
            };
            occurrence_view = occurrence_view.child(
                div()
                    .h(px(180.0))
                    .w_full()
                    .overflow_hidden()
                    .child(native.size_full().object_fit(ObjectFit::Contain)),
            );
        } else {
            occurrence_view = occurrence_view.child(
                crate::Text::new("Image blocked by host policy").text_color(theme.neutral.text_3),
            );
        }
        if !alt.is_empty() {
            occurrence_view = occurrence_view.child(crate::Text::new(alt.to_owned()));
        }
        root = root.child(occurrence_view);
    }
    if let Kind::List(start) = block.kind {
        for (index, item) in block.children.iter().enumerate() {
            let marker = start
                .map(|start| format!("{}.", start.saturating_add(index as u64)))
                .unwrap_or_else(|| "•".to_owned());
            root = root.child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .min_w_0()
                    .child(crate::Text::new(marker))
                    .child(render_block(
                        item,
                        resolver,
                        images,
                        Alignment::None,
                        &[],
                        fonts,
                        window,
                        cx,
                    )),
            );
        }
    } else {
        let child_columns = match &block.kind {
            Kind::Table(columns) => columns.as_slice(),
            _ => columns,
        };
        for (index, child) in block.children.iter().enumerate() {
            let child_alignment = if matches!(block.kind, Kind::Row(_)) {
                child_columns.get(index).copied().unwrap_or(Alignment::None)
            } else {
                Alignment::None
            };
            let element = div().min_w_0().flex_1().child(render_block(
                child,
                resolver,
                images,
                child_alignment,
                child_columns,
                fonts,
                window,
                cx,
            ));
            root = root.child(element);
        }
    }
    root.into_any_element()
}
