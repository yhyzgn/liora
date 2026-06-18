//! Shape, empty, and fallback states for image content.

use gpui::{IntoElement, prelude::*};
use liora_components::{Image, ImageRadius, ImageRoundOptions, Space, Text};

pub fn image_states() -> impl IntoElement {
    let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

    Space::new()
        .wrap()
        .gap_md()
        .child(labeled(
            Image::new(local.clone()).square_lg().cover().round(),
            "Circle",
        ))
        .child(labeled(
            Image::new(local.clone())
                .thumbnail_sm()
                .cover()
                .round_options(ImageRoundOptions::without_square_crop()),
            "Round bounds",
        ))
        .child(labeled(
            Image::new(local.clone()).square_lg().cover().round_sleeve(),
            "Ring sleeve",
        ))
        .child(labeled(
            Image::new(local)
                .thumbnail()
                .cover()
                .radius(ImageRadius::Large)
                .shadow(true),
            "Shadow",
        ))
        .child(labeled(
            Image::new("liora://missing-image.png")
                .thumbnail()
                .alt("加载失败"),
            "Fallback",
        ))
        .child(labeled(Image::empty().thumbnail(), "Empty"))
}

fn labeled(image: Image, label: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .align_center()
        .gap_sm()
        .child(image)
        .child(Text::new(label).nowrap())
}
