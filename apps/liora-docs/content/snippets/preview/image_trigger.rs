//! Preview wrapping image thumbnails.

use gpui::IntoElement;
use liora_components::{Image, Preview, Space};

pub fn preview_image_trigger() -> impl IntoElement {
    let remote = "https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg";

    Space::new()
        .wrap()
        .gap_md()
        .child(Preview::new(remote).child(Image::new(remote).thumbnail().cover().preview(false)))
}
