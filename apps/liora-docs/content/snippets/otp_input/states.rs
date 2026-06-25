//! OtpInput state examples.

use gpui::{App, AppContext, Entity};
use liora_components::{OtpInput, Space};

pub fn otp_input_states(cx: &mut App) -> impl gpui::IntoElement {
    let success: Entity<OtpInput> = cx.new(|cx| OtpInput::new("934201", cx).success());
    let error: Entity<OtpInput> = cx.new(|cx| OtpInput::new("128", cx).length(4, cx).error());

    Space::new().vertical().gap_md().child(success).child(error)
}
