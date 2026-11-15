//! Bounded native Mermaid flowchart and sequence rendering; SVG never reaches a web engine.

use std::{
    collections::HashMap,
    fmt,
    sync::{Arc, LazyLock},
};

use gpui::RenderImage;
use image::Frame;

const MAX_SOURCE_BYTES: usize = 16 * 1024;
const MAX_SVG_BYTES: usize = 2 * 1024 * 1024;
const MAX_SIDE: u32 = 4096;
const MAX_PIXELS: u64 = 4_000_000;
const MAX_PNG_BYTES: usize = 8 * 1024 * 1024;
const LOCAL_TEXT_SENTINEL: [u8; 3] = [1, 2, 3];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DiagramDiagnostic {
    pub(crate) source_bytes: usize,
    pub(crate) width_pixels: u32,
    pub(crate) height_pixels: u32,
    pub(crate) png_bytes: usize,
}

#[derive(Clone)]
pub(crate) struct DiagramRender {
    pub(crate) png: Vec<u8>,
    pub(crate) image: Arc<RenderImage>,
    pub(crate) diagnostic: DiagramDiagnostic,
}

#[derive(Debug)]
pub(crate) enum DiagramError {
    SourceTooLong { actual: usize, limit: usize },
    Unsupported,
    FontsUnavailable,
    Parse(mermaid_svg::RenderError),
    SvgTooLong { actual: usize, limit: usize },
    Svg(usvg::Error),
    PixelLimit { width: u32, height: u32 },
    Raster,
    Png(String),
    PngTooLong { actual: usize, limit: usize },
    ImageDecode(String),
}

impl fmt::Display for DiagramError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceTooLong { actual, limit } => {
                write!(f, "diagram source is {actual} bytes; limit is {limit}")
            }
            Self::Unsupported => write!(
                f,
                "only Mermaid flowchart and sequence diagrams are supported"
            ),
            Self::FontsUnavailable => write!(f, "diagram labels require an installed system font"),
            Self::Parse(error) => write!(f, "Mermaid syntax error: {error}"),
            Self::SvgTooLong { actual, limit } => {
                write!(f, "diagram SVG is {actual} bytes; limit is {limit}")
            }
            Self::Svg(error) => write!(f, "diagram SVG parse failed: {error}"),
            Self::PixelLimit { width, height } => write!(
                f,
                "diagram dimensions {width}x{height} exceed raster limits"
            ),
            Self::Raster => write!(f, "diagram raster allocation failed"),
            Self::Png(error) => write!(f, "diagram PNG encoding failed: {error}"),
            Self::PngTooLong { actual, limit } => {
                write!(f, "diagram PNG is {actual} bytes; limit is {limit}")
            }
            Self::ImageDecode(error) => write!(f, "diagram PNG decode failed: {error}"),
        }
    }
}

impl std::error::Error for DiagramError {}

fn diagram_font_selector() -> usvg::FontSelectionFn<'static> {
    let select = usvg::FontResolver::default_font_selector();
    Box::new(move |font, fonts| {
        // Generic aliases can name uninstalled fonts; usvg otherwise drops the entire label.
        select(font, fonts).or_else(|| fonts.faces().next().map(|face| face.id))
    })
}

fn luminance(rgb: [u8; 3]) -> f64 {
    let linear = rgb.map(|channel| {
        let value = f64::from(channel) / 255.0;
        if value <= 0.04045 {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    });
    linear[0] * 0.2126 + linear[1] * 0.7152 + linear[2] * 0.0722
}

fn contrast(a: [u8; 3], b: [u8; 3]) -> f64 {
    let a = luminance(a);
    let b = luminance(b);
    (a.max(b) + 0.05) / (a.min(b) + 0.05)
}

fn parse_css_color(
    value: &str,
    cache: &mut HashMap<String, Option<([u8; 3], f64)>>,
) -> Option<([u8; 3], f64)> {
    if let Some(color) = cache.get(value) {
        return *color;
    }
    let parsed = if value.eq_ignore_ascii_case("none") || value.eq_ignore_ascii_case("transparent")
    {
        Some(([0; 3], 0.0))
    } else {
        value.parse::<svgtypes::Color>().ok().map(|color| {
            (
                [color.red, color.green, color.blue],
                f64::from(color.alpha) / 255.0,
            )
        })
    };
    cache.insert(value.to_owned(), parsed);
    parsed
}

fn composite(foreground: [u8; 3], background: [u8; 3], opacity: f64) -> [u8; 3] {
    let opacity = opacity.clamp(0.0, 1.0);
    std::array::from_fn(|index| {
        (f64::from(foreground[index]) * opacity + f64::from(background[index]) * (1.0 - opacity))
            .round() as u8
    })
}

fn contrasting_ink(background: [u8; 3]) -> [u8; 3] {
    if contrast([0; 3], background) >= 4.5 {
        [0; 3]
    } else {
        [255; 3]
    }
}

fn color_hex([red, green, blue]: [u8; 3]) -> String {
    format!("#{red:02x}{green:02x}{blue:02x}")
}

fn push_markdown_run(output: &mut String, run: &str, bold: bool, italic: bool) {
    if bold {
        output.push_str("<b>");
    }
    if italic {
        output.push_str("<i>");
    }
    for character in run.chars() {
        if character == '<' {
            output.push_str("&lt;");
        } else {
            output.push(character);
        }
    }
    if italic {
        output.push_str("</i>");
    }
    if bold {
        output.push_str("</b>");
    }
}

fn markdown_label_markup(markdown: &str) -> String {
    let mut output = String::with_capacity(markdown.len() + 32);
    let mut run = String::new();
    let (mut bold, mut italic) = (false, false);
    let mut index = 0;
    while index < markdown.len() {
        let bytes = markdown.as_bytes();
        if bytes[index] == b'*' || bytes[index] == b'_' {
            push_markdown_run(&mut output, &run, bold, italic);
            run.clear();
            if index + 1 < bytes.len() && bytes[index + 1] == bytes[index] {
                bold = !bold;
                index += 2;
            } else {
                italic = !italic;
                index += 1;
            }
        } else {
            let character = markdown[index..]
                .chars()
                .next()
                .expect("valid UTF-8 boundary");
            run.push(character);
            index += character.len_utf8();
        }
    }
    push_markdown_run(&mut output, &run, bold, italic);
    output
}

fn color_text(text: &mut String, color: [u8; 3]) {
    let original = std::mem::take(text);
    let trimmed = original.trim();
    let has_line_break = trimmed.contains('\n')
        || trimmed.contains("\\n")
        || trimmed.as_bytes().windows(3).any(|tag| {
            tag[0] == b'<'
                && tag[1].eq_ignore_ascii_case(&b'b')
                && tag[2].eq_ignore_ascii_case(&b'r')
        });
    let markdown = if !has_line_break
        && trimmed.len() >= 2
        && trimmed.starts_with('`')
        && trimmed.ends_with('`')
    {
        Some(markdown_label_markup(&trimmed[1..trimmed.len() - 1]))
    } else {
        None
    };
    let content = markdown.as_deref().unwrap_or(trimmed);
    let prefix_len = original.len() - original.trim_start().len();
    let suffix_start = original.trim_end().len();
    *text = format!(
        "{}<span style=\"color:{}\">{content}</span>{}",
        &original[..prefix_len],
        color_hex(color),
        &original[suffix_start..]
    );
}

fn adapt_sequence_items(items: &mut [mermaid_svg::ast::SequenceItem], in_rect: bool) {
    use mermaid_svg::ast::SequenceItem;
    for item in items {
        match item {
            SequenceItem::Message(message) if in_rect => {
                color_text(&mut message.text, LOCAL_TEXT_SENTINEL);
            }
            SequenceItem::Rect(rect) => adapt_sequence_items(&mut rect.items, true),
            SequenceItem::Alt(branches)
            | SequenceItem::Par(branches)
            | SequenceItem::Critical(branches) => {
                for branch in branches {
                    if in_rect {
                        color_text(&mut branch.label, LOCAL_TEXT_SENTINEL);
                    }
                    adapt_sequence_items(&mut branch.items, in_rect);
                }
            }
            SequenceItem::Loop(block) | SequenceItem::Opt(block) | SequenceItem::Break(block) => {
                if in_rect {
                    color_text(&mut block.label, LOCAL_TEXT_SENTINEL);
                }
                adapt_sequence_items(&mut block.items, in_rect);
            }
            _ => {}
        }
    }
}

fn sequence_path_sample(path: &str) -> Option<(f64, f64)> {
    let (mut current, mut points) = ((0.0, 0.0), Vec::new());
    for segment in svgtypes::PathParser::from(path) {
        current = match segment.ok()? {
            svgtypes::PathSegment::MoveTo { abs, x, y }
            | svgtypes::PathSegment::LineTo { abs, x, y } => {
                if abs {
                    (x, y)
                } else {
                    (current.0 + x, current.1 + y)
                }
            }
            _ => return points.first().copied(),
        };
        points.push(current);
    }
    let first = *points.first()?;
    if points.len() == 4 {
        let min_x = points
            .iter()
            .map(|point| point.0)
            .fold(f64::INFINITY, f64::min);
        let max_x = points
            .iter()
            .map(|point| point.0)
            .fold(f64::NEG_INFINITY, f64::max);
        let min_y = points
            .iter()
            .map(|point| point.1)
            .fold(f64::INFINITY, f64::min);
        let max_y = points
            .iter()
            .map(|point| point.1)
            .fold(f64::NEG_INFINITY, f64::max);
        // Self-message paths are 20px loops; sample their exposed right side, not activation fill.
        if (max_x - min_x - 20.0).abs() < 0.01 && (max_y - min_y - 20.0).abs() < 0.01 {
            return Some((max_x, (min_y + max_y) / 2.0));
        }
    }
    Some(first)
}

fn adapt_sequence_local_paints(
    svg: &mut String,
    foreground: [u8; 3],
    background: [u8; 3],
    color_cache: &mut HashMap<String, Option<([u8; 3], f64)>>,
) -> Result<(), DiagramError> {
    let document =
        usvg::roxmltree::Document::parse(svg).map_err(|error| DiagramError::Svg(error.into()))?;
    let number =
        |node: usvg::roxmltree::Node<'_, '_>, name| node.attribute(name)?.parse::<f64>().ok();
    let elements: Vec<_> = document
        .descendants()
        .filter(|node| node.is_element())
        .collect();
    let rect_fills: Vec<_> = elements
        .iter()
        .filter_map(|node| {
            if !node.has_tag_name("rect") {
                return None;
            }
            Some((
                node.range().start,
                number(*node, "x")?,
                number(*node, "y")?,
                number(*node, "width")?,
                number(*node, "height")?,
                parse_css_color(node.attribute("fill")?, color_cache)?,
            ))
        })
        .collect();
    let sentinel = color_hex(LOCAL_TEXT_SENTINEL);
    let mut replacements = Vec::new();
    for node in &elements {
        if node.has_tag_name("tspan") && node.attribute("fill") == Some(sentinel.as_str()) {
            let ink = node
                .ancestors()
                .find(|ancestor| ancestor.has_tag_name("text"))
                .and_then(|text| {
                    let (x, y) = number(text, "x").zip(number(text, "y"))?;
                    let mut local = background;
                    for &(rect_start, left, top, width, height, (rgb, opacity)) in &rect_fills {
                        if rect_start < text.range().start
                            && x >= left
                            && x <= left + width
                            && y >= top
                            && y <= top + height
                        {
                            local = composite(rgb, local, opacity);
                        }
                    }
                    Some(if contrast(foreground, local) >= 4.5 {
                        foreground
                    } else {
                        contrasting_ink(local)
                    })
                })
                .unwrap_or(foreground);
            if let Some(attribute) = node.attribute_node("fill") {
                replacements.push((attribute.range_value(), color_hex(ink)));
            }
        }
        if node
            .parent()
            .is_some_and(|parent| parent.has_tag_name("marker"))
        {
            for attribute in node.attributes().filter(|attribute| {
                matches!(attribute.name(), "fill" | "stroke") && attribute.value() != "none"
            }) {
                replacements.push((attribute.range_value(), "context-stroke".to_owned()));
            }
            continue;
        }
        if node.attribute("stroke-width") != Some("1.5") {
            continue;
        }
        let point = if node.has_tag_name("line") {
            number(*node, "x1")
                .zip(number(*node, "x2"))
                .zip(number(*node, "y1"))
                .map(|((x1, x2), y)| ((x1 + x2) / 2.0, y))
        } else {
            node.attribute("d").and_then(sequence_path_sample)
        };
        if let Some((x, y)) = point {
            let mut local = background;
            for &(rect_start, left, top, width, height, (rgb, opacity)) in &rect_fills {
                if rect_start < node.range().start
                    && x >= left
                    && x <= left + width
                    && y >= top
                    && y <= top + height
                {
                    local = composite(rgb, local, opacity);
                }
            }
            let ink = if contrast(foreground, local) >= 3.0 {
                foreground
            } else {
                contrasting_ink(local)
            };
            if let Some(attribute) = node.attribute_node("stroke") {
                replacements.push((attribute.range_value(), color_hex(ink)));
            }
        }
    }
    replacements.sort_unstable_by_key(|(range, _)| range.start);
    for (range, value) in replacements.into_iter().rev() {
        svg.replace_range(range, &value);
    }
    Ok(())
}

fn diagram_svg(
    source: &str,
    foreground: [u8; 3],
    background: [u8; 3],
) -> Result<String, DiagramError> {
    let (mut diagram, meta) =
        mermaid_svg::parse_with_meta(source).map_err(|error| DiagramError::Parse(error.into()))?;
    let mut theme = meta
        .theme
        .as_deref()
        .and_then(mermaid_svg::Theme::by_name)
        .unwrap_or_default();
    theme.apply_theme_variables(&meta.theme_variables);
    if let Some(family) = meta.font_family {
        theme.font_family = family.into();
    }
    if let Some(size) = meta.font_size {
        theme.font_size = size;
    }
    if let Some(responsive) = meta.use_max_width {
        theme.responsive = responsive;
    }

    let foreground = if contrast(foreground, background) >= 4.5 {
        foreground
    } else {
        contrasting_ink(background)
    };
    let ink = color_hex(foreground);
    let surface = color_hex(background);
    theme.fg = ink.clone().into();
    theme.fg_muted = ink.clone().into();
    theme.actor_text_color = Some(ink.clone().into());
    theme.signal_text_color = Some(ink.clone().into());
    theme.title_color = Some(ink.clone().into());
    theme.actor_stroke = ink.clone().into();
    theme.lifeline = ink.clone().into();
    theme.arrow_stroke = ink.clone().into();
    theme.note_stroke = ink.clone().into();
    theme.activation_stroke = ink.clone().into();
    theme.flow_node_stroke = ink.clone().into();
    theme.flow_edge_stroke = ink.clone().into();
    theme.flow_cluster_stroke = ink.into();
    // Replace theme defaults, not the transparent SVG canvas or explicit shape fills.
    theme.actor_fill = surface.clone().into();
    theme.note_fill = surface.clone().into();
    theme.activation_fill = surface.clone().into();
    theme.frame_label_fill = surface.clone().into();
    theme.flow_node_fill = surface.clone().into();
    theme.flow_label_bg = surface.clone().into();
    theme.flow_cluster_fill = surface.into();

    let mut color_cache = HashMap::new();
    if let mermaid_svg::Diagram::Flowchart(flowchart) = &mut diagram {
        for node in &mut flowchart.nodes {
            let mut fill = None;
            let mut text_color = None;
            let mut opacity = None;
            let mut merge = |style: &[(String, String)]| {
                for (key, value) in style {
                    match key.as_str() {
                        "fill" => fill = Some(value.clone()),
                        "color" => text_color = Some(value.clone()),
                        "opacity" => opacity = Some(value.clone()),
                        _ => {}
                    }
                }
            };
            if let Some(style) = flowchart.class_defs.get("default") {
                merge(style);
            }
            for class in &node.classes {
                if let Some(style) = flowchart.class_defs.get(class) {
                    merge(style);
                }
            }
            merge(&node.style);
            if text_color.is_none() {
                if let Some(fill) = fill.and_then(|value| parse_css_color(&value, &mut color_cache))
                {
                    let shape_opacity = opacity
                        .and_then(|value| value.trim().parse::<svgtypes::Length>().ok())
                        .and_then(|value| match value.unit {
                            svgtypes::LengthUnit::Percent => Some(value.number / 100.0),
                            svgtypes::LengthUnit::None => Some(value.number),
                            _ => None,
                        })
                        .map(|value| value.clamp(0.0, 1.0))
                        .unwrap_or(1.0);
                    let local_background = composite(fill.0, background, fill.1 * shape_opacity);
                    node.style
                        .push(("color".into(), color_hex(contrasting_ink(local_background))));
                }
            }
        }
    }
    let sequence_autonumber = matches!(
        &diagram,
        mermaid_svg::Diagram::Sequence(sequence) if sequence.autonumber
    );
    if let mermaid_svg::Diagram::Sequence(sequence) = &mut diagram {
        adapt_sequence_items(&mut sequence.items, false);
    }

    let mut svg =
        mermaid_svg::render_diagram_with(&diagram, &theme).map_err(DiagramError::Parse)?;
    if matches!(&diagram, mermaid_svg::Diagram::Sequence(_)) {
        adapt_sequence_local_paints(&mut svg, foreground, background, &mut color_cache)?;
    }
    if sequence_autonumber {
        let badge_ink = color_hex(contrasting_ink(foreground));
        let old = "text-anchor=\"middle\" fill=\"#fff\" font-size=\"11\"";
        let new = format!("text-anchor=\"middle\" fill=\"{badge_ink}\" font-size=\"11\"");
        svg = svg.replace(old, &new);
    }
    Ok(svg)
}

/// Render Mermaid source to a PNG and GPUI image without resolving any SVG image href.
pub(crate) fn render_diagram(
    source: &str,
    foreground: [u8; 3],
    background: [u8; 3],
) -> Result<DiagramRender, DiagramError> {
    let source_bytes = source.len();
    if source_bytes > MAX_SOURCE_BYTES {
        return Err(DiagramError::SourceTooLong {
            actual: source_bytes,
            limit: MAX_SOURCE_BYTES,
        });
    }
    let mut lines = source.trim_start_matches('\u{feff}').trim_start().lines();
    let mut line = lines.next().unwrap_or("").trim();
    if line == "---" {
        let _ = lines.find(|line| line.trim() == "---");
        line = "";
    }
    let header = std::iter::once(line)
        .chain(lines)
        .find(|line| {
            let line = line.trim();
            !line.is_empty() && !line.starts_with("%%")
        })
        .unwrap_or("")
        .trim();
    let kind = header.split_whitespace().next().unwrap_or("");
    if !matches!(kind, "flowchart" | "graph" | "sequenceDiagram") {
        return Err(DiagramError::Unsupported);
    }

    let svg = diagram_svg(source, foreground, background)?;
    if svg.len() > MAX_SVG_BYTES {
        return Err(DiagramError::SvgTooLong {
            actual: svg.len(),
            limit: MAX_SVG_BYTES,
        });
    }
    let mut options = usvg::Options::default();
    options.image_href_resolver = usvg::ImageHrefResolver {
        resolve_data: Box::new(|_, _, _| None),
        resolve_string: Box::new(|_, _| None),
    };
    // Font lookup is independent of SVG hrefs; never let SVG select a host file or URL.
    static FONTS: LazyLock<Arc<usvg::fontdb::Database>> = LazyLock::new(|| {
        let mut fonts = usvg::fontdb::Database::new();
        fonts.load_system_fonts();
        Arc::new(fonts)
    });
    if FONTS.faces().next().is_none() {
        return Err(DiagramError::FontsUnavailable);
    }
    options.fontdb = FONTS.clone();
    options.font_resolver.select_font = diagram_font_selector();
    let tree = usvg::Tree::from_str(&svg, &options).map_err(DiagramError::Svg)?;
    let size = tree.size();
    let width = size.width().ceil() as u32;
    let height = size.height().ceil() as u32;
    if width == 0
        || height == 0
        || width > MAX_SIDE
        || height > MAX_SIDE
        || u64::from(width) * u64::from(height) > MAX_PIXELS
    {
        return Err(DiagramError::PixelLimit { width, height });
    }
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or(DiagramError::Raster)?;
    resvg::render(
        &tree,
        tiny_skia::Transform::identity(),
        &mut pixmap.as_mut(),
    );
    let png = pixmap
        .encode_png()
        .map_err(|error| DiagramError::Png(error.to_string()))?;
    if png.len() > MAX_PNG_BYTES {
        return Err(DiagramError::PngTooLong {
            actual: png.len(),
            limit: MAX_PNG_BYTES,
        });
    }
    let png_bytes = png.len();
    let mut rgba = image::load_from_memory(&png)
        .map_err(|error| DiagramError::ImageDecode(error.to_string()))?
        .into_rgba8();
    for pixel in rgba.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    let image = Arc::new(RenderImage::new([Frame::new(rgba)]));
    Ok(DiagramRender {
        png,
        image,
        diagnostic: DiagramDiagnostic {
            source_bytes,
            width_pixels: width,
            height_pixels: height,
            png_bytes,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_text_contrast(svg: &str, label: &str, background: [u8; 3]) {
        let mut options = usvg::Options::default();
        options.fontdb_mut().load_system_fonts();
        options.font_resolver.select_font = diagram_font_selector();
        let tree = usvg::Tree::from_str(svg, &options).unwrap();

        fn visit(group: &usvg::Group, label: &str, background: [u8; 3]) -> bool {
            for node in group.children() {
                match node {
                    usvg::Node::Group(group) if visit(group, label, background) => return true,
                    usvg::Node::Text(text)
                        if text
                            .chunks()
                            .iter()
                            .any(|chunk| chunk.text().contains(label)) =>
                    {
                        let bounds = node.abs_layer_bounding_box().unwrap();
                        let mut raster = tiny_skia::Pixmap::new(
                            bounds.width().ceil() as u32,
                            bounds.height().ceil() as u32,
                        )
                        .unwrap();
                        resvg::render_node(
                            node,
                            tiny_skia::Transform::identity(),
                            &mut raster.as_mut(),
                        )
                        .unwrap();
                        let pixels: Vec<_> = raster
                            .pixels()
                            .iter()
                            .filter(|pixel| pixel.alpha() >= 250)
                            .collect();
                        assert!(!pixels.is_empty(), "label must rasterize");
                        for pixel in pixels {
                            let alpha = u32::from(pixel.alpha());
                            let foreground = [pixel.red(), pixel.green(), pixel.blue()];
                            let visible = std::array::from_fn(|i| {
                                (u32::from(foreground[i])
                                    + (u32::from(background[i]) * (255 - alpha) + 127) / 255)
                                    .min(255) as u8
                            });
                            assert!(
                                contrast(visible, background) >= 4.5,
                                "{label} is unreadable against {background:?}"
                            );
                        }
                        return true;
                    }
                    _ => {}
                }
            }
            false
        }
        assert!(
            visit(tree.root(), label, background),
            "missing text label {label}"
        );
    }

    #[test]
    fn renders_flowchart_and_sequence_to_native_images() {
        for source in [
            "flowchart LR\nA[Start] --> B[Done]\n",
            "sequenceDiagram\nAlice->>Bob: Hello\n",
        ] {
            let rendered = render_diagram(source, [32; 3], [255; 3])
                .expect("supported diagram should rasterize");
            assert!(rendered.png.starts_with(b"\x89PNG\r\n\x1a\n"));
            assert_eq!(rendered.diagnostic.source_bytes, source.len());
            assert_eq!(rendered.diagnostic.png_bytes, rendered.png.len());
            let rgba = image::load_from_memory(&rendered.png).unwrap().into_rgba8();
            assert!(rgba.pixels().any(|pixel| pixel[3] == 0));
            assert!(rgba.pixels().any(|pixel| pixel[3] == 255));
            assert_eq!(
                rendered.image.size(0).width.0 as u32,
                rendered.diagnostic.width_pixels
            );
            assert_eq!(
                rendered.image.size(0).height.0 as u32,
                rendered.diagnostic.height_pixels
            );
        }
    }

    #[test]
    fn transparent_diagrams_keep_text_and_edges_readable_on_host_surfaces() {
        fn check_nodes(group: &usvg::Group, background: [u8; 3]) -> (usize, usize) {
            let mut counts = (0, 0);
            for node in group.children() {
                let is_text = matches!(node, usvg::Node::Text(_));
                let is_edge = matches!(node, usvg::Node::Path(path) if path.stroke().is_some());
                if let usvg::Node::Group(group) = node {
                    let children = check_nodes(group, background);
                    counts.0 += children.0;
                    counts.1 += children.1;
                } else if is_edge {
                    let usvg::Node::Path(path) = node else {
                        unreachable!()
                    };
                    let stroke = path.stroke().unwrap();
                    let usvg::Paint::Color(color) = stroke.paint() else {
                        panic!("solid edge ink required")
                    };
                    assert!(contrast([color.red, color.green, color.blue], background) >= 3.0);
                    counts.1 += 1;
                } else if is_text {
                    let bounds = node.abs_layer_bounding_box().expect("drawable text bounds");
                    let mut raster = tiny_skia::Pixmap::new(
                        bounds.width().ceil() as u32,
                        bounds.height().ceil() as u32,
                    )
                    .unwrap();
                    resvg::render_node(
                        node,
                        tiny_skia::Transform::identity(),
                        &mut raster.as_mut(),
                    )
                    .unwrap();
                    let mut visible = 0;
                    for pixel in raster.pixels().iter().filter(|pixel| pixel.alpha() >= 250) {
                        // tiny-skia stores premultiplied channels: composite on the real host.
                        let alpha = u32::from(pixel.alpha());
                        let channels = [pixel.red(), pixel.green(), pixel.blue()];
                        let composited = std::array::from_fn(|i| {
                            (u32::from(channels[i])
                                + (u32::from(background[i]) * (255 - alpha) + 127) / 255)
                                .min(255) as u8
                        });
                        assert!(
                            contrast(composited, background) >= if is_text { 4.5 } else { 3.0 }
                        );
                        visible += 1;
                    }
                    assert!(
                        visible > 0,
                        "text and edges must actually paint contrasting pixels"
                    );
                    if is_text {
                        counts.0 += 1;
                    } else {
                        counts.1 += 1;
                    }
                }
            }
            counts
        }

        for (foreground, background) in [
            ([32; 3], [255; 3]),
            ([230; 3], [24; 3]),
            ([125; 3], [125; 3]),
            ([100, 140, 120], [100, 140, 120]),
        ] {
            for source in [
                "flowchart LR\nA[Native] --> B[GPUI]\n",
                "sequenceDiagram\nAlice->>Bob: Hello\n",
                "%%{init: {'theme': 'dark', 'themeVariables': {'primaryTextColor': '#ffffff', 'lineColor': '#ffffff'}}}%%\nflowchart LR\nA[Native] --> B[GPUI]\n",
                "%%{init: {'theme': 'dark'}}%%\nsequenceDiagram\nAlice->>Bob: Hello\n",
            ] {
                let svg = diagram_svg(source, foreground, background).unwrap();
                let mut options = usvg::Options::default();
                options.fontdb_mut().load_system_fonts();
                options.font_resolver.select_font = diagram_font_selector();
                let tree = usvg::Tree::from_str(&svg, &options).unwrap();
                let (text, edges) = check_nodes(tree.root(), background);
                assert!(text >= 2, "labels must survive shaping");
                assert!(edges > 0, "edges must survive conversion: {source:?}");
                let rendered = render_diagram(source, foreground, background).unwrap();
                let image = image::load_from_memory(&rendered.png).unwrap().into_rgba8();
                assert!(
                    image.pixels().any(|pixel| pixel[3] == 0),
                    "canvas must remain transparent"
                );
            }
        }
    }

    #[test]
    fn flowchart_explicit_fills_get_local_default_text_ink() {
        let source = "flowchart LR\nA[Class fill] --> B[Inline fill]\nC[Explicit ink]\nclassDef pale fill:#fff\nclass A pale\nstyle B fill:#000\nstyle C fill:#fff,color:#ff0000\n";
        let svg = diagram_svg(source, [230; 3], [24; 3]).unwrap();

        assert!(svg.contains("fill=\"#fff\""), "preserve class fill");
        assert!(svg.contains("fill=\"#000\""), "preserve inline fill");
        assert!(svg.contains("fill=\"#000000\""), "dark ink on white fill");
        assert!(svg.contains("fill=\"#ffffff\""), "light ink on black fill");
        assert!(
            svg.contains("fill=\"#ff0000\""),
            "preserve explicit text color"
        );
    }

    #[test]
    fn sequence_rects_get_local_message_ink_and_keep_transparent_canvas() {
        let source = "sequenceDiagram\nrect rgb(255,255,255)\nA->>B: Light region\nrect #000\nB->>A: Dark region\nend\nend\n";
        let svg = diagram_svg(source, [230; 3], [24; 3]).unwrap();

        assert!(svg.contains("fill=\"rgb(255,255,255)\""));
        assert!(svg.contains("fill=\"#000\""));
        assert!(svg.contains("fill=\"#000000\">Light region</tspan>"));
        assert!(svg.contains("fill=\"#000000\">Dark region</tspan>"));
        let rendered = render_diagram(source, [230; 3], [24; 3]).unwrap();
        let image = image::load_from_memory(&rendered.png).unwrap().into_rgba8();
        assert!(image.pixels().any(|pixel| pixel[3] == 0));
    }

    #[test]
    fn sequence_nested_rects_use_close_order_for_local_raster_contrast() {
        for (outer, local_background) in [
            ("rgba(255,255,255,0.1)", composite([255; 3], [0; 3], 0.1)),
            ("none", [0; 3]),
        ] {
            let source =
                format!("sequenceDiagram\nrect {outer}\nrect #000\nA->>B: Unreadable\nend\nend\n");
            let svg = diagram_svg(&source, [32; 3], [255; 3]).unwrap();
            assert_text_contrast(&svg, "Unreadable", local_background);
            assert!(svg.contains("stroke=\"#ffffff\" stroke-width=\"1.5\""));
            assert!(render_diagram(&source, [32; 3], [255; 3]).is_ok());
        }
    }

    #[test]
    fn self_message_label_uses_actual_backdrop_outside_its_band() {
        let source = "sequenceDiagram\nrect #000\nA->>A: Invisible\nend\n";
        let svg = diagram_svg(source, [32; 3], [255; 3]).unwrap();
        assert_text_contrast(&svg, "Invisible", [255; 3]);
    }

    #[test]
    fn active_self_message_loop_uses_band_backdrop_not_activation_fill() {
        let source =
            "sequenceDiagram\nrect #000\nactivate A\nA->>A: Self message\ndeactivate A\nend\n";
        let svg = diagram_svg(source, [32; 3], [255; 3]).unwrap();
        assert!(svg.contains("stroke=\"#ffffff\" stroke-width=\"1.5\""));
        assert!(svg.contains("context-stroke"));
        assert_text_contrast(&svg, "Self message", [255; 3]);
    }

    #[test]
    fn percentage_opacity_uses_svg_semantics_for_inline_and_class_fills() {
        let local_background = composite([255; 3], [24; 3], 0.1);
        for source in [
            "flowchart LR\nA[Unreadable]\nstyle A fill:#fff,opacity:10%\n",
            "flowchart LR\nclassDef translucent fill:#fff,opacity:10%\nA[Unreadable]\nclass A translucent\n",
        ] {
            let svg = diagram_svg(source, [230; 3], [24; 3]).unwrap();
            assert_text_contrast(&svg, "Unreadable", local_background);
            assert!(render_diagram(source, [230; 3], [24; 3]).is_ok());
        }
    }

    #[test]
    fn flowchart_labels_rasterize_when_generic_font_aliases_are_missing() {
        let svg = mermaid_svg::render("flowchart LR\nA[Native] --> B[GPUI]\n").unwrap();
        assert!(svg.contains("font-family=\"sans-serif\""));
        assert!(svg.contains(">Native</text>"));
        assert!(svg.contains(">GPUI</text>"));
        assert!(!svg.contains("foreignObject"));

        let mut fonts = usvg::fontdb::Database::new();
        fonts.load_system_fonts();
        assert!(
            fonts.faces().next().is_some(),
            "label regression requires a system font"
        );
        // Reproduce fontconfig aliases that name families absent from the loaded database.
        fonts.set_sans_serif_family("__missing_diagram_sans__");
        fonts.set_serif_family("__missing_diagram_serif__");
        let mut options = usvg::Options::default();
        options.fontdb = Arc::new(fonts);
        options.font_resolver.select_font = diagram_font_selector();
        let tree = usvg::Tree::from_str(&svg, &options).unwrap();

        fn assert_visible_labels(group: &usvg::Group) -> usize {
            let mut labels = 0;
            for node in group.children() {
                match node {
                    usvg::Node::Group(group) => labels += assert_visible_labels(group),
                    usvg::Node::Text(_) => {
                        let bounds = node.abs_layer_bounding_box().unwrap();
                        let mut pixels = tiny_skia::Pixmap::new(
                            bounds.width().ceil() as u32,
                            bounds.height().ceil() as u32,
                        )
                        .unwrap();
                        resvg::render_node(
                            node,
                            tiny_skia::Transform::identity(),
                            &mut pixels.as_mut(),
                        )
                        .expect("label must have drawable bounds");
                        assert!(
                            pixels
                                .pixels()
                                .iter()
                                .filter(|pixel| pixel.alpha() > 128)
                                .count()
                                > 12,
                            "each label must paint glyph pixels, independently of boxes and arrows"
                        );
                        labels += 1;
                    }
                    _ => {}
                }
            }
            labels
        }
        assert_eq!(assert_visible_labels(tree.root()), 2);
    }

    #[test]
    fn accepts_comments_and_frontmatter_before_supported_diagrams() {
        for source in [
            "%% a comment\nflowchart LR\nA --> B\n",
            "---\ntitle: Example\n---\nsequenceDiagram\nA->>B: hi\n",
            "%%{init: {}}%%\nflowchart LR\nA --> B\n",
        ] {
            assert!(render_diagram(source, [32; 3], [255; 3]).is_ok());
        }
        assert!(matches!(
            render_diagram("%% comment\npie\nA: 1", [32; 3], [255; 3]),
            Err(DiagramError::Unsupported)
        ));
    }

    #[test]
    fn rejects_invalid_and_unsupported_diagrams() {
        assert!(matches!(
            render_diagram("flowchart INVALID\nA --> B\n", [32; 3], [255; 3]),
            Err(DiagramError::Parse(_))
        ));
        assert!(matches!(
            render_diagram("pie\n\"A\": 1\n", [32; 3], [255; 3]),
            Err(DiagramError::Unsupported)
        ));
        assert!(matches!(
            render_diagram(&"x".repeat(MAX_SOURCE_BYTES + 1), [32; 3], [255; 3]),
            Err(DiagramError::SourceTooLong { .. })
        ));
    }
}
