//! Watermark module.
//!
//! This public module implements the Liora watermark overlay component for repeated background labels. It keeps the reusable
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

use gpui::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, LayoutId, Pixels, Point, RenderOnce, ScaledPixels, SharedString, Style,
    TransformationMatrix, ViewElement as Component, Window, div, point, prelude::*, px, relative,
};
use liora_core::Config;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex, OnceLock};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control watermark placement behavior.
pub enum WatermarkPlacement {
    #[default]
    /// Places the watermark in the cover region.
    Cover,
    /// Places the watermark in the header region.
    Header,
    /// Places the watermark in the footer region.
    Footer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Normalized origin for watermark rotation and skew transforms.
pub struct WatermarkTransformOrigin {
    /// Horizontal origin from `0.0` (left) to `1.0` (right).
    pub x: f32,
    /// Vertical origin from `0.0` (top) to `1.0` (bottom).
    pub y: f32,
}

impl WatermarkTransformOrigin {
    /// Tile center transform origin.
    pub const CENTER: Self = Self { x: 0.5, y: 0.5 };
    /// Tile top-left transform origin.
    pub const TOP_LEFT: Self = Self { x: 0.0, y: 0.0 };

    /// Creates a normalized transform origin. Values outside `0.0..=1.0` are clamped.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }.clamped()
    }

    fn clamped(self) -> Self {
        Self {
            x: self.x.clamp(0.0, 1.0),
            y: self.y.clamp(0.0, 1.0),
        }
    }
}

/// Fluent native GPUI component for rendering Liora watermark.
pub struct Watermark {
    content: AnyElement,
    text: SharedString,
    placement: WatermarkPlacement,
    opacity: f32,
    color: Option<Hsla>,
    gap_x: Pixels,
    gap_y: Pixels,
    rotate_degrees: f32,
    skew_x_degrees: f32,
    skew_y_degrees: f32,
    transform_origin: WatermarkTransformOrigin,
    z_index: i32,
    rows: usize,
    columns: usize,
}

impl Watermark {
    /// Creates `Watermark` initialized from the supplied content, and text.
    pub fn new(content: impl IntoElement, text: impl Into<SharedString>) -> Self {
        Self {
            content: content.into_any_element(),
            text: text.into(),
            placement: WatermarkPlacement::Cover,
            opacity: 0.16,
            color: None,
            gap_x: px(96.0),
            gap_y: px(72.0),
            rotate_degrees: -22.0,
            skew_x_degrees: 0.0,
            skew_y_degrees: 0.0,
            transform_origin: WatermarkTransformOrigin::CENTER,
            z_index: 10,
            rows: 4,
            columns: 4,
        }
    }

    /// Selects the popup, label, or overlay placement.
    pub fn placement(mut self, placement: WatermarkPlacement) -> Self {
        self.placement = placement;
        self
    }
    /// Sets the header value used by the component.
    pub fn header(self) -> Self {
        self.placement(WatermarkPlacement::Header)
    }
    /// Sets the footer value used by the component.
    pub fn footer(self) -> Self {
        self.placement(WatermarkPlacement::Footer)
    }
    /// Sets the opacity value used by the component.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity.clamp(0.0, 1.0);
        self
    }
    /// Applies an explicit color instead of the theme-derived default.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }
    /// Sets the spacing between child elements.
    pub fn gap(mut self, x: impl Into<Pixels>, y: impl Into<Pixels>) -> Self {
        self.gap_x = x.into().max(px(8.0));
        self.gap_y = y.into().max(px(8.0));
        self
    }
    /// Sets the clockwise rotation, in degrees, applied to each rendered watermark tile.
    pub fn rotate(mut self, degrees: f32) -> Self {
        self.rotate_degrees = degrees;
        self
    }
    /// Sets the horizontal skew/tilt angle, in degrees, applied to each watermark tile.
    pub fn skew_x(mut self, degrees: f32) -> Self {
        self.skew_x_degrees = degrees;
        self
    }
    /// Sets the vertical skew/tilt angle, in degrees, applied to each watermark tile.
    pub fn skew_y(mut self, degrees: f32) -> Self {
        self.skew_y_degrees = degrees;
        self
    }
    /// Sets both horizontal and vertical skew/tilt angles, in degrees.
    pub fn skew(mut self, x_degrees: f32, y_degrees: f32) -> Self {
        self.skew_x_degrees = x_degrees;
        self.skew_y_degrees = y_degrees;
        self
    }
    /// Sets the normalized transform origin used by rotation and skew, where `(0.5, 0.5)` is the tile center.
    pub fn rotation_center(mut self, x: f32, y: f32) -> Self {
        self.transform_origin = WatermarkTransformOrigin::new(x, y);
        self
    }
    /// Sets the normalized transform origin used by rotation and skew.
    pub fn transform_origin(mut self, origin: WatermarkTransformOrigin) -> Self {
        self.transform_origin = origin.clamped();
        self
    }
    /// Sets the density value used by the component.
    pub fn density(mut self, rows: usize, columns: usize) -> Self {
        self.rows = rows.max(1);
        self.columns = columns.max(1);
        self
    }
    /// Sets the z index value used by the component.
    pub fn z_index(mut self, z: i32) -> Self {
        self.z_index = z;
        self
    }
    /// Performs the tile count operation used by this component.
    pub fn tile_count(&self) -> usize {
        match self.placement {
            WatermarkPlacement::Cover => self.rows * self.columns,
            WatermarkPlacement::Header | WatermarkPlacement::Footer => self.columns,
        }
    }
}

impl RenderOnce for Watermark {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let color = self
            .color
            .unwrap_or(theme.neutral.text_3)
            .opacity(self.opacity);
        let count = self.tile_count();
        let transform = WatermarkTileTransform {
            rotate_degrees: self.rotate_degrees,
            skew_x_degrees: self.skew_x_degrees,
            skew_y_degrees: self.skew_y_degrees,
            origin: self.transform_origin,
        };
        let tile = WatermarkTile::new(self.text.clone(), color, transform);
        let overlay = div()
            .absolute()
            .top_0()
            .left_0()
            .right_0()
            .bottom_0()
            .overflow_hidden()
            .when(self.placement == WatermarkPlacement::Header, |s| {
                s.bottom_auto().h(px(72.0))
            })
            .when(self.placement == WatermarkPlacement::Footer, |s| {
                s.top_auto().h(px(72.0))
            })
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_x(self.gap_x)
                    .gap_y(self.gap_y)
                    .p_4()
                    .children((0..count).map(|_| {
                        div()
                            .flex_none()
                            .w(WATERMARK_TILE_WIDTH)
                            .h(WATERMARK_TILE_HEIGHT)
                            .child(tile.clone())
                            .into_any_element()
                    })),
            );
        div()
            .id(liora_core::unique_id("watermark"))
            .relative()
            .overflow_hidden()
            .child(self.content)
            .child(overlay)
    }
}

const WATERMARK_TILE_WIDTH: Pixels = px(160.0);
const WATERMARK_TILE_HEIGHT: Pixels = px(42.0);
const WATERMARK_TILE_FONT_SIZE: f32 = 14.0;
const WATERMARK_MAX_SKEW_DEGREES: f32 = 75.0;

#[derive(Clone, Copy, Debug, PartialEq)]
struct WatermarkTileTransform {
    rotate_degrees: f32,
    skew_x_degrees: f32,
    skew_y_degrees: f32,
    origin: WatermarkTransformOrigin,
}

#[derive(Clone)]
struct WatermarkTile {
    text: SharedString,
    color: Hsla,
    transform: WatermarkTileTransform,
    svg: Arc<[u8]>,
    svg_key: SharedString,
}

impl WatermarkTile {
    fn new(text: SharedString, color: Hsla, transform: WatermarkTileTransform) -> Self {
        let (svg_key, svg) = cached_watermark_svg(&text);
        Self {
            text,
            color,
            transform,
            svg,
            svg_key,
        }
    }
}

impl IntoElement for WatermarkTile {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for WatermarkTile {
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
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _window: &mut Window,
        _cx: &mut App,
    ) -> Self::PrepaintState {
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        if self.text.is_empty() {
            return;
        }
        let transformation = watermark_transformation_matrix(
            bounds,
            window.scale_factor(),
            self.transform.rotate_degrees,
            self.transform.skew_x_degrees,
            self.transform.skew_y_degrees,
            self.transform.origin,
        );
        let _ = window.paint_svg(
            bounds,
            self.svg_key.clone(),
            Some(self.svg.as_ref()),
            transformation,
            self.color,
            cx,
        );
    }
}

#[derive(Clone, Debug, Eq)]
struct WatermarkSvgKey {
    text: String,
    width: u32,
    height: u32,
}

impl PartialEq for WatermarkSvgKey {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text && self.width == other.width && self.height == other.height
    }
}

impl Hash for WatermarkSvgKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.text.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

fn watermark_svg_cache() -> &'static Mutex<HashMap<WatermarkSvgKey, Arc<[u8]>>> {
    static CACHE: OnceLock<Mutex<HashMap<WatermarkSvgKey, Arc<[u8]>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached_watermark_svg(text: &SharedString) -> (SharedString, Arc<[u8]>) {
    let key = WatermarkSvgKey {
        text: text.to_string(),
        width: WATERMARK_TILE_WIDTH.ceil().as_f32() as u32,
        height: WATERMARK_TILE_HEIGHT.ceil().as_f32() as u32,
    };
    let svg_key = SharedString::from(format!(
        "liora-watermark-{}-{}-{:016x}.svg",
        key.width,
        key.height,
        stable_watermark_hash(&key)
    ));
    let mut cache = watermark_svg_cache()
        .lock()
        .expect("watermark svg cache lock should not be poisoned");
    let svg = cache
        .entry(key)
        .or_insert_with_key(|key| Arc::<[u8]>::from(watermark_svg_document(key).into_bytes()))
        .clone();
    (svg_key, svg)
}

fn stable_watermark_hash(key: &WatermarkSvgKey) -> u64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in key.text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash ^= u64::from(key.width);
    hash = hash.wrapping_mul(0x100000001b3);
    hash ^= u64::from(key.height);
    hash
}

fn watermark_svg_document(key: &WatermarkSvgKey) -> String {
    let escaped = escape_svg_text(&key.text);
    let x = key.width as f32 / 2.0;
    let y = key.height as f32 / 2.0 + WATERMARK_TILE_FONT_SIZE / 2.8;
    format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}"><text x="{x:.2}" y="{y:.2}" text-anchor="middle" font-family="-apple-system, BlinkMacSystemFont, Segoe UI, sans-serif" font-size="{font_size:.2}" font-weight="700" fill="black">{escaped}</text></svg>"#,
        width = key.width,
        height = key.height,
        x = x,
        y = y,
        font_size = WATERMARK_TILE_FONT_SIZE,
        escaped = escaped
    )
}

fn escape_svg_text(text: &str) -> String {
    let mut escaped = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn watermark_transformation_matrix(
    bounds: Bounds<Pixels>,
    scale_factor: f32,
    rotate_degrees: f32,
    skew_x_degrees: f32,
    skew_y_degrees: f32,
    origin: WatermarkTransformOrigin,
) -> TransformationMatrix {
    let origin = origin.clamped();
    let scaled_bounds = bounds.scale(scale_factor);
    let transform_origin = point(
        ScaledPixels(scaled_bounds.origin.x.0 + scaled_bounds.size.width.0 * origin.x),
        ScaledPixels(scaled_bounds.origin.y.0 + scaled_bounds.size.height.0 * origin.y),
    );
    TransformationMatrix::unit()
        .translate(transform_origin)
        .rotate(gpui::radians(rotate_degrees.to_radians()))
        .compose(skew_matrix(skew_x_degrees, skew_y_degrees))
        .translate(Point::new(
            ScaledPixels(-transform_origin.x.0),
            ScaledPixels(-transform_origin.y.0),
        ))
}

fn skew_matrix(skew_x_degrees: f32, skew_y_degrees: f32) -> TransformationMatrix {
    let skew_x = clamp_skew_degrees(skew_x_degrees).to_radians().tan();
    let skew_y = clamp_skew_degrees(skew_y_degrees).to_radians().tan();
    TransformationMatrix {
        rotation_scale: [[1.0, skew_x], [skew_y, 1.0]],
        translation: [0.0, 0.0],
    }
}

fn clamp_skew_degrees(degrees: f32) -> f32 {
    degrees.clamp(-WATERMARK_MAX_SKEW_DEGREES, WATERMARK_MAX_SKEW_DEGREES)
}

impl IntoElement for Watermark {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn watermark_tracks_density_and_opacity() {
        let watermark = Watermark::new("body", "CONFIDENTIAL")
            .density(3, 5)
            .opacity(2.0);
        assert_eq!(watermark.tile_count(), 15);
        assert_eq!(watermark.opacity, 1.0);
    }
    #[test]
    fn watermark_header_uses_columns_only() {
        let watermark = Watermark::new("body", "LIORA").header().density(8, 3);
        assert_eq!(watermark.tile_count(), 3);
    }

    #[test]
    fn watermark_tracks_transform_builders() {
        let watermark = Watermark::new("body", "LIORA")
            .rotate(-32.0)
            .skew(12.0, -4.0)
            .rotation_center(0.25, 0.75);
        assert_eq!(watermark.rotate_degrees, -32.0);
        assert_eq!(watermark.skew_x_degrees, 12.0);
        assert_eq!(watermark.skew_y_degrees, -4.0);
        assert_eq!(
            watermark.transform_origin,
            WatermarkTransformOrigin { x: 0.25, y: 0.75 }
        );
    }

    #[test]
    fn watermark_transform_matrix_contains_rotation_skew_and_custom_origin() {
        let bounds = Bounds::new(point(px(10.0), px(20.0)), gpui::size(px(100.0), px(40.0)));
        let matrix = watermark_transformation_matrix(
            bounds,
            1.0,
            30.0,
            10.0,
            0.0,
            WatermarkTransformOrigin::new(0.0, 0.0),
        );
        assert_ne!(matrix, TransformationMatrix::unit());
        assert!(matrix.rotation_scale[0][1].abs() > 0.1);
        assert!(matrix.translation[0].abs() > 0.0 || matrix.translation[1].abs() > 0.0);
    }

    #[test]
    fn watermark_svg_cache_preserves_unicode_and_escapes_xml() {
        let key = WatermarkSvgKey {
            text: "保密 & <draft>".to_string(),
            width: 160,
            height: 42,
        };
        let svg = watermark_svg_document(&key);
        assert!(svg.contains("保密"));
        assert!(svg.contains("&amp;"));
        assert!(svg.contains("&lt;draft&gt;"));
    }
}
