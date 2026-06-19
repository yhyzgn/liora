# Zed / GPUI local patch

This directory contains a small repo-local Zed source patch used for Liora Gallery / Docs startup-window verification. It is **not** part of the published Liora SDK dependency surface.

Upstream base: `zed-industries/zed` commit `c97ad4692f99df4cc5eaa0a83a91afaefb641076`.

Why this exists:

- Upstream GPUI currently opens the platform window with restore bounds and only applies `WindowBounds::Maximized` afterward via `platform_window.zoom()`.
- On Linux/Wayland the first `surface.commit()` happens during platform window creation, before that post-open zoom request.
- On Linux/X11 the initial EWMH maximized state likewise needs to be present before `MapWindow`.
- The local patch carries an `InitialWindowState` through `WindowParams` and applies it before the first Wayland commit / X11 map.

Dependency boundary:

- Published Liora crates keep using crates.io `open-gpui` / `open-gpui-platform` and must not depend on this directory.
- Liora's root `Cargo.toml` must not contain `[patch.crates-io]` or path GPUI overrides for SDK publication.
- Cargo patches are root-application decisions; a library crate cannot safely force downstream applications to use a patched backend.

How Gallery / Docs can test this patch locally:

1. Use a throwaway local branch for verification.
2. Temporarily change the root `[workspace.dependencies]` aliases so every Liora crate resolves `gpui` and `gpui_platform` from Zed git packages instead of crates.io `open-gpui` / `open-gpui-platform`. Do not change only one app crate, or the workspace can end up with incompatible GPUI types.
3. Add this app-root git-source patch:

```toml
[patch."https://github.com/zed-industries/zed"]
gpui = { path = "third_party/zed/crates/gpui" }
gpui_linux = { path = "third_party/zed/crates/gpui_linux" }
```

Do not commit that override to publishable Liora SDK manifests. The vendored packages are named upstream `gpui` / `gpui_linux`, so they cannot directly patch crates.io packages named `open-gpui` / `open-gpui-linux`; apps that stay on `open-gpui` need a renamed private fork instead.

Kept intentionally small:

- Only `crates/gpui`, `crates/gpui_linux`, the workspace manifest required for inherited dependencies, the Apache license, and GPUI font assets required by `include_bytes!` are included.
- Upstream examples, docs, build targets, and GPUI self-tests are excluded from Liora's root workspace.

Preferred long-term exit:

1. Upstream this patch to `zed-industries/zed` and remove this directory.
2. Or move the patch to a dedicated fork branch and let application roots opt into a pinned git revision.
