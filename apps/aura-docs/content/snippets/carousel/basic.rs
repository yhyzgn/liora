use aura_components::{Carousel, CarouselItem};
use gpui::{IntoElement, px, rgb};

pub fn basic_carousel() -> impl IntoElement {
    Carousel::new(vec![
        CarouselItem::new("Native Rust UI")
            .description("Pure GPUI rendering with Aura components.")
            .accent(rgb(0x2563eb).into()),
        CarouselItem::new("Charts & Dashboards")
            .description("Native chart primitives for desktop apps.")
            .accent(rgb(0x16a34a).into()),
    ])
    .height(px(240.0))
}
