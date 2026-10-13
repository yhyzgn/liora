# Risks

## R-001 — GPUI upstream drift

- Severity: High
- Evidence: `Cargo.toml` pins `gpui` and `gpui_platform` to official `zed-industries/zed` revisions.
- Trigger: GPUI/Zed API changes, platform behavior changes, or dependency upgrades.
- Failure mode: compile errors, runtime panics, packaging regressions, or platform-specific behavior differences.
- Mitigation: verify against official upstream sources and run focused compile/tests before claiming completion.
- Rollback: revert dependency/API changes and restore the previous lockfile revision.
- Status: Open.

## R-002 — SDK/application coupling

- Severity: High
- Evidence: prior tray/packager/updater/icon optimizer work separated SDK code from Gallery/Docs business behavior.
- Trigger: moving demo app assets, menus, updater selectors, or business flows into reusable crates.
- Failure mode: downstream developers inherit Liora demo behavior or resources unintentionally.
- Mitigation: keep Gallery/Docs code in `apps/`; SDK crates expose generic configuration and primitives only.
- Rollback: move business-specific code back to app layer and add tests/docs for generic boundaries.
- Status: Open.

## R-003 — Legacy context migration semantic loss

- Severity: Medium
- Evidence: CTX takeover archives `prompt.md`, `.memory/`, and `.prompt/` after content-unit migration.
- Trigger: future agents relying on old paths or failing to read `.context/` first.
- Failure mode: stale phase state, missed project red lines, or broken compile-time include paths.
- Mitigation: keep source markers, migration state, archive bytes, and tests updated to canonical `.context` paths.
- Rollback: restore archived legacy sources from `.context/archive/legacy-sources/` if needed.
- Status: Open.

## R-004 — Upgraded platform rendering and lint validation gaps

- Severity: High for platform runtime coverage; Medium for existing strict Clippy debt.
- Evidence: GPUI refresh changes layout/text/GPU/input/platform behavior; current shell has no DISPLAY/WAYLAND_DISPLAY. Workspace `clippy -D warnings` reports existing optimizer/packager/tray code debt.
- Trigger: release after dependency refresh or declaring universal touch/headless/cross-platform compatibility based only on compilation.
- Failure mode: geometry, fonts, IME/selection, real touch, multi-display or native window regressions; strict lint quality gate is not green.
- Mitigation: run native Linux headless smoke plus workspace tests, preserve explicit Windows/macOS/real-display validation gaps, and remediate existing lint debt in a separately scoped maintenance task without warning suppressions.
- Rollback: revert this refresh's pin/lock/API/docs as one bounded change.
- Status: Open.
