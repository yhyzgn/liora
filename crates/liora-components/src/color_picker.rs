//! Color Picker module.
//!
//! This public module implements the Liora color picker component with hue, saturation/value, and alpha controls. It keeps the reusable
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

use crate::gpui_compat::element_id;
use gpui::{
    App, Bounds, Context, Corners, Element, ElementId, GlobalElementId, Hsla, InspectorElementId,
    IntoElement, LayoutId, MouseButton, Pixels, Point, Render, RenderImage, Rgba, SharedString,
    Style, Window, actions, div, fill, point, prelude::*, px, size,
};
use image::{ImageBuffer, Rgba as ImageRgba};
use liora_core::{Config, push_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(color_picker, [ColorPickerClose]);

pub struct ColorPicker {
    id: SharedString,
    value: SharedString,
    alpha: f32,
    hue: f32,
    presets: Vec<SharedString>,
    disabled: bool,
    show_label: bool,
    width: Option<Pixels>,
    is_open: bool,
    last_bounds: Option<Bounds<Pixels>>,
    sv_bounds: Option<Bounds<Pixels>>,
    hue_bounds: Option<Bounds<Pixels>>,
    alpha_bounds: Option<Bounds<Pixels>>,
    sv_image: Option<(u16, Arc<RenderImage>)>,
    hue_image: Option<Arc<RenderImage>>,
    alpha_image: Option<(SharedString, Arc<RenderImage>)>,
    on_change: Option<Arc<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl ColorPicker {
    pub fn new(value: impl Into<SharedString>) -> Self {
        let value = value.into();
        Self {
            id: liora_core::unique_id("color-picker"),
            value: Self::normalize_hex(value.as_ref()).unwrap_or_else(|| "#409EFF".into()),
            alpha: 1.0,
            hue: 210.0,
            presets: default_presets(),
            disabled: false,
            show_label: true,
            width: None,
            is_open: false,
            last_bounds: None,
            sv_bounds: None,
            hue_bounds: None,
            alpha_bounds: None,
            sv_image: None,
            hue_image: None,
            alpha_image: None,
            on_change: None,
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, value: impl AsRef<str>) -> Self {
        if let Some(value) = Self::normalize_hex(value.as_ref()) {
            self.value = value;
        }
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = alpha.clamp(0.0, 1.0);
        self
    }

    pub fn hue(mut self, hue: f32) -> Self {
        self.hue = normalize_hue(hue);
        self
    }

    pub fn presets(mut self, presets: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.presets = presets
            .into_iter()
            .filter_map(|value| {
                let value = value.into();
                Self::normalize_hex(value.as_ref())
            })
            .collect();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn show_label(mut self, show_label: bool) -> Self {
        self.show_label = show_label;
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn width_md(self) -> Self {
        self.width(px(360.0))
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new("escape", ColorPickerClose, None)]);
    }

    fn close_on_escape_action(
        &mut self,
        _: &ColorPickerClose,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    pub fn on_change(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Arc::new(f));
        self
    }

    pub fn normalize_hex(input: &str) -> Option<SharedString> {
        let trimmed = input.trim();
        let raw = trimmed.strip_prefix('#').unwrap_or(trimmed);
        let expanded = match raw.len() {
            3 => raw.chars().flat_map(|ch| [ch, ch]).collect::<String>(),
            6 => raw.to_string(),
            _ => return None,
        };
        if !expanded.chars().all(|ch| ch.is_ascii_hexdigit()) {
            return None;
        }
        Some(format!("#{}", expanded.to_ascii_uppercase()).into())
    }

    pub fn rainbow_palette() -> Vec<SharedString> {
        [
            "#FF0000", "#FF3B00", "#FF7A00", "#FFB800", "#FFFF00", "#B8FF00", "#7AFF00", "#3BFF00",
            "#00FF00", "#00FF7A", "#00FFFF", "#00B8FF", "#007AFF", "#003BFF", "#0000FF", "#3B00FF",
            "#7A00FF", "#B800FF", "#FF00FF", "#FF00B8", "#FF007A", "#FF003B", "#FFFFFF", "#000000",
            "#F2F3F5", "#C0C4CC", "#909399", "#606266", "#303133", "#1F2D3D",
        ]
        .into_iter()
        .map(Into::into)
        .collect()
    }

    pub fn rgba_display(input: &str, alpha: f32) -> Option<SharedString> {
        let (r, g, b) = Self::hex_rgb(input)?;
        Some(format!("rgba({}, {}, {}, {:.2})", r, g, b, alpha.clamp(0.0, 1.0)).into())
    }

    pub fn hex_from_hsv(hue: f32, saturation: f32, value: f32) -> SharedString {
        let (r, g, b) = hsv_to_rgb(hue, saturation, value);
        format!("#{:02X}{:02X}{:02X}", r, g, b).into()
    }

    pub fn hex_rgb(input: &str) -> Option<(u8, u8, u8)> {
        let normalized = Self::normalize_hex(input)?;
        let raw = normalized.as_ref().trim_start_matches('#');
        Some((
            u8::from_str_radix(&raw[0..2], 16).ok()?,
            u8::from_str_radix(&raw[2..4], 16).ok()?,
            u8::from_str_radix(&raw[4..6], 16).ok()?,
        ))
    }

    fn select_sv_at(
        &mut self,
        position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.sv_bounds {
            let saturation = ((position.x - bounds.left()) / bounds.size.width).clamp(0.0, 1.0);
            let value = (1.0 - ((position.y - bounds.top()) / bounds.size.height)).clamp(0.0, 1.0);
            let color = Self::hex_from_hsv(self.hue, saturation, value);
            self.select_color(color, window, cx);
        }
    }

    fn select_hue_at(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(bounds) = self.hue_bounds {
            let ratio = ((position.y - bounds.top()) / bounds.size.height).clamp(0.0, 1.0);
            self.select_hue(ratio * 360.0, cx);
        }
    }

    fn select_alpha_at(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        if let Some(bounds) = self.alpha_bounds {
            let ratio = ((position.x - bounds.left()) / bounds.size.width).clamp(0.0, 1.0);
            self.select_alpha(ratio, cx);
        }
    }

    fn sv_render_image(&mut self) -> Arc<RenderImage> {
        let hue_key = normalize_hue(self.hue).round() as u16;
        if let Some((cached_hue, image)) = &self.sv_image {
            if *cached_hue == hue_key {
                return image.clone();
            }
        }

        let image = render_image_from_pixels(SV_WIDTH, SV_HEIGHT, |x, y| {
            let saturation = if SV_WIDTH <= 1 {
                0.0
            } else {
                x as f32 / (SV_WIDTH - 1) as f32
            };
            let value = if SV_HEIGHT <= 1 {
                1.0
            } else {
                1.0 - y as f32 / (SV_HEIGHT - 1) as f32
            };
            bgra_pixel(hsla_from_hsv(hue_key as f32, saturation, value, 1.0))
        });
        self.sv_image = Some((hue_key, image.clone()));
        image
    }

    fn hue_render_image(&mut self) -> Arc<RenderImage> {
        if let Some(image) = &self.hue_image {
            return image.clone();
        }

        let image = render_image_from_pixels(HUE_WIDTH, HUE_HEIGHT, |_, y| {
            let hue = if HUE_HEIGHT <= 1 {
                0.0
            } else {
                y as f32 / (HUE_HEIGHT - 1) as f32 * 360.0
            };
            bgra_pixel(hsla_from_hsv(hue, 1.0, 1.0, 1.0))
        });
        self.hue_image = Some(image.clone());
        image
    }

    fn alpha_render_image(&mut self) -> Arc<RenderImage> {
        if let Some((cached_value, image)) = &self.alpha_image {
            if cached_value == &self.value {
                return image.clone();
            }
        }

        let base = hex_to_hsla(&self.value).unwrap_or_else(|| hsla_from_hsv(210.0, 0.75, 1.0, 1.0));
        let image = render_image_from_pixels(ALPHA_WIDTH, ALPHA_HEIGHT, |x, y| {
            let alpha = if ALPHA_WIDTH <= 1 {
                1.0
            } else {
                x as f32 / (ALPHA_WIDTH - 1) as f32
            };
            alpha_checker_bgra_pixel(base, x, y, alpha)
        });
        self.alpha_image = Some((self.value.clone(), image.clone()));
        image
    }

    fn select_color(&mut self, color: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled || self.value == color {
            return;
        }
        self.value = color.clone();
        if let Some(on_change) = &self.on_change {
            on_change(color, window, cx);
        }
        cx.notify();
    }

    fn select_color_and_close(
        &mut self,
        color: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.select_color(color, window, cx);
        self.is_open = false;
        cx.notify();
    }

    fn select_alpha(&mut self, alpha: f32, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.alpha = alpha.clamp(0.0, 1.0);
        cx.notify();
    }

    fn select_hue(&mut self, hue: f32, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.hue = normalize_hue(hue);
        cx.notify();
    }
}

impl Render for ColorPicker {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let selected = self.value.clone();
        let swatch_color = hex_to_hsla(&selected)
            .unwrap_or(theme.primary.base)
            .opacity(self.alpha);
        let id = self.id.clone();
        let disabled = self.disabled;
        let entity = cx.entity().clone();

        if self.is_open && !disabled {
            let bounds = self.last_bounds;
            let panel_id = id.clone();
            let theme_portal = theme.clone();
            let selected_for_panel = selected.clone();
            let presets = self.presets.clone();
            let self_alpha = self.alpha;
            let self_hue = self.hue;
            let entity_for_portal = entity.clone();
            let close_on_click_outside = self.close_on_click_outside;

            push_portal(
                move |_window, _cx| {
                    let (top, left) = if let Some(bounds) = bounds {
                        (bounds.bottom() + px(6.0), bounds.left())
                    } else {
                        (px(100.0), px(100.0))
                    };
                    let close_entity = entity_for_portal.clone();
                    let panel = render_color_panel(
                        panel_id.clone(),
                        selected_for_panel.clone(),
                        self_alpha,
                        self_hue,
                        presets.clone(),
                        theme_portal.clone(),
                        entity_for_portal.clone(),
                    )
                    .absolute()
                    .top(top)
                    .left(left);

                    div()
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full()
                        .bg(gpui::transparent_black())
                        .when(close_on_click_outside, |s| {
                            s.on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                close_entity.update(cx, |picker, cx| {
                                    picker.is_open = false;
                                    cx.notify();
                                });
                            })
                        })
                        .child(panel)
                        .into_any_element()
                },
                cx,
            );
        }

        div()
            .flex()
            .items_center()
            .gap_2()
            .when_some(self.width, |s, width| s.w(width))
            .child(
                div()
                    .id(element_id(format!("{}-trigger", id)))
                    .relative()
                    .w(px(40.0))
                    .h(px(40.0))
                    .rounded(px(theme.radius.md))
                    .border_1()
                    .border_color(if self.is_open {
                        theme.primary.base
                    } else {
                        theme.neutral.border
                    })
                    .bg(if disabled {
                        theme.neutral.hover
                    } else {
                        swatch_color
                    })
                    .when(!disabled, |s| {
                        s.cursor_pointer()
                            .hover(|s| s.cursor_pointer().border_color(theme.primary.base))
                    })
                    .when(disabled, |s| s.cursor_not_allowed().opacity(0.55))
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .child(BoundsCapturer {
                                picker: entity.clone(),
                            }),
                    )
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                div()
                                    .rounded(px(3.0))
                                    .bg(theme.neutral.card.opacity(0.88))
                                    .p(px(2.0))
                                    .child(
                                        Icon::new(IconName::ChevronDown)
                                            .size(px(12.0))
                                            .color(theme.neutral.icon),
                                    ),
                            ),
                    )
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _, _, cx| {
                            if !this.disabled {
                                this.is_open = !this.is_open;
                                cx.notify();
                            }
                        }),
                    )
                    .on_action(cx.listener(Self::close_on_escape_action)),
            )
            .when(self.show_label, |s| {
                s.child(
                    div()
                        .text_sm()
                        .font_family("monospace")
                        .text_color(if disabled {
                            theme.neutral.text_3
                        } else {
                            theme.neutral.text_1
                        })
                        .child(
                            ColorPicker::rgba_display(&selected, self.alpha).unwrap_or(selected),
                        ),
                )
            })
    }
}

fn render_color_panel(
    id: SharedString,
    selected: SharedString,
    alpha: f32,
    hue: f32,
    presets: Vec<SharedString>,
    theme: liora_theme::Theme,
    picker: gpui::Entity<ColorPicker>,
) -> gpui::Stateful<gpui::Div> {
    div()
        .id(element_id(format!("{}-panel", id)))
        .occlude()
        .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
        .w(px(326.0))
        .p_3()
        .flex()
        .flex_col()
        .gap_3()
        .rounded(px(theme.radius.lg))
        .border_1()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .shadow_lg()
        .child(
            div()
                .flex()
                .gap_2()
                .child(sv_panel(
                    format!("{}-sv", id),
                    hue,
                    theme.clone(),
                    picker.clone(),
                ))
                .child(hue_bar(
                    format!("{}-hue", id),
                    hue,
                    theme.clone(),
                    picker.clone(),
                )),
        )
        .child(alpha_bar(
            format!("{}-alpha", id),
            selected.clone(),
            alpha,
            theme.clone(),
            picker.clone(),
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_2()
                .children(presets.into_iter().enumerate().map({
                    let picker = picker.clone();
                    let theme = theme.clone();
                    move |(index, color)| {
                        let hsla = hex_to_hsla(&color).unwrap_or(theme.primary.base);
                        div()
                            .id(element_id(format!("{}-preset-{}", id, index)))
                            .w(px(20.0))
                            .h(px(20.0))
                            .rounded(px(4.0))
                            .border_1()
                            .border_color(theme.neutral.border)
                            .bg(hsla)
                            .cursor_pointer()
                            .hover(|s| s.cursor_pointer().border_color(theme.primary.base))
                            .on_mouse_down(MouseButton::Left, {
                                let picker = picker.clone();
                                move |_, window, cx| {
                                    picker.update(cx, |picker, cx| {
                                        picker.select_color_and_close(color.clone(), window, cx);
                                    });
                                    cx.stop_propagation();
                                }
                            })
                    }
                })),
        )
        .child(
            div()
                .px_2()
                .py_1()
                .rounded(px(theme.radius.sm))
                .border_1()
                .border_color(theme.neutral.border)
                .text_xs()
                .font_family("monospace")
                .text_color(theme.neutral.text_1)
                .child(ColorPicker::rgba_display(&selected, alpha).unwrap_or(selected)),
        )
}

fn sv_panel(
    id: String,
    _hue: f32,
    _theme: liora_theme::Theme,
    picker: gpui::Entity<ColorPicker>,
) -> impl IntoElement {
    let picker_for_click = picker.clone();
    let picker_for_drag = picker.clone();
    div()
        .id(element_id(id))
        .w(px(280.0))
        .h(px(180.0))
        .cursor_pointer()
        .overflow_hidden()
        .on_mouse_down(MouseButton::Left, move |event, window, cx| {
            picker_for_click.update(cx, |picker, cx| {
                picker.select_sv_at(event.position, window, cx);
            });
            cx.stop_propagation();
        })
        .on_mouse_move(move |event, window, cx| {
            if event.pressed_button == Some(MouseButton::Left) {
                picker_for_drag.update(cx, |picker, cx| {
                    picker.select_sv_at(event.position, window, cx);
                });
                cx.stop_propagation();
            }
        })
        .child(ColorSurfaceElement {
            picker,
            kind: ColorSurfaceKind::SaturationValue,
        })
}

fn hue_bar(
    id: String,
    _selected_hue: f32,
    theme: liora_theme::Theme,
    picker: gpui::Entity<ColorPicker>,
) -> impl IntoElement {
    let picker_for_click = picker.clone();
    let picker_for_drag = picker.clone();
    div()
        .id(element_id(id))
        .w(px(14.0))
        .h(px(180.0))
        .rounded(px(4.0))
        .overflow_hidden()
        .border_1()
        .border_color(theme.neutral.border)
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, move |event, _, cx| {
            picker_for_click.update(cx, |picker, cx| picker.select_hue_at(event.position, cx));
            cx.stop_propagation();
        })
        .on_mouse_move(move |event, _, cx| {
            if event.pressed_button == Some(MouseButton::Left) {
                picker_for_drag.update(cx, |picker, cx| picker.select_hue_at(event.position, cx));
                cx.stop_propagation();
            }
        })
        .child(ColorSurfaceElement {
            picker,
            kind: ColorSurfaceKind::Hue,
        })
}

fn alpha_bar(
    id: String,
    _selected: SharedString,
    _alpha: f32,
    theme: liora_theme::Theme,
    picker: gpui::Entity<ColorPicker>,
) -> impl IntoElement {
    let picker_for_click = picker.clone();
    let picker_for_drag = picker.clone();
    div()
        .id(element_id(id))
        .w_full()
        .h(px(14.0))
        .rounded(px(3.0))
        .overflow_hidden()
        .border_1()
        .border_color(theme.neutral.border)
        .cursor_pointer()
        .on_mouse_down(MouseButton::Left, move |event, _, cx| {
            picker_for_click.update(cx, |picker, cx| picker.select_alpha_at(event.position, cx));
            cx.stop_propagation();
        })
        .on_mouse_move(move |event, _, cx| {
            if event.pressed_button == Some(MouseButton::Left) {
                picker_for_drag.update(cx, |picker, cx| picker.select_alpha_at(event.position, cx));
                cx.stop_propagation();
            }
        })
        .child(ColorSurfaceElement {
            picker,
            kind: ColorSurfaceKind::Alpha,
        })
}

#[derive(Clone, Copy)]
enum ColorSurfaceKind {
    SaturationValue,
    Hue,
    Alpha,
}

struct ColorSurfaceElement {
    picker: gpui::Entity<ColorPicker>,
    kind: ColorSurfaceKind,
}

impl IntoElement for ColorSurfaceElement {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for ColorSurfaceElement {
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
        match self.kind {
            ColorSurfaceKind::SaturationValue => {
                style.size.width = px(280.0).into();
                style.size.height = px(180.0).into();
            }
            ColorSurfaceKind::Hue => {
                style.size.width = px(14.0).into();
                style.size.height = px(180.0).into();
            }
            ColorSurfaceKind::Alpha => {
                style.size.width = gpui::relative(1.0).into();
                style.size.height = px(14.0).into();
            }
        }
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.picker.update(cx, |picker, _| match self.kind {
            ColorSurfaceKind::SaturationValue => picker.sv_bounds = Some(bounds),
            ColorSurfaceKind::Hue => picker.hue_bounds = Some(bounds),
            ColorSurfaceKind::Alpha => picker.alpha_bounds = Some(bounds),
        });
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
        let (image, marker) = self.picker.update(cx, |picker, _| match self.kind {
            ColorSurfaceKind::SaturationValue => (picker.sv_render_image(), None),
            ColorSurfaceKind::Hue => (
                picker.hue_render_image(),
                Some((normalize_hue(picker.hue) / 360.0, false)),
            ),
            ColorSurfaceKind::Alpha => (
                picker.alpha_render_image(),
                Some((picker.alpha.clamp(0.0, 1.0), true)),
            ),
        });

        let _ = window.paint_image(bounds, Corners::all(px(0.0)), image, 0, false);
        if let Some((ratio, horizontal)) = marker {
            paint_surface_marker(bounds, ratio, horizontal, window);
        }
    }
}

const SV_WIDTH: u32 = 280;
const SV_HEIGHT: u32 = 180;
const HUE_WIDTH: u32 = 14;
const HUE_HEIGHT: u32 = 180;
const ALPHA_WIDTH: u32 = 280;
const ALPHA_HEIGHT: u32 = 14;

fn render_image_from_pixels(
    width: u32,
    height: u32,
    mut pixel: impl FnMut(u32, u32) -> ImageRgba<u8>,
) -> Arc<RenderImage> {
    let buffer = ImageBuffer::from_fn(width, height, |x, y| pixel(x, y));
    Arc::new(RenderImage::new([image::Frame::new(buffer)]))
}

fn bgra_pixel(color: Hsla) -> ImageRgba<u8> {
    let rgba = Rgba::from(color);
    bgra_from_rgba_channels(rgba.r, rgba.g, rgba.b, rgba.a)
}

fn bgra_from_rgba_channels(r: f32, g: f32, b: f32, a: f32) -> ImageRgba<u8> {
    ImageRgba([
        (b.clamp(0.0, 1.0) * 255.0).round() as u8,
        (g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (a.clamp(0.0, 1.0) * 255.0).round() as u8,
    ])
}

fn alpha_checker_bgra_pixel(base: Hsla, x: u32, y: u32, alpha: f32) -> ImageRgba<u8> {
    const CHECKER_SIZE: u32 = 7;
    let checker_is_light = ((x / CHECKER_SIZE) + (y / CHECKER_SIZE)).is_multiple_of(2);
    let checker = if checker_is_light { 1.0 } else { 0.0 };
    let rgba = Rgba::from(base);
    let alpha = alpha.clamp(0.0, 1.0);
    bgra_from_rgba_channels(
        rgba.r * alpha + checker * (1.0 - alpha),
        rgba.g * alpha + checker * (1.0 - alpha),
        rgba.b * alpha + checker * (1.0 - alpha),
        1.0,
    )
}

fn paint_surface_marker(bounds: Bounds<Pixels>, ratio: f32, horizontal: bool, window: &mut Window) {
    let ratio = ratio.clamp(0.0, 1.0);
    let marker_bounds = if horizontal {
        Bounds::new(
            point(bounds.left() + bounds.size.width * ratio, bounds.top()),
            size(px(1.0), bounds.size.height),
        )
    } else {
        Bounds::new(
            point(bounds.left(), bounds.top() + bounds.size.height * ratio),
            size(bounds.size.width, px(1.0)),
        )
    };
    window.paint_quad(fill(marker_bounds, gpui::white()));
}

fn hsla_from_hsv(hue: f32, saturation: f32, value: f32, alpha: f32) -> Hsla {
    let (r, g, b) = hsv_to_rgb(hue, saturation, value);
    Hsla::from(Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a: alpha,
    })
}

struct BoundsCapturer {
    picker: gpui::Entity<ColorPicker>,
}

impl IntoElement for BoundsCapturer {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for BoundsCapturer {
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
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, ()) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.picker.update(cx, |picker, _| {
            picker.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
}

fn normalize_hue(hue: f32) -> f32 {
    let mut hue = hue % 360.0;
    if hue < 0.0 {
        hue += 360.0;
    }
    hue
}

fn hsv_to_rgb(hue: f32, saturation: f32, value: f32) -> (u8, u8, u8) {
    let hue = normalize_hue(hue);
    let saturation = saturation.clamp(0.0, 1.0);
    let value = value.clamp(0.0, 1.0);
    let chroma = value * saturation;
    let x = chroma * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - chroma;
    let (r1, g1, b1) = if hue < 60.0 {
        (chroma, x, 0.0)
    } else if hue < 120.0 {
        (x, chroma, 0.0)
    } else if hue < 180.0 {
        (0.0, chroma, x)
    } else if hue < 240.0 {
        (0.0, x, chroma)
    } else if hue < 300.0 {
        (x, 0.0, chroma)
    } else {
        (chroma, 0.0, x)
    };
    (
        ((r1 + m) * 255.0).round() as u8,
        ((g1 + m) * 255.0).round() as u8,
        ((b1 + m) * 255.0).round() as u8,
    )
}

fn hex_to_hsla(value: &str) -> Option<Hsla> {
    let (r, g, b) = ColorPicker::hex_rgb(value)?;
    Some(gpui::rgb((u32::from(r) << 16) | (u32::from(g) << 8) | u32::from(b)).into())
}

fn default_presets() -> Vec<SharedString> {
    [
        "#409EFF", "#67C23A", "#E6A23C", "#F56C6C", "#909399", "#000000", "#FFFFFF", "#626AEF",
        "#13C2C2", "#722ED1", "#EB2F96", "#FA541C",
    ]
    .into_iter()
    .map(Into::into)
    .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_picker_width_md_sets_demo_width() {
        assert_eq!(
            ColorPicker::new("#409eff").width_md().width,
            Some(px(360.0))
        );
    }

    #[test]
    fn alpha_slider_renders_checkerboard_under_transparent_color() {
        let base = gpui::rgb(0xff0000).into();

        let transparent_light = alpha_checker_bgra_pixel(base, 0, 0, 0.0);
        let transparent_dark = alpha_checker_bgra_pixel(base, 7, 0, 0.0);
        let opaque_red = alpha_checker_bgra_pixel(base, 0, 0, 1.0);
        let half_over_dark = alpha_checker_bgra_pixel(base, 7, 0, 0.5);

        assert_eq!(transparent_light.0, [255, 255, 255, 255]);
        assert_eq!(transparent_dark.0, [0, 0, 0, 255]);
        assert_eq!(opaque_red.0, [0, 0, 255, 255]);
        assert_eq!(half_over_dark.0, [0, 0, 128, 255]);
    }
}
