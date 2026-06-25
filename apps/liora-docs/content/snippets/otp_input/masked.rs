//! Masked OtpInput example.

use gpui::{App, AppContext, Entity};
use liora_components::OtpInput;

pub fn otp_input_masked(cx: &mut App) -> Entity<OtpInput> {
    cx.new(|cx| OtpInput::new("42", cx).length(4, cx).masked(true))
}
