use gpui::{App, Window};
use liora_components::{request_window_frame_mode, WindowFrameChange, WindowFrameMode};
use liora_core::attach_system_theme_observer;

use super::{
    docs_window_options, handle_docs_window_should_close, markdown::DocsShell, DocsTrayState,
};

pub(super) fn set_docs_frame_mode(mode: WindowFrameMode, window: &mut Window, cx: &mut App) {
    window.defer(cx, move |window, cx| {
        let Some(Some(root)) = window.root::<DocsShell>() else {
            return;
        };
        let previous = root.read(cx).frame_mode();
        if previous == mode {
            return;
        }
        if matches!(
            request_window_frame_mode(window, mode),
            WindowFrameChange::RequiresWindowReopen
        ) {
            if !reopen_docs_window(mode, window, cx) {
                return;
            }
            liora_components::toast_info!("Docs window frame reopened to apply {}", mode.label());
        } else {
            root.update(cx, |root, cx| {
                root.set_frame_mode(mode, cx);
            });
            update_docs_frame_state(mode, cx);
            liora_components::toast_info!("Docs window frame switched to {}", mode.label());
        }
    });
}

pub(super) fn reopen_docs_window(mode: WindowFrameMode, window: &mut Window, cx: &mut App) -> bool {
    let Some(Some(root)) = window.root::<DocsShell>() else {
        return false;
    };
    let previous = root.read(cx).frame_mode();
    if previous == mode {
        return true;
    }
    let rem_size = window.rem_size();
    let focused = window.focused(cx);
    match cx.open_window(
        docs_window_options(cx, mode, Some(window)),
        |replacement, cx| {
            root.update(cx, |root, cx| {
                root.set_frame_mode(mode, cx);
            });
            attach_system_theme_observer(replacement, cx);
            replacement.set_rem_size(rem_size);
            if let Some(focused) = &focused {
                replacement.focus(focused, cx);
            }
            replacement.on_window_should_close(cx, handle_docs_window_should_close);
            root.clone()
        },
    ) {
        Ok(handle) => {
            let handle: gpui::AnyWindowHandle = handle.into();
            if cx.has_global::<DocsTrayState>() {
                cx.global_mut::<DocsTrayState>().window = Some(handle);
            }
            update_docs_frame_state(mode, cx);
            window.remove_window();
            let _ = handle.update(cx, |_, window, _| window.activate_window());
            true
        }
        Err(error) => {
            root.update(cx, |root, cx| {
                root.set_frame_mode(previous, cx);
            });
            eprintln!("failed to reopen Liora Docs window: {error:?}");
            liora_components::toast_error!("Failed to change window frame: {error}");
            false
        }
    }
}

fn update_docs_frame_state(mode: WindowFrameMode, cx: &mut App) {
    if cx.has_global::<DocsTrayState>() {
        let state = cx.global_mut::<DocsTrayState>();
        state.frame_mode = mode;
        state.window_visible = true;
    }
}
