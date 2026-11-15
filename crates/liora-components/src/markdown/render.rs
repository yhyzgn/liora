#[path = "inline.rs"]
mod inline;

use super::{
    ImageResolver, MarkdownImage, is_safe_markdown_link,
    model::{Block, Kind},
};
use crate::{Button, SelectableText, SelectableTextOptions, Tooltip};
use gpui::{
    AnyElement, App, Bounds, ClipboardItem, ElementId, Entity, Font, FontStyle, FontWeight,
    GlobalElementId, InspectorElementId, LayoutId, ObjectFit, Pixels, SharedString,
    StrikethroughStyle, Subscription, TextRun, UnderlineStyle, Window, div, img, prelude::*, px,
};
use liora_core::{Config, TooltipData, clear_tooltip, set_exclusive_active_tooltip};
use pulldown_cmark::Alignment;
use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
    ops::Range,
    rc::Rc,
};

pub(super) type ImageCache = HashMap<SharedString, Option<MarkdownImage>>;
pub(super) type TaskToggleHandler = Rc<dyn Fn(u64, Range<usize>, bool, &mut App)>;
pub(super) type NavigationHandler = Rc<dyn Fn(SharedString, &mut App)>;

struct DocumentFonts {
    ui: Option<SharedString>,
    code: SharedString,
    selection_scope: Option<SharedString>,
    order: Cell<usize>,
    assets: super::assets::AssetCache,
}

#[derive(Default)]
struct FootnoteInfo {
    references: usize,
    preview: Option<SharedString>,
}

fn footnote_reference_number(block: &Block, span: &super::model::Span) -> Option<usize> {
    span.style.footnote.as_ref()?;
    block.text[span.range.clone()]
        .strip_prefix('[')?
        .strip_suffix(']')?
        .parse()
        .ok()
}

fn footnote_preview(block: &Block, text: &mut String) {
    const MAX_PREVIEW_BYTES: usize = 1024;
    if text.len() >= MAX_PREVIEW_BYTES {
        return;
    }
    if !block.text.is_empty() {
        if !text.is_empty() {
            text.push(' ');
        }
        let mut end = (MAX_PREVIEW_BYTES - text.len()).min(block.text.len());
        while !block.text.is_char_boundary(end) {
            end -= 1;
        }
        text.push_str(&block.text[..end]);
    }
    for child in &block.children {
        footnote_preview(child, text);
    }
}

fn footnote_info(blocks: &[Block]) -> HashMap<usize, FootnoteInfo> {
    fn collect(blocks: &[Block], notes: &mut HashMap<usize, FootnoteInfo>) {
        for block in blocks {
            for span in &block.spans {
                if let Some(number) = footnote_reference_number(block, span) {
                    notes.entry(number).or_default().references += 1;
                }
            }
            if let (Kind::Footnote(_), Some(number)) = (&block.kind, block.footnote_number) {
                let mut text = String::new();
                footnote_preview(block, &mut text);
                notes.entry(number).or_default().preview = Some(text.into());
            }
            collect(&block.children, notes);
        }
    }
    let mut notes = HashMap::new();
    collect(blocks, &mut notes);
    notes
}

struct FocusPreviewBounds {
    child: AnyElement,
    bounds: Rc<Cell<Bounds<Pixels>>>,
    _focus_in: Subscription,
    _focus_out: Subscription,
}

impl gpui::Element for FocusPreviewBounds {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.child.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.child.prepaint_at(bounds.origin, window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.bounds.set(bounds);
        self.child.paint(window, cx);
    }
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
    selection_scope: SharedString,
    resolver: Option<&ImageResolver>,
    images: &mut ImageCache,
    on_task_toggle: Option<&TaskToggleHandler>,
    on_navigate: Option<&NavigationHandler>,
    navigation_anchor: Option<&Entity<crate::Anchor>>,
    assets: &super::assets::AssetCache,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let fonts = DocumentFonts {
        ui: liora_core::ui_font_family(cx),
        code: liora_core::code_font_family(cx),
        selection_scope: Some(selection_scope),
        order: Cell::new(0),
        assets: assets.clone(),
    };
    let footnotes = footnote_info(blocks);
    let mut root = div().flex().flex_col().gap_3().w_full().min_w_0();
    for block in blocks {
        root = root.child(render_block(
            block,
            resolver,
            images,
            on_task_toggle,
            on_navigate,
            navigation_anchor,
            Alignment::None,
            &[],
            &footnotes,
            &fonts,
            None,
            window,
            cx,
        ));
    }
    root.into_any_element()
}

pub(super) fn document_with_editable(
    blocks: &[Block],
    resolver: Option<&ImageResolver>,
    images: &mut ImageCache,
    on_task_toggle: Option<&TaskToggleHandler>,
    on_navigate: Option<&NavigationHandler>,
    editable: &dyn Fn(&Block) -> Option<AnyElement>,
    assets: &super::assets::AssetCache,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let fonts = DocumentFonts {
        ui: liora_core::ui_font_family(cx),
        code: liora_core::code_font_family(cx),
        selection_scope: None,
        order: Cell::new(0),
        assets: assets.clone(),
    };
    let footnotes = footnote_info(blocks);
    let mut root = div().flex().flex_col().gap_3().w_full().min_w_0();
    for block in blocks {
        let element = editable(block).unwrap_or_else(|| {
            render_block(
                block,
                resolver,
                images,
                on_task_toggle,
                on_navigate,
                None,
                Alignment::None,
                &[],
                &footnotes,
                &fonts,
                Some(editable),
                window,
                cx,
            )
        });
        root = root.child(element);
    }
    root.into_any_element()
}

fn render_block(
    block: &Block,
    resolver: Option<&ImageResolver>,
    images: &mut ImageCache,
    on_task_toggle: Option<&TaskToggleHandler>,
    on_navigate: Option<&NavigationHandler>,
    navigation_anchor: Option<&Entity<crate::Anchor>>,
    alignment: Alignment,
    columns: &[Alignment],
    footnotes: &HashMap<usize, FootnoteInfo>,
    fonts: &DocumentFonts,
    editable: Option<&dyn Fn(&Block) -> Option<AnyElement>>,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let id = if let Some(heading_id) = &block.heading_id {
        heading_id.clone()
    } else if let Some(number) = block.footnote_number {
        format!("_footnote-{number}").into()
    } else {
        SharedString::from(format!("markdown-block-{}", block.id))
    };
    let native_kind = match &block.kind {
        Kind::Code(language) if language.as_ref() == "mermaid" => {
            let foreground = gpui::Rgba::from(theme.neutral.text_1);
            let background = gpui::Rgba::from(theme.neutral.body);
            let foreground = [foreground.r, foreground.g, foreground.b]
                .map(|channel| (channel * 255.0).round() as u8);
            let background = [background.r, background.g, background.b]
                .map(|channel| (channel * 255.0).round() as u8);
            Some(super::assets::AssetKind::Mermaid(foreground, background))
        }
        _ => None,
    };
    if let Some(kind) = native_kind {
        return native_asset(kind, block, id, fonts, window, cx);
    }
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
            .child(selectable_leaf(options, fonts, window, cx))
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
        Kind::Callout(kind) => {
            root = root
                .border_l_2()
                .border_color(theme.primary.base)
                .pl_3()
                .child(crate::Text::new(format!("[{:?}]", kind)).text_color(theme.primary.base));
        }
        Kind::Footnote(label) => {
            root = root
                .border_l_2()
                .border_color(theme.neutral.border)
                .pl_3()
                .child(crate::Text::new(format!("Footnote [{}]", label)));
            if let Some(number) = block.footnote_number {
                let count = footnotes.get(&number).map_or(0, |info| info.references);
                for occurrence in 1..=count {
                    let navigation = on_navigate.cloned();
                    let fragment: SharedString =
                        format!("_footnote-ref-{number}-{occurrence}").into();
                    let label: SharedString = if count == 1 {
                        "Back to reference".into()
                    } else {
                        format!("Back to reference {occurrence}").into()
                    };
                    let keyboard_fragment = fragment.clone();
                    let keyboard_navigation = navigation.clone();
                    root = root.child(
                        div()
                            .id(SharedString::from(format!("{id}-back-{occurrence}-focus")))
                            .focusable()
                            .tab_index(0)
                            .on_key_down(move |event: &gpui::KeyDownEvent, _, cx| {
                                if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                                    if let Some(navigate) = &keyboard_navigation {
                                        navigate(keyboard_fragment.clone(), cx);
                                    }
                                    cx.stop_propagation();
                                }
                            })
                            .child(
                                Button::new(label)
                                    .id(SharedString::from(format!("{id}-back-{occurrence}")))
                                    .text()
                                    .small()
                                    .on_click(move |_, _, cx| {
                                        if let Some(navigate) = &navigation {
                                            navigate(fragment.clone(), cx);
                                        }
                                    }),
                            ),
                    );
                }
            }
        }
        Kind::DefinitionList => {
            root = root.border_l_1().border_color(theme.neutral.border).pl_3();
        }
        Kind::DefinitionTitle => {
            root = root.child(crate::Text::new("Term").weight(FontWeight::BOLD));
        }
        Kind::DefinitionDefinition => {
            root = root.pl_3();
        }
        Kind::Metadata(_) => {
            root = root
                .p_2()
                .bg(theme.neutral.hover)
                .rounded_md()
                .child(crate::Text::new("Document metadata"));
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
                let color = if style.footnote.is_some()
                    || style.link.as_deref().is_some_and(|destination| {
                        is_safe_markdown_link(destination) || is_safe_markdown_fragment(destination)
                    }) {
                    theme.primary.base
                } else {
                    theme.neutral.text_1
                };
                TextRun {
                    len: span.range.len(),
                    font,
                    color,
                    background_color: (style.code || style.footnote.is_some() || style.highlight)
                        .then_some(theme.neutral.hover),
                    underline: style
                        .link
                        .as_deref()
                        .is_some_and(|destination| {
                            is_safe_markdown_link(destination)
                                || is_safe_markdown_fragment(destination)
                        })
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
        if let Kind::Math(display) = block.kind {
            let scope = fonts.selection_scope.clone().unwrap_or_else(|| id.clone());
            return inline::formula(options, display, fonts, &scope, true, window, cx);
        }
        if block
            .spans
            .iter()
            .any(|span| span.style.math.is_some() || span.style.superscript || span.style.subscript)
        {
            root = root.child(inline::render(block, options, fonts, window, cx));
        } else {
            root = root.child(selectable_leaf(options, fonts, window, cx));
        }
        let mut links = HashSet::new();
        for span in &block.spans {
            if let Some(destination) = &span.style.link {
                let is_fragment = destination
                    .strip_prefix('#')
                    .is_some_and(is_safe_markdown_fragment);
                if (is_safe_markdown_link(destination) || is_fragment)
                    && links.insert(destination.clone())
                {
                    let destination = destination.clone();
                    let keyboard_destination = destination.clone();
                    let keyboard_navigation = on_navigate.cloned();
                    let click_navigation = on_navigate.cloned();
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
                                    if let Some(fragment) = keyboard_destination.strip_prefix('#') {
                                        if is_safe_markdown_fragment(fragment) {
                                            if let Some(navigate) = &keyboard_navigation {
                                                navigate(fragment.into(), cx);
                                            }
                                        }
                                    } else if is_safe_markdown_link(&keyboard_destination) {
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
                                        if let Some(fragment) = destination.strip_prefix('#') {
                                            if is_safe_markdown_fragment(fragment) {
                                                if let Some(navigate) = &click_navigation {
                                                    navigate(fragment.into(), cx);
                                                }
                                            }
                                        } else if is_safe_markdown_link(&destination) {
                                            cx.open_url(destination.as_ref());
                                        }
                                    }),
                            ),
                    );
                }
            }
        }
        for span in &block.spans {
            let Some(number) = footnote_reference_number(block, span) else {
                continue;
            };
            let navigation = on_navigate.cloned();
            let fragment: SharedString = format!("_footnote-{number}").into();
            let keyboard_fragment = fragment.clone();
            let keyboard_navigation = navigation.clone();
            let reference: SharedString =
                format!("_footnote-ref-{number}-{}", span.style.footnote_occurrence).into();
            let button = Button::new(format!("Footnote [{number}]"))
                .id(SharedString::from(format!(
                    "{id}-footnote-{}",
                    span.range.start
                )))
                .text()
                .small()
                .on_click(move |_, _, cx| {
                    if let Some(navigate) = &navigation {
                        navigate(fragment.clone(), cx);
                    }
                });
            let trigger = if let Some(anchor) = navigation_anchor {
                crate::AnchorTarget::new(reference, anchor.clone(), button).into_any_element()
            } else {
                button.into_any_element()
            };
            let focus = cx.focus_handle();
            let trigger = div()
                .id(SharedString::from(format!(
                    "{id}-footnote-focus-{}",
                    span.range.start
                )))
                .track_focus(&focus)
                .tab_index(0)
                .on_key_down(move |event: &gpui::KeyDownEvent, _, cx| {
                    if matches!(event.keystroke.key.as_str(), "enter" | "space") {
                        if let Some(navigate) = &keyboard_navigation {
                            navigate(keyboard_fragment.clone(), cx);
                        }
                        cx.stop_propagation();
                    }
                })
                .child(trigger);
            root = root.child(
                if let Some(preview) = footnotes
                    .get(&number)
                    .and_then(|info| info.preview.as_ref())
                    .filter(|text| !text.is_empty())
                {
                    let tooltip_id: SharedString =
                        format!("{id}-footnote-preview-{}", span.range.start).into();
                    let bounds = Rc::new(Cell::new(Bounds::default()));
                    let focus_bounds = bounds.clone();
                    let focus_id = tooltip_id.clone();
                    let focus_content = preview.clone();
                    let focus_in = window.on_focus_in(&focus, cx, move |_, cx| {
                        let anchor_bounds = focus_bounds.get();
                        if anchor_bounds.size.width > px(0.0) {
                            set_exclusive_active_tooltip(
                                TooltipData {
                                    id: focus_id.clone(),
                                    content: focus_content.clone(),
                                    anchor_bounds,
                                    placement: liora_core::Placement::Top,
                                    offset: px(8.0),
                                },
                                cx,
                            );
                        }
                    });
                    let blur_id = tooltip_id.clone();
                    let focus_out = window.on_focus_out(&focus, cx, move |_, _, cx| {
                        clear_tooltip(&blur_id, cx);
                    });
                    Tooltip::new(FocusPreviewBounds {
                        child: trigger.into_any_element(),
                        bounds,
                        _focus_in: focus_in,
                        _focus_out: focus_out,
                    })
                    .content(preview.clone())
                    .id(tooltip_id)
                    .into_any_element()
                } else {
                    trigger.into_any_element()
                },
            );
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
            let (native, width, height) = match source {
                MarkdownImage::File(path) => (
                    img(path.clone())
                        .size_full()
                        .object_fit(ObjectFit::Contain)
                        .into_any_element(),
                    None,
                    None,
                ),
                MarkdownImage::Render(image) => {
                    let (width, height) = image_box_size(image);
                    (
                        img(image.clone())
                            .size_full()
                            .object_fit(ObjectFit::Contain)
                            .into_any_element(),
                        Some(width),
                        Some(height),
                    )
                }
            };
            let mut frame = div()
                .max_w(px(640.0))
                .max_h(px(420.0))
                .overflow_hidden()
                .child(native);
            if let (Some(width), Some(height)) = (width, height) {
                frame = frame.w(width).h(height);
            } else {
                // Files are approved paths but their intrinsic size is not available until GPUI loads them.
                frame = frame.w_full().h(px(240.0));
            }
            occurrence_view = occurrence_view.child(frame);
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
            let marker: AnyElement = if let Some(checked) = item.task_checked {
                let range = item
                    .task_marker_range
                    .clone()
                    .unwrap_or_else(|| item.source_range.clone());
                let toggle = on_task_toggle.cloned();
                let item_id = item.id;
                cx.new(|cx| {
                    crate::Checkbox::new(checked, cx).on_change(move |checked, _, cx| {
                        if let Some(toggle) = &toggle {
                            toggle(item_id, range.clone(), checked, cx);
                        }
                    })
                })
                .into_any_element()
            } else {
                crate::Text::new(marker).into_any_element()
            };
            root = root.child(
                div()
                    .flex()
                    .flex_row()
                    .gap_2()
                    .min_w_0()
                    .child(marker)
                    .child(
                        editable
                            .and_then(|editable| editable(item))
                            .unwrap_or_else(|| {
                                render_block(
                                    item,
                                    resolver,
                                    images,
                                    on_task_toggle,
                                    on_navigate,
                                    navigation_anchor,
                                    Alignment::None,
                                    &[],
                                    footnotes,
                                    fonts,
                                    editable,
                                    window,
                                    cx,
                                )
                            }),
                    ),
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
            let element = editable
                .and_then(|editable| editable(child))
                .unwrap_or_else(|| {
                    render_block(
                        child,
                        resolver,
                        images,
                        on_task_toggle,
                        on_navigate,
                        navigation_anchor,
                        child_alignment,
                        child_columns,
                        footnotes,
                        fonts,
                        editable,
                        window,
                        cx,
                    )
                });
            root = root.child(element);
        }
    }
    if matches!(block.kind, Kind::Heading(_) | Kind::Footnote(_)) {
        if let Some(anchor) = navigation_anchor {
            return crate::AnchorTarget::new(id, anchor.clone(), root).into_any_element();
        }
    }
    root.into_any_element()
}

fn selectable_leaf(
    options: SelectableTextOptions,
    fonts: &DocumentFonts,
    window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    if let Some(scope) = &fonts.selection_scope {
        let order = fonts.order.get();
        fonts.order.set(order + 1);
        SelectableText::view_in_scope(options, scope.clone(), order, "\n".into(), window, cx)
    } else {
        SelectableText::view(options, window, cx)
    }
}
fn image_box_size(image: &gpui::RenderImage) -> (gpui::Pixels, gpui::Pixels) {
    let size = image.size(0);
    bounded_image_size(
        i32::from(size.width).max(1) as f32,
        i32::from(size.height).max(1) as f32,
        640.0,
        420.0,
    )
}

fn native_asset(
    kind: super::assets::AssetKind,
    block: &Block,
    id: SharedString,
    fonts: &DocumentFonts,
    _window: &mut Window,
    cx: &mut App,
) -> AnyElement {
    let theme = cx.global::<Config>().theme.clone();
    let mut options = SelectableTextOptions::new(
        SharedString::from(format!("{id}-source")),
        block.text.clone(),
    );
    options.text_color = theme.neutral.text_1;
    options.runs = vec![TextRun {
        len: block.text.len(),
        font: gpui::font(fonts.code.clone()),
        color: theme.neutral.text_1,
        background_color: None,
        underline: None,
        strikethrough: None,
    }];
    let source = selectable_leaf(options, fonts, _window, cx);
    match fonts.assets.render(kind, &block.text) {
        Ok(image) => {
            let (width, height) = image_box_size(&image);
            div()
                .id(id)
                .flex()
                .flex_col()
                .gap_2()
                .min_w_0()
                .w_full()
                .child(
                    div()
                        .max_w(px(640.0))
                        .max_h(px(420.0))
                        .overflow_hidden()
                        .w(width)
                        .h(height)
                        .child(img(image).size_full().object_fit(ObjectFit::Contain)),
                )
                .child(source)
                .into_any_element()
        }
        Err(error) => div()
            .id(id)
            .p_2()
            .border_l_2()
            .border_color(theme.danger.base)
            .child(crate::Text::new(format!("Native render failed: {error}")))
            .child(source)
            .into_any_element(),
    }
}

fn bounded_image_size(
    width: f32,
    height: f32,
    max_width: f32,
    max_height: f32,
) -> (gpui::Pixels, gpui::Pixels) {
    let scale = (max_width / width).min(max_height / height).min(1.0);
    (px(width * scale), px(height * scale))
}

fn is_safe_markdown_fragment(fragment: &str) -> bool {
    !fragment.is_empty()
        && fragment
            .chars()
            .all(|ch| ch.is_alphanumeric() || matches!(ch, '-' | '_'))
}
#[cfg(test)]
mod tests {
    use super::{bounded_image_size, footnote_info};
    use crate::markdown::MarkdownDocument;

    #[test]
    fn image_size_preserves_ratio_and_bounds_large_images() {
        let (width, height) = bounded_image_size(1600.0, 800.0, 640.0, 420.0);
        assert_eq!(width.as_f32(), 640.0);
        assert_eq!(height.as_f32(), 320.0);
    }

    #[test]
    fn image_size_does_not_upscale_small_images() {
        let (width, height) = bounded_image_size(320.0, 200.0, 640.0, 420.0);
        assert_eq!(width.as_f32(), 320.0);
        assert_eq!(height.as_f32(), 200.0);
    }

    #[test]
    fn footnotes_count_every_reference_and_preview_definition_content() {
        let document = MarkdownDocument::new(
            "First[^long] and again[^long].\n\n[^long]: A multi-word definition.\n",
        );
        let notes = footnote_info(&document.blocks);
        let note = &notes[&1];
        assert_eq!(note.references, 2);
        assert_eq!(note.preview.as_deref(), Some("A multi-word definition."));
    }

    #[test]
    fn footnote_preview_stays_on_utf8_boundary_under_large_definitions() {
        let source = format!("See[^long].\n\n[^long]: {}\n", "é".repeat(800));
        let document = MarkdownDocument::new(source);
        let preview = footnote_info(&document.blocks)[&1].preview.clone().unwrap();
        assert_eq!(preview.len(), 1024);
        assert!(preview.chars().all(|ch| ch == 'é'));
    }
}
