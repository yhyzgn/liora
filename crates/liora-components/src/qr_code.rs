//! Qr Code module.
//!
//! This public module implements the Liora QR code generation and rendering component. It keeps the reusable
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

use crate::gpui_compat::PixelsExt;
use gpui::{
    AnyElement, App, Hsla, IntoElement, Pixels, RenderImage, RenderOnce, Rgba, SharedString,
    Window, div, img, prelude::*, px,
};
use image::{DynamicImage, ImageBuffer, Rgba as ImageRgba, RgbaImage};
use liora_core::Config;
use qrcode::{EcLevel, QrCode as QrEncoder, types::Color as QrModuleColor};
use std::path::Path;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Options that control qr ec level behavior.
pub enum QrEcLevel {
    /// Uses low QR error correction for maximum capacity.
    Low,
    /// Uses medium/default sizing metrics.
    Medium,
    /// Uses quartile QR error correction.
    Quartile,
    /// Uses high QR error correction for maximum resilience.
    High,
}

impl QrEcLevel {
    fn into_qrcode(self) -> EcLevel {
        match self {
            QrEcLevel::Low => EcLevel::L,
            QrEcLevel::Medium => EcLevel::M,
            QrEcLevel::Quartile => EcLevel::Q,
            QrEcLevel::High => EcLevel::H,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
/// Fluent native GPUI component for rendering Liora qr decoded.
pub struct QrDecoded {
    /// Content rendered inside the component body.
    pub content: SharedString,
    /// QR error-correction level encoded in the generated symbol.
    pub ecc_level: u8,
    /// Version string associated with this package, release, or update.
    pub version: i32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Options that control qr code error behavior.
pub enum QrCodeError {
    /// Reports that QR encoding failed.
    Encode(String),
    /// Reports a image failure.
    Image(String),
    /// Reports that no QR payload could be found.
    NotFound,
    /// Reports that QR decoding failed.
    Decode(String),
}

/// Type alias for qr code result values used by the qr code API.
pub type QrCodeResult<T> = Result<T, QrCodeError>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control qr module style behavior.
pub enum QrModuleStyle {
    #[default]
    /// Uses square geometry.
    Square,
    /// Uses rounded-corner geometry.
    Rounded,
    /// Uses dot-style QR modules.
    Dots,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control qr finder style behavior.
pub enum QrFinderStyle {
    #[default]
    /// Uses square geometry.
    Square,
    /// Uses rounded-corner geometry.
    Rounded,
    /// Uses circular geometry.
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control qr gradient direction behavior.
pub enum QrGradientDirection {
    /// Draws the QR gradient from bottom to top.
    ToTop,
    /// Draws the QR gradient toward the top-right corner.
    ToTopRight,
    /// Draws the QR gradient from left to right.
    ToRight,
    /// Draws the QR gradient toward the bottom-right corner.
    ToBottomRight,
    #[default]
    /// Draws the QR gradient from top to bottom.
    ToBottom,
    /// Draws the QR gradient toward the bottom-left corner.
    ToBottomLeft,
    /// Draws the QR gradient from right to left.
    ToLeft,
    /// Draws the QR gradient toward the top-left corner.
    ToTopLeft,
}

/// Fluent native GPUI component for rendering Liora qr code.
pub struct QrCode {
    value: SharedString,
    size: Pixels,
    quiet_zone: u32,
    module_radius: Pixels,
    foreground: Option<Hsla>,
    gradient_colors: Option<Vec<Hsla>>,
    gradient_direction: QrGradientDirection,
    background: Option<Hsla>,
    ec_level: QrEcLevel,
    show_text: bool,
    module_style: QrModuleStyle,
    finder_style: QrFinderStyle,
    logo: Option<AnyElement>,
    logo_text: Option<SharedString>,
    logo_size_ratio: f32,
    logo_background: Option<Hsla>,
    logo_color: Option<Hsla>,
    corner_logo: Option<AnyElement>,
    corner_logo_text: Option<SharedString>,
}

impl QrCode {
    /// Creates `QrCode` initialized from the supplied value.
    pub fn new(value: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            size: px(180.0),
            quiet_zone: 4,
            module_radius: px(0.0),
            foreground: None,
            gradient_colors: None,
            gradient_direction: QrGradientDirection::ToBottom,
            background: None,
            ec_level: QrEcLevel::Medium,
            show_text: false,
            module_style: QrModuleStyle::Square,
            finder_style: QrFinderStyle::Square,
            logo: None,
            logo_text: None,
            logo_size_ratio: 0.24,
            logo_background: None,
            logo_color: None,
            corner_logo: None,
            corner_logo_text: None,
        }
    }

    /// Sets an explicit icon size while preserving the default color behavior.
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }

    /// Sets the quiet zone value used by the component.
    pub fn quiet_zone(mut self, modules: u32) -> Self {
        self.quiet_zone = modules;
        self
    }

    /// Sets the module radius value used by the component.
    pub fn module_radius(mut self, radius: impl Into<Pixels>) -> Self {
        self.module_radius = radius.into();
        self
    }

    /// Sets the foreground value used by the component.
    pub fn foreground(mut self, color: Hsla) -> Self {
        self.foreground = Some(color);
        self.gradient_colors = None;
        self
    }

    /// Performs the gradient operation used by this component.
    pub fn gradient(
        mut self,
        colors: impl IntoIterator<Item = Hsla>,
        direction: QrGradientDirection,
    ) -> Self {
        let colors = colors.into_iter().collect::<Vec<_>>();
        self.gradient_colors = (colors.len() >= 2).then_some(colors);
        self.gradient_direction = direction;
        self
    }

    /// Performs the foreground gradient operation used by this component.
    pub fn foreground_gradient(
        self,
        colors: impl IntoIterator<Item = Hsla>,
        direction: QrGradientDirection,
    ) -> Self {
        self.gradient(colors, direction)
    }

    /// Sets the gradient colors value used by the component.
    pub fn gradient_colors(mut self, colors: impl IntoIterator<Item = Hsla>) -> Self {
        let colors = colors.into_iter().collect::<Vec<_>>();
        self.gradient_colors = (colors.len() >= 2).then_some(colors);
        self
    }

    /// Sets the gradient direction value used by the component.
    pub fn gradient_direction(mut self, direction: QrGradientDirection) -> Self {
        self.gradient_direction = direction;
        self
    }

    /// Toggles or applies the component background treatment.
    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    /// Sets the colors value used by the component.
    pub fn colors(self, foreground: Hsla, background: Hsla) -> Self {
        self.foreground(foreground).background(background)
    }

    /// Sets the ec level value used by the component.
    pub fn ec_level(mut self, level: QrEcLevel) -> Self {
        self.ec_level = level;
        self
    }

    /// Sets the high recovery value used by the component.
    pub fn high_recovery(self) -> Self {
        self.ec_level(QrEcLevel::High)
    }

    /// Configures whether text is visible in the rendered component.
    pub fn show_text(mut self, show: bool) -> Self {
        self.show_text = show;
        self
    }

    /// Sets the module style value used by the component.
    pub fn module_style(mut self, style: QrModuleStyle) -> Self {
        self.module_style = style;
        self
    }

    /// Sets the square modules value used by the component.
    pub fn square_modules(self) -> Self {
        self.module_style(QrModuleStyle::Square)
    }

    /// Sets the rounded modules value used by the component.
    pub fn rounded_modules(self) -> Self {
        self.module_style(QrModuleStyle::Rounded)
    }

    /// Sets the dot modules value used by the component.
    pub fn dot_modules(self) -> Self {
        self.module_style(QrModuleStyle::Dots)
    }

    /// Sets the finder style value used by the component.
    pub fn finder_style(mut self, style: QrFinderStyle) -> Self {
        self.finder_style = style;
        self
    }

    /// Sets the rounded finders value used by the component.
    pub fn rounded_finders(self) -> Self {
        self.finder_style(QrFinderStyle::Rounded)
    }

    /// Sets the circle finders value used by the component.
    pub fn circle_finders(self) -> Self {
        self.finder_style(QrFinderStyle::Circle)
    }

    /// Sets the logo value used by the component.
    pub fn logo(mut self, logo: impl IntoElement) -> Self {
        self.logo = Some(logo.into_any_element());
        self.ec_level = QrEcLevel::High;
        self
    }

    /// Sets the logo text value used by the component.
    pub fn logo_text(mut self, text: impl Into<SharedString>) -> Self {
        self.logo_text = Some(text.into());
        self.ec_level = QrEcLevel::High;
        self
    }

    /// Sets the logo size ratio value used by the component.
    pub fn logo_size_ratio(mut self, ratio: f32) -> Self {
        self.logo_size_ratio = ratio.clamp(0.12, 0.36);
        self
    }

    /// Sets the logo background value used by the component.
    pub fn logo_background(mut self, color: Hsla) -> Self {
        self.logo_background = Some(color);
        self
    }

    /// Sets the logo color used by the rendered component.
    pub fn logo_color(mut self, color: Hsla) -> Self {
        self.logo_color = Some(color);
        self
    }

    /// Sets the corner logo value used by the component.
    pub fn corner_logo(mut self, logo: impl IntoElement) -> Self {
        self.corner_logo = Some(logo.into_any_element());
        self.ec_level = QrEcLevel::High;
        self
    }

    /// Sets the corner logo text value used by the component.
    pub fn corner_logo_text(mut self, text: impl Into<SharedString>) -> Self {
        self.corner_logo_text = Some(text.into());
        self
    }

    /// Performs the encode matrix operation used by this component.
    pub fn encode_matrix(value: &str, ec_level: QrEcLevel) -> QrCodeResult<QrMatrix> {
        let code = QrEncoder::with_error_correction_level(value.as_bytes(), ec_level.into_qrcode())
            .map_err(|err| QrCodeError::Encode(err.to_string()))?;
        let width = code.width();
        let modules = code
            .to_colors()
            .into_iter()
            .map(|color| color == QrModuleColor::Dark)
            .collect();

        Ok(QrMatrix { width, modules })
    }

    /// Renders the render image layer into native GPUI elements.
    pub fn render_image(
        value: &str,
        size_px: u32,
        quiet_zone: u32,
        foreground: [u8; 4],
        background: [u8; 4],
        ec_level: QrEcLevel,
    ) -> QrCodeResult<RgbaImage> {
        let matrix = Self::encode_matrix(value, ec_level)?;
        Ok(matrix.render_image(size_px, quiet_zone, foreground, background))
    }

    /// Performs the decode image operation used by this component.
    pub fn decode_image(image: DynamicImage) -> QrCodeResult<Vec<QrDecoded>> {
        let luma = image.to_luma8();
        let mut prepared = rqrr::PreparedImage::prepare(luma);
        let grids = prepared.detect_grids();
        if grids.is_empty() {
            return Err(QrCodeError::NotFound);
        }

        let mut decoded = Vec::new();
        for grid in grids {
            let (meta, content) = grid
                .decode()
                .map_err(|err| QrCodeError::Decode(err.to_string()))?;
            decoded.push(QrDecoded {
                content: content.into(),
                ecc_level: meta.ecc_level as u8,
                version: meta.version.0 as i32,
            });
        }
        Ok(decoded)
    }

    /// Performs the decode bytes operation used by this component.
    pub fn decode_bytes(bytes: &[u8]) -> QrCodeResult<Vec<QrDecoded>> {
        let image =
            image::load_from_memory(bytes).map_err(|err| QrCodeError::Image(err.to_string()))?;
        Self::decode_image(image)
    }

    /// Performs the decode file operation used by this component.
    pub fn decode_file(path: impl AsRef<Path>) -> QrCodeResult<Vec<QrDecoded>> {
        let image = image::open(path).map_err(|err| QrCodeError::Image(err.to_string()))?;
        Self::decode_image(image)
    }
}

/// Fluent native GPUI component for rendering Liora qr matrix.
pub struct QrMatrix {
    /// Width used by layout or hit-testing calculations.
    pub width: usize,
    /// Matrix of QR modules after encoding.
    pub modules: Vec<bool>,
}

impl QrMatrix {
    /// Returns whether dark is currently true for this value.
    pub fn is_dark(&self, x: usize, y: usize) -> bool {
        self.modules[y * self.width + x]
    }

    /// Renders the render image layer into native GPUI elements.
    pub fn render_image(
        &self,
        size_px: u32,
        quiet_zone: u32,
        foreground: [u8; 4],
        background: [u8; 4],
    ) -> RgbaImage {
        self.render_styled_image(
            size_px,
            quiet_zone,
            foreground,
            background,
            None,
            QrModuleStyle::Square,
            QrFinderStyle::Square,
            None,
        )
    }

    /// Renders the render styled image layer into native GPUI elements.
    pub fn render_styled_image(
        &self,
        size_px: u32,
        quiet_zone: u32,
        foreground: [u8; 4],
        background: [u8; 4],
        gradient: Option<&QrGradientBytes>,
        module_style: QrModuleStyle,
        finder_style: QrFinderStyle,
        logo_size_ratio: Option<f32>,
    ) -> RgbaImage {
        let total_modules = self.width as u32 + quiet_zone.saturating_mul(2);
        let scale = (size_px / total_modules).max(1);
        let actual = total_modules * scale;
        let mut image = ImageBuffer::from_pixel(actual, actual, ImageRgba(background));

        let logo_clear = logo_size_ratio.map(|ratio| {
            let clear_modules = ((self.width as f32) * ratio.clamp(0.12, 0.36)).ceil() as usize;
            let clear_modules = clear_modules.max(5);
            let start = self.width.saturating_sub(clear_modules) / 2;
            let end = (start + clear_modules).min(self.width);
            (start, end)
        });

        for y in 0..self.width {
            for x in 0..self.width {
                if !self.is_dark(x, y) {
                    continue;
                }
                if let Some((start, end)) = logo_clear {
                    if x >= start && x < end && y >= start && y < end {
                        continue;
                    }
                }
                let start_x = (x as u32 + quiet_zone) * scale;
                let start_y = (y as u32 + quiet_zone) * scale;
                let is_finder = self.is_finder_module(x, y);
                let style = if is_finder {
                    match finder_style {
                        QrFinderStyle::Square => QrModuleStyle::Square,
                        QrFinderStyle::Rounded => QrModuleStyle::Rounded,
                        QrFinderStyle::Circle => QrModuleStyle::Dots,
                    }
                } else {
                    module_style
                };
                let module_color = gradient
                    .map(|gradient| {
                        gradient.color_at(start_x + scale / 2, start_y + scale / 2, actual)
                    })
                    .unwrap_or(foreground);
                draw_module(&mut image, start_x, start_y, scale, module_color, style);
            }
        }

        image
    }

    fn is_finder_module(&self, x: usize, y: usize) -> bool {
        let w = self.width;
        let in_top = y < 7;
        let in_left = x < 7;
        let in_right = x + 7 >= w;
        let in_bottom = y + 7 >= w;
        (in_top && (in_left || in_right)) || (in_bottom && in_left)
    }
}

fn draw_module(
    image: &mut RgbaImage,
    start_x: u32,
    start_y: u32,
    scale: u32,
    color: [u8; 4],
    style: QrModuleStyle,
) {
    match style {
        QrModuleStyle::Square => fill_rect(image, start_x, start_y, scale, color),
        QrModuleStyle::Rounded => fill_rounded_rect(image, start_x, start_y, scale, color),
        QrModuleStyle::Dots => fill_circle(image, start_x, start_y, scale, color),
    }
}

fn fill_rect(image: &mut RgbaImage, start_x: u32, start_y: u32, scale: u32, color: [u8; 4]) {
    for py in start_y..start_y + scale {
        for px in start_x..start_x + scale {
            image.put_pixel(px, py, ImageRgba(color));
        }
    }
}

fn fill_rounded_rect(
    image: &mut RgbaImage,
    start_x: u32,
    start_y: u32,
    scale: u32,
    color: [u8; 4],
) {
    if scale <= 2 {
        fill_rect(image, start_x, start_y, scale, color);
        return;
    }
    let radius = scale as f32 * 0.32;
    let max = scale as f32 - 1.0;
    for y in 0..scale {
        for x in 0..scale {
            let xf = x as f32;
            let yf = y as f32;
            let cx = if xf < radius {
                radius
            } else if xf > max - radius {
                max - radius
            } else {
                xf
            };
            let cy = if yf < radius {
                radius
            } else if yf > max - radius {
                max - radius
            } else {
                yf
            };
            let dx = xf - cx;
            let dy = yf - cy;
            if dx * dx + dy * dy <= radius * radius + 0.75 {
                image.put_pixel(start_x + x, start_y + y, ImageRgba(color));
            }
        }
    }
}

fn fill_circle(image: &mut RgbaImage, start_x: u32, start_y: u32, scale: u32, color: [u8; 4]) {
    if scale <= 2 {
        fill_rect(image, start_x, start_y, scale, color);
        return;
    }
    let center = (scale as f32 - 1.0) / 2.0;
    let radius = scale as f32 * 0.43;
    let radius_sq = radius * radius;
    for y in 0..scale {
        for x in 0..scale {
            let dx = x as f32 - center;
            let dy = y as f32 - center;
            if dx * dx + dy * dy <= radius_sq {
                image.put_pixel(start_x + x, start_y + y, ImageRgba(color));
            }
        }
    }
}

fn hsla_to_rgba_bytes(color: Hsla) -> [u8; 4] {
    let rgba = Rgba::from(color);
    [
        (rgba.r.clamp(0.0, 1.0) * 255.0).round() as u8,
        (rgba.g.clamp(0.0, 1.0) * 255.0).round() as u8,
        (rgba.b.clamp(0.0, 1.0) * 255.0).round() as u8,
        (rgba.a.clamp(0.0, 1.0) * 255.0).round() as u8,
    ]
}

#[derive(Clone)]
/// Fluent native GPUI component for rendering Liora qr gradient bytes.
pub struct QrGradientBytes {
    colors: Vec<[u8; 4]>,
    direction: QrGradientDirection,
}

impl QrGradientBytes {
    fn new(colors: &[Hsla], direction: QrGradientDirection) -> Self {
        Self {
            colors: colors.iter().copied().map(hsla_to_rgba_bytes).collect(),
            direction,
        }
    }

    fn color_at(&self, x: u32, y: u32, size: u32) -> [u8; 4] {
        if self.colors.is_empty() {
            return [0, 0, 0, 255];
        }
        if self.colors.len() == 1 {
            return self.colors[0];
        }

        let max = size.saturating_sub(1).max(1) as f32;
        let nx = x as f32 / max;
        let ny = y as f32 / max;
        let t = match self.direction {
            QrGradientDirection::ToTop => 1.0 - ny,
            QrGradientDirection::ToTopRight => (nx + (1.0 - ny)) / 2.0,
            QrGradientDirection::ToRight => nx,
            QrGradientDirection::ToBottomRight => (nx + ny) / 2.0,
            QrGradientDirection::ToBottom => ny,
            QrGradientDirection::ToBottomLeft => ((1.0 - nx) + ny) / 2.0,
            QrGradientDirection::ToLeft => 1.0 - nx,
            QrGradientDirection::ToTopLeft => ((1.0 - nx) + (1.0 - ny)) / 2.0,
        }
        .clamp(0.0, 1.0);

        let scaled = t * (self.colors.len() - 1) as f32;
        let index = scaled.floor() as usize;
        let next = (index + 1).min(self.colors.len() - 1);
        let local_t = scaled - index as f32;
        lerp_rgba(self.colors[index], self.colors[next], local_t)
    }
}

fn lerp_rgba(from: [u8; 4], to: [u8; 4], t: f32) -> [u8; 4] {
    [
        lerp_u8(from[0], to[0], t),
        lerp_u8(from[1], to[1], t),
        lerp_u8(from[2], to[2], t),
        lerp_u8(from[3], to[3], t),
    ]
}

fn lerp_u8(from: u8, to: u8, t: f32) -> u8 {
    (from as f32 + (to as f32 - from as f32) * t)
        .round()
        .clamp(0.0, 255.0) as u8
}

fn render_image_from_matrix(
    matrix: &QrMatrix,
    size_px: u32,
    quiet_zone: u32,
    foreground: Hsla,
    background: Hsla,
    gradient: Option<QrGradientBytes>,
    module_style: QrModuleStyle,
    finder_style: QrFinderStyle,
    logo_size_ratio: Option<f32>,
) -> Arc<RenderImage> {
    let image = matrix.render_styled_image(
        size_px,
        quiet_zone,
        hsla_to_rgba_bytes(foreground),
        hsla_to_rgba_bytes(background),
        gradient.as_ref(),
        module_style,
        finder_style,
        logo_size_ratio,
    );
    Arc::new(RenderImage::new([image::Frame::new(image)]))
}

impl RenderOnce for QrCode {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let foreground = self.foreground.unwrap_or(theme.neutral.text_1);
        let gradient = self
            .gradient_colors
            .as_ref()
            .map(|colors| QrGradientBytes::new(colors, self.gradient_direction));
        let background = self.background.unwrap_or(theme.neutral.card);
        let logo_bg = self.logo_background.unwrap_or(theme.neutral.card);
        let logo_color = self.logo_color.unwrap_or(foreground);
        let size_px = self.size.as_f32().max(24.0).round() as u32;
        let logo_size = self.size * self.logo_size_ratio;
        let corner_logo_size = self.size * 0.18;
        let has_logo = self.logo.is_some() || self.logo_text.is_some();

        let content = match Self::encode_matrix(self.value.as_ref(), self.ec_level) {
            Ok(matrix) => {
                let image = render_image_from_matrix(
                    &matrix,
                    size_px,
                    self.quiet_zone,
                    foreground,
                    background,
                    gradient,
                    self.module_style,
                    self.finder_style,
                    has_logo.then_some(self.logo_size_ratio),
                );
                let mut qr = div()
                    .relative()
                    .size(self.size)
                    .child(img(image).size(self.size));
                if let Some(logo) = self.logo {
                    qr = qr.child(
                        div()
                            .absolute()
                            .top((self.size - logo_size) / 2.0)
                            .left((self.size - logo_size) / 2.0)
                            .size(logo_size)
                            .rounded_full()
                            .bg(logo_bg)
                            .border_1()
                            .border_color(background)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(logo),
                    );
                } else if let Some(logo_text) = self.logo_text.clone() {
                    qr = qr.child(
                        div()
                            .absolute()
                            .top((self.size - logo_size) / 2.0)
                            .left((self.size - logo_size) / 2.0)
                            .size(logo_size)
                            .rounded_full()
                            .bg(logo_bg)
                            .border_1()
                            .border_color(background)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(logo_color)
                            .text_size(logo_size * 0.38)
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(logo_text),
                    );
                }
                if let Some(corner_logo) = self.corner_logo {
                    qr = qr.child(
                        div()
                            .absolute()
                            .right(px(8.0))
                            .bottom(px(8.0))
                            .size(corner_logo_size)
                            .rounded_full()
                            .bg(logo_color)
                            .border_1()
                            .border_color(background)
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(corner_logo),
                    );
                } else if let Some(corner_logo_text) = self.corner_logo_text.clone() {
                    qr = qr.child(
                        div()
                            .absolute()
                            .right(px(8.0))
                            .bottom(px(8.0))
                            .size(corner_logo_size)
                            .rounded_full()
                            .bg(logo_color)
                            .border_1()
                            .border_color(background)
                            .flex()
                            .items_center()
                            .justify_center()
                            .text_color(background)
                            .text_size(corner_logo_size * 0.42)
                            .font_weight(gpui::FontWeight::BOLD)
                            .child(corner_logo_text),
                    );
                }
                qr.into_any_element()
            }
            Err(err) => div()
                .flex()
                .items_center()
                .justify_center()
                .size(self.size)
                .rounded(px(theme.radius.md))
                .border_1()
                .border_color(theme.danger.base)
                .text_color(theme.danger.base)
                .text_size(px(theme.font_size.sm))
                .child(format!("QR error: {err:?}"))
                .into_any_element(),
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_2()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .p(px(6.0))
                    .rounded(px(theme.radius.lg))
                    .bg(background)
                    .child(content),
            )
            .when(self.show_text, |s| {
                s.child(
                    div()
                        .max_w(self.size)
                        .text_xs()
                        .text_color(theme.neutral.text_3)
                        .child(self.value),
                )
            })
    }
}

impl IntoElement for QrCode {
    type Element = gpui::Component<Self>;

    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qr_matrix_encodes_content() {
        let matrix = QrCode::encode_matrix("https://liora-ui.dev", QrEcLevel::Medium).unwrap();
        assert!(matrix.width >= 21);
        assert_eq!(matrix.modules.len(), matrix.width * matrix.width);
        assert!(matrix.modules.iter().any(|dark| *dark));
    }

    #[test]
    fn qr_matrix_renders_styled_image() {
        let matrix = QrCode::encode_matrix("styled", QrEcLevel::High).unwrap();
        let image = matrix.render_styled_image(
            240,
            4,
            [20, 20, 20, 255],
            [255, 255, 255, 255],
            None,
            QrModuleStyle::Dots,
            QrFinderStyle::Circle,
            Some(0.24),
        );
        assert!(image.width() >= 200);
        assert_eq!(image.width(), image.height());
    }

    #[test]
    fn qr_gradient_interpolates_in_all_directions() {
        let gradient = QrGradientBytes {
            colors: vec![[0, 0, 0, 255], [255, 255, 255, 255]],
            direction: QrGradientDirection::ToRight,
        };
        assert_eq!(gradient.color_at(0, 5, 10)[0], 0);
        assert_eq!(gradient.color_at(9, 5, 10)[0], 255);

        for direction in [
            QrGradientDirection::ToTop,
            QrGradientDirection::ToTopRight,
            QrGradientDirection::ToRight,
            QrGradientDirection::ToBottomRight,
            QrGradientDirection::ToBottom,
            QrGradientDirection::ToBottomLeft,
            QrGradientDirection::ToLeft,
            QrGradientDirection::ToTopLeft,
        ] {
            let gradient = QrGradientBytes {
                colors: vec![[0, 0, 0, 255], [255, 255, 255, 255]],
                direction,
            };
            let color = gradient.color_at(4, 4, 10);
            assert_eq!(color[3], 255);
        }
    }

    #[test]
    fn qr_decode_round_trips_generated_image() {
        let content = "liora://component/qr-code";
        let image = QrCode::render_image(
            content,
            256,
            4,
            [0, 0, 0, 255],
            [255, 255, 255, 255],
            QrEcLevel::High,
        )
        .unwrap();
        let decoded = QrCode::decode_image(DynamicImage::ImageRgba8(image)).unwrap();
        assert_eq!(decoded[0].content.as_ref(), content);
    }
}
