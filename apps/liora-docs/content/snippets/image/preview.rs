//! Enable native preview overlay for clickable images.

use liora_components::Image;

pub fn preview_image(local: &'static [u8]) -> Image {
    Image::embedded(local).thumbnail().cover().preview(true)
}
