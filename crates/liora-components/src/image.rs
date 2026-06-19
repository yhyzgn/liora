//! Image module.
//!
//! This public module implements the Liora image component with local/remote loading, preview, and thumbnail helpers. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
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
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::preview::Preview;
pub use crate::preview::render_image_preview;
use gpui::{
    AnyElement, App, Bounds, Component, Corners, Element, ElementId, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, LayoutId, ObjectFit, Pixels, RenderImage, RenderOnce,
    SharedString, Style, Window, div, prelude::*, px, relative,
};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::{
    collections::HashMap,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFit {
    Fill,
    #[default]
    Contain,
    Cover,
    ScaleDown,
    None,
}

impl ImageFit {
    pub fn as_object_fit(self) -> ObjectFit {
        match self {
            ImageFit::Fill => ObjectFit::Fill,
            ImageFit::Contain => ObjectFit::Contain,
            ImageFit::Cover => ObjectFit::Cover,
            ImageFit::ScaleDown => ObjectFit::ScaleDown,
            ImageFit::None => ObjectFit::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageRadius {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageRing {
    pub width: Pixels,
    pub color: Hsla,
}

impl ImageRing {
    pub fn new(width: impl Into<Pixels>, color: impl Into<Hsla>) -> Self {
        Self {
            width: width.into(),
            color: color.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ImageRoundOptions {
    pub crop_to_square: bool,
    pub ring: Option<ImageRing>,
}

impl Default for ImageRoundOptions {
    fn default() -> Self {
        Self::circle()
    }
}

impl ImageRoundOptions {
    pub fn circle() -> Self {
        Self {
            crop_to_square: true,
            ring: None,
        }
    }

    pub fn without_square_crop() -> Self {
        Self {
            crop_to_square: false,
            ring: None,
        }
    }

    pub fn ring(mut self, ring: ImageRing) -> Self {
        self.ring = Some(ring);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImageSource {
    Url(SharedString),
    File(PathBuf),
}

impl ImageSource {
    pub fn from_input(input: impl Into<SharedString>) -> Self {
        let input = input.into();
        if let Some(path) = parse_file_protocol(input.as_ref()) {
            ImageSource::File(path)
        } else {
            ImageSource::Url(input)
        }
    }

    pub fn is_file(&self) -> bool {
        matches!(self, ImageSource::File(_))
    }

    pub fn is_url(&self) -> bool {
        matches!(self, ImageSource::Url(_))
    }
}

pub struct Image {
    src: Option<ImageSource>,
    alt: Option<SharedString>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    fit: ImageFit,
    radius: ImageRadius,
    bordered: bool,
    shadow: bool,
    grayscale: bool,
    preview: bool,
    round_options: ImageRoundOptions,
    placeholder: Option<Arc<dyn Fn() -> AnyElement + 'static>>,
    fallback: Option<Arc<dyn Fn() -> AnyElement + 'static>>,
}

impl Image {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: Some(ImageSource::from_input(src)),
            alt: None,
            width: None,
            height: None,
            fit: ImageFit::Contain,
            radius: ImageRadius::Medium,
            bordered: true,
            shadow: false,
            grayscale: false,
            preview: false,
            round_options: ImageRoundOptions::default(),
            placeholder: None,
            fallback: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            src: None,
            alt: None,
            width: None,
            height: None,
            fit: ImageFit::Contain,
            radius: ImageRadius::Medium,
            bordered: true,
            shadow: false,
            grayscale: false,
            preview: false,
            round_options: ImageRoundOptions::default(),
            placeholder: None,
            fallback: None,
        }
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(ImageSource::from_input(src));
        self
    }

    pub fn file(mut self, path: impl Into<PathBuf>) -> Self {
        self.src = Some(ImageSource::File(path.into()));
        self
    }

    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::empty().file(path)
    }

    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn size(mut self, width: impl Into<Pixels>, height: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self.height = Some(height.into());
        self
    }

    pub fn thumbnail(self) -> Self {
        self.size(px(180.0), px(120.0))
    }

    pub fn thumbnail_sm(self) -> Self {
        self.size(px(132.0), px(88.0))
    }

    pub fn square(mut self, size: impl Into<Pixels>) -> Self {
        let size = size.into();
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    pub fn square_lg(self) -> Self {
        self.square(px(96.0))
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn fill(mut self) -> Self {
        self.fit = ImageFit::Fill;
        self
    }

    pub fn contain(mut self) -> Self {
        self.fit = ImageFit::Contain;
        self
    }

    pub fn cover(mut self) -> Self {
        self.fit = ImageFit::Cover;
        self
    }

    pub fn scale_down(mut self) -> Self {
        self.fit = ImageFit::ScaleDown;
        self
    }

    pub fn radius(mut self, radius: ImageRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn no_radius(mut self) -> Self {
        self.radius = ImageRadius::None;
        self
    }

    pub fn round(mut self) -> Self {
        self.radius = ImageRadius::Round;
        self.round_options = ImageRoundOptions::default();
        self
    }

    pub fn round_options(mut self, options: ImageRoundOptions) -> Self {
        self.radius = ImageRadius::Round;
        self.round_options = options;
        self
    }

    pub fn round_ring(mut self, ring: ImageRing) -> Self {
        self.radius = ImageRadius::Round;
        self.round_options = self.round_options.ring(ring);
        self
    }

    pub fn round_sleeve(self) -> Self {
        self.round_ring(ImageRing::new(px(6.0), gpui::white().opacity(0.72)))
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn no_border(mut self) -> Self {
        self.bordered = false;
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = preview;
        self
    }

    pub fn placeholder<E>(mut self, placeholder: impl Fn() -> E + 'static) -> Self
    where
        E: IntoElement,
    {
        self.placeholder = Some(Arc::new(move || placeholder().into_any_element()));
        self
    }

    pub fn fallback<E>(mut self, fallback: impl Fn() -> E + 'static) -> Self
    where
        E: IntoElement,
    {
        self.fallback = Some(Arc::new(move || fallback().into_any_element()));
        self
    }

    pub fn fit_kind(&self) -> ImageFit {
        self.fit
    }

    pub fn radius_kind(&self) -> ImageRadius {
        self.radius
    }

    pub fn round_config(&self) -> ImageRoundOptions {
        self.round_options
    }

    pub fn dimensions(&self) -> (Option<Pixels>, Option<Pixels>) {
        (self.width, self.height)
    }

    pub fn source(&self) -> Option<&ImageSource> {
        self.src.as_ref()
    }
    pub fn preview_enabled(&self) -> bool {
        self.preview
    }
}

impl RenderOnce for Image {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let radius = match self.radius {
            ImageRadius::None => px(0.0),
            ImageRadius::Small => px(theme.radius.sm),
            ImageRadius::Medium => px(theme.radius.md),
            ImageRadius::Large => px(theme.radius.lg),
            ImageRadius::Round => px(999.0),
        };
        let alt = self.alt.clone().unwrap_or_else(|| "Image".into());

        let mut frame = div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .rounded(radius)
            .bg(theme.neutral.hover);

        if let Some(width) = self.width {
            frame = frame.w(width);
        }
        if let Some(height) = self.height {
            frame = frame.h(height);
        }
        if self.width.is_none() && self.height.is_none() {
            frame = frame.w(px(160.0)).h(px(100.0));
        }
        if self.bordered {
            frame = frame.border_1().border_color(theme.neutral.border);
        }
        if self.shadow {
            frame = frame.shadow_md();
        }

        let preview_src = self.src.clone();

        if let Some(src) = self.src {
            let raster_image = match &src {
                ImageSource::File(path) => load_local_render_image(path),
                ImageSource::Url(url) => load_remote_render_image(url.as_ref(), window, cx),
            };
            let loading = self.placeholder.unwrap_or_else({
                let theme = theme.clone();
                || Arc::new(move || default_loading(&theme))
            });
            let fallback = self.fallback.unwrap_or_else({
                let theme = theme.clone();
                || Arc::new(move || default_fallback(&theme, alt.clone()))
            });
            if let Some(raster_image) = raster_image {
                frame = frame.child(div().absolute().top_0().left_0().size_full().child(
                    RasterImageElement {
                        image: raster_image,
                        fit: self.fit.as_object_fit(),
                        grayscale: self.grayscale,
                        radius,
                        round: self.radius == ImageRadius::Round,
                        round_options: self.round_options,
                    },
                ));
            } else if matches!(src, ImageSource::Url(_)) {
                frame = frame.child(loading());
            } else {
                frame = frame.child(fallback());
            }
        } else {
            frame = frame.child(
                self.fallback
                    .map(|fallback| fallback())
                    .unwrap_or_else(|| default_empty(&theme)),
            );
        }

        if let Some(ring) = self.round_options.ring
            && self.radius == ImageRadius::Round
        {
            frame = frame.child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(RingSleeveElement { ring }),
            );
        }

        if self.preview {
            frame = frame
                .cursor_pointer()
                .hover(|s| s.border_color(theme.primary.base).shadow_lg());
            return Preview::empty()
                .src_from_image_source(preview_src)
                .hover_effect(false)
                .child(frame)
                .into_any_element();
        }

        frame.into_any_element()
    }
}

impl IntoElement for Image {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn parse_file_protocol(input: &str) -> Option<PathBuf> {
    let path = input.strip_prefix("file://")?;
    if path.is_empty() {
        return None;
    }
    Some(expand_tilde_path(path))
}

fn expand_tilde_path(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix("~/")
        && let Some(home) = std::env::var_os("HOME")
    {
        return PathBuf::from(home).join(rest);
    }
    PathBuf::from(path)
}

pub(crate) struct RasterImageElement {
    pub(crate) image: Arc<RenderImage>,
    pub(crate) fit: ObjectFit,
    pub(crate) grayscale: bool,
    pub(crate) radius: Pixels,
    pub(crate) round: bool,
    pub(crate) round_options: ImageRoundOptions,
}

impl IntoElement for RasterImageElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for RasterImageElement {
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
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        _cx: &mut App,
    ) {
        if self.image.frame_count() == 0 {
            return;
        }
        let (image, image_bounds, corner_radii) = if self.round && self.round_options.crop_to_square
        {
            let side = bounds.size.width.min(bounds.size.height);
            let square_bounds = Bounds {
                origin: gpui::point(
                    bounds.origin.x + (bounds.size.width - side) / 2.0,
                    bounds.origin.y + (bounds.size.height - side) / 2.0,
                ),
                size: gpui::size(side, side),
            };
            (
                square_cropped_render_image(&self.image),
                square_bounds,
                Corners::all(side / 2.0),
            )
        } else {
            let image_bounds = self.fit.get_bounds(bounds, self.image.size(0));
            let corner_radii = if self.round {
                Corners::all(bounds.size.width.min(bounds.size.height) / 2.0)
                    .clamp_radii_for_quad_size(bounds.size)
            } else {
                Corners::all(self.radius).clamp_radii_for_quad_size(image_bounds.size)
            };
            (self.image.clone(), image_bounds, corner_radii)
        };
        let _ = window.paint_image(image_bounds, corner_radii, image, 0, self.grayscale);
    }
}

struct RingSleeveElement {
    ring: ImageRing,
}

impl IntoElement for RingSleeveElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for RingSleeveElement {
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
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        _cx: &mut App,
    ) {
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        window: &mut Window,
        _cx: &mut App,
    ) {
        let side = bounds.size.width.min(bounds.size.height);
        let sleeve_bounds = Bounds {
            origin: gpui::point(
                bounds.origin.x + (bounds.size.width - side) / 2.0,
                bounds.origin.y + (bounds.size.height - side) / 2.0,
            ),
            size: gpui::size(side, side),
        };
        window.paint_quad(gpui::PaintQuad {
            bounds: sleeve_bounds,
            corner_radii: Corners::all(side / 2.0),
            background: gpui::transparent_black().into(),
            border_widths: gpui::Edges::all(self.ring.width),
            border_color: self.ring.color,
            border_style: gpui::BorderStyle::Solid,
        });
    }
}

fn square_cropped_image_cache() -> &'static Mutex<HashMap<usize, Arc<RenderImage>>> {
    static CACHE: OnceLock<Mutex<HashMap<usize, Arc<RenderImage>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn square_cropped_render_image(image: &Arc<RenderImage>) -> Arc<RenderImage> {
    if let Some(cached) = square_cropped_image_cache()
        .lock()
        .ok()
        .and_then(|cache| cache.get(&image.id.0).cloned())
    {
        return cached;
    }

    let Some(bytes) = image.as_bytes(0) else {
        return image.clone();
    };
    let image_size = image.size(0);
    let width = u32::from(image_size.width);
    let height = u32::from(image_size.height);
    let side = width.min(height);
    if side == 0 {
        return image.clone();
    }

    let Some(source) = image::RgbaImage::from_raw(width, height, bytes.to_vec()) else {
        return image.clone();
    };
    let x = (width - side) / 2;
    let y = (height - side) / 2;
    let cropped = image::imageops::crop_imm(&source, x, y, side, side).to_image();
    let cropped = Arc::new(RenderImage::new([image::Frame::new(cropped)]));

    if let Ok(mut cache) = square_cropped_image_cache().lock() {
        cache.insert(image.id.0, cropped.clone());
    }

    cropped
}

#[derive(Clone)]
enum RemoteImageState {
    Loading,
    Ready(Arc<RenderImage>),
    Failed,
}

impl RemoteImageState {
    fn should_request_animation_frame(&self) -> bool {
        false
    }
}

fn remote_image_cache() -> &'static Mutex<HashMap<String, RemoteImageState>> {
    static CACHE: OnceLock<Mutex<HashMap<String, RemoteImageState>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn load_remote_render_image(
    url: &str,
    window: &mut Window,
    _cx: &mut App,
) -> Option<Arc<RenderImage>> {
    if let Some(cached) = remote_image_cache().lock().ok()?.get(url).cloned() {
        if cached.should_request_animation_frame() {
            window.request_animation_frame();
        }
        return match cached {
            RemoteImageState::Ready(image) => Some(image),
            RemoteImageState::Loading | RemoteImageState::Failed => None,
        };
    }

    if let Ok(mut cache) = remote_image_cache().lock() {
        cache.insert(url.to_string(), RemoteImageState::Loading);
    }

    let url = url.to_string();
    let async_cx = _cx.to_async();
    let executor = _cx.background_executor().clone();
    _cx.foreground_executor()
        .spawn(async move {
            let fetch_url = url.clone();
            let image = executor
                .spawn(async move { fetch_remote_render_image(&fetch_url) })
                .await;
            let state = image
                .map(RemoteImageState::Ready)
                .unwrap_or(RemoteImageState::Failed);
            if let Ok(mut cache) = remote_image_cache().lock() {
                cache.insert(url, state);
            }
            let _ = async_cx.update(|cx| cx.refresh_windows());
        })
        .detach();
    window.request_animation_frame();

    None
}

fn fetch_remote_render_image(url: &str) -> Option<Arc<RenderImage>> {
    ureq::get(url).call().ok().and_then(|response| {
        let mut bytes = Vec::new();
        response.into_reader().read_to_end(&mut bytes).ok()?;
        render_image_from_bytes(&bytes)
    })
}

fn local_image_cache() -> &'static Mutex<HashMap<PathBuf, Arc<RenderImage>>> {
    static CACHE: OnceLock<Mutex<HashMap<PathBuf, Arc<RenderImage>>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn load_local_render_image(path: &Path) -> Option<Arc<RenderImage>> {
    if let Some(cached) = local_image_cache()
        .lock()
        .ok()
        .and_then(|cache| cache.get(path).cloned())
    {
        return Some(cached);
    }

    let bytes = std::fs::read(path).ok()?;
    let image = render_image_from_bytes(&bytes)?;
    if let Ok(mut cache) = local_image_cache().lock() {
        cache.insert(path.to_path_buf(), image.clone());
    }
    Some(image)
}

fn render_image_from_bytes(bytes: &[u8]) -> Option<Arc<RenderImage>> {
    let format = image::guess_format(bytes).ok()?;
    let mut data = image::load_from_memory_with_format(bytes, format)
        .ok()?
        .into_rgba8();
    for pixel in data.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    Some(Arc::new(RenderImage::new([image::Frame::new(data)])))
}

fn default_loading(theme: &liora_theme::Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .size_full()
        .text_color(theme.neutral.text_3)
        .child(
            Icon::new(IconName::LoaderCircle)
                .size(px(20.0))
                .color(theme.neutral.icon),
        )
        .child(div().text_xs().child("Loading"))
        .into_any_element()
}

fn default_fallback(theme: &liora_theme::Theme, alt: SharedString) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .size_full()
        .text_color(theme.neutral.text_3)
        .child(
            Icon::new(IconName::ImageOff)
                .size(px(24.0))
                .color(theme.neutral.icon),
        )
        .child(div().text_xs().child(alt))
        .into_any_element()
}

fn default_empty(theme: &liora_theme::Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .size_full()
        .text_color(theme.neutral.text_3)
        .child(
            Icon::new(IconName::Image)
                .size(px(24.0))
                .color(theme.neutral.icon),
        )
        .child(div().text_xs().child("No image"))
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_thumbnail_sets_preview_dimensions() {
        let image = Image::new("https://example.com/image.png").thumbnail();

        assert_eq!(image.width, Some(px(180.0)));
        assert_eq!(image.height, Some(px(120.0)));
    }

    #[test]
    fn image_demo_size_helpers_track_common_examples() {
        let thumbnail_sm = Image::new("https://example.com/image.png").thumbnail_sm();
        assert_eq!(thumbnail_sm.width, Some(px(132.0)));
        assert_eq!(thumbnail_sm.height, Some(px(88.0)));

        let square_lg = Image::new("https://example.com/image.png").square_lg();
        assert_eq!(square_lg.width, Some(px(96.0)));
        assert_eq!(square_lg.height, Some(px(96.0)));
    }

    #[test]
    fn image_round_sleeve_sets_ring_configuration() {
        let image = Image::new("https://example.com/image.png").round_sleeve();

        assert_eq!(image.radius, ImageRadius::Round);
        assert_eq!(
            image.round_options.ring.map(|ring| ring.width),
            Some(px(6.0))
        );
    }

    #[test]
    fn remote_image_loading_state_is_passive_after_first_fetch() {
        assert!(
            !RemoteImageState::Loading.should_request_animation_frame(),
            "loading remote images should not request animation frames on every render; completion refreshes windows explicitly"
        );
        assert!(!RemoteImageState::Failed.should_request_animation_frame());
    }

    #[test]
    fn remote_url_rendering_uses_single_liora_fetch_path() {
        let source = include_str!("image.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();

        assert!(
            !production.contains("img(src)"),
            "remote Image rendering should not start GPUI img loading after scheduling the Liora remote cache fetch"
        );
    }

    #[test]
    fn local_image_loading_uses_render_image_cache() {
        let source = include_str!("image.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();

        assert!(production.contains("fn local_image_cache()"));
        assert!(production.contains("cache.get(path).cloned()"));
        assert!(production.contains("cache.insert(path.to_path_buf(), image.clone())"));
    }
}
