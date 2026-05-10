use aura_components::{Image, ImageFit};
use gpui::prelude::*;

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

#[test]
fn image_supports_local_file_sources() {
    let image = Image::local(std::path::PathBuf::from("/tmp/local.jpeg"));
    assert!(image.source().is_some_and(|source| source.is_file()));
}

#[test]
fn image_supports_remote_url_sources() {
    let image = Image::new("https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg");
    assert!(image.source().is_some_and(|source| source.is_url()));
}

#[test]
fn local_demo_asset_exists() {
    assert!(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../apps/aura-gallery/assets/local.jpeg")
            .exists()
    );
}

#[test]
fn image_supports_file_protocol_sources() {
    let image = Image::new("file:///tmp/local.jpeg");
    assert!(image.source().is_some_and(|source| source.is_file()));
}

#[test]
fn image_supports_round_options() {
    let image = Image::new("https://example.com/image.png")
        .round_options(aura_components::ImageRoundOptions::circle());

    assert_eq!(image.radius_kind(), aura_components::ImageRadius::Round);
    assert_eq!(
        image.round_config(),
        aura_components::ImageRoundOptions::circle()
    );
}

#[test]
fn image_supports_transparent_ring_sleeve() {
    let ring = aura_components::ImageRing::new(gpui::px(4.0), gpui::white().opacity(0.76));
    let image = Image::new("https://example.com/image.png").round_ring(ring);

    assert_eq!(image.radius_kind(), aura_components::ImageRadius::Round);
    assert_eq!(image.round_config().ring, Some(ring));
}

#[test]
fn preview_builder_tracks_source_and_trigger() {
    let preview = aura_components::Preview::new("https://example.com/full.png")
        .child(gpui::div().child("Open preview"));

    assert!(preview.source().is_some_and(|source| source.is_url()));
    assert!(preview.has_trigger());
}

#[test]
fn image_preview_builder_still_tracks_enabled_state() {
    let image = Image::new("https://example.com/image.png").preview(true);

    assert!(image.preview_enabled());
}
