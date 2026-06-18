//! Basic remote and local image rendering.

use gpui::IntoElement;
use liora_components::{Image, Space};

pub fn basic_images() -> impl IntoElement {
    let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";
    let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

    Space::new()
        .wrap()
        .gap_md()
        .child(Image::new(remote).thumbnail().cover())
        .child(Image::new(local.clone()).thumbnail().cover())
        .child(Image::new(local).thumbnail().contain())
}
