//! SettingsPage composition example.

use gpui::{AppContext, Context, Entity, IntoElement, px};
use liora_components::{Button, Input, Select, SettingsGroup, SettingsItem, SettingsPage, Switch};
use liora_icons_lucide::IconName;

pub struct SettingsState {
    pub auto_save: Entity<Switch>,
    pub theme: Entity<Select>,
    pub font_size: Entity<Input>,
}

pub fn settings_state(cx: &mut Context<SettingsState>) -> SettingsState {
    SettingsState {
        auto_save: cx.new(|cx| Switch::new(true, cx)),
        theme: cx.new(|cx| Select::new(vec!["System", "Light", "Dark"], Some(0), cx)),
        font_size: cx.new(|cx| Input::new("14", cx).width(px(88.0))),
    }
}

pub fn settings_page(state: &SettingsState) -> impl IntoElement {
    SettingsPage::new("Application Settings")
        .description("Settings rows can host Switch, Select, Input, Button, or custom content.")
        .group(
            SettingsGroup::new("Editor")
                .description("Editing and save behavior")
                .item(
                    SettingsItem::new("Auto save")
                        .description("Save files when focus leaves the editor.")
                        .icon(IconName::Save)
                        .control(state.auto_save.clone())
                        .primary(),
                )
                .item(
                    SettingsItem::new("Font size")
                        .description("Controls editor UI font size.")
                        .icon(IconName::CaseSensitive)
                        .control(state.font_size.clone()),
                ),
        )
        .group(
            SettingsGroup::new("Appearance")
                .item(
                    SettingsItem::new("Theme mode")
                        .description("Follow system or force a light/dark theme.")
                        .icon(IconName::Palette)
                        .control(state.theme.clone()),
                )
                .item(
                    SettingsItem::new("Preview")
                        .description("Open a small preview action.")
                        .control(Button::new("Preview").small()),
                ),
        )
}
