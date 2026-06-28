use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::layout_helpers::{page, section, showcase_card_wide, showcase_grid};
use liora_components::{
    Button, Input, Select, SettingsGroup, SettingsItem, SettingsPage, Space, Switch, Text,
};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| SettingsDemo::new(cx)).into()
}

struct SettingsDemo {
    auto_save: gpui::Entity<Switch>,
    telemetry: gpui::Entity<Switch>,
    theme: gpui::Entity<Select>,
    font_size: gpui::Entity<Input>,
}

impl SettingsDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            auto_save: cx.new(|cx| Switch::new(true, cx)),
            telemetry: cx.new(|cx| Switch::new(false, cx)),
            theme: cx.new(|cx| Select::new(vec!["System", "Light", "Dark"], Some(0), cx)),
            font_size: cx.new(|cx| Input::new("14", cx).width(px(88.0))),
        }
    }
}

impl Render for SettingsDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page("Settings UI 设置页", "SettingsPage / SettingsGroup / SettingsItem 统一桌面设置页面模式，承载 Switch、Select、Input、Button 等已有控件。",
            Space::new().vertical().gap_xl().child(section(
                "Settings showcase",
                "设置页示例统一使用宽卡片展示，保持桌面设置页面的阅读宽度和层级。",
                showcase_grid(vec![
                    showcase_card_wide(
                        "完整设置页",
                        "设置页不是新的表单校验系统，而是 app settings 的信息架构和视觉布局。",
                        SettingsPage::new("Application Settings")
                            .description("All rows use Liora theme tokens and can host any component as control or extra content.")
                            .group(SettingsGroup::new("Editor").description("Editing and save behavior")
                                .item(SettingsItem::new("Auto save").description("Save files when focus leaves the editor.").icon(IconName::Save).control(self.auto_save.clone()).primary())
                                .item(SettingsItem::new("Font size").description("Controls editor UI font size.").icon(IconName::CaseSensitive).control(self.font_size.clone()).extra(Text::new("Recommended range: 12-18 px").xs()))
                            )
                            .group(SettingsGroup::new("Appearance")
                                .item(SettingsItem::new("Theme mode").description("Follow system or force a light/dark theme.").icon(IconName::Palette).control(self.theme.clone()))
                                .item(SettingsItem::new("Compact rows").description("Dense setting rows for advanced panels.").icon(IconName::Rows3).compact().control(Button::new("Preview").small()))
                            ),
                    )
                    .into_any_element(),
                    showcase_card_wide(
                        "敏感设置",
                        "Danger tone 用于破坏性或隐私敏感选项，disabled 表达不可用策略。",
                        SettingsPage::new("Sensitive settings")
                            .max_width(px(720.0))
                            .group(SettingsGroup::new("Privacy")
                                .item(SettingsItem::new("Telemetry").description("Share anonymous product diagnostics.").icon(IconName::Activity).control(self.telemetry.clone()))
                                .item(SettingsItem::new("Delete local cache").description("Remove generated indexes and temporary package files.").icon(IconName::Trash2).danger().control(Button::new("Delete").danger().small()))
                                .item(SettingsItem::new("Enterprise policy").description("Managed by organization policy.").icon(IconName::Lock).disabled(true).control(Button::new("Locked").small().disabled(true)))),
                    )
                    .into_any_element(),
                ]),
            ))
        )
    }
}
