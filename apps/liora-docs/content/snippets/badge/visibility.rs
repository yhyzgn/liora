//! The host retains `hidden` and updates it in `on_toggle`, then notifies its view.

use gpui::{App, ClickEvent, Window};
use liora_components::{Badge, Button, Space};

pub fn badge_visibility(
    hidden: bool,
    on_toggle: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> Space {
    Space::new()
        .wrap()
        .gap_lg()
        .child(Button::new("Toggle badges").on_click(on_toggle))
        .child(Badge::new(Button::new("Count")).value("7").hidden(hidden))
        .child(Badge::new(Button::new("Dot")).is_dot(true).hidden(hidden))
}
