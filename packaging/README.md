# Liora Packaging Resources

This directory contains static resources consumed by `cargo xtask package` and
`crates/liora-packager`.

Liora remains a pure Rust + GPUI native application. These files describe package
metadata and platform integration; they do not introduce Tauri or a WebView
runtime.

## Icon set

The icon set is generated from deterministic SVG sources in `packaging/icons/`:

- `liora.*` — main Liora component-library brand logo.
- `liora-gallery.*` — application icon for `liora-gallery` packages; component-grid mark.
- `liora-docs.*` — application icon for `liora-docs` packages; document/fold mark.

Each icon currently ships as:

- `.svg` source, committed for future edits.
- `.png` 512x512 8-bit RGBA app icon used by generic package metadata.
- `hicolor/<size>x<size>/apps/*.png` Linux application-menu/taskbar icons for 16/24/32/48/64/128/256/512 sizes.
- `.ico` multi-size Windows icon.
- `.icns` multi-size macOS icon.

Regenerate these assets from the SVG sources when the brand mark changes. The
packaging validator checks that all required icon files exist and have the
expected file headers. Runtime tray icons are app-owned assets under
`apps/liora-gallery/assets/tray-icons/` and `apps/liora-docs/assets/tray-icons/`.
Window header marks live under each app's `assets/status-icons/` directory, and
bottom status-bar state icons live under each app's `assets/status-bar-icons/`
directory. These assets intentionally do not live inside the reusable
`liora-tray` SDK crate.

## Desktop integration policy

Liora packages must register app launchers with each platform's native application center:

- Linux `.deb` / `.AppImage` / RPM install `.desktop`, AppStream metainfo, SVG icons, and hicolor PNG icons so GNOME, KDE/Plasma, and other desktop environments can resolve `app_id` icons.
- Linux portable `.tar.gz` archives include `install-desktop.sh` and `uninstall-desktop.sh` to copy the same desktop entry and hicolor icons into the current user's XDG data directory.
- macOS `.app` / `.dmg` rely on the bundle `Info.plist` generated from `productName`, bundle identifier, category, and `.icns` icon; dragging the app into `/Applications` registers it with LaunchServices/Launchpad.
- Windows NSIS and WiX installers create Start Menu shortcuts through cargo-packager's default templates, assign the app `.ico` to installer/package metadata, and remove those shortcuts during uninstall. Each Windows app binary also embeds its own `.ico` through a Windows-only `build.rs`, so Start Menu and Desktop shortcuts resolve the target executable icon. The Liora packager intentionally does not add an extra WiX shortcut fragment, because cargo-packager already emits the Start Menu shortcut and a second fragment would duplicate launchers.
