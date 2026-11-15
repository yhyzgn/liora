use super::{PdfPageSize, PdfRotation, PdfZoom};
use gpui::RenderImage;
use std::sync::Arc;

pub(super) fn dimensions(
    size: PdfPageSize,
    rotation: PdfRotation,
    zoom: PdfZoom,
    viewport: (f32, f32),
) -> (f32, f32) {
    let size = PdfPageSize::new(size.width, size.height);
    let (width, height) = if rotation.is_quarter_turn() {
        (size.height, size.width)
    } else {
        (size.width, size.height)
    };
    // Reserve the page's native padding and border inside the measured viewport.
    let available = ((viewport.0 - 26.0).max(1.0), (viewport.1 - 26.0).max(1.0));
    let scale = match zoom {
        PdfZoom::FitPage => (available.0 / width).min(available.1 / height),
        PdfZoom::FitWidth => available.0 / width,
        PdfZoom::Percent(percent) => f32::from(percent.clamp(25, 400)) / 100.0,
    };
    (width * scale, height * scale)
}

pub(super) fn rotate_image(
    image: &Arc<RenderImage>,
    rotation: PdfRotation,
) -> Option<Arc<RenderImage>> {
    if image.frame_count() == 0 {
        return None;
    }
    if rotation == PdfRotation::Zero {
        return Some(image.clone());
    }
    let size = image.size(0);
    // Image operations only rearrange pixels; GPUI's BGRA channel order is preserved.
    let source = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
        u32::try_from(size.width.0).ok()?,
        u32::try_from(size.height.0).ok()?,
        image.as_bytes(0)?,
    )?;
    let rotated = match rotation {
        PdfRotation::Quarter => image::imageops::rotate90(&source),
        PdfRotation::Half => image::imageops::rotate180(&source),
        PdfRotation::ThreeQuarter => image::imageops::rotate270(&source),
        PdfRotation::Zero => return Some(image.clone()),
    };
    Some(Arc::new(RenderImage::new([image::Frame::new(rotated)])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_uses_actual_viewport_after_rotation_and_keeps_aspect() {
        let portrait = PdfPageSize::new(600.0, 900.0);
        let (w, h) = dimensions(
            portrait,
            PdfRotation::Quarter,
            PdfZoom::FitPage,
            (500.0, 300.0),
        );
        assert!(w <= 474.0 && h <= 274.0);
        assert!((w / h - 1.5).abs() < 0.001);
        let (w, h) = dimensions(
            portrait,
            PdfRotation::Zero,
            PdfZoom::FitWidth,
            (326.0, 200.0),
        );
        assert_eq!((w, h), (300.0, 450.0));
        assert_eq!(
            dimensions(
                portrait,
                PdfRotation::Zero,
                PdfZoom::Percent(400),
                (326.0, 200.0)
            ),
            (2400.0, 3600.0)
        );
    }

    #[test]
    fn clockwise_rotation_changes_pixels_not_just_bounds() {
        let pixels = image::RgbaImage::from_raw(2, 1, vec![1, 2, 3, 255, 4, 5, 6, 255]).unwrap();
        let original = Arc::new(RenderImage::new([image::Frame::new(pixels)]));
        let quarter = rotate_image(&original, PdfRotation::Quarter).unwrap();
        assert_eq!(quarter.size(0).width.0, 1);
        assert_eq!(quarter.size(0).height.0, 2);
        assert_eq!(quarter.as_bytes(0).unwrap(), &[1, 2, 3, 255, 4, 5, 6, 255]);
        let half = rotate_image(&original, PdfRotation::Half).unwrap();
        assert_eq!(half.as_bytes(0).unwrap(), &[4, 5, 6, 255, 1, 2, 3, 255]);
    }
}
