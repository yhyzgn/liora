//! Live demo marker resolving to a real native Button node.

use liora_components::{Button, toast_success};

fn live_button() -> Button {
    Button::new("Native Button").primary().on_click(|_, _, _| {
        // Toast macros dispatch through the global MessageManager.
        toast_success!("Live demo clicked: {}", "Button");
    })
}

fn main() {
    let _ = live_button();
}
