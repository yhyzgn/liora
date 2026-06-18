use std::path::{Path, PathBuf};

fn workspace_file(relative: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(relative);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()))
}

fn assert_before(source: &str, first: &str, second: &str, label: &str) {
    let first_index = source
        .find(first)
        .unwrap_or_else(|| panic!("missing {first:?} in {label}"));
    let second_index = source
        .find(second)
        .unwrap_or_else(|| panic!("missing {second:?} in {label}"));
    assert!(
        first_index < second_index,
        "expected {first:?} to appear before {second:?} in {label}"
    );
}

fn committed_root(path: &str) -> bool {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(path)
        .exists()
}

#[test]
fn gpui_window_bounds_carry_initial_state_to_platform_params() {
    assert!(
        committed_root("third_party/zed/crates/gpui/src/window.rs"),
        "the GPUI startup-state patch must stay in the vendored GPUI source"
    );

    let platform = workspace_file("third_party/zed/crates/gpui/src/platform.rs");
    assert!(platform.contains("pub enum InitialWindowState"));
    assert!(platform.contains("pub initial_window_state: InitialWindowState"));

    let window = workspace_file("third_party/zed/crates/gpui/src/window.rs");
    assert!(window.contains("WindowBounds::Maximized(_) => crate::InitialWindowState::Maximized"));
    assert!(
        window.contains("WindowBounds::Fullscreen(_) => crate::InitialWindowState::Fullscreen")
    );
    assert!(window.contains("initial_window_state,"));
    assert!(
        window.contains("#[cfg(not(any(target_os = \"linux\", target_os = \"freebsd\")))]"),
        "Linux/FreeBSD must not rely on post-open zoom/toggle_fullscreen for the initial state"
    );
}

#[test]
fn linux_backends_request_initial_maximized_before_first_map_or_commit() {
    let wayland = workspace_file("third_party/zed/crates/gpui_linux/src/linux/wayland/window.rs");
    assert!(wayland.contains("InitialWindowState::Maximized => toplevel.set_maximized()"));
    assert!(wayland.contains("InitialWindowState::Fullscreen => toplevel.set_fullscreen(None)"));
    assert!(wayland.contains(
        "maximized: matches!(options.initial_window_state, InitialWindowState::Maximized)"
    ));
    assert_before(
        &wayland,
        "InitialWindowState::Maximized => toplevel.set_maximized()",
        "surface.commit();",
        "Wayland window startup",
    );

    let x11 = workspace_file("third_party/zed/crates/gpui_linux/src/linux/x11/window.rs");
    assert!(x11.contains("InitialWindowState::Maximized =>"));
    assert!(x11.contains("atoms._NET_WM_STATE_MAXIMIZED_VERT"));
    assert!(x11.contains("atoms._NET_WM_STATE_MAXIMIZED_HORZ"));
    assert!(x11.contains(
        "maximized_vertical: matches!(params.initial_window_state, InitialWindowState::Maximized)"
    ));
    assert_before(
        &x11,
        "InitialWindowState::Maximized =>",
        "xcb.map_window(self.0.x_window)",
        "X11 window startup",
    );
}
