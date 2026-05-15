//! Close confirmation policy for tray-resident apps.
//!
//! The window close hook should ask the user whether to quit the process or
//! close the main window to tray. If the user checks "remember", save the chosen
//! `TrayCloseAction` and skip the prompt on later close attempts.

use aura_tray::{TrayCloseAction, TrayCommand, TrayControlCenter};
use gpui::App;

pub fn remembered_close_action(cx: &App) -> TrayCloseAction {
    cx.has_global::<TrayControlCenter>()
        .then(|| {
            cx.global::<TrayControlCenter>()
                .state
                .remembered_close_action
        })
        .unwrap_or(TrayCloseAction::Ask)
}

pub fn remember_close_action(cx: &mut App, action: TrayCloseAction) {
    if cx.has_global::<TrayControlCenter>() {
        cx.global_mut::<TrayControlCenter>()
            .set_remembered_close_action(action);
    }
}

pub fn hide_to_tray_from_close_prompt(cx: &mut App, remember: bool) {
    if remember {
        remember_close_action(cx, TrayCloseAction::HideToTray);
    }
    if cx.has_global::<TrayControlCenter>() {
        cx.global::<TrayControlCenter>().dispatch(TrayCommand::Hide);
    }
}

pub fn exit_from_close_prompt(cx: &mut App, remember: bool) {
    if remember {
        remember_close_action(cx, TrayCloseAction::ExitProcess);
    }
    cx.quit();
}
