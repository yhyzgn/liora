//! Shape, empty, and fallback states for image content.

use gpui::{IntoElement, prelude::*};
use liora_components::{Image, ImageRadius, ImageRoundOptions, Space, Text};

pub fn image_states(local: &'static [u8]) -> impl IntoElement {
    Space::new()
        .wrap()
        .gap_md()
        .child(labeled(
            Image::embedded(local).square_lg().cover().round(),
            "Circle",
        ))
        .child(labeled(
            Image::embedded(local)
                .thumbnail_sm()
                .cover()
                .round_options(ImageRoundOptions::without_square_crop()),
            "Round bounds",
        ))
        .child(labeled(
            Image::embedded(local).square_lg().cover().round_sleeve(),
            "Ring sleeve",
        ))
        .child(labeled(
            Image::embedded(local)
                .thumbnail()
                .cover()
                .radius(ImageRadius::Large)
                .shadow(true),
            "Shadow",
        ))
        .child(labeled(
            Image::new("http://127.0.0.1:0/missing-image.png")
                .thumbnail()
                .alt("加载失败")
                .placeholder(|| Text::new("Loading image…"))
                .fallback(|| Text::new("Image unavailable")),
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
