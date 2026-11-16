//! Bounded, in-memory PDF rasterization with optional first-page preview.

use super::{PdfDocument, PdfPage, PdfPageSize};
use gpui::{RenderImage, SharedString};
use hayro::hayro_syntax::page::{Page, Rotation};
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

/// Parsed PDF metadata that can rasterize one page on demand.
pub struct PdfDecoder {
    title: SharedString,
    pdf: Pdf,
    sizes: Vec<PdfPageSize>,
}

impl PdfDecoder {
    /// Parses and validates metadata without allocating page rasters.
    pub fn new(title: SharedString, bytes: Vec<u8>) -> Result<Self, PdfDecodeError> {
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
        let mut budget = 0;
        let mut sizes = Vec::with_capacity(pdf.pages().len());
        for (index, page) in pdf.pages().iter().enumerate() {
            let number = index + 1;
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
            sizes.push(page_size(page));
        }
        Ok(Self { title, pdf, sizes })
    }

    /// Returns page metadata with no page raster allocated.
    pub fn document(&self) -> PdfDocument {
        PdfDocument::new(
            self.title.clone(),
            self.sizes
                .iter()
                .enumerate()
                .map(|(index, size)| PdfPage {
                    number: index + 1,
                    size: *size,
                    image: None,
                    thumbnail: None,
                })
                .collect(),
        )
    }

    /// Rasterizes exactly one validated page, including its thumbnail.
    pub fn render_page(&self, page: usize) -> Result<PdfPage, PdfDecodeError> {
        let index = page
            .checked_sub(1)
            .ok_or(PdfDecodeError::InvalidPageDimensions(page))?;
        let source = self
            .pdf
            .pages()
            .get(index)
            .ok_or(PdfDecodeError::InvalidPageDimensions(page))?;
        let (width, height) = source.render_dimensions();
        let (scale, _, _) = raster_geometry(width, height, page)?;
        let cache = RenderCache::new();
        let pixmap = hayro::render(
            source,
            &cache,
            &InterpreterSettings::default(),
            &RenderSettings::default(),
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
            .ok_or(PdfDecodeError::InvalidPageDimensions(page))?;
        let (thumb_width, thumb_height) = thumbnail_dimensions(raster_width, raster_height);
        let mut thumbnail = image::imageops::thumbnail(&image, thumb_width, thumb_height);
        premultiplied_rgba_to_bgra(image.as_mut());
        premultiplied_rgba_to_bgra(thumbnail.as_mut());
        Ok(PdfPage {
            number: page,
            size: self.sizes[index],
            image: Some(Arc::new(RenderImage::new([Frame::new(image)]))),
            thumbnail: Some(Arc::new(RenderImage::new([Frame::new(thumbnail)]))),
        })
    }
}

pub(super) fn decode(title: SharedString, bytes: Vec<u8>) -> Result<PdfDocument, PdfDecodeError> {
    decode_inner(title, bytes, None::<fn(PdfDocument)>)
}

pub(super) fn decode_with_preview(
    title: SharedString,
    bytes: Vec<u8>,
    on_preview: impl FnOnce(PdfDocument),
) -> Result<PdfDocument, PdfDecodeError> {
    decode_inner(title, bytes, Some(on_preview))
}

fn decode_inner(
    title: SharedString,
    bytes: Vec<u8>,
    mut on_preview: Option<impl FnOnce(PdfDocument)>,
) -> Result<PdfDocument, PdfDecodeError> {
    let decoder = PdfDecoder::new(title, bytes)?;
    let mut document = decoder.document();
    for index in 0..document.pages.len() {
        document.pages[index] = decoder.render_page(index + 1)?;
        if index == 0
            && let Some(on_preview) = on_preview.take()
        {
            let mut preview = decoder.document();
            preview.pages[0].image = document.pages[0].image.clone();
            preview.pages[0].thumbnail = document.pages[0].thumbnail.clone();
            on_preview(preview);
        }
    }
    Ok(document)
}

fn page_size(page: &Page<'_>) -> PdfPageSize {
    let crop = page.intersected_crop_box();
    let (mut width, mut height) = (crop.width() as f32, crop.height() as f32);
    if matches!(
        page.rotation(),
        Rotation::Horizontal | Rotation::FlippedHorizontal
    ) {
        std::mem::swap(&mut width, &mut height);
    }
    PdfPageSize { width, height }
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
        if alpha == 0 {
            pixel[..3].fill(0);
        } else if alpha != 255 {
            for channel in &mut pixel[..3] {
                *channel = ((u32::from(*channel) * 255 + alpha / 2) / alpha).min(255) as u8;
            }
        }
        pixel.swap(0, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &[u8] = include_bytes!("../../../../assets/samples/reader.pdf");

    #[test]
    fn lazy_decoder_delivers_only_requested_pages_and_rejects_invalid_demands() {
        let decoder = PdfDecoder::new("Lazy PDF".into(), SAMPLE.to_vec()).unwrap();
        let mut document = decoder.document();
        assert!(
            document
                .pages
                .iter()
                .all(|page| page.image.is_none() && page.thumbnail.is_none())
        );
        document.pages[0] = decoder.render_page(1).unwrap();
        assert!(document.pages[0].image.is_some());
        assert!(document.pages[1].image.is_none());
        assert!(document.pages[1].thumbnail.is_none());
        assert!(matches!(
            decoder.render_page(0),
            Err(PdfDecodeError::InvalidPageDimensions(0))
        ));
        assert!(matches!(
            decoder.render_page(3),
            Err(PdfDecodeError::InvalidPageDimensions(3))
        ));
        document.pages[1] = decoder.render_page(2).unwrap();
        assert!(
            document
                .pages
                .iter()
                .all(|page| page.image.is_some() && page.thumbnail.is_some())
        );
    }

    #[test]
    fn lazy_decoder_preflights_invalid_later_pages_before_delivery() {
        let bytes = std::str::from_utf8(SAMPLE)
            .unwrap()
            .replacen("[0 0 300 400]", "[0 0 000 400]", 1)
            .into_bytes();
        assert!(matches!(
            PdfDecoder::new("Invalid".into(), bytes),
            Err(PdfDecodeError::InvalidPageDimensions(2))
        ));
    }

    #[test]
    #[ignore = "manual CPU timing; run with --ignored --nocapture"]
    fn measure_pdf_decode_stages() {
        let bytes = std::env::var_os("LIORA_PDF_BENCH_PATH")
            .map(|path| std::fs::read(path).unwrap())
            .unwrap_or_else(|| SAMPLE.to_vec());
        let lazy_start = std::time::Instant::now();
        let decoder = PdfDecoder::new("Lazy timing".into(), bytes.clone()).unwrap();
        let preflight = lazy_start.elapsed();
        let mut initial = decoder.document();
        initial.pages[0] = decoder.render_page(1).unwrap();
        let lazy_first = lazy_start.elapsed();
        assert_eq!(
            initial
                .pages
                .iter()
                .filter(|page| page.image.is_some())
                .count(),
            1
        );
        eprintln!(
            "lazy_pages={} preflight={preflight:?} first_ready={lazy_first:?} rasterized_pages=1",
            initial.page_count()
        );
        let start = std::time::Instant::now();
        let pdf = Pdf::new(bytes.clone()).unwrap();
        let parse = start.elapsed();
        let cache = RenderCache::new();
        let interpreter = InterpreterSettings::default();
        let settings = RenderSettings::default();
        let mut first = None;
        let render_start = std::time::Instant::now();
        for (index, page) in pdf.pages().iter().enumerate() {
            let (width, height) = page.render_dimensions();
            let (scale, _, _) = raster_geometry(width, height, index + 1).unwrap();
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
            std::hint::black_box(pixmap);
            first.get_or_insert_with(|| render_start.elapsed());
        }
        let all_render = render_start.elapsed();
        let decode_start = std::time::Instant::now();
        let mut preview_ready = None;
        let document = decode_with_preview("Timing".into(), bytes, |_| {
            preview_ready = Some(decode_start.elapsed());
        })
        .unwrap();
        eprintln!(
            "PDF pages={} parse={parse:?} first_render={:?} all_render={all_render:?} preview_ready={:?} full_decode={:?}",
            document.page_count(),
            first.unwrap(),
            preview_ready.unwrap(),
            decode_start.elapsed()
        );
    }

    #[test]
    fn preview_has_all_page_metadata_and_shares_only_the_first_raster() {
        let mut preview = None;
        let document = PdfDocument::from_bytes_with_preview("Preview", SAMPLE.to_vec(), |first| {
            assert_eq!(first.page_count(), 2);
            assert!(first.pages[0].image.is_some());
            assert!(first.pages[1].image.is_none());
            assert!(first.pages[1].thumbnail.is_none());
            preview = Some(first);
        })
        .unwrap();
        let preview = preview.expect("the consumer receives a preview before decode returns");
        assert_eq!(preview.title, document.title);
        for (early, complete) in preview.pages.iter().zip(&document.pages) {
            assert_eq!(early.number, complete.number);
            assert_eq!(early.size, complete.size);
            assert!(complete.image.is_some());
            assert!(complete.thumbnail.is_some());
        }
        assert!(Arc::ptr_eq(
            preview.pages[0].image.as_ref().unwrap(),
            document.pages[0].image.as_ref().unwrap()
        ));
        assert!(Arc::ptr_eq(
            preview.pages[0].thumbnail.as_ref().unwrap(),
            document.pages[0].thumbnail.as_ref().unwrap()
        ));
    }

    #[test]
    fn preview_preserves_fractional_page_geometry() {
        let source = std::str::from_utf8(SAMPLE).unwrap();
        let bytes = source
            .replacen("[0 0 400 300]", "[0 0 0.5 002]", 1)
            .into_bytes();
        let mut preview_size = None;
        let document = PdfDocument::from_bytes_with_preview("Fractional", bytes, |preview| {
            preview_size = Some(preview.pages[0].size);
        })
        .unwrap();
        assert_eq!(
            document.pages[0].size,
            PdfPageSize {
                width: 0.5,
                height: 2.0
            }
        );
        assert_eq!(preview_size, Some(document.pages[0].size));
    }

    #[test]
    fn invalid_later_page_never_reaches_preview_consumer() {
        let source = std::str::from_utf8(SAMPLE).unwrap();
        let bytes = source
            .replacen("[0 0 300 400]", "[0 0 000 400]", 1)
            .into_bytes();
        let mut called = false;
        assert!(matches!(
            PdfDocument::from_bytes_with_preview("Invalid later page", bytes, |_| called = true),
            Err(PdfDecodeError::InvalidPageDimensions(2))
        ));
        assert!(!called);
    }

    #[test]
    fn integer_thumbnails_keep_premultiplied_transparent_edges() {
        let source = RgbaImage::from_raw(2, 1, vec![128, 0, 0, 128, 0, 0, 0, 0]).unwrap();
        let mut thumbnail = image::imageops::thumbnail(&source, 1, 1);
        assert_eq!(thumbnail.as_raw(), &[64, 0, 0, 64]);
        premultiplied_rgba_to_bgra(thumbnail.as_mut());
        assert_eq!(thumbnail.as_raw(), &[0, 0, 255, 64]);
    }

    #[test]
    fn over_budget_document_never_reaches_preview_consumer() {
        let kids: String = (3..=10).map(|number| format!("{number} 0 R ")).collect();
        let mut objects = vec![
            "<< /Type /Catalog /Pages 2 0 R >>".to_owned(),
            format!("<< /Type /Pages /Kids [{kids}] /Count 8 >>"),
        ];
        objects.extend(
            (0..8).map(|_| "<< /Type /Page /Parent 2 0 R /MediaBox [0 0 2048 2048] >>".to_owned()),
        );
        let mut source = "%PDF-1.4\n".to_owned();
        let mut offsets = vec![0];
        for (index, object) in objects.iter().enumerate() {
            offsets.push(source.len());
            source.push_str(&format!("{} 0 obj\n{object}\nendobj\n", index + 1));
        }
        let xref = source.len();
        source.push_str("xref\n0 11\n0000000000 65535 f \n");
        for offset in &offsets[1..] {
            source.push_str(&format!("{offset:010} 00000 n \n"));
        }
        source.push_str(&format!(
            "trailer\n<< /Size 11 /Root 1 0 R >>\nstartxref\n{xref}\n%%EOF\n"
        ));
        let mut called = false;
        assert!(matches!(
            PdfDocument::from_bytes_with_preview("Over budget", source.into_bytes(), |_| called =
                true),
            Err(PdfDecodeError::RasterBudgetExceeded)
        ));
        assert!(!called);
    }

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
