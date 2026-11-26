# Historical Zed / GPUI patch reference

This directory contains a small repo-local Zed source patch used for Liora Gallery / Docs startup-window verification. It is **not** part of the published Liora SDK dependency surface.

Historical comparison base: `zed-industries/zed` commit `492acd6c815cbe8c7366d54e6092341340afa6c7`; the archived manifests retain their original older dependency pins. This is not the current Liora backend.

Why this exists:

- Upstream GPUI currently opens the platform window with restore bounds and only applies `WindowBounds::Maximized` afterward via `platform_window.zoom()`.
- On Linux/Wayland the first `surface.commit()` happens during platform window creation, before that post-open zoom request.
- On Linux/X11 the initial EWMH maximized state likewise needs to be present before `MapWindow`.
- The local patch carries an `InitialWindowState` through `WindowParams` and applies it before the first Wayland commit / X11 map.

Dependency boundary:

- Current GPUI source maintenance lives in [`../gpui`](../gpui/README.md); do not restore these historical manifests into the dependency graph.
- The three `Cargo.toml.reference` files retain their original bytes but are deliberately not named `Cargo.toml`: Cargo recursively discovers Git packages even inside excluded workspaces, and the old package names would collide with the current GPUI crates.
- Cargo patches are root-application decisions; a library crate cannot safely force downstream applications to use a patched backend.

The reference is not directly buildable. Keep historical source and manifests inert; any future experiment must use a separate checkout so Git package discovery remains unambiguous.

Kept intentionally small:

- Only `crates/gpui`, `crates/gpui_linux`, the workspace manifest required for inherited dependencies, the Apache license, and GPUI font assets required by `include_bytes!` are included.
- Upstream examples, docs, build targets, and GPUI self-tests are excluded from Liora's root workspace.

The historical startup-window experiment is not included in the current appearance repair and is not a verified fix for the present GPUI baseline.
