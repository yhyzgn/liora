//! Eager, in-memory PDF rasterization. No filesystem or network access.

use super::{PdfDocument, PdfPage, PdfPageSize};
use gpui::{RenderImage, SharedString};
use hayro::hayro_syntax::page::Rotation;
use hayro::hayro_syntax::{LoadPdfError, Pdf};
use hayro::vello_cpu::peniko::ImageAlphaType;
use hayro::{PixmapSettings, RenderCache, RenderSettings, hayro_interpret::InterpreterSettings};
use image::{Frame, RgbaImage};
use std::{fmt, sync::Arc};

const MAX_INPUT_BYTES: usize = 64 * 1024 * 1024;
const MAX_EAGER_BYTES: u64 = 128 * 1024 * 1024;
const MAX_RENDER_SIDE: u32 = 4096;
const PAGE_SIDE: f32 = 2048.0;
const THUMBNAIL_SIDE: u32 = 120;

/// A PDF could not be parsed or safely rasterized within the reader's limits.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfDecodeError {
    /// The supplied byte buffer contains no data.
    EmptyInput,
    /// The supplied byte buffer exceeds 64 MiB.
    InputTooLarge,
    /// The PDF structure is malformed or unsupported by the parser.
    InvalidPdf,
    /// The parser could not decrypt the PDF; password entry is not supported.
    EncryptedPdf,
    /// The parsed document contains no pages.
    NoPages,
    /// A one-based page has nonfinite, nonpositive, or unrenderable dimensions.
    InvalidPageDimensions(usize),
    /// A one-based page would produce a raster side larger than 4096 pixels.
    RenderDimensionsTooLarge(usize),
    /// All page rasters and thumbnails together would exceed 128 MiB of RGBA.
    RasterBudgetExceeded,
}

impl fmt::Display for PdfDecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyInput => f.write_str("PDF input is empty"),
            Self::InputTooLarge => f.write_str("PDF input exceeds 64 MiB"),
            Self::InvalidPdf => f.write_str("PDF is invalid or unsupported"),
            Self::EncryptedPdf => {
                f.write_str("PDF could not be decrypted; passwords are unsupported")
            }
            Self::NoPages => f.write_str("PDF contains no pages"),
            Self::InvalidPageDimensions(page) => {
                write!(f, "PDF page {page} has invalid dimensions")
            }
            Self::RenderDimensionsTooLarge(page) => {
                write!(f, "PDF page {page} exceeds the 4096-pixel raster limit")
            }
            Self::RasterBudgetExceeded => f.write_str("PDF rasters exceed the 128 MiB budget"),
        }
    }
}

impl std::error::Error for PdfDecodeError {}

pub(super) fn decode(title: SharedString, bytes: Vec<u8>) -> Result<PdfDocument, PdfDecodeError> {
    if bytes.is_empty() {
        return Err(PdfDecodeError::EmptyInput);
    }
    if bytes.len() > MAX_INPUT_BYTES {
        return Err(PdfDecodeError::InputTooLarge);
    }
    let pdf = Pdf::new(bytes).map_err(|error| match error {
        LoadPdfError::Invalid => PdfDecodeError::InvalidPdf,
        LoadPdfError::Decryption(_) => PdfDecodeError::EncryptedPdf,
    })?;
    if pdf.pages().is_empty() {
        return Err(PdfDecodeError::NoPages);
    }

    // Validate every page before hayro can allocate any raster buffers.
    let mut budget = 0;
    for (index, page) in pdf.pages().iter().enumerate() {
        let number = index + 1;
        // Match Hayro's near-zero fallback threshold so metadata matches its raster.
        for rect in [
            page.media_box(),
            page.crop_box(),
            page.intersected_crop_box(),
        ] {
            if ![rect.x0, rect.y0, rect.x1, rect.y1]
                .iter()
                .all(|value| value.is_finite())
                || (rect.width() as f32) <= 1.0 / 4096.0
                || (rect.height() as f32) <= 1.0 / 4096.0
            {
                return Err(PdfDecodeError::InvalidPageDimensions(number));
            }
        }
        let (width, height) = page.render_dimensions();
        let (_, raster_width, raster_height) = raster_geometry(width, height, number)?;
        reserve_raster(&mut budget, raster_width, raster_height)?;
    }

    let cache = RenderCache::new();
    let interpreter = InterpreterSettings::default();
    let settings = RenderSettings::default();
    let mut pages = Vec::with_capacity(pdf.pages().len());
    for (index, page) in pdf.pages().iter().enumerate() {
        let number = index + 1;
        let (width, height) = page.render_dimensions();
        let (scale, _, _) = raster_geometry(width, height, number)?;
        let pixmap = hayro::render(
            page,
            &cache,
            &interpreter,
            &settings,
            &PixmapSettings {
                x_scale: scale,
                y_scale: scale,
                ..Default::default()
            },
        );
        let raster_width = u32::from(pixmap.width());
        let raster_height = u32::from(pixmap.height());
        let pixels = pixmap.take_rgba8(ImageAlphaType::AlphaPremultiplied);
        let mut image = RgbaImage::from_raw(raster_width, raster_height, pixels)
            .ok_or(PdfDecodeError::InvalidPageDimensions(number))?;
        let (thumb_width, thumb_height) = thumbnail_dimensions(raster_width, raster_height);
        // Resize premultiplied pixels to keep transparent edges free of color halos.
        let mut thumbnail = image::imageops::resize(
            &image,
            thumb_width,
            thumb_height,
            image::imageops::FilterType::Triangle,
        );
        let crop = page.intersected_crop_box();
        let (mut point_width, mut point_height) = (crop.width() as f32, crop.height() as f32);
        if matches!(
            page.rotation(),
            Rotation::Horizontal | Rotation::FlippedHorizontal
        ) {
            std::mem::swap(&mut point_width, &mut point_height);
        }
        premultiplied_rgba_to_bgra(image.as_mut());
        premultiplied_rgba_to_bgra(thumbnail.as_mut());
        pages.push(PdfPage {
            number,
            size: PdfPageSize {
                width: point_width,
                height: point_height,
            },
            image: Some(Arc::new(RenderImage::new([Frame::new(image)]))),
            thumbnail: Some(Arc::new(RenderImage::new([Frame::new(thumbnail)]))),
        });
    }
    Ok(PdfDocument::new(title, pages))
}

fn raster_geometry(
    width: f32,
    height: f32,
    page: usize,
) -> Result<(f32, u32, u32), PdfDecodeError> {
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return Err(PdfDecodeError::InvalidPageDimensions(page));
    }
    let scale = (PAGE_SIDE / width.max(height)).min(1.0);
    // hayro 0.9 truncates scaled dimensions when creating its u16 RenderContext.
    let raster_width = (width * scale) as u32;
    let raster_height = (height * scale) as u32;
    if raster_width == 0 || raster_height == 0 {
        return Err(PdfDecodeError::InvalidPageDimensions(page));
    }
    if raster_width > MAX_RENDER_SIDE || raster_height > MAX_RENDER_SIDE {
        return Err(PdfDecodeError::RenderDimensionsTooLarge(page));
    }
    Ok((scale, raster_width, raster_height))
}

fn thumbnail_dimensions(width: u32, height: u32) -> (u32, u32) {
    let side = width.max(height);
    if side <= THUMBNAIL_SIDE {
        (width, height)
    } else {
        (
            (width * THUMBNAIL_SIDE / side).max(1),
            (height * THUMBNAIL_SIDE / side).max(1),
        )
    }
}

fn reserve_raster(budget: &mut u64, width: u32, height: u32) -> Result<(), PdfDecodeError> {
    let (thumb_width, thumb_height) = thumbnail_dimensions(width, height);
    let bytes = (u64::from(width) * u64::from(height)
        + u64::from(thumb_width) * u64::from(thumb_height))
        * 4;
    *budget = budget
        .checked_add(bytes)
        .ok_or(PdfDecodeError::RasterBudgetExceeded)?;
    if *budget > MAX_EAGER_BYTES {
        return Err(PdfDecodeError::RasterBudgetExceeded);
    }
    Ok(())
}

fn premultiplied_rgba_to_bgra(bytes: &mut [u8]) {
    for pixel in bytes.chunks_exact_mut(4) {
        let alpha = u32::from(pixel[3]);
        for channel in &mut pixel[..3] {
            *channel = if alpha == 0 {
                0
            } else {
                ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8
            };
        }
        pixel.swap(0, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../../../../assets/samples/reader.pdf");

    #[test]
    fn rejects_empty_malformed_and_oversized_input() {
        assert!(matches!(
            decode("empty".into(), vec![]),
            Err(PdfDecodeError::EmptyInput)
        ));
        assert!(matches!(
            decode("bad".into(), b"not a PDF".to_vec()),
            Err(PdfDecodeError::InvalidPdf)
        ));
        assert!(matches!(
            decode("large".into(), vec![0; MAX_INPUT_BYTES + 1]),
            Err(PdfDecodeError::InputTooLarge)
        ));
    }

    #[test]
    fn geometry_is_bounded_without_losing_point_dimensions() {
        for (width, height) in [
            (0.0, 10.0),
            (-1.0, 10.0),
            (f32::NAN, 10.0),
            (10.0, f32::INFINITY),
            (1.0, f32::MAX),
        ] {
            assert_eq!(
                raster_geometry(width, height, 3),
                Err(PdfDecodeError::InvalidPageDimensions(3))
            );
        }
        assert_eq!(raster_geometry(400.0, 300.0, 1), Ok((1.0, 400, 300)));
        assert_eq!(raster_geometry(8192.0, 4096.0, 1), Ok((0.25, 2048, 1024)));
        assert_eq!(thumbnail_dimensions(2048, 1024), (120, 60));
        assert_eq!(thumbnail_dimensions(1, 2048), (1, 120));
        let mut budget = 0;
        for _ in 0..7 {
            reserve_raster(&mut budget, 2048, 2048).unwrap();
        }
        assert_eq!(
            reserve_raster(&mut budget, 2048, 2048),
            Err(PdfDecodeError::RasterBudgetExceeded)
        );
    }

    #[test]
    fn rejects_degenerate_page_before_hayro_substitutes_a4() {
        let source = std::str::from_utf8(SAMPLE).unwrap();
        // Same byte lengths keep xref offsets and stream lengths intact.
        for replacement in ["[0 0 000 300]", "[0 0 .0001 3]"] {
            let degenerate = source.replacen("[0 0 400 300]", replacement, 1);
            assert!(matches!(
                decode("degenerate".into(), degenerate.into_bytes()),
                Err(PdfDecodeError::InvalidPageDimensions(1))
            ));
        }
    }

    #[test]
    fn converts_known_colors_and_unpremultiplies_alpha() {
        let mut pixels = [
            255, 0, 0, 255, 0, 0, 255, 255, 0, 255, 0, 255, 128, 32, 0, 128, 0, 0, 0, 0,
        ];
        premultiplied_rgba_to_bgra(&mut pixels);
        assert_eq!(
            pixels,
            [
                0, 0, 255, 255, 255, 0, 0, 255, 0, 255, 0, 255, 0, 64, 255, 128, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    fn decodes_original_vector_color_and_text_pages() {
        let document = decode("Original sample".into(), SAMPLE.to_vec()).unwrap();
        assert_eq!(document.title.as_ref(), "Original sample");
        assert_eq!(document.pages.len(), 2);
        let page = &document.pages[0];
        assert_eq!(
            page.size,
            PdfPageSize {
                width: 400.0,
                height: 300.0
            }
        );
        assert_eq!(
            document.pages[1].size,
            PdfPageSize {
                width: 300.0,
                height: 400.0
            }
        );
        let image = page.image.as_ref().unwrap();
        let bytes = image.as_bytes(0).unwrap();
        let pixel = |x: usize, y: usize| &bytes[(y * 400 + x) * 4..(y * 400 + x + 1) * 4];
        assert_eq!(pixel(50, 120), [0, 0, 255, 255]);
        assert_eq!(pixel(180, 120), [255, 0, 0, 255]);
        assert_eq!(pixel(290, 120), [0, 255, 0, 255]);
        assert_eq!(pixel(390, 290), [255, 255, 255, 255]);
        // The title region contains actual rasterized glyphs, not just page vectors.
        assert!(
            (20..42)
                .any(|y| (24..350).any(|x| pixel(x, y)[..3].iter().all(|channel| *channel < 100)))
        );
        for page in &document.pages {
            let thumbnail = page.thumbnail.as_ref().unwrap();
            assert!(thumbnail.as_bytes(0).unwrap().len() <= 120 * 120 * 4);
        }
    }
}
