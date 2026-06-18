use gpui::{IntoElement, div, prelude::*, rgb};
use liora_components::{Carousel, CarouselItem};

pub fn custom_carousel() -> impl IntoElement {
    Carousel::new(vec![
        CarouselItem::new("Custom content")
            .description("The slot below is another GPUI element.")
            .accent(rgb(0xf97316).into())
            .content(div().rounded_full().px_3().py_1().child("Composable slot")),
    ])
    .show_arrows(false)
    .hide_indicators()
}
