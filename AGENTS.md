# Liora Repository Agent Contract

## Reading order
1. `AGENTS.md`
2. `.context/README.md`
3. Plan file: `.context/plans/004_gpui_native_replacement_20261013.md`
4. Task file: `.context/tasks/004_gpui_native_replacement_20261013.md`
5. `.context/system/overview.md`
6. `.context/system/conventions.md`
7. `.context/system/risks.md`

## Project summary
Liora is a Rust edition 2024 Cargo workspace for a pure Rust + GPUI native enterprise UI component library with canonical apps `apps/liora-gallery` and `apps/liora-docs`.

## Business boundaries
- SDK crates stay app-agnostic.
- Gallery/Docs are the only canonical dogfooding surfaces.
- Keep implementations pure Rust + native GPUI.

## Invariants
- Prefer evidence over memory.
- Verify before claiming completion.
- Keep diffs small, reviewable, and reversible.
- Do not use warning-suppression bypasses.
- Do not introduce WebView/Tauri/HTML/CSS/DOM runtime paths.

## Technical red lines
- Use official `zed-industries/zed` GPUI sources and the pinned revision in the workspace.
- Do not couple SDK crates to Gallery/Docs business logic.
- Keep public entry/core files thin.

## Lifecycle and context rules
- `.context/` is the only repository-owned long-lived context system.
- Legacy `prompt.md`, `.memory/`, and `.prompt/` live only under `.context/archive/legacy-sources/` after migration.
- Update `system/` for stable facts, `plans/` for phase direction, and `tasks/` for bounded work.

## External infrastructure
- Release/signing/notarization/public release actions remain protected-environment work.
- Follow the repository release workflows and checklist before any tag release.

## Safe validation commands
- `python scripts/context_bootstrap.py validate --root .`
- `cargo check --workspace --all-targets`
- `cargo test --workspace`
- `cargo run --release -p xtask -- package validate`
- `cargo run --release -p xtask -- package release-readiness`
- `git diff --check -- . ':(exclude).omx'`

## Risk and rollback
- If CTX migration or validation fails, restore from `.context/archive/legacy-sources/` or `git restore .`.
- If a change affects release or packaging behavior, rerun the packaging gates before merging.

## Review, commit, and push policy
- Verify first, then commit.
- Keep commits focused.
- Push only after validation unless the user explicitly asks otherwise.

## Legacy migration coverage
- `prompt.md`, `.memory/`, and `.prompt/` have been migrated into `.context/` and archived.
- Future work must use `.context/` pointers only.

## Current context pointers
- Current plan: `.context/plans/004_gpui_native_replacement_20261013.md`
- Current task: `.context/tasks/004_gpui_native_replacement_20261013.md`


<claude-mem-context>
# Memory Context

# claude-mem status

This project has no memory yet. The current session will seed it; subsequent sessions will receive auto-injected context for relevant past work.

Memory injection starts on your second session in a project.

`/learn-codebase` is available if the user wants to front-load the entire repo into memory in a single pass (~5 minutes on a typical repo, optional). Otherwise memory builds passively as work happens.

Live activity: http://localhost:37777
How it works: `/how-it-works`

This message disappears once the first observation lands.
</claude-mem-context>
