//! Interactive OtpInput example.

use gpui::{App, AppContext, Entity};
use liora_components::OtpInput;

pub fn otp_input_interactive(cx: &mut App) -> Entity<OtpInput> {
    cx.new(|cx| OtpInput::new("", cx))
}
