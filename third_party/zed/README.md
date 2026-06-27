# Zed / GPUI local patch

This directory contains a small repo-local Zed source patch used for Liora Gallery / Docs startup-window verification. It is **not** part of the published Liora SDK dependency surface.

Upstream base: `zed-industries/zed` commit `2c346f60a76fe3f0367ef924927f50a6efdf5718` for current Liora dependency comparison; older patch experiments may have originated from earlier upstream commits.

Why this exists:

- Upstream GPUI currently opens the platform window with restore bounds and only applies `WindowBounds::Maximized` afterward via `platform_window.zoom()`.
- On Linux/Wayland the first `surface.commit()` happens during platform window creation, before that post-open zoom request.
- On Linux/X11 the initial EWMH maximized state likewise needs to be present before `MapWindow`.
- The local patch carries an `InitialWindowState` through `WindowParams` and applies it before the first Wayland commit / X11 map.

Dependency boundary:

- Published Liora crates use only the official `zed-industries/zed` git dependency pinned in the workspace and must not depend on this directory.
- Liora's root `Cargo.toml` must not contain `[patch.crates-io]` or path GPUI overrides for SDK publication.
- Cargo patches are root-application decisions; a library crate cannot safely force downstream applications to use a patched backend.

How Gallery / Docs can test this patch locally:

1. Use a throwaway local branch for verification.
2. Keep the normal workspace dependency on official `zed-industries/zed` `gpui` / `gpui_platform` git sources.
3. Add this app-root patch only on the throwaway branch:

```toml
[patch."https://github.com/zed-industries/zed"]
gpui = { path = "third_party/zed/crates/gpui" }
gpui_platform = { path = "third_party/zed/crates/gpui_platform" }
```

Do not commit that override to publishable Liora SDK manifests. The SDK release path must remain official `zed-industries/zed` git sources with no local path override.

Kept intentionally small:

- Only `crates/gpui`, `crates/gpui_linux`, the workspace manifest required for inherited dependencies, the Apache license, and GPUI font assets required by `include_bytes!` are included.
- Upstream examples, docs, build targets, and GPUI self-tests are excluded from Liora's root workspace.

Preferred long-term exit:

1. Upstream this patch to `zed-industries/zed` and remove this directory.
2. Or move the patch to a dedicated fork branch and let application roots opt into a pinned git revision.
