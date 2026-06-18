# P11 — Native Tray / Process Resident Support

## Goal

Add a native Rust system-tray integration layer for Liora applications so GPUI apps can remain resident after their windows close, expose native tray menus, and update tray state dynamically.

## Technical Direction

- Create and maintain `crates/liora-tray` as the public Liora facade.
- Use `tray-icon` for cross-platform tray icons and its `muda` menu re-export for native menu items.
- Do not vendor or fork `tauri-apps/tray-icon`/`muda` unless a future customization requirement cannot be met through public APIs.
- GPUI apps that enable tray residency must use `QuitMode::Explicit` and keep the `LioraTray` handle alive for the whole process lifetime.

## Required Capabilities

- Basic tray install from `TrayConfig`.
- Dynamic icon updates:
  - `set_icon`
  - `clear_icon`
  - `set_icon_from_rgba`
  - `set_icon_from_path`
- Tooltip and visibility updates.
- Native menu DSL:
  - action item
  - checkbox item
  - separator
  - recursive submenu for 2nd/3rd/N-level menus
- Stable command mapping:
  - `Show`
  - `Hide`
  - `Toggle`
  - `Quit`
  - `SetIcon(name)`
  - `Custom(name)`
- Checkbox state sync via command id.

## Demo / Docs Contract

- `liora-gallery` must include a Tray demo entry with large, readable examples of dynamic icon state, CheckBox preferences, and deep submenus.
- `liora-docs` must include a Tray page with external compile-checked snippets for basic install, dynamic icon switching, checkbox menus, and nested menus.
- Normal docs/gallery previews should avoid creating a real OS tray icon unless a future explicit runtime integration demo is requested; use config previews to avoid intrusive tray side effects during component browsing.

## Platform Notes

- Linux requires GTK/AppIndicator dependencies and same-thread event-loop/tray creation.
- macOS tray creation must happen on the main thread; template icon mode is useful for menu-bar appearance.
- Windows/Linux/macOS all require the tray handle to be retained; dropping it removes the icon.
