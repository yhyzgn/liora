# Zed / GPUI local patch

This directory contains a small repo-local Zed source patch used for Liora Gallery / Docs startup-window verification. It is **not** part of the published Liora SDK dependency surface.

Upstream base: `zed-industries/zed` commit `c97ad4692f99df4cc5eaa0a83a91afaefb641076`.

Why this exists:

- Upstream GPUI currently opens the platform window with restore bounds and only applies `WindowBounds::Maximized` afterward via `platform_window.zoom()`.
- On Linux/Wayland the first `surface.commit()` happens during platform window creation, before that post-open zoom request.
- On Linux/X11 the initial EWMH maximized state likewise needs to be present before `MapWindow`.
- The local patch carries an `InitialWindowState` through `WindowParams` and applies it before the first Wayland commit / X11 map.

Dependency boundary:

- Published Liora crates use only the official crates.io `gpui` package and must not depend on this directory.
- Liora's root `Cargo.toml` must not contain `[patch.crates-io]` or path GPUI overrides for SDK publication.
- Cargo patches are root-application decisions; a library crate cannot safely force downstream applications to use a patched backend.

How Gallery / Docs can test this patch locally:

1. Use a throwaway local branch for verification.
2. Keep the normal workspace dependency on official crates.io `gpui`.
3. Add this app-root patch only on the throwaway branch:

```toml
[patch.crates-io]
gpui = { path = "third_party/zed/crates/gpui" }
```

Do not commit that override to publishable Liora SDK manifests. The SDK release path must remain official crates.io `gpui` with no local path override.

Kept intentionally small:

- Only `crates/gpui`, `crates/gpui_linux`, the workspace manifest required for inherited dependencies, the Apache license, and GPUI font assets required by `include_bytes!` are included.
- Upstream examples, docs, build targets, and GPUI self-tests are excluded from Liora's root workspace.

Preferred long-term exit:

1. Upstream this patch to `zed-industries/zed` and remove this directory.
2. Or move the patch to a dedicated fork branch and let application roots opt into a pinned git revision.
