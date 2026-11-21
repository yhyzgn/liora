use gpui::{App, Window};
use liora_components::{request_window_frame_mode, toast_info, WindowFrameChange, WindowFrameMode};
use liora_core::{attach_system_theme_observer, tr};

use super::{
    gallery_window_options, handle_gallery_window_should_close, locale_template, locales, Gallery,
    GalleryTrayState,
};

pub(super) fn set_gallery_frame_mode(mode: WindowFrameMode, window: &mut Window, cx: &mut App) {
    // The Switch callback still owns its entity until this deferred update.
    window.defer(cx, move |window, cx| {
        let Some(Some(root)) = window.root::<Gallery>() else {
            eprintln!("failed to change Gallery frame: missing Gallery root");
            return;
        };
        let previous = root.read(cx).frame_mode;
        if previous == mode {
            return;
        }
        if matches!(
            request_window_frame_mode(window, mode),
            WindowFrameChange::RequiresWindowReopen
        ) {
            if !reopen_gallery_window(mode, window, cx) {
                return;
            }
            toast_info!(
                "{}",
                locale_template(cx, locales::window_frame::gallery_reopen)
                    .replace("{mode}", &mode_label(cx, mode))
            );
        } else {
            root.update(cx, |root, cx| {
                root.set_frame_mode(mode, cx);
            });
            update_gallery_frame_state(mode, cx);
            toast_info!(
                "{}",
                locale_template(cx, locales::window_frame::gallery_switched)
                    .replace("{mode}", &mode_label(cx, mode))
            );
        }
    });
}

pub(super) fn reopen_gallery_window(
    mode: WindowFrameMode,
    window: &mut Window,
    cx: &mut App,
) -> bool {
    let Some(Some(root)) = window.root::<Gallery>() else {
        eprintln!("failed to reopen Liora Gallery window: missing Gallery root");
        return false;
    };
    let previous = root.read(cx).frame_mode;
    if previous == mode {
        return true;
    }
    let rem_size = window.rem_size();
    let focused = window.focused(cx);
    match cx.open_window(
        gallery_window_options(cx, mode, Some(window)),
        |replacement, cx| {
            root.update(cx, |root, cx| {
                root.set_frame_mode(mode, cx);
            });
            attach_system_theme_observer(replacement, cx);
            replacement.set_rem_size(rem_size);
            if let Some(focused) = &focused {
                replacement.focus(focused, cx);
            }
            replacement.on_window_should_close(cx, handle_gallery_window_should_close);
            root.clone()
        },
    ) {
        Ok(handle) => {
            let handle: gpui::AnyWindowHandle = handle.into();
            if cx.has_global::<GalleryTrayState>() {
                cx.global_mut::<GalleryTrayState>().window = Some(handle);
            }
            update_gallery_frame_state(mode, cx);
            window.remove_window();
            let _ = handle.update(cx, |_, window, _| window.activate_window());
            true
        }
        Err(error) => {
            root.update(cx, |root, cx| {
                root.set_frame_mode(previous, cx);
            });
            eprintln!("failed to reopen Liora Gallery window: {error:?}");
            liora_components::toast_error!("Failed to change window frame: {error}");
            false
        }
    }
}

fn update_gallery_frame_state(mode: WindowFrameMode, cx: &mut App) {
    if cx.has_global::<GalleryTrayState>() {
        let state = cx.global_mut::<GalleryTrayState>();
        state.frame_mode = mode;
        state.window_visible = true;
    }
}

fn mode_label(cx: &App, mode: WindowFrameMode) -> String {
    if mode.is_custom() {
        tr(cx, locales::window_frame::custom).to_string()
    } else {
        tr(cx, locales::window_frame::system).to_string()
    }
}
