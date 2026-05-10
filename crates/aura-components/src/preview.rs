use crate::image::{
    ImageRoundOptions, ImageSource, RasterImageElement, load_local_render_image,
    load_remote_render_image,
};
use aura_core::{Config, push_portal};
use gpui::{
    AnyElement, App, Component, Global, IntoElement, ObjectFit, RenderImage, RenderOnce,
    SharedString, Window, div, prelude::*, px,
};
use std::{path::PathBuf, sync::Arc};

pub struct Preview {
    src: Option<ImageSource>,
    trigger: Option<AnyElement>,
    hover_effect: bool,
}

pub struct ActiveImagePreview {
    image: Option<Arc<RenderImage>>,
}

impl Global for ActiveImagePreview {}

impl Preview {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: Some(ImageSource::from_input(src)),
            trigger: None,
            hover_effect: true,
        }
    }

    pub fn empty() -> Self {
        Self {
            src: None,
            trigger: None,
            hover_effect: true,
        }
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(ImageSource::from_input(src));
        self
    }

    pub(crate) fn src_from_image_source(mut self, src: Option<ImageSource>) -> Self {
        self.src = src;
        self
    }

    pub fn file(mut self, path: impl Into<PathBuf>) -> Self {
        self.src = Some(ImageSource::File(path.into()));
        self
    }

    pub fn local(path: impl Into<PathBuf>) -> Self {
        Self::empty().file(path)
    }

    pub fn child(mut self, trigger: impl IntoElement) -> Self {
        self.trigger = Some(trigger.into_any_element());
        self
    }

    pub fn hover_effect(mut self, enabled: bool) -> Self {
        self.hover_effect = enabled;
        self
    }

    pub fn source(&self) -> Option<&ImageSource> {
        self.src.as_ref()
    }

    pub fn has_trigger(&self) -> bool {
        self.trigger.is_some()
    }
}

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
                            round_options: ImageRoundOptions::without_square_crop(),
                        }),
                )
                .into_any_element()
        },
        cx,
    );

    let _ = window;
}

fn load_preview_image(
    src: &Option<ImageSource>,
    window: &mut Window,
    cx: &mut App,
) -> Option<Arc<RenderImage>> {
    match src {
        Some(ImageSource::File(path)) => load_local_render_image(path),
        Some(ImageSource::Url(url)) => load_remote_render_image(url.as_ref(), window, cx),
        None => None,
    }
}

impl RenderOnce for Preview {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let preview_image = load_preview_image(&self.src, window, cx);
        let mut trigger = div()
            .relative()
            .cursor_pointer()
            .child(self.trigger.unwrap_or_else(|| div().into_any_element()))
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

        if self.hover_effect {
            trigger = trigger.hover(|s| s.border_color(theme.primary.base).shadow_lg());
        }

        trigger
    }
}

impl IntoElement for Preview {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
