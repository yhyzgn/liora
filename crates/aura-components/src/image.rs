use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Bounds, Component, Corners, Element, ElementId, Global, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, ObjectFit, Pixels, RenderImage, RenderOnce,
    SharedString, Style, Window, div, img, prelude::*, px, relative,
};
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

    pub fn square(mut self, size: impl Into<Pixels>) -> Self {
        let size = size.into();
        self.width = Some(size);
        self.height = Some(size);
        self
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
        self
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

    pub fn dimensions(&self) -> (Option<Pixels>, Option<Pixels>) {
        (self.width, self.height)
    }

    pub fn source(&self) -> Option<&ImageSource> {
        self.src.as_ref()
    }
}

pub struct ActiveImagePreview {
    image: Option<Arc<RenderImage>>,
}

impl Global for ActiveImagePreview {}

pub fn render_image_preview(window: &mut Window, cx: &mut App) {
    let Some(image) = cx
        .try_global::<ActiveImagePreview>()
        .and_then(|preview| preview.image.clone())
    else {
        return;
    };

    let theme = cx.global::<Config>().theme.clone();
    push_portal(
        move |window, _cx| {
            let viewport = window.viewport_size();
            div()
                .absolute()
                .top_0()
                .left_0()
                .size_full()
                .flex()
                .items_center()
                .justify_center()
                .bg(gpui::black().opacity(0.55))
                .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                    if cx.has_global::<ActiveImagePreview>() {
                        cx.global_mut::<ActiveImagePreview>().image = None;
                    }
                    cx.refresh_windows();
                    cx.stop_propagation();
                })
                .child(
                    div()
                        .w(viewport.width * 0.72)
                        .h(viewport.height * 0.72)
                        .rounded(px(theme.radius.lg))
                        .overflow_hidden()
                        .shadow_xl()
                        .on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                            cx.stop_propagation();
                        })
                        .child(RasterImageElement {
                            image,
                            fit: ObjectFit::Contain,
                            grayscale: false,
                            radius: px(theme.radius.lg),
                            round: false,
                        }),
                )
                .into_any_element()
        },
        cx,
    );

    let _ = window;
}

fn src_for_preview(src: &Option<ImageSource>) -> Option<ImageSource> {
    src.clone()
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
                    },
                ));
            } else if let ImageSource::Url(src) = src {
                frame = frame.child(
                    img(src)
                        .size_full()
                        .object_fit(self.fit.as_object_fit())
                        .grayscale(self.grayscale)
                        .with_loading(move || loading())
                        .with_fallback(move || fallback()),
                );
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

        if self.preview {
            let preview_image = match &src_for_preview(&preview_src) {
                Some(ImageSource::File(path)) => load_local_render_image(path),
                Some(ImageSource::Url(url)) => load_remote_render_image(url.as_ref(), window, cx),
                None => None,
            };
            frame = frame
                .cursor_pointer()
                .hover(|s| s.border_color(theme.primary.base).shadow_lg())
                .on_mouse_down(gpui::MouseButton::Left, move |_, _, cx| {
                    if let Some(image) = preview_image.clone() {
                        if !cx.has_global::<ActiveImagePreview>() {
                            cx.set_global(ActiveImagePreview { image: None });
                        }
                        cx.global_mut::<ActiveImagePreview>().image = Some(image);
                        cx.refresh_windows();
                    }
                    cx.stop_propagation();
                });
        }

        frame
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

struct RasterImageElement {
    image: Arc<RenderImage>,
    fit: ObjectFit,
    grayscale: bool,
    radius: Pixels,
    round: bool,
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
        let (image, image_bounds, corner_radii) = if self.round {
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
            (
                self.image.clone(),
                image_bounds,
                Corners::all(self.radius).clamp_radii_for_quad_size(image_bounds.size),
            )
        };
        let _ = window.paint_image(image_bounds, corner_radii, image, 0, self.grayscale);
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

fn remote_image_cache() -> &'static Mutex<HashMap<String, RemoteImageState>> {
    static CACHE: OnceLock<Mutex<HashMap<String, RemoteImageState>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(HashMap::new()))
}

fn load_remote_render_image(
    url: &str,
    window: &mut Window,
    _cx: &mut App,
) -> Option<Arc<RenderImage>> {
    if let Some(cached) = remote_image_cache().lock().ok()?.get(url).cloned() {
        return match cached {
            RemoteImageState::Ready(image) => Some(image),
            RemoteImageState::Loading => {
                window.request_animation_frame();
                None
            }
            RemoteImageState::Failed => None,
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
            async_cx.update(|cx| cx.refresh_windows());
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

fn load_local_render_image(path: &Path) -> Option<Arc<RenderImage>> {
    let bytes = std::fs::read(path).ok()?;
    render_image_from_bytes(&bytes)
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

fn default_loading(theme: &aura_theme::Theme) -> AnyElement {
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

fn default_fallback(theme: &aura_theme::Theme, alt: SharedString) -> AnyElement {
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

fn default_empty(theme: &aura_theme::Theme) -> AnyElement {
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
