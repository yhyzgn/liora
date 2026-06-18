# Contributing to Aura

Aura is developed as a pure Rust + GPUI native component library. Contributions must preserve that architecture.

## Ground rules

- Do not convert Aura apps to Tauri.
- Do not introduce WebView, HTML/CSS/DOM, browser runtime, or frontend build systems as application runtime dependencies.
- Prefer improving existing components over adding parallel replacements.
- Keep diffs small, reversible, and covered by tests or docs checks.
- Update Gallery, Docs, snippets, and tests whenever public behavior changes.

## Local verification

Before submitting a phase or PR, run the relevant targeted tests plus the standard gate:

```bash
cargo fmt --all --check
cargo check --workspace --all-targets
cargo test --workspace
cargo check -p aura-docs --bin check_snippets
cargo run -p xtask -- package validate
cargo run -p xtask -- package release-readiness
cargo run -p xtask -- package ci --all-apps --format platform-defaults --dry-run --skip-build
cargo run -p xtask -- package install-smoke --all-apps --format platform-defaults --dry-run
```

For documentation/adoption changes, also run:

```bash
cargo check -p aura-minimal-app
cargo doc --workspace --no-deps
```

## Component documentation standard

Every public component or new public configuration should have:

1. a Gallery example when visual behavior is involved;
2. a Docs page section;
3. compile-checked snippet coverage when code is shown;
4. a regression test for API shape, rendering policy, or docs wiring;
5. notes about keyboard, overlay, selection, drag, or performance behavior when relevant.

Docs pages should keep each effect next to its corresponding code block: effect → code → next effect → code.

## Release changes

Packaging and release changes must use `xtask package` and `aura-packager`. Update `docs/packaging-installer-technical-plan.md`, `apps/aura-docs/content/pages/packaging_workflow.md`, and `.prompt/P12-packaging.md` when release policy changes.

Formal release signing/notarization credentials must be provided only through protected release environments and secrets documented in `packaging/signing-policy.md`.
