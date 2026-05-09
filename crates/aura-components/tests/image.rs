use aura_components::{Image, ImageFit};

#[test]
fn image_defaults_to_contain_fit() {
    let image = Image::new("https://example.com/image.png");
    assert_eq!(image.fit_kind(), ImageFit::Contain);
}

#[test]
fn image_builder_tracks_dimensions_and_fit() {
    let image = Image::new("https://example.com/image.png")
        .square(gpui::px(88.0))
        .cover();

    assert_eq!(image.fit_kind(), ImageFit::Cover);
    assert_eq!(
        image.dimensions(),
        (Some(gpui::px(88.0)), Some(gpui::px(88.0)))
    );
}

#[test]
fn image_empty_has_no_dimensions_until_configured() {
    let image = Image::empty().fill();
    assert_eq!(image.fit_kind(), ImageFit::Fill);
    assert_eq!(image.dimensions(), (None, None));
}
