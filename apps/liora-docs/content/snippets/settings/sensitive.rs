//! Sensitive SettingsItem states example.

use gpui::{AppContext, Context, Entity, IntoElement};
use liora_components::{Button, SettingsGroup, SettingsItem, SettingsPage, Switch};
use liora_icons_lucide::IconName;

pub struct PrivacyState {
    pub telemetry: Entity<Switch>,
}

pub fn privacy_state(cx: &mut Context<PrivacyState>) -> PrivacyState {
    PrivacyState {
        telemetry: cx.new(|cx| Switch::new(false, cx)),
    }
}

pub fn settings_sensitive(state: &PrivacyState) -> impl IntoElement {
    SettingsPage::new("Sensitive settings").group(
        SettingsGroup::new("Privacy")
            .item(
                SettingsItem::new("Telemetry")
                    .description("Share anonymous product diagnostics.")
                    .icon(IconName::Activity)
                    .control(state.telemetry.clone()),
            )
            .item(
                SettingsItem::new("Delete local cache")
                    .description("Remove generated indexes and temporary package files.")
                    .icon(IconName::Trash2)
                    .danger()
                    .control(Button::new("Delete").danger().small()),
            )
            .item(
                SettingsItem::new("Enterprise policy")
                    .description("Managed by organization policy.")
                    .icon(IconName::Lock)
                    .disabled(true)
                    .control(Button::new("Locked").small().disabled(true)),
            ),
    )
}
