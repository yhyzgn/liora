//! Basic remote and local image rendering.

use gpui::IntoElement;
use liora_components::{Image, Space};

pub fn basic_images(local: &'static [u8]) -> impl IntoElement {
    let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";

    Space::new()
        .wrap()
        .gap_md()
        .child(Image::new(remote).thumbnail().cover())
        .child(Image::embedded(local).thumbnail().cover())
        .child(Image::embedded(local).thumbnail().contain())
}
