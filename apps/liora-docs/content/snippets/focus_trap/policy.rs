use gpui::IntoElement;
use liora_components::{FocusTrap, Text};

pub fn focus_trap_policy() -> impl IntoElement {
    let policy = FocusTrap::new().restore_focus(true).close_on_escape(false);
    Text::new(format!(
        "trap={}, restore={}, esc={}",
        policy.enabled, policy.restore_focus, policy.close_on_escape
    ))
}
