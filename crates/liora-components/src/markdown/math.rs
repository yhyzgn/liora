//! Bounded native LaTeX math rendering backed by `latex-rust`.
//!
//! `latex-rust` embeds STIX Two Math, licensed under the SIL Open Font License
//! 1.1; its bundled dvips color data is LPPL-1.3c-or-later. See that crate's
//! `NOTICE` for the complete notices. Unsupported or malformed input is an
//! error, never a text or placeholder fallback.

use std::{fmt, sync::Arc};

use gpui::RenderImage;
use image::{Frame, ImageEncoder};
use latex_rust::{Dim, MathFont, MathStyle, PngOptions, layout_with_em_size_pt, parse, render_png};

const DEFAULT_MAX_SOURCE_BYTES: usize = 64 * 1024;
const DEFAULT_MAX_PIXELS: u64 = 4_000_000;
const DEFAULT_MAX_PNG_BYTES: usize = 8 * 1024 * 1024;

/// Limits applied before and after native math rasterization.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MathLimits {
    pub(crate) max_source_bytes: usize,
    pub(crate) max_pixels: u64,
    pub(crate) max_png_bytes: usize,
}

impl Default for MathLimits {
    fn default() -> Self {
        Self {
            max_source_bytes: DEFAULT_MAX_SOURCE_BYTES,
            max_pixels: DEFAULT_MAX_PIXELS,
            max_png_bytes: DEFAULT_MAX_PNG_BYTES,
        }
    }
}

/// Measurements useful to diagnostics and layout callers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MathDiagnostic {
    pub(crate) source_bytes: usize,
    pub(crate) width_pixels: u32,
    pub(crate) height_pixels: u32,
    pub(crate) png_bytes: usize,
}

/// Native output in both transport-friendly and GPUI-ready forms.
#[derive(Clone)]
pub(crate) struct MathRender {
    pub(crate) png: Vec<u8>,
    pub(crate) image: Arc<RenderImage>,
    pub(crate) diagnostic: MathDiagnostic,
}

#[derive(Debug)]
pub(crate) enum MathError {
    SourceTooLong { actual: usize, limit: usize },
    PixelLimit { actual: u64, limit: u64 },
    PngLimit { actual: usize, limit: usize },
    Latex(latex_rust::Error),
    ImageDecode(String),
    ImageEncode(String),
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SourceTooLong { actual, limit } => {
                write!(f, "math source is {actual} bytes; limit is {limit}")
            }
            Self::PixelLimit { actual, limit } => {
                write!(f, "math image is {actual} pixels; limit is {limit}")
            }
            Self::PngLimit { actual, limit } => {
                write!(f, "math PNG is {actual} bytes; limit is {limit}")
            }
            Self::Latex(error) => write!(f, "LaTeX rendering failed: {error}"),
            Self::ImageDecode(error) => write!(f, "rendered PNG could not be decoded: {error}"),
            Self::ImageEncode(error) => write!(f, "rendered image could not be encoded: {error}"),
        }
    }
}

impl std::error::Error for MathError {}

impl From<latex_rust::Error> for MathError {
    fn from(error: latex_rust::Error) -> Self {
        Self::Latex(error)
    }
}

/// Parse, lay out, and render bounded LaTeX math to PNG and a native GPUI image.
/// Theme alpha applies uniformly to the formula, preserving explicit LaTeX RGB scopes.
pub(crate) fn render_math(
    source: &str,
    display: bool,
    foreground: [u8; 4],
    limits: MathLimits,
) -> Result<MathRender, MathError> {
    let source_bytes = source.len();
    if source_bytes > limits.max_source_bytes {
        return Err(MathError::SourceTooLong {
            actual: source_bytes,
            limit: limits.max_source_bytes,
        });
    }

    let ast = parse(source).map_err(latex_rust::Error::from)?;
    let font = MathFont::stix_two_math().map_err(latex_rust::Error::from)?;
    let options = PngOptions {
        color: latex_rust::Color::rgb(foreground[0], foreground[1], foreground[2]),
        ..PngOptions::new()
    };
    let style = if display {
        MathStyle::Display
    } else {
        MathStyle::Text
    };
    let boxed = layout_with_em_size_pt(&ast, &font, style, &options.font_size_pt)?;
    let em_px = &options.font_size_pt * &options.dpi / Dim::from_i64(72);
    let width = (&boxed.width * &em_px)
        .ceil_to_u32()
        .map_err(latex_rust::Error::from)?
        .max(1);
    let height = ((&boxed.height + &boxed.depth) * &em_px)
        .ceil_to_u32()
        .map_err(latex_rust::Error::from)?
        .max(1);
    let pixels = u64::from(width)
        .checked_mul(u64::from(height))
        .ok_or(MathError::PixelLimit {
            actual: u64::MAX,
            limit: limits.max_pixels,
        })?;
    if pixels > limits.max_pixels {
        return Err(MathError::PixelLimit {
            actual: pixels,
            limit: limits.max_pixels,
        });
    }

    let mut png = render_png(&boxed, &font, &options)?;
    let mut rgba = image::load_from_memory(&png)
        .map_err(|error| MathError::ImageDecode(error.to_string()))?
        .into_rgba8();
    // latex-rust 3.0.3 Color is RGB-only; apply parent text opacity after rasterization.
    if foreground[3] != 255 {
        for pixel in rgba.pixels_mut() {
            pixel[3] = ((u16::from(pixel[3]) * u16::from(foreground[3]) + 127) / 255) as u8;
        }
        png.clear();
        image::codecs::png::PngEncoder::new(&mut png)
            .write_image(
                rgba.as_raw(),
                rgba.width(),
                rgba.height(),
                image::ExtendedColorType::Rgba8,
            )
            .map_err(|error| MathError::ImageEncode(error.to_string()))?;
    }
    let png_bytes = png.len();
    if png_bytes > limits.max_png_bytes {
        return Err(MathError::PngLimit {
            actual: png_bytes,
            limit: limits.max_png_bytes,
        });
    }
    for pixel in rgba.chunks_exact_mut(4) {
        pixel.swap(0, 2);
    }
    let image = Arc::new(RenderImage::new([Frame::new(rgba)]));
    Ok(MathRender {
        png,
        image,
        diagnostic: MathDiagnostic {
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

    #[test]
    fn renders_fraction_to_png_and_image() {
        let rendered = render_math(r"\frac{1}{2}", true, [0, 0, 0, 255], MathLimits::default())
            .expect("fraction should render");
        assert!(rendered.png.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert!(rendered.diagnostic.width_pixels > 0);
        assert!(rendered.diagnostic.height_pixels > 0);
        assert_eq!(rendered.diagnostic.source_bytes, 11);
        assert_eq!(
            rendered.image.size(0).width.0 as u32,
            rendered.diagnostic.width_pixels
        );
    }

    #[test]
    fn default_glyphs_contrast_with_light_and_dark_backgrounds() {
        for display in [false, true] {
            for (foreground, background) in [([0, 0, 0, 255], 255u8), ([255, 255, 255, 255], 0)] {
                let rendered =
                    render_math(r"\frac{x}{2}", display, foreground, MathLimits::default())
                        .expect("theme-colored fraction should render");
                let rgba = image::load_from_memory(&rendered.png).unwrap().into_rgba8();
                assert!(rgba.pixels().any(|pixel| pixel[3] == 0));
                let mut visible = 0;
                for pixel in rgba.pixels().filter(|pixel| pixel[3] >= 128) {
                    visible += 1;
                    for channel in 0..3 {
                        assert_eq!(pixel[channel], foreground[channel]);
                        let alpha = f32::from(pixel[3]) / 255.0;
                        let composited = f32::from(pixel[channel]) * alpha
                            + f32::from(background) * (1.0 - alpha);
                        assert!((composited - f32::from(background)).abs() >= 128.0);
                    }
                }
                assert!(visible > 0);
            }
        }
    }

    #[test]
    fn explicit_latex_colors_override_theme_default_only_in_their_scope() {
        for foreground in [[0, 0, 0, 255], [255, 255, 255, 255]] {
            let rendered = render_math(
                r"\textcolor{red}{x}+y",
                false,
                foreground,
                MathLimits::default(),
            )
            .expect("explicit color should render");
            let rgba = image::load_from_memory(&rendered.png).unwrap().into_rgba8();
            assert!(rgba.pixels().any(|pixel| pixel.0 == [255, 0, 0, 255]));
            assert!(rgba.pixels().any(|pixel| pixel.0 == foreground));
        }
    }

    #[test]
    fn theme_opacity_scales_coverage_without_changing_latex_color_scopes() {
        let source = r"\textcolor{red}{x}+y";
        let opaque = render_math(source, false, [24, 48, 72, 255], MathLimits::default())
            .expect("opaque formula should render");
        let translucent = render_math(source, false, [24, 48, 72, 128], MathLimits::default())
            .expect("translucent formula should render");
        let opaque = image::load_from_memory(&opaque.png).unwrap().into_rgba8();
        let translucent_png = image::load_from_memory(&translucent.png)
            .unwrap()
            .into_rgba8();
        assert_eq!(opaque.dimensions(), translucent_png.dimensions());
        assert!(opaque.pixels().any(|pixel| pixel.0 == [255, 0, 0, 255]));
        assert!(opaque.pixels().any(|pixel| pixel.0 == [24, 48, 72, 255]));
        for (original, faded) in opaque.pixels().zip(translucent_png.pixels()) {
            assert_eq!(&original.0[..3], &faded.0[..3]);
            assert_eq!(faded[3], ((u16::from(original[3]) * 128 + 127) / 255) as u8);
        }
        assert_eq!(translucent.png.len(), translucent.diagnostic.png_bytes);
    }

    #[test]
    fn rejects_invalid_latex_with_real_diagnostic() {
        let error = render_math(
            r"\frac{1",
            false,
            [255, 255, 255, 255],
            MathLimits::default(),
        )
        .err()
        .expect("unterminated fraction must fail");
        assert!(matches!(error, MathError::Latex(_)));
        assert!(error.to_string().contains("LaTeX rendering failed"));
    }
}
