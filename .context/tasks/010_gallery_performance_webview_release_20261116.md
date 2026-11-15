# 010 Gallery Performance and 0.6.1 Release Execution - 2026-11-16

- 状态: 进行中
- 计划: `.context/plans/010_gallery_performance_webview_release_20261116.md`

## Findings

- GitHub issue #2 is open and contains no reproduction steps, target platform, or measurable workload. No Gallery-specific root cause or fix is confirmed.
- The Docs live-demo prefix-index change is separate from issue #2 and must not be described as fixing Gallery performance.
- Workspace package manifests were prepared for 0.6.1. The published registry still reported 0.6.0 for `liora`, while `liora-webview` was not yet present; release evidence must be rechecked after publication.
- `release-sdk.yml` already defines protected crate packaging/publication. `ci.yml` already installs the Linux WebKitGTK 4.1 development dependency.
- The xtask readiness gate initially rejected the valid `CRATES_IO_CRATES` order because it hardcoded an older list. Added a regression test and aligned the check with the workflow, including `liora-webview`.
- GitHub's `crates-io` environment has no protection rules (`protection_rules: []`). Repository-level `CRATES_IO_TOKEN` and `LIORA_RPM_SIGNING_PRIVATE_KEY` secrets are present (names checked 2026-11-16) and the workflows reference them through `secrets`. Do not push the release tag until required environment protection rules are configured and all release gates pass.

## Acceptance

- [x] Preserve issue #2 as open and state that reproducible details are needed.
- [x] Keep the Docs optimization distinct from any Gallery performance claim.
- [x] Update every affected README and Docs page with accurate 0.6.1 APIs, package boundaries, dependencies, and runtime limits.
- [x] Make release notes render the versioned changelog section and WebView/platform limits accurately.
- [x] Pass focused package, documentation, format, and CTX checks; record each command and result.
- [x] Commit and push the documentation coverage fix to `main`.
- [ ] Verify GitHub Actions CI for the pushed fix.
- [ ] After CI passes and `crates-io` protection rules are configured, push the release tag and verify SDK/app workflows and crate publication.
- [ ] Verify registry installation and release assets.
- [x] Reply to issue #2 with verified findings and request reproducible details.

## Verification log

- Completed checks and release evidence will be appended here with exact commands, dates, and outcomes. A failed workflow is not a release blocker: diagnose and fix it before tagging or publication.
- 2026-11-16: 21 repository READMEs scanned; 57 local Markdown links and 31 fenced TOML snippets validated successfully. External URLs were not fetched by this check.
- 2026-11-16: `.github/workflows/package.yml` parsed as YAML; its release-note `run` block passed `bash -n`; the `0.6.1` changelog AWK extraction found WebView and open-issue wording.
- 2026-11-16: `.github/workflows/release-sdk.yml` registry metadata audit passed, including all publishable crate metadata.
- 2026-11-16: `cargo package -p liora-webview --no-verify --allow-dirty` passed and packaged 8 files. Cargo warned that locked `async-executor 1.14.0` and `spin 0.9.8` are yanked; package generation succeeded.
- 2026-11-16: Initial `cargo test --workspace` found one stale Docs contract-test expectation for the `CRATES_IO_CRATES` order; the release workflow already included `liora-webview`. Updated the assertion to the exact current order. Targeted `cargo test -p liora-docs --bin liora-docs markdown::tests::packaging_docs_and_workflows_include_release_readiness_gate` passed; the full workspace rerun is pending.
- 2026-11-16: `cargo fmt --all --check` passed. Final README validation passed for 21 files, 61 local links, and 31 fenced TOML snippets; package workflow YAML and release-note shell syntax passed. External URLs were not fetched.
- 2026-11-16: `python scripts/context_bootstrap.py validate --root .` passed.
- 2026-11-16: Follow-up documentation review found no remaining issues in the previously flagged publication-status or runtime-verification wording.
- 2026-11-16: `cargo check -p liora-docs --bin check_snippets --locked`, `cargo doc --workspace --no-deps`, `cargo run --release -p xtask -- package validate`, and a final `cargo package -p liora-webview --no-verify --allow-dirty` passed. Package output contained 8 files; Cargo warned about yanked locked `async-executor 1.14.0` and `spin 0.9.8`.
- 2026-11-16: `cargo run --release -p xtask -- package release-readiness` initially failed on the stale expected publish order; after the checker fix, it passed with the existing signing-policy WARN.
- 2026-11-16: `cargo test --workspace` completed with 804 passed, 1 ignored across 46 suites. After the readiness fix, `cargo test -p xtask` passed all 14 tests; `sdk_release_readiness_uses_publish_env_not_audit_list` failed against the incorrect fixture before the checker fix.
- 2026-11-16: Final focused gates passed: `cargo fmt --all --check`, `cargo check -p liora-docs --bin check_snippets --locked`, `cargo doc --workspace --no-deps`, `cargo run --release -p xtask -- package validate`, `cargo run --release -p xtask -- package release-readiness`, and `python scripts/context_bootstrap.py validate --root .`. Readiness passed with the existing signing-policy WARN; release tag remains blocked until signing requirements are satisfied.
- 2026-11-16: After reviewer corrections, `RUSTDOCFLAGS="-D missing_docs" cargo doc -p liora-components --no-deps` passed.
- 2026-11-16: Posted the issue #2 follow-up at https://github.com/yhyzgn/liora/issues/2#issuecomment-6900960276, requesting platform, reproduction steps, workload, and the specific stall pattern.
- 2026-11-16: Live GitHub checks found both required repository secret names, but `crates-io` has zero protection rules; release tagging remains blocked by the protected-environment requirement.
