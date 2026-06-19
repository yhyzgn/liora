use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};
use liora_components::{Button, Card, Space, Tag, Text};
use liora_tray::{
    TrayCloseAction, TrayCommand, TrayControlCenter, TrayMenuItemSpec, default_liora_tray_menu,
};

use liora_components::layout_helpers::{page, section};

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
            "liora-tray 封装 tray-icon/muda，提供进程常驻、动态图标、CheckBox 菜单和任意层级子菜单配置。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "托盘状态预览",
                    "Gallery 和 Docs 启动时都会创建各自独立的演示托盘图标；这里展示页面内如何配置状态栏驻留、显隐与动态图标。",
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(
                            Card::new(
                                Space::new()
                                    .vertical()
                                    .gap_md()
                                    .child(
                                        Space::new()
                                            .gap_md()
                                            .align_center()
                                            .child(active_icon_tag(self.active_icon))
                                            .child(
                                                Space::new()
                                                    .vertical()
                                                    .gap_xs()
                                                    .child(Text::new("Liora Gallery 正在后台运行").bold())
                                                    .child(Text::new(format!(
                                                        "当前图标：{} · 状态栏驻留：{} · 托盘可见：{} · 自动显示：{}",
                                                        self.active_icon,
                                                        if self.resident_enabled { "开启" } else { "关闭" },
                                                        if self.tray_visible { "显示" } else { "隐藏" },
                                                        if self.auto_show { "开启" } else { "关闭" }
                                                    ))),
                                            ),
                                    )
                                    .child(
                                        Space::new()
                                            .gap_md()
                                            .wrap()
                                            .child(icon_button("默认", "default", entity.clone()))
                                            .child(icon_button("同步中", "syncing", entity.clone()))
                                            .child(icon_button("错误", "error", entity.clone()))
                                            .child(tray_command_button(
                                                "显示主窗口",
                                                TrayCommand::Show,
                                                entity.clone(),
                                            ))
                                            .child(tray_command_button(
                                                "隐藏主窗口",
                                                TrayCommand::Hide,
                                                entity.clone(),
                                            ))
                                            .child(toggle_resident_button(
                                                self.resident_enabled,
                                                entity.clone(),
                                            ))
                                            .child(toggle_tray_visible_button(
                                                self.tray_visible,
                                                entity.clone(),
                                            ))
                                            .child(toggle_auto_show_button(self.auto_show, entity.clone())),
                                    ),
                            )
                            .no_shadow(),
                        )
                        .child(
                            Space::new()
                                .gap_sm()
                                .wrap()
                                .child(Tag::new("App-owned close policy").warning())
                                .child(Tag::new("TrayIcon::set_icon").success())
                                .child(Tag::new("CheckMenuItem").info())
                                .child(Tag::new("N-level Submenu").info()),
                        ),
                ))
                .child(section(
                    "状态栏驻留开关",
                    "实际应用中可通过页面配置决定是否启用驻留：开启时保留托盘并在关闭窗口时隐藏到托盘；关闭时隐藏托盘并由应用在退出动作中调用 cx.quit()。",
                    residency_preview(self.resident_enabled, self.tray_visible),
                ))
                .child(section(
                    "关闭窗口确认",
                    "点击窗口关闭按钮时先确认：关闭进程，或关闭主窗口并驻留在 tray；勾选「记住本次选择」后后续关闭会直接执行该选择。",
                    close_confirm_preview(self.remembered_close_action, entity.clone()),
                ))
                .child(section(
                    "丰富菜单配置",
                    "同一套 TrayMenuItemSpec 支持普通动作、CheckBox、动态图标命令、二级/三级/更多层级菜单和分隔线。",
                    menu_preview(&default_liora_tray_menu(), 0),
                ))
                .child(section(
                    "运行时命令",
                    "菜单事件会被映射为稳定的 TrayCommand，GPUI 主窗口只需要处理 Show/Hide/Toggle/Quit/SetIcon/Custom。",
                    command_table(),
                )),
        )
    }
}

fn active_icon_tag(name: &str) -> Tag {
    match name {
        "syncing" => Tag::new("↻ syncing").warning().large(),
        "error" => Tag::new("! error").danger().large(),
        _ => Tag::new("A default").success().large(),
    }
}

fn icon_button(label: &'static str, icon: &'static str, entity: Entity<TrayDemo>) -> Button {
    Button::new(label).primary().on_click(move |_, _, cx| {
        if cx.has_global::<TrayControlCenter>() {
            cx.global::<TrayControlCenter>()
                .dispatch(TrayCommand::SetIcon(icon.into()));
        }
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
    Button::new(label).on_click(move |_, _, cx| {
        if cx.has_global::<TrayControlCenter>() {
            cx.global::<TrayControlCenter>().dispatch(command.clone());
        }
        let _ = entity.update(cx, |_, cx| cx.notify());
    })
}

fn toggle_auto_show_button(auto_show: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if auto_show {
        "关闭 Auto Show"
    } else {
        "开启 Auto Show"
    })
    .on_click(move |_, _, cx| {
        if cx.has_global::<TrayControlCenter>() {
            cx.global::<TrayControlCenter>()
                .dispatch(TrayCommand::Custom("auto-show".into()));
        }
        let _ = entity.update(cx, |demo, cx| {
            demo.auto_show = !demo.auto_show;
            cx.notify();
        });
    })
}

fn toggle_resident_button(resident_enabled: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if resident_enabled {
        "关闭状态栏驻留"
    } else {
        "开启状态栏驻留"
    })
    .warning()
    .on_click(move |_, _, cx| {
        if cx.has_global::<TrayControlCenter>() {
            cx.global::<TrayControlCenter>()
                .dispatch(TrayCommand::Custom("resident-enabled".into()));
        }
        let _ = entity.update(cx, |demo, cx| {
            demo.resident_enabled = !demo.resident_enabled;
            if !demo.resident_enabled {
                demo.tray_visible = false;
            } else {
                demo.tray_visible = true;
            }
            cx.notify();
        });
    })
}

fn toggle_tray_visible_button(tray_visible: bool, entity: Entity<TrayDemo>) -> Button {
    Button::new(if tray_visible {
        "隐藏托盘图标"
    } else {
        "显示托盘图标"
    })
    .on_click(move |_, _, cx| {
        if cx.has_global::<TrayControlCenter>() {
            cx.global::<TrayControlCenter>()
                .dispatch(TrayCommand::Custom("tray-visible".into()));
        }
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
    Space::new().vertical().gap_sm().child(Card::new(
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
    ))
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

    Space::new()
        .vertical()
        .gap_sm()
        .child(Card::new(
            Space::new()
                .vertical()
                .gap_xs()
                .child(Text::new("窗口关闭时：退出进程 / 隐藏到托盘"))
                .child(Text::new(format!("当前策略：{remembered}"))),
        ))
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
        )
}

fn close_action_button(
    label: &'static str,
    action: TrayCloseAction,
    entity: Entity<TrayDemo>,
) -> Button {
    Button::new(label).on_click(move |_, _, cx| {
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

fn menu_preview(specs: &[TrayMenuItemSpec], depth: usize) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_sm()
        .children(specs.iter().map(|spec| render_menu_item(spec, depth)))
}

fn render_menu_item(spec: &TrayMenuItemSpec, depth: usize) -> gpui::AnyElement {
    match spec {
        TrayMenuItemSpec::Action { label, command, .. } => menu_line(
            depth,
            "Action",
            label,
            format!("id = {}", command.id()),
            false,
        ),
        TrayMenuItemSpec::Check {
            label,
            command,
            checked,
            ..
        } => menu_line(
            depth,
            if *checked {
                "☑ CheckBox"
            } else {
                "☐ CheckBox"
            },
            label,
            format!("id = {}", command.id()),
            false,
        ),
        TrayMenuItemSpec::Submenu {
            label, children, ..
        } => Space::new()
            .vertical()
            .gap_sm()
            .child(menu_line(
                depth,
                "Submenu",
                label,
                format!("{} children", children.len()),
                true,
            ))
            .child(menu_preview(children, depth + 1))
            .into_any_element(),
        TrayMenuItemSpec::Separator => {
            Text::new(format!("{}────────", indent(depth))).into_any_element()
        }
    }
}

fn menu_line(
    depth: usize,
    kind: &'static str,
    label: &str,
    detail: String,
    submenu: bool,
) -> gpui::AnyElement {
    Card::new(
        Space::new()
            .vertical()
            .gap_xs()
            .child(
                Space::new()
                    .gap_sm()
                    .child(Tag::new(kind).small())
                    .child(Text::new(format!(
                        "{}{}{}",
                        indent(depth),
                        label,
                        if submenu { " ›" } else { "" }
                    ))),
            )
            .child(Text::new(detail)),
    )
    .no_shadow()
    .into_any_element()
}

fn command_table() -> impl IntoElement {
    let commands = [
        TrayCommand::Show,
        TrayCommand::Hide,
        TrayCommand::Toggle,
        TrayCommand::SetIcon("syncing".into()),
        TrayCommand::Custom("deep-action".into()),
        TrayCommand::Quit,
    ];

    Space::new()
        .vertical()
        .gap_sm()
        .children(commands.map(|command| {
            Card::new(Text::new(format!("{command:?}  →  {}", command.id())).nowrap()).no_shadow()
        }))
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}
