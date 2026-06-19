use crate::gpui_compat::PixelsExt;
use crate::image::{
    ImageRoundOptions, ImageSource, RasterImageElement, load_local_render_image,
    load_remote_render_image,
};
use crate::motion::{FadeDirection, MotionDuration, fade, pop_in};
use gpui::{
    AnyElement, App, BoxShadow, Component, Global, IntoElement, KeyBinding, ObjectFit, Pixels,
    RenderImage, RenderOnce, SharedString, Size, Window, actions, div, prelude::*, px, size,
};
use liora_core::{Config, push_portal};
use std::{path::PathBuf, sync::Arc, time::Duration};

actions!(preview, [PreviewClose]);

pub struct Preview {
    src: Option<ImageSource>,
    trigger: Option<AnyElement>,
    hover_effect: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

pub struct ActiveImagePreview {
    image: Option<Arc<RenderImage>>,
    closing: bool,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl Global for ActiveImagePreview {}

impl Preview {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: Some(ImageSource::from_input(src)),
            trigger: None,
            hover_effect: true,
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    pub fn empty() -> Self {
        Self {
            src: None,
            trigger: None,
            hover_effect: true,
            close_on_click_outside: true,
            close_on_escape: true,
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

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    pub fn source(&self) -> Option<&ImageSource> {
        self.src.as_ref()
    }

    pub fn has_trigger(&self) -> bool {
        self.trigger.is_some()
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", PreviewClose, None)]);
        cx.on_action(|_: &PreviewClose, cx| {
            close_active_preview_if_escape_enabled(cx);
        });
    }
}

pub fn render_image_preview(window: &mut Window, cx: &mut App) {
    let Some((image, closing, close_on_click_outside, close_on_escape)) =
        cx.try_global::<ActiveImagePreview>().and_then(|preview| {
            preview.image.clone().map(|image| {
                (
                    image,
                    preview.closing,
                    preview.close_on_click_outside,
                    preview.close_on_escape,
                )
            })
        })
    else {
        return;
    };

    let theme = cx.global::<Config>().theme.clone();
    push_portal(
        move |window, _cx| {
            let viewport = window.viewport_size();
            let preview_size =
                preview_image_box_size(&image, viewport.width * 0.72, viewport.height * 0.72);
            let overlay_motion = if closing {
                FadeDirection::Out
            } else {
                FadeDirection::In
            };
            fade(
                if closing {
                    "liora-preview-overlay-exit"
                } else {
                    "liora-preview-overlay-enter"
                },
                overlay_motion,
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(gpui::black().opacity(0.55))
                    .when(close_on_escape, |s| {
                        s.on_action(|_: &PreviewClose, _, cx| {
                            close_active_preview(cx);
                        })
                    })
                    .when(close_on_click_outside, |s| {
                        s.on_mouse_down(gpui::MouseButton::Left, |_, _, cx| {
                            close_active_preview(cx);
                            cx.stop_propagation();
                        })
                    })
                    .child(pop_in(
                        if closing {
                            "liora-preview-frame-exit"
                        } else {
                            "liora-preview-frame-enter"
                        },
                        div()
                            .w(preview_size.width)
                            .h(preview_size.height)
                            .rounded(px(theme.radius.lg))
                            .border_1()
                            .border_color(gpui::white().opacity(0.28))
                            .overflow_hidden()
                            .shadow(preview_image_frame_shadow())
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
                    )),
            )
            .into_any_element()
        },
        cx,
    );

    let _ = window;
}

fn close_active_preview_if_escape_enabled(cx: &mut App) {
    let close_on_escape = cx
        .try_global::<ActiveImagePreview>()
        .is_some_and(|preview| preview.close_on_escape);
    if close_on_escape {
        close_active_preview(cx);
    }
}

fn close_active_preview(cx: &mut App) {
    if cx.has_global::<ActiveImagePreview>() {
        let preview = cx.global_mut::<ActiveImagePreview>();
        if preview.image.is_none() || preview.closing {
            return;
        }
        preview.closing = true;
        cx.refresh_windows();

        let async_cx = cx.to_async();
        let executor = cx.background_executor().clone();
        cx.foreground_executor()
            .spawn(async move {
                executor.timer(preview_close_duration()).await;
                let _ = async_cx.update(|cx| {
                    if cx.has_global::<ActiveImagePreview>() {
                        let preview = cx.global_mut::<ActiveImagePreview>();
                        if preview.closing {
                            preview.image = None;
                            preview.closing = false;
                            cx.refresh_windows();
                        }
                    }
                });
            })
            .detach();
    }
}

fn preview_close_duration() -> Duration {
    MotionDuration::Fast.as_duration()
}

fn preview_image_box_size(
    image: &RenderImage,
    max_width: Pixels,
    max_height: Pixels,
) -> Size<Pixels> {
    let image_size = image.size(0);
    let image_width = i32::from(image_size.width).max(1) as f32;
    let image_height = i32::from(image_size.height).max(1) as f32;
    let scale = (max_width.as_f32() / image_width).min(max_height.as_f32() / image_height);

    size(px(image_width * scale), px(image_height * scale))
}

fn preview_image_frame_shadow() -> Vec<BoxShadow> {
    vec![
        BoxShadow {
            color: gpui::black().opacity(0.48),
            offset: gpui::point(px(0.0), px(28.0)),
            blur_radius: px(64.0),
            spread_radius: px(4.0),
        },
        BoxShadow {
            color: gpui::black().opacity(0.34),
            offset: gpui::point(px(0.0), px(10.0)),
            blur_radius: px(24.0),
            spread_radius: px(-2.0),
        },
        BoxShadow {
            color: gpui::white().opacity(0.22),
            offset: gpui::point(px(0.0), px(-2.0)),
            blur_radius: px(8.0),
            spread_radius: px(-4.0),
        },
    ]
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
                        cx.set_global(ActiveImagePreview {
                            image: None,
                            closing: false,
                            close_on_click_outside: self.close_on_click_outside,
                            close_on_escape: self.close_on_escape,
                        });
                    }
                    let preview = cx.global_mut::<ActiveImagePreview>();
                    preview.image = Some(image);
                    preview.closing = false;
                    preview.close_on_click_outside = self.close_on_click_outside;
                    preview.close_on_escape = self.close_on_escape;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn test_render_image(width: u32, height: u32) -> RenderImage {
        RenderImage::new([::image::Frame::new(::image::RgbaImage::new(width, height))])
    }

    #[test]
    fn preview_image_box_size_matches_contained_image_bounds() {
        let wide = test_render_image(400, 200);
        let wide_size = preview_image_box_size(&wide, px(300.0), px(300.0));
        assert_eq!(wide_size.width, px(300.0));
        assert_eq!(wide_size.height, px(150.0));

        let tall = test_render_image(200, 400);
        let tall_size = preview_image_box_size(&tall, px(300.0), px(300.0));
        assert_eq!(tall_size.width, px(150.0));
        assert_eq!(tall_size.height, px(300.0));
    }

    #[test]
    fn preview_overlay_has_escape_close_action_and_image_sized_hitbox() {
        let source = include_str!("preview.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();

        assert!(production.contains("actions!(preview, [PreviewClose])"));
        assert!(production.contains("KeyBinding::new(\"escape\", PreviewClose, None)"));
        assert!(production.contains("cx.on_action(|_: &PreviewClose"));
        assert!(production.contains(".on_action(|_: &PreviewClose"));
        assert!(production.contains("fn close_active_preview"));
        assert!(production.contains("close_on_click_outside"));
        assert!(production.contains("pub fn close_on_click_outside("));
        assert!(production.contains("preview_close_duration"));
        assert!(production.contains("closing: bool"));
        assert!(production.contains("fn preview_image_box_size"));
        assert!(production.contains(".w(preview_size.width)"));
        assert!(production.contains(".h(preview_size.height)"));
        assert!(production.contains(".shadow(preview_image_frame_shadow())"));
        assert!(production.contains("FadeDirection::Out"));
        assert!(production.contains("pop_in("));
        assert!(
            !production.contains(
                ".w(viewport.width * 0.72)\n                        .h(viewport.height * 0.72)"
            ),
            "preview should not consume clicks in the whole max viewport box; only the fitted image box should stop backdrop close"
        );
    }

    #[test]
    fn preview_frame_shadow_keeps_3d_border_depth() {
        let shadow = preview_image_frame_shadow();

        assert_eq!(shadow.len(), 3);
        assert_eq!(shadow[0].offset.y, px(28.0));
        assert_eq!(shadow[0].blur_radius, px(64.0));
        assert_eq!(shadow[1].offset.y, px(10.0));
        assert_eq!(shadow[2].offset.y, px(-2.0));
    }
}
