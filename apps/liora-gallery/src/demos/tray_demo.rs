use crate::tray_menu::gallery_tray_menu;
use gpui::{AnyElement, AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, Card, Flex, Space, Tag, Text};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use liora_theme::Theme;
use liora_tray::{TrayCloseAction, TrayCommand, TrayControlCenter, TrayMenuItemSpec};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| TrayDemo {
        active_icon: "default",
        resident_enabled: true,
        tray_visible: true,
        auto_show: true,
        remembered_close_action: TrayCloseAction::Ask,
    })
    .into()
}

struct TrayDemo {
    active_icon: &'static str,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    remembered_close_action: TrayCloseAction,
}

impl Render for TrayDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();
        let theme = cx.global::<Config>().theme.clone();
        let state = if cx.has_global::<TrayControlCenter>() {
            Some(cx.global::<TrayControlCenter>().state.clone())
        } else {
            None
        };
        if let Some(state) = &state {
            self.active_icon = match state.active_icon.as_str() {
                "syncing" => "syncing",
                "error" => "error",
                _ => "default",
            };
            self.resident_enabled = state.resident_enabled;
            self.tray_visible = state.tray_visible;
            self.auto_show = state.auto_show;
            self.remembered_close_action = state.remembered_close_action;
        }

        page(
            "Tray 系统托盘",
            "liora-tray 封装 tray-icon/muda，展示真实托盘驻留、动态图标、CheckBox 菜单、窗口显隐命令和关闭到托盘流程。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "真实托盘效果预览",
                    "这个示例不是普通文档列表：左侧是应用后台状态，底部是系统状态栏托盘区域，右侧是由 Gallery 应用本地 tray_menu::gallery_tray_menu() 生成的托盘弹出菜单；按钮会 dispatch 真实 TrayCommand。",
                    desktop_tray_showcase(
                        &theme,
                        self.active_icon,
                        self.resident_enabled,
                        self.tray_visible,
                        self.auto_show,
                        self.remembered_close_action,
                        entity.clone(),
                    ),
                ))
                .child(section(
                    "驻留与关闭策略",
                    "实际应用中，关闭窗口不等于退出进程。启用驻留后 close action 会隐藏主窗口并保留状态栏托盘入口；关闭驻留时应由应用退出动作调用 cx.quit()。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(residency_preview(self.resident_enabled, self.tray_visible))
                        .child(close_confirm_preview(
                            self.remembered_close_action,
                            entity.clone(),
                        )),
                ))
                .child(section(
                    "TrayMenuItemSpec 与命令映射",
                    "同一套声明式菜单会同时驱动平台 tray-icon/muda 后端和这里的可视化预览；MenuEvent id 会映射回稳定 TrayCommand。",
                    command_table(&theme),
                )),
        )
    }
}

fn desktop_tray_showcase(
    theme: &Theme,
    active_icon: &'static str,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    remembered_close_action: TrayCloseAction,
    entity: Entity<TrayDemo>,
) -> impl IntoElement {
    Card::new(
        Flex::new()
            .column()
            .gap_lg()
            .padding_lg()
            .rounded_units(22.0)
            .border()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.body)
            .child(
                Flex::new()
                    .row()
                    .wrap()
                    .gap_lg()
                    .align_start()
                    .child(app_background_panel(
                        theme,
                        active_icon,
                        resident_enabled,
                        tray_visible,
                        auto_show,
                        remembered_close_action,
                        entity.clone(),
                    ))
                    .child(tray_menu_panel(theme, &gallery_tray_menu(), entity.clone())),
            )
            .child(tray_status_bar(
                theme,
                active_icon,
                resident_enabled,
                tray_visible,
                auto_show,
                entity,
            )),
    )
    .no_shadow()
}

fn app_background_panel(
    theme: &Theme,
    active_icon: &'static str,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    remembered_close_action: TrayCloseAction,
    entity: Entity<TrayDemo>,
) -> Flex {
    Flex::new()
        .column()
        .gap_md()
        .width_px(420.0)
        .padding_lg()
        .rounded_units(18.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.card)
        .child(
            Flex::new()
                .row()
                .align_center()
                .justify_between()
                .gap_md()
                .child(
                    Flex::new()
                        .row()
                        .align_center()
                        .gap_md()
                        .child(tray_icon_badge(theme, active_icon, true))
                        .child(
                            Flex::new()
                                .column()
                                .gap_px(4.0)
                                .child(Text::new("Liora Gallery 后台运行中").bold())
                                .child(
                                    Text::new(
                                        "这里模拟真实应用关闭窗口后仍保留的 resident tray 状态。",
                                    )
                                    .sm()
                                    .text_color(theme.neutral.text_3),
                                ),
                        ),
                )
                .child(active_icon_tag(active_icon)),
        )
        .child(
            Flex::new()
                .row()
                .wrap()
                .gap_sm()
                .child(state_chip(
                    theme,
                    "驻留",
                    resident_enabled,
                    theme.success.base,
                ))
                .child(state_chip(theme, "托盘可见", tray_visible, theme.info.base))
                .child(state_chip(
                    theme,
                    "点击自动显示",
                    auto_show,
                    theme.warning.base,
                ))
                .child(close_action_chip(theme, remembered_close_action)),
        )
        .child(
            Flex::new()
                .row()
                .wrap()
                .gap_sm()
                .child(icon_button("默认图标", "default", entity.clone()))
                .child(icon_button("同步中", "syncing", entity.clone()))
                .child(icon_button("错误态", "error", entity.clone()))
                .child(tray_command_button(
                    "显示主窗口",
                    TrayCommand::Show,
                    entity.clone(),
                ))
                .child(tray_command_button(
                    "隐藏到托盘",
                    TrayCommand::Hide,
                    entity.clone(),
                ))
                .child(tray_command_button("切换窗口", TrayCommand::Toggle, entity)),
        )
}

fn tray_status_bar(
    theme: &Theme,
    active_icon: &'static str,
    resident_enabled: bool,
    tray_visible: bool,
    auto_show: bool,
    entity: Entity<TrayDemo>,
) -> Flex {
    Flex::new()
        .row()
        .wrap()
        .align_center()
        .justify_between()
        .gap_md()
        .padding_md()
        .rounded_units(18.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.popover)
        .child(
            Flex::new()
                .row()
                .align_center()
                .gap_sm()
                .child(
                    Icon::new(IconName::Wifi)
                        .size_units(15.0)
                        .color(theme.neutral.icon),
                )
                .child(Text::new("System tray area").sm().bold())
                .child(Text::new("12:42").sm().text_color(theme.neutral.text_3)),
        )
        .child(
            Flex::new()
                .row()
                .wrap()
                .align_center()
                .gap_sm()
                .child(
                    Text::new("托盘图标：")
                        .sm()
                        .text_color(theme.neutral.text_3),
                )
                .child(tray_icon_badge(theme, active_icon, tray_visible))
                .child(toggle_resident_button(resident_enabled, entity.clone()))
                .child(toggle_tray_visible_button(tray_visible, entity.clone()))
                .child(toggle_auto_show_button(auto_show, entity)),
        )
}

fn tray_menu_panel(
    theme: &Theme,
    specs: &[TrayMenuItemSpec],
    entity: Entity<TrayDemo>,
) -> impl IntoElement {
    Flex::new()
        .column()
        .gap_px(4.0)
        .width_px(360.0)
        .padding_sm()
        .rounded_units(18.0)
        .border()
        .border_color(theme.neutral.border)
        .bg(theme.neutral.popover)
        .child(
            Flex::new()
                .row()
                .align_center()
                .justify_between()
                .padding_x_units(10.0)
                .padding_y_px(8.0)
                .child(
                    Flex::new()
                        .row()
                        .align_center()
                        .gap_sm()
                        .child(tray_icon_badge(theme, "default", true))
                        .child(
                            Flex::new()
                                .column()
                                .gap_px(2.0)
                                .child(Text::new("Liora Gallery").bold())
                                .child(
                                    Text::new("platform tray menu")
                                        .xs()
                                        .text_color(theme.neutral.text_3),
                                ),
                        ),
                )
                .child(Tag::new("live spec").small().info()),
        )
        .children(
            specs
                .iter()
                .map(|spec| render_tray_menu_item(spec, 0, theme, entity.clone())),
        )
}

fn render_tray_menu_item(
    spec: &TrayMenuItemSpec,
    depth: usize,
    theme: &Theme,
    entity: Entity<TrayDemo>,
) -> AnyElement {
    match spec {
        TrayMenuItemSpec::Action {
            label,
            command,
            enabled,
        } => tray_action_row(theme, depth, label, command.clone(), *enabled, entity)
            .into_any_element(),
        TrayMenuItemSpec::Check {
            label,
            command,
            checked,
            enabled,
        } => tray_check_row(
            theme,
            depth,
            label,
            command.clone(),
            *checked,
            *enabled,
            entity,
        )
        .into_any_element(),
        TrayMenuItemSpec::Submenu {
            label,
            children,
            enabled,
        } => Flex::new()
            .column()
            .gap_px(4.0)
            .child(tray_submenu_row(
                theme,
                depth,
                label,
                children.len(),
                *enabled,
            ))
            .children(
                children
                    .iter()
                    .map(|child| render_tray_menu_item(child, depth + 1, theme, entity.clone())),
            )
            .into_any_element(),
        TrayMenuItemSpec::Separator => tray_separator(theme, depth).into_any_element(),
    }
}

fn tray_action_row(
    theme: &Theme,
    depth: usize,
    label: &str,
    command: TrayCommand,
    enabled: bool,
    entity: Entity<TrayDemo>,
) -> Flex {
    tray_row_shell(theme, depth, enabled)
        .child(command_icon(theme, &command, enabled))
        .child(
            Flex::new()
                .column()
                .gap_px(2.0)
                .flex_1()
                .child(Text::new(label.to_string()).sm().text_color(if enabled {
                    theme.neutral.text_1
                } else {
                    theme.neutral.text_disabled
                }))
                .child(
                    Text::new(command.id())
                        .xs()
                        .text_color(theme.neutral.text_3)
                        .nowrap(),
                ),
        )
        .child(menu_trigger_button("触发", command, enabled, entity))
}

fn tray_check_row(
    theme: &Theme,
    depth: usize,
    label: &str,
    command: TrayCommand,
    checked: bool,
    enabled: bool,
    entity: Entity<TrayDemo>,
) -> Flex {
    tray_row_shell(theme, depth, enabled)
        .child(
            Icon::new(if checked {
                IconName::Check
            } else {
                IconName::X
            })
            .size_units(15.0)
            .color(if checked {
                theme.success.base
            } else {
                theme.neutral.text_3
            }),
        )
        .child(
            Flex::new()
                .column()
                .gap_px(2.0)
                .flex_1()
                .child(Text::new(label.to_string()).sm().text_color(if enabled {
                    theme.neutral.text_1
                } else {
                    theme.neutral.text_disabled
                }))
                .child(
                    Text::new(if checked { "checked" } else { "unchecked" })
                        .xs()
                        .text_color(theme.neutral.text_3),
                ),
        )
        .child(menu_trigger_button("切换", command, enabled, entity))
}

fn tray_submenu_row(
    theme: &Theme,
    depth: usize,
    label: &str,
    child_count: usize,
    enabled: bool,
) -> Flex {
    tray_row_shell(theme, depth, enabled)
        .child(
            Icon::new(IconName::Settings)
                .size_units(15.0)
                .color(if enabled {
                    theme.neutral.icon
                } else {
                    theme.neutral.text_disabled
                }),
        )
        .child(
            Flex::new()
                .column()
                .gap_px(2.0)
                .flex_1()
                .child(
                    Text::new(label.to_string())
                        .sm()
                        .bold()
                        .text_color(if enabled {
                            theme.neutral.text_1
                        } else {
                            theme.neutral.text_disabled
                        }),
                )
                .child(
                    Text::new(format!("{child_count} nested items"))
                        .xs()
                        .text_color(theme.neutral.text_3),
                ),
        )
        .child(
            Icon::new(IconName::ChevronRight)
                .size_units(15.0)
                .color(theme.neutral.text_3),
        )
}

fn tray_row_shell(theme: &Theme, depth: usize, enabled: bool) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .gap_sm()
        .padding_x_units(10.0)
        .padding_y_px(7.0)
        .rounded_units(12.0)
        .bg(if enabled {
            theme.neutral.popover
        } else {
            theme.neutral.hover
        })
        .child(depth_spacer(depth))
}

fn depth_spacer(depth: usize) -> Flex {
    Flex::new()
        .flex_none()
        .width_px((depth as f32 * 18.0).min(72.0))
}

fn tray_separator(theme: &Theme, depth: usize) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .gap_sm()
        .padding_x_units(10.0)
        .padding_y_px(4.0)
        .child(depth_spacer(depth))
        .child(
            Flex::new()
                .height_px(1.0)
                .flex_1()
                .bg(theme.neutral.divider),
        )
}

fn menu_trigger_button(
    label: &'static str,
    command: TrayCommand,
    enabled: bool,
    entity: Entity<TrayDemo>,
) -> Button {
    let command_id = command.id();
    let mut button = Button::new(label)
        .id(gpui::ElementId::from(format!(
            "tray-preview-command-{}",
            sanitize_command_id(&command_id)
        )))
        .small();
    if !enabled {
        button = button.disabled(true);
    }
    button.on_click(move |_, _, cx| {
        dispatch_preview_tray_command(cx, command.clone(), entity.clone());
    })
}

fn command_icon(theme: &Theme, command: &TrayCommand, enabled: bool) -> Icon {
    let (icon, color) = match command {
        TrayCommand::Show => (IconName::Eye, theme.success.base),
        TrayCommand::Hide => (IconName::EyeOff, theme.warning.base),
        TrayCommand::Toggle => (IconName::RefreshCw, theme.info.base),
        TrayCommand::Quit => (IconName::Power, theme.danger.base),
        TrayCommand::SetIcon(_) => (IconName::Activity, theme.primary.base),
        TrayCommand::Custom(_) => (IconName::Settings, theme.neutral.icon),
    };
    Icon::new(icon).size_units(15.0).color(if enabled {
        color
    } else {
        theme.neutral.text_disabled
    })
}

fn active_icon_tag(name: &str) -> Tag {
    match name {
        "syncing" => Tag::new("↻ syncing").warning().large(),
        "error" => Tag::new("! error").danger().large(),
        _ => Tag::new("default").success().large(),
    }
}

fn tray_icon_badge(theme: &Theme, name: &str, visible: bool) -> Flex {
    let (glyph, bg, fg) = match name {
        "syncing" => ("↻", theme.warning.light_9, theme.warning.base),
        "error" => ("!", theme.danger.light_9, theme.danger.base),
        _ => ("L", theme.primary.light_9, theme.primary.base),
    };

    Flex::new()
        .row()
        .align_center()
        .justify_center()
        .width_px(34.0)
        .height_px(34.0)
        .rounded_units(11.0)
        .border()
        .border_color(if visible {
            fg.opacity(0.40)
        } else {
            theme.neutral.border
        })
        .bg(if visible { bg } else { theme.neutral.hover })
        .child(
            Text::new(if visible { glyph } else { "·" })
                .bold()
                .text_color(if visible { fg } else { theme.neutral.text_3 }),
        )
}

fn state_chip(theme: &Theme, label: &'static str, active: bool, color: gpui::Hsla) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .gap_px(6.0)
        .padding_x_units(9.0)
        .padding_y_px(5.0)
        .rounded_pill()
        .border()
        .border_color(if active {
            color.opacity(0.36)
        } else {
            theme.neutral.border
        })
        .bg(if active {
            color.opacity(0.12)
        } else {
            theme.neutral.hover
        })
        .child(
            Icon::new(if active { IconName::Check } else { IconName::X })
                .size_units(12.0)
                .color(if active { color } else { theme.neutral.text_3 }),
        )
        .child(
            Text::new(format!("{label}: {}", if active { "on" } else { "off" }))
                .xs()
                .bold()
                .text_color(if active { color } else { theme.neutral.text_3 }),
        )
}

fn close_action_chip(theme: &Theme, action: TrayCloseAction) -> Flex {
    let (label, color) = match action {
        TrayCloseAction::Ask => ("关闭: ask", theme.info.base),
        TrayCloseAction::ExitProcess => ("关闭: exit", theme.danger.base),
        TrayCloseAction::HideToTray => ("关闭: hide-to-tray", theme.success.base),
    };
    Flex::new()
        .row()
        .align_center()
        .padding_x_units(9.0)
        .padding_y_px(5.0)
        .rounded_pill()
        .border()
        .border_color(color.opacity(0.36))
        .bg(color.opacity(0.12))
        .child(Text::new(label).xs().bold().text_color(color))
}

fn icon_button(label: &'static str, icon: &'static str, entity: Entity<TrayDemo>) -> Button {
    Button::new(label)
        .primary()
        .small()
        .on_click(move |_, _, cx| {
            dispatch_live_tray_command(cx, TrayCommand::SetIcon(icon.into()));
            let _ = entity.update(cx, |demo, cx| {
                demo.active_icon = icon;
                cx.notify();
            });
        })
}

fn tray_command_button(
    label: &'static str,
    command: TrayCommand,
    entity: Entity<TrayDemo>,
) -> Button {
    Button::new(label).small().on_click(move |_, _, cx| {
        dispatch_preview_tray_command(cx, command.clone(), entity.clone());
    })
}

fn toggle_auto_show_button(auto_show: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if auto_show {
        "关闭 Auto Show"
    } else {
        "开启 Auto Show"
    })
    .small()
    .on_click(move |_, _, cx| {
        dispatch_live_tray_command(cx, TrayCommand::Custom("auto-show".into()));
        let _ = entity.update(cx, |demo, cx| {
            demo.auto_show = !demo.auto_show;
            cx.notify();
        });
    })
}

fn toggle_resident_button(resident_enabled: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if resident_enabled {
        "关闭驻留"
    } else {
        "开启驻留"
    })
    .small()
    .warning()
    .on_click(move |_, _, cx| {
        dispatch_live_tray_command(cx, TrayCommand::Custom("resident-enabled".into()));
        let _ = entity.update(cx, |demo, cx| {
            demo.resident_enabled = !demo.resident_enabled;
            demo.tray_visible = demo.resident_enabled;
            cx.notify();
        });
    })
}

fn toggle_tray_visible_button(tray_visible: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if tray_visible {
        "隐藏托盘"
    } else {
        "显示托盘"
    })
    .small()
    .on_click(move |_, _, cx| {
        dispatch_live_tray_command(cx, TrayCommand::Custom("tray-visible".into()));
        let _ = entity.update(cx, |demo, cx| {
            demo.tray_visible = !demo.tray_visible;
            if demo.tray_visible {
                demo.resident_enabled = true;
            }
            cx.notify();
        });
    })
}

fn residency_preview(resident_enabled: bool, tray_visible: bool) -> impl IntoElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_xs()
            .child(Text::new(format!(
                "close_policy = {}",
                if resident_enabled {
                    "hide-to-tray"
                } else {
                    "exit-on-close"
                }
            )))
            .child(Text::new(format!(
                "tray.set_visible({})",
                if tray_visible { "true" } else { "false" }
            )))
            .child(Text::new(if resident_enabled {
                "关闭窗口后保留托盘入口，可从状态栏恢复。"
            } else {
                "关闭托盘驻留后，用户退出动作应调用 cx.quit()。"
            })),
    )
    .no_shadow()
}

fn close_confirm_preview(
    remembered_close_action: TrayCloseAction,
    entity: Entity<TrayDemo>,
) -> impl IntoElement {
    let remembered = match remembered_close_action {
        TrayCloseAction::Ask => "每次询问",
        TrayCloseAction::ExitProcess => "已记住：关闭进程",
        TrayCloseAction::HideToTray => "已记住：关闭窗口并驻留托盘",
    };

    Card::new(
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new("窗口关闭时：退出进程 / 隐藏到托盘").bold())
            .child(Text::new(format!("当前策略：{remembered}")))
            .child(
                Space::new()
                    .gap_md()
                    .wrap()
                    .child(close_action_button(
                        "恢复每次询问",
                        TrayCloseAction::Ask,
                        entity.clone(),
                    ))
                    .child(close_action_button(
                        "记住关闭进程",
                        TrayCloseAction::ExitProcess,
                        entity.clone(),
                    ))
                    .child(close_action_button(
                        "记住隐藏到托盘",
                        TrayCloseAction::HideToTray,
                        entity,
                    )),
            ),
    )
    .no_shadow()
}

fn close_action_button(
    label: &'static str,
    action: TrayCloseAction,
    entity: Entity<TrayDemo>,
) -> Button {
    Button::new(label).small().on_click(move |_, _, cx| {
        if cx.has_global::<TrayControlCenter>() {
            cx.global_mut::<TrayControlCenter>()
                .set_remembered_close_action(action);
        }
        let _ = entity.update(cx, |demo, cx| {
            demo.remembered_close_action = action;
            cx.notify();
        });
    })
}

fn command_table(theme: &Theme) -> impl IntoElement {
    let commands = [
        TrayCommand::Show,
        TrayCommand::Hide,
        TrayCommand::Toggle,
        TrayCommand::SetIcon("syncing".into()),
        TrayCommand::Custom("deep-action".into()),
        TrayCommand::Quit,
    ];

    Card::new(
        Flex::new()
            .column()
            .gap_sm()
            .children(commands.map(|command| command_mapping_row(theme, command))),
    )
    .no_shadow()
}

fn command_mapping_row(theme: &Theme, command: TrayCommand) -> Flex {
    Flex::new()
        .row()
        .align_center()
        .gap_sm()
        .padding_sm()
        .rounded_units(12.0)
        .bg(theme.neutral.hover)
        .child(command_icon(theme, &command, true))
        .child(
            Text::new(format!("{command:?}"))
                .sm()
                .bold()
                .text_color(theme.neutral.text_1),
        )
        .child(
            Text::new(format!("→ {}", command.id()))
                .sm()
                .text_color(theme.neutral.text_3)
                .nowrap(),
        )
}

fn dispatch_preview_tray_command(cx: &mut App, command: TrayCommand, entity: Entity<TrayDemo>) {
    let _ = entity.update(cx, |demo, cx| {
        match command {
            TrayCommand::Show => {
                demo.auto_show = true;
            }
            TrayCommand::Hide => {
                demo.resident_enabled = true;
                demo.tray_visible = true;
            }
            TrayCommand::Toggle => {
                demo.auto_show = !demo.auto_show;
            }
            TrayCommand::Quit => {
                demo.remembered_close_action = TrayCloseAction::ExitProcess;
            }
            TrayCommand::SetIcon(name) => {
                demo.active_icon = match name.as_str() {
                    "syncing" => "syncing",
                    "error" => "error",
                    _ => "default",
                };
            }
            TrayCommand::Custom(name) if name == "auto-show" => {
                demo.auto_show = !demo.auto_show;
            }
            TrayCommand::Custom(name) if name == "resident-enabled" => {
                demo.resident_enabled = !demo.resident_enabled;
                demo.tray_visible = demo.resident_enabled;
            }
            TrayCommand::Custom(name) if name == "tray-visible" => {
                demo.tray_visible = !demo.tray_visible;
                if demo.tray_visible {
                    demo.resident_enabled = true;
                }
            }
            TrayCommand::Custom(_) => {}
        }
        cx.notify();
    });
}

fn dispatch_live_tray_command(cx: &mut App, command: TrayCommand) {
    if cx.has_global::<TrayControlCenter>() {
        cx.global::<TrayControlCenter>().dispatch(command);
    }
}

fn sanitize_command_id(id: &str) -> String {
    id.chars()
        .map(|ch| if ch.is_ascii_alphanumeric() { ch } else { '-' })
        .collect()
}

#[cfg(test)]
mod tests {
    #[test]
    fn tray_demo_renders_realistic_tray_surface_and_menu_panel() {
        let source = include_str!("tray_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();

        assert!(
            source.contains("fn desktop_tray_showcase"),
            "tray demo should include a desktop/system-tray style visual surface"
        );
        assert!(
            source.contains("fn tray_menu_panel"),
            "tray demo should render the actual tray menu as a popover-like menu panel"
        );
        assert!(
            source.contains("fn tray_status_bar"),
            "tray demo should show where the tray icon lives in the OS status area"
        );
        assert!(
            source.contains("cx.global::<TrayControlCenter>().dispatch")
                || source.contains("cx.global::<TrayControlCenter>()\n                .dispatch"),
            "demo actions should dispatch real TrayCommand values through TrayControlCenter"
        );
        assert!(source.contains("theme.neutral.popover"));
        assert!(source.contains("theme.neutral.border"));
        assert!(source.contains("theme.neutral.text_1"));
        assert!(
            !source.contains("fn indent("),
            "menu indentation should be layout-driven, not string-space driven"
        );
        assert!(
            !source.contains("Card::new(\n        Space::new()\n            .vertical()\n            .gap_px(4.0)\n            .child(\n                Space::new()\n                    .gap_sm()\n                    .child(Tag::new(kind).small())"),
            "tray menu rows should not be rendered as stacked Cards because that does not resemble platform tray menus"
        );
    }

    #[test]
    fn tray_demo_preview_actions_do_not_control_the_real_window_or_process() {
        let source = include_str!("tray_demo.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();

        assert!(
            source.contains("fn dispatch_preview_tray_command"),
            "preview buttons should update the demo model without dispatching destructive window/process commands"
        );
        assert!(
            source.contains("fn dispatch_live_tray_command"),
            "live controls should keep an explicit path for commands that intentionally touch the real tray"
        );
        assert!(
            source.contains("tray-preview-command-"),
            "preview menu controls should use stable unique ids derived from command ids"
        );

        let preview_dispatch = source
            .split("fn dispatch_preview_tray_command")
            .nth(1)
            .expect("preview dispatch helper should exist")
            .split("fn dispatch_live_tray_command")
            .next()
            .expect("preview dispatch helper should be before live dispatch helper");
        assert!(!preview_dispatch.contains(".dispatch(command"));
        assert!(!preview_dispatch.contains("TrayCommand::Quit => cx.quit()"));
        assert!(!preview_dispatch.contains("TrayCommand::Hide => hide_gallery_window"));
        assert!(!preview_dispatch.contains("TrayCommand::Toggle => toggle_gallery_window"));
    }
}
