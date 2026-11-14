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
- Evidence: GPUI refresh changes layout/text/GPU/input/platform behavior. An isolated Linux Xwayland session rendered all nine SDK demos and passed real Markdown cross-block clipboard copy plus aligned pointer selection. The official Linux headless clipboard write/read path is no-op/None; Composer smoke injects payloads and marked composition. Actual IME candidate windows and macOS/Windows remain unverified. Earlier workspace `clippy -D warnings` reports existing optimizer/packager/tray code debt.
- Trigger: release after dependency refresh or declaring universal touch/headless/cross-platform compatibility based only on compilation.
- Failure mode: geometry, fonts, IME/selection, real touch, multi-display or native window regressions; strict lint quality gate is not green.
- Mitigation: retain native headless regression and isolated real-display evidence separately; preserve explicit IME, Windows/macOS and hardware coverage gaps. Remediate existing lint debt in a separately scoped maintenance task without warning suppressions.
- Rollback: revert this refresh's pin/lock/API/docs as one bounded change.
- Status: Open.

## R-005 — RPM signature enforcement and publisher trust

- Severity: High.
- Evidence: Fedora 45 / RPM 6 require trusted package signatures; existing v0.5.0 RPM assets were released unsigned. `cargo-generate-rpm` signs only when explicitly given a key, not through Fedora's `rpmbuild` autosign configuration.
- Failure mode: unsigned/untrusted RPM updates are rejected; silently re-signing downloads would erase publisher provenance. A valid RPM signature alone does not prove dependency/ABI compatibility across distributions.
- Mitigation: `xtask` requires `LIORA_RPM_SIGNING_KEY` for RPM release channels/tags; CI compares `LIORA_RPM_SIGNING_PRIVATE_KEY` against the independently provisioned full v4 `LIORA_RPM_SIGNING_FINGERPRINT` before signing, signs before checksums, verifies both font variants in an isolated RPM keyring, and publishes the public key/fingerprint. Users independently verify the fingerprint before an explicit RPM key import. The updater remains manual and never changes trust or disables verification.
- Rollback: revert the RPM signing workflow/xtask/updater/docs change together; do not bypass system signature enforcement. Existing v0.5.0 assets remain unchanged.
- Status: Open for independent fingerprint distribution and real native distro installation smoke. Repository RPM secret/fingerprint were provisioned for v0.5.1 using the owner-requested non-expiring development signing identity; macOS/Windows platform signing remains opt-in and unconfigured.
- Verification (2026-11-11): focused xtask/updater/packager tests passed (31 tests); packaging validate and local release-readiness passed (missing local signing key remains a warning). Disposable real-backend builds verified both font variants, signed-byte SHA-256, trusted-key acceptance, untrusted-key rejection and no false signed flag on a stale unsigned RPM. Tagged dry-run/Docs-only readiness succeeds without RPM secrets; real release RPM generation still rejects unsigned overrides. Exact CI scripts rejected missing/malformed/mismatched fingerprints, removed temporary private keys on success/failure, and assembled both signed RPM variants plus the public key/fingerprint with matching release SHA256SUMS. Rust/workflow reviews found no remaining actionable issues after the two scoped fixes. No protected release signing or native distro installation was performed.
