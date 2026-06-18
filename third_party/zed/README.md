# Zed / GPUI patch

This directory contains the smallest repo-local source patch Liora currently needs for GPUI startup window state handling.

Upstream base: `zed-industries/zed` commit `c97ad4692f99df4cc5eaa0a83a91afaefb641076`.

Why this exists:

- Upstream GPUI currently opens the platform window with restore bounds and only applies `WindowBounds::Maximized` afterward via `platform_window.zoom()`.
- On Linux/Wayland the first `surface.commit()` happens during platform window creation, before that post-open zoom request.
- On Linux/X11 the initial EWMH maximized state likewise needs to be present before `MapWindow`.
- Liora needs first-frame maximized startup, so the local patch carries an `InitialWindowState` through `WindowParams` and applies it before the first Wayland commit / X11 map.

Kept intentionally small:

- Only `crates/gpui`, `crates/gpui_linux`, the workspace manifest required for inherited dependencies, the Apache license, and two GPUI font assets required by `include_bytes!` are included.
- Upstream examples, docs, build targets, and GPUI self-tests are excluded from Liora's root workspace.

Preferred long-term exit:

1. Upstream this patch to `zed-industries/zed` and remove this directory plus the root `[patch]` entries.
2. Or move the patch to a dedicated fork branch and depend on a pinned git revision instead of vendoring source here.
