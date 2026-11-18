//! Image module.
//!
//! This public module implements the Liora image component with local/remote loading, preview, and thumbnail helpers. It keeps the reusable
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

use crate::preview::Preview;
pub use crate::preview::render_image_preview;
use gpui::{
    AnyElement, App, Bounds, Corners, Element, ElementId, GlobalElementId, Hsla,
    InspectorElementId, IntoElement, LayoutId, ObjectFit, Pixels, RenderImage, RenderOnce,
    SharedString, Style, ViewElement as Component, Window, div, prelude::*, px, relative,
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
/// Options that control image fit behavior.
pub enum ImageFit {
    /// Scales the image to fill the frame, cropping when necessary.
    Fill,
    #[default]
    /// Scales the image to fit fully inside the frame.
    Contain,
    /// Places the watermark in the cover region.
    Cover,
    /// Only scales the image down when it exceeds the frame.
    ScaleDown,
    /// Disables the optional visual affordance.
    None,
}

impl ImageFit {
    /// Borrows this value as object fit.
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
/// Options that control image radius behavior.
pub enum ImageRadius {
    /// Disables the optional visual affordance.
    None,
    /// Uses compact sizing metrics.
    Small,
    #[default]
    /// Uses medium/default sizing metrics.
    Medium,
    /// Uses expanded sizing metrics.
    Large,
    /// Uses the `Round` option for `ImageRadius`.
    Round,
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Fluent native GPUI component for rendering Liora image ring.
pub struct ImageRing {
    /// Width used by layout or hit-testing calculations.
    pub width: Pixels,
    /// Color token or explicit color applied to the visual element.
    pub color: Hsla,
}

impl ImageRing {
    /// Creates `ImageRing` initialized from the supplied width, and color.
    pub fn new(width: impl Into<Pixels>, color: impl Into<Hsla>) -> Self {
        Self {
            width: width.into(),
            color: color.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Configuration options for image round options rendering and interaction behavior.
pub struct ImageRoundOptions {
    /// Whether the image is clipped to a square frame.
    pub crop_to_square: bool,
    /// Optional decorative ring drawn around the image.
    pub ring: Option<ImageRing>,
}

impl Default for ImageRoundOptions {
    fn default() -> Self {
        Self::circle()
    }
}

impl ImageRoundOptions {
    /// Uses circular shape or clipping.
    pub fn circle() -> Self {
        Self {
            crop_to_square: true,
            ring: None,
        }
    }

    /// Sets the without square crop value used by the component.
    pub fn without_square_crop() -> Self {
        Self {
            crop_to_square: false,
            ring: None,
        }
    }

    /// Sets the ring value used by the component.
    pub fn ring(mut self, ring: ImageRing) -> Self {
        self.ring = Some(ring);
        self
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options that control image source behavior.
pub enum ImageSource {
    /// Loads image content from a URL-style asset path.
    Url(SharedString),
    /// Loads image content from a local filesystem path.
    File(PathBuf),
}

impl ImageSource {
    /// Creates this value from input.
    pub fn from_input(input: impl Into<SharedString>) -> Self {
        let input = input.into();
        if let Some(path) = parse_file_protocol(input.as_ref()) {
            ImageSource::File(path)
        } else {
            ImageSource::Url(input)
        }
    }

    /// Returns whether file is currently true for this value.
    pub fn is_file(&self) -> bool {
        matches!(self, ImageSource::File(_))
    }

    /// Returns whether url is currently true for this value.
    pub fn is_url(&self) -> bool {
        matches!(self, ImageSource::Url(_))
    }
}

/// Fluent native GPUI component for rendering Liora image.
pub struct Image {
    src: Option<ImageSource>,
    embedded: Option<&'static [u8]>,
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
    /// Creates `Image` initialized from the supplied src.
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: Some(ImageSource::from_input(src)),
            embedded: None,
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

    /// Creates an image from bytes embedded in the application binary.
    pub fn embedded(bytes: &'static [u8]) -> Self {
        let mut image = Self::empty();
        image.embedded = Some(bytes);
        image
    }

    /// Creates or applies an empty-state visual treatment.
    pub fn empty() -> Self {
        Self {
            embedded: None,
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

    /// Sets the image or preview source.
    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(ImageSource::from_input(src));
        self.embedded = None;
        self
    }

    /// Sets the local filesystem path.
    pub fn file(mut self, path: impl Into<PathBuf>) -> Self {
        self.src = Some(ImageSource::File(path.into()));
        self.embedded = None;
        self
    }

    /// Creates a component source from a local file path.
    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::empty().file(path)
    }

    /// Sets alternate text used when an image source is unavailable.
    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Sets the component height token used during GPUI layout.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, width: impl Into<Pixels>, height: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self.height = Some(height.into());
        self
    }

    /// Sets the thumbnail value used by the component.
    pub fn thumbnail(self) -> Self {
        self.size(px(180.0), px(120.0))
    }

    /// Applies the predefined thumbnail sm sizing preset.
    pub fn thumbnail_sm(self) -> Self {
        self.size(px(132.0), px(88.0))
    }

    /// Uses square shape or clipping.
    pub fn square(mut self, size: impl Into<Pixels>) -> Self {
        let size = size.into();
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    /// Applies the predefined square lg sizing preset.
    pub fn square_lg(self) -> Self {
        self.square(px(96.0))
    }

    /// Sets the fit value used by the component.
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    /// Sets the fill value used by the component.
    pub fn fill(mut self) -> Self {
        self.fit = ImageFit::Fill;
        self
    }

    /// Sets the contain value used by the component.
    pub fn contain(mut self) -> Self {
        self.fit = ImageFit::Contain;
        self
    }

    /// Sets the cover value used by the component.
    pub fn cover(mut self) -> Self {
        self.fit = ImageFit::Cover;
        self
    }

    /// Sets the scale down value used by the component.
    pub fn scale_down(mut self) -> Self {
        self.fit = ImageFit::ScaleDown;
        self
    }

    /// Sets the corner radius used by the rendered frame.
    pub fn radius(mut self, radius: ImageRadius) -> Self {
        self.radius = radius;
        self
    }

    /// Disables radius rendering or behavior.
    pub fn no_radius(mut self) -> Self {
        self.radius = ImageRadius::None;
        self
    }

    /// Toggles round behavior.
    pub fn round(mut self) -> Self {
        self.radius = ImageRadius::Round;
        self.round_options = ImageRoundOptions::default();
        self
    }

    /// Sets the round options value used by the component.
    pub fn round_options(mut self, options: ImageRoundOptions) -> Self {
        self.radius = ImageRadius::Round;
        self.round_options = options;
        self
    }

    /// Sets the round ring value used by the component.
    pub fn round_ring(mut self, ring: ImageRing) -> Self {
        self.radius = ImageRadius::Round;
        self.round_options = self.round_options.ring(ring);
        self
    }

    /// Sets the round sleeve value used by the component.
    pub fn round_sleeve(self) -> Self {
        self.round_ring(ImageRing::new(px(6.0), gpui::white().opacity(0.72)))
    }

    /// Sets the bordered value used by the component.
    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    /// Disables border rendering or behavior.
    pub fn no_border(mut self) -> Self {
        self.bordered = false;
        self
    }

    /// Sets the shadow value used by the component.
    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    /// Sets the grayscale value used by the component.
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }

    /// Sets the preview value used by the component.
    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = preview;
        self
    }

    /// Renders the supplied element while a remote image is loading.
    pub fn placeholder<E>(mut self, placeholder: impl Fn() -> E + 'static) -> Self
    where
        E: IntoElement,
    {
        self.placeholder = Some(Arc::new(move || placeholder().into_any_element()));
        self
    }

    /// Renders the supplied element when loading fails or no source is set.
    pub fn fallback<E>(mut self, fallback: impl Fn() -> E + 'static) -> Self
    where
        E: IntoElement,
    {
        self.fallback = Some(Arc::new(move || fallback().into_any_element()));
        self
    }

    /// Performs the fit kind operation used by this component.
    pub fn fit_kind(&self) -> ImageFit {
        self.fit
    }

    /// Performs the radius kind operation used by this component.
    pub fn radius_kind(&self) -> ImageRadius {
        self.radius
    }

    /// Performs the round config operation used by this component.
    pub fn round_config(&self) -> ImageRoundOptions {
        self.round_options
    }

    /// Performs the dimensions operation used by this component.
    pub fn dimensions(&self) -> (Option<Pixels>, Option<Pixels>) {
        (self.width, self.height)
    }

    /// Returns the configured image source, if one is available.
    pub fn source(&self) -> Option<&ImageSource> {
        self.src.as_ref()
    }
    /// Returns whether preview enabled is currently enabled or available.
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

        let raster_image = if let Some(bytes) = self.embedded {
            load_embedded_render_image(bytes)
                .map(RemoteImageState::Ready)
                .unwrap_or(RemoteImageState::Failed)
        } else {
            match &self.src {
                Some(ImageSource::File(path)) => load_local_render_image(path)
                    .map(RemoteImageState::Ready)
                    .unwrap_or(RemoteImageState::Failed),
                Some(ImageSource::Url(url)) => load_remote_image_state(url.as_ref(), window, cx),
                None => RemoteImageState::Failed,
            }
        };
        let preview_image = match &raster_image {
            RemoteImageState::Ready(image) => Some(image.clone()),
            RemoteImageState::Loading | RemoteImageState::Failed => None,
        };
        let has_source = self.src.is_some() || self.embedded.is_some();
        if has_source {
            let loading = self.placeholder.unwrap_or_else({
                let theme = theme.clone();
                || Arc::new(move || default_loading(&theme))
            });
            let fallback = self.fallback.unwrap_or_else({
                let theme = theme.clone();
                || Arc::new(move || default_fallback(&theme, alt.clone()))
            });
            match raster_image {
                RemoteImageState::Ready(image) => {
                    frame = frame.child(div().absolute().top_0().left_0().size_full().child(
                        RasterImageElement {
                            image,
                            fit: self.fit.as_object_fit(),
                            grayscale: self.grayscale,
                            radius,
                            round: self.radius == ImageRadius::Round,
                            round_options: self.round_options,
                        },
                    ));
                }
                RemoteImageState::Loading => frame = frame.child(loading()),
                RemoteImageState::Failed => frame = frame.child(fallback()),
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
            return Preview::from_render_image(preview_image)
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
        let _ = window.paint_image(
            image_bounds,
            image_bounds,
            corner_radii,
            image,
            0,
            self.grayscale,
        );
    }
}

struct RingSleeveElement {
    ring: ImageRing,
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
    cx: &mut App,
) -> Option<Arc<RenderImage>> {
    match load_remote_image_state(url, window, cx) {
        RemoteImageState::Ready(image) => Some(image),
        RemoteImageState::Loading | RemoteImageState::Failed => None,
    }
}

fn load_remote_image_state(url: &str, window: &mut Window, cx: &mut App) -> RemoteImageState {
    let Ok(cache) = remote_image_cache().lock() else {
        return RemoteImageState::Failed;
    };
    let cached = cache.get(url).cloned();
    drop(cache);
    if let Some(cached) = cached {
        if cached.should_request_animation_frame() {
            window.request_animation_frame();
        }
        return cached;
    }

    if let Ok(mut cache) = remote_image_cache().lock() {
        cache.insert(url.to_string(), RemoteImageState::Loading);
    }

    let url = url.to_string();
    let async_cx = cx.to_async();
    let executor = cx.background_executor().clone();
    cx.foreground_executor()
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

    RemoteImageState::Loading
}

fn fetch_remote_render_image(url: &str) -> Option<Arc<RenderImage>> {
    ureq::get(url).call().ok().and_then(|response| {
        let mut bytes = Vec::new();
        response.into_reader().read_to_end(&mut bytes).ok()?;
        render_image_from_bytes(&bytes)
    })
}

pub(crate) fn load_embedded_render_image(bytes: &'static [u8]) -> Option<Arc<RenderImage>> {
    type EmbeddedCache = HashMap<(usize, usize), Option<Arc<RenderImage>>>;
    static CACHE: std::sync::LazyLock<Mutex<EmbeddedCache>> =
        std::sync::LazyLock::new(|| Mutex::new(EmbeddedCache::new()));
    let key = (bytes.as_ptr() as usize, bytes.len());
    CACHE
        .lock()
        .ok()?
        .entry(key)
        .or_insert_with(|| render_image_from_bytes(bytes))
        .clone()
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
    fn remote_image_renders_placeholder_only_while_loading() {
        use gpui::{Context, Render, TestApp};
        use std::{cell::Cell, rc::Rc};

        struct ImageProbe {
            url: SharedString,
            loading: Rc<Cell<usize>>,
            failed: Rc<Cell<usize>>,
        }

        impl Render for ImageProbe {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let loading = self.loading.clone();
                let failed = self.failed.clone();
                Image::new(self.url.clone())
                    .placeholder(move || {
                        let loading = loading.clone();
                        div()
                            .child("Pending image")
                            .on_children_prepainted(move |_, _, _| {
                                loading.set(loading.get() + 1);
                            })
                    })
                    .fallback(move || {
                        let failed = failed.clone();
                        div()
                            .child("Unavailable image")
                            .on_children_prepainted(move |_, _, _| {
                                failed.set(failed.get() + 1);
                            })
                    })
            }
        }

        let mut app = TestApp::new();
        app.update(crate::init_liora);
        for (url, state, expected) in [
            (
                "liora://test-image-loading",
                RemoteImageState::Loading,
                (true, false),
            ),
            (
                "liora://test-image-ready",
                RemoteImageState::Ready(Arc::new(RenderImage::new([image::Frame::new(
                    image::RgbaImage::new(1, 1),
                )]))),
                (false, false),
            ),
        ] {
            remote_image_cache()
                .lock()
                .unwrap()
                .insert(url.into(), state);
            let loading = Rc::new(Cell::new(0));
            let failed = Rc::new(Cell::new(0));
            let mut window = app.open_window(|_, _| ImageProbe {
                url: url.into(),
                loading: loading.clone(),
                failed: failed.clone(),
            });
            window.draw();
            window.update(|_, window, _| window.remove_window());
            remote_image_cache().lock().unwrap().remove(url);
            assert_eq!((loading.get() > 0, failed.get() > 0), expected, "{url}");
        }

        let endpoint = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!("http://{}/missing.png", endpoint.local_addr().unwrap());
        drop(endpoint);
        let loading = Rc::new(Cell::new(0));
        let failed = Rc::new(Cell::new(0));
        let mut window = app.open_window(|_, _| ImageProbe {
            url: url.clone().into(),
            loading: loading.clone(),
            failed: failed.clone(),
        });
        window.draw();
        assert!(loading.get() > 0);
        app.run_until_parked();
        loading.set(0);
        window.draw();
        window.update(|_, window, _| window.remove_window());
        remote_image_cache().lock().unwrap().remove(&url);
        assert_eq!(loading.get(), 0);
        assert!(failed.get() > 0);
    }

    #[test]
    fn local_image_loading_reuses_decoded_raster() {
        const BYTES: &[u8] =
            b"qoif\0\0\0\x01\0\0\0\x01\x04\0\xff\x01\x02\x03\xff\0\0\0\0\0\0\0\x01";
        let path =
            std::env::temp_dir().join(format!("liora-image-cache-{}.qoi", std::process::id()));
        std::fs::write(&path, BYTES).unwrap();

        let first = load_local_render_image(&path).expect("valid local QOI image");
        let second = load_local_render_image(&path).expect("cached local QOI image");
        assert!(Arc::ptr_eq(&first, &second));
        std::fs::remove_file(path).unwrap();
    }
    #[test]
    fn embedded_image_reuses_raster_without_extending_public_source_enum() {
        const BYTES: &[u8] =
            b"qoif\0\0\0\x01\0\0\0\x01\x04\0\xff\x01\x02\x03\xff\0\0\0\0\0\0\0\x01";

        fn exhaustive_source_match(source: &ImageSource) {
            match source {
                ImageSource::Url(_) => {}
                ImageSource::File(_) => {}
            }
        }

        let image = Image::embedded(BYTES).fit(ImageFit::Fill).preview(true);
        assert!(image.source().is_none());
        assert_eq!(image.fit_kind(), ImageFit::Fill);
        exhaustive_source_match(&ImageSource::from_input("https://example.com/image.png"));

        let decoded = load_embedded_render_image(BYTES).expect("valid embedded QOI image");
        assert_eq!(decoded.as_bytes(0).unwrap(), &[3, 2, 1, 255]);
        assert!(Arc::ptr_eq(
            &decoded,
            &load_embedded_render_image(BYTES).expect("cached embedded image")
        ));
        assert!(load_embedded_render_image(b"invalid image").is_none());
    }
}
