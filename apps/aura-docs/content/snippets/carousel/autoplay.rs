use aura_components::{Carousel, CarouselIndicatorPosition, CarouselItem};
use gpui::{IntoElement, px, rgb};

pub fn autoplay_carousel() -> impl IntoElement {
    // autoplay/interval 是受控配置；上层可按 interval 驱动 active_index。
    Carousel::new(vec![
        CarouselItem::new("Preview").accent(rgb(0x2563eb).into()),
        CarouselItem::new("Release").accent(rgb(0x9333ea).into()),
    ])
    .autoplay(true)
    .interval_ms(1800)
    .pause_on_hover(true)
    .indicator_position(CarouselIndicatorPosition::Outside)
    .height(px(220.0))
}
