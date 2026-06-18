//! Compare fixed-size image fit modes.

use gpui::{IntoElement, prelude::*};
use liora_components::{Card, Image, ImageFit, Space, Text};

pub fn image_fit_modes() -> impl IntoElement {
    let local = format!("file://{}/assets/local.jpeg", env!("CARGO_MANIFEST_DIR"));

    Space::new().wrap().gap_md().children(
        [
            ("Fill", ImageFit::Fill),
            ("Contain", ImageFit::Contain),
            ("Cover", ImageFit::Cover),
            ("ScaleDown", ImageFit::ScaleDown),
        ]
        .into_iter()
        .map(|(label, fit)| {
            Card::new(
                Space::new()
                    .vertical()
                    .gap_sm()
                    .child(Image::new(local.clone()).thumbnail_sm().fit(fit))
                    .child(Text::new(label).nowrap()),
            )
            .no_shadow()
        }),
    )
}
