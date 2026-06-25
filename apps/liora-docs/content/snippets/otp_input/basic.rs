//! OtpInput examples.

use gpui::IntoElement;
use liora_components::{OtpInput, Space};

pub fn otp_input_basic() -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(OtpInput::new("1284").length(6).active_index(4))
        .child(OtpInput::new("934201").length(6).success())
        .child(OtpInput::new("12 8").length(4).masked(true).error())
}
