//! Enable native preview overlay for clickable images.

use liora_components::Image;

pub fn preview_image() -> Image {
    Image::new("https://cube.elemecdn.com/6/94/4d3ea53c084bad6931a56d5158a48jpeg.jpeg")
        .thumbnail()
        .cover()
        .preview(true)
}
