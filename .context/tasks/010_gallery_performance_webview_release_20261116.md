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
- [x] Commit and push the `liora-components` rustdoc correction to `main`.
- [x] Commit and push the `liora-webview` rustdoc correction to `main`.
- [x] Verify GitHub Actions CI for the final rustdoc fixes.
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
- 2026-11-16: GitHub Actions run 43759396220 passed formatting, workspace check/tests, Docs snippets, and packaging, then failed strict SDK rustdoc coverage on undocumented `liora-webview::Error` and `Event` variants. Added accurate public docs; `RUSTDOCFLAGS="-D missing_docs" cargo doc -p liora-webview --no-deps` passed locally.
- 2026-11-16: Commit 33f99f4c5c passed GitHub Actions run 43761445101: formatting, workspace check/tests, Docs snippets, SDK rustdoc coverage, packaging validation, release-readiness dry-run, and install/uninstall dry-run. The release tag remains blocked because `crates-io` has no protection rules.
- 2026-11-16: Following the existing successful `v0.6.0` release precedent and the plan's tag-release phase, pushed annotated tag `v0.6.1` at `a3be642fa79056d1e3bc1c935fcf2342b58fdefa`. SDK publication run `43777098428` and native package/release run `43777098473` started; final workflow, registry, and asset verification remain pending.
- 2026-11-16: SDK release run 43777098428 completed successfully, including patched crates.io consumer verification and dependency-ordered publication. `cargo info liora-webview@0.6.1` resolved the published version; crates.io API returned HTTP 200 for both `liora/0.6.1` and `liora-webview/0.6.1`. Native package/release run 43777098473 is still building Linux, macOS, and Windows assets.

## Shared Markdown / PDF demo repair — 2026-11-16

- Scope: user-reported Markdown layout/sizing, PDF first-visible-page loading, and PdfReader preview sizing/position. This is not evidence that issue #2 is fixed, and does not change release/tag state above.
- Markdown diagnosis: the shared demo lacked a stable scroll-container ID; GPUI `Flex` therefore did not attach overflow/scroll state. `AnchorTarget` and Markdown focus-preview wrappers reused the child's layout ID but called `prepaint_at(bounds.origin)`, applying the native layout origin twice.
- PDF diagnosis: the worker decoded every page and thumbnail before returning any visible page. Add one optional first-page callback after full metadata/budget preflight; keep the final result/error mandatory. Preserve FitPage proportions and host height; fix viewport/row/sidebar flex constraints instead of stretching pages.
- [x] Verify Markdown padded anchor coordinates, actual shared demo, and scroll behavior.
- [x] Verify PDF first-preview timing, final result, full preflight and previous-document restoration on failure.
- [x] Verify PdfReader actual bounds under resize, sidebar, fit modes, rotation and zoom overflow.
- [x] Record focused compile/tests, native screenshot evidence and review findings; no unrelated workspace/release gates.

### Repair verification

- `cargo build -p liora-gallery --example document_layout_smoke --example pdf_reader_smoke --offline`: PASS after adding the shared worker channel dependency. An earlier build ran during manifest edits and failed with unresolved `futures`; no dependency fallback or suppression was added.
- `target/debug/examples/document_layout_smoke && target/debug/examples/pdf_reader_smoke`: PASS. Padded anchor child measured origin `(32,112)`, size `736×60`; PDF bounds assertions covered four viewport sizes, sidebar visibility, FitPage/FitWidth, rotation, manual-zoom left-edge reachability and continuous stacking. Existing real-PDF keyboard/scroll/rotation/Find smoke also passed.
- `cargo test -p liora-gallery --lib shared_demo_has_a_bounded_scrollable_document --locked -- --nocapture`: PASS (1). Actual shared demo measured a 380px viewport with overflow and consumed a wheel event. Initial test compilation used an obsolete `PlatformInput::ScrollWheel` variant; corrected to pinned GPUI `PlatformInput::Scroll` before the passing run.
- `cargo test -p liora-components --lib pdf_reader::backend::tests --locked -- --nocapture`: PASS (9), manual timing test ignored (1). Covers shared first raster, complete preview metadata, invalid later-page/budget preflight, alpha/color preservation and real sample decoding.
- `cargo test -p liora-gallery --lib pdf_reader_demo::tests --locked -- --nocapture`: PASS (1). Preview replaces the visible entity; final error restores the original entity, page and zoom, and clears loading state. Final successful `set_document` still resets any scrolling performed during preview; no per-page cache/API was added.
- Isolated Xwayland ran the actual shared Markdown and PDF demo modules through `document_layout_smoke` (`--pdf` for PDF). Both native windows rendered at 1568×830, exited 0, and screenshots were inspected: `/tmp/liora-document-layout.lxBb2i/markdown.png` and `/tmp/liora-document-layout.lxBb2i/pdf.png`. Headings/paragraphs/table are positioned in the Markdown viewport; the PDF is centered beside the fixed sidebar, preserves aspect ratio, and displays real text/RGB vectors and both thumbnails. This does not exercise a native file chooser or establish macOS/Windows behavior.
- Paired CPU stage harness, `assets/samples/reader.pdf` (2 pages, cached debug Hayro/image rlibs): original first-ready 45.56ms / all-ready 95.41ms; integer-thumbnail/opaque-BGRA path first-ready 29.09ms / all-ready 55.29ms. Thumbnail work 44.13→15.46ms; BGRA conversion 21.38→13.23ms. Source: `/tmp/liora-pdf-stage-results.txt`; the throwaway harness reproduces the original pipeline, not native chooser/GPU timing. The earlier Cargo timing attempt timed out before executing the benchmark, so is not test evidence. These small-fixture timings do not reproduce or resolve a seconds-long report for an unknown PDF.
- `cargo check -p liora-docs --bin check_snippets --locked` and focused `rustfmt --check` for all eight changed Rust files: PASS. Docs consumes Gallery's shared demo through its existing dependency; no SDK-to-app coupling was added.
- `cargo run -p liora-gallery --example markdown_navigation_smoke --locked`: PASS (TOC target scrolling and host event). Temporary stage-measurement source/binary were removed after recording results; logs and native screenshots remain under `/tmp`.
- Independent Markdown review found no correctness blockers. PDF review identified that debug-only bounds selectors would panic in release smoke; both the diagnostic function and call are now gated by `debug_assertions`, retaining release runtime smoke. A second finding is valid: pinned Hayro `Page::render_dimensions()` clamps a positive fractional box to at least one pixel before Liora preflight, while final `page_size` retains the original point dimensions. Construct preview placeholders with that same raw size rather than the public constructor's one-point clamp; a real fractional-box PDF regression exercises the public API.
- `cargo run --release -p liora-gallery --example pdf_reader_smoke --locked`: PASS after the debug-selector gate; release retains real-PDF drawing, zoom, navigation, scrolling, rotation and Find routing. `python scripts/context_bootstrap.py validate --root .`: PASS.
- Fractional-box regression RED: `cargo test -p liora-components --lib preview_preserves_fractional_page_geometry --locked -- --nocapture` failed exactly on preview `(1.0,2.0)` versus final `(0.5,2.0)`. Fix uses raw `PdfPageSize` in preview placeholders, matching final pages. Reviewer confirmed both repair directions and no other pending defects.
- Fractional-box regression GREEN: the same focused command passed after the raw-size fix (1 test); changed backend formatted successfully. Combined focused automated results: 12 passed, 1 manual timing test ignored, plus debug/release native smoke and TOC navigation checks above. No full workspace rerun, native chooser round-trip, Windows/macOS runtime, commit, push, tag or publication in this repair scope.

## User-rejected visual acceptance — follow-up

- The previous repair did not satisfy the user's report. The supplied image is actually 3840×2114 (displayed scaled to 1568×863); the Gallery's fixed 640px PDF demo occupied only a top strip and left most of the right pane unused. Isolated demo centering screenshots were insufficient acceptance evidence.
- Current scope: make the actual Gallery PDF pane fill remaining height while keeping Docs embedded sizing; investigate the still-slow Open PDF path; make the actual Markdown demo render representative supported features and expose immediate editing using the existing native editor.
- Acceptance requires actual Gallery screenshots, targeted interaction/performance evidence, then a focused commit and push to `origin/main` as explicitly requested. No release tag/publication is authorized by this follow-up.

### Follow-up implementation and validation (in progress)

- Gallery alone uses a filling PDF registry entry and disables its outer main scroll; Docs retains the bounded 640px shared demo entry. Actual-host geometry regression includes 1568×863 and 3840×2114 viewport containment, not only reader-to-card alignment.
- PDF demand-loading review found and corrected invalid retained Hayro cache ownership, fractional placeholder geometry, demand reset on document replacement, empty-document requests, and rem-dependent row arithmetic. Native scroll item visibility now drives requests. Delivery review required serialized replacement and restoration of the complete previous session, including its visible request list; targeted runtime checks remain pending.
- Markdown follow-up addresses exact display-text pointer hit testing, focus-dependent preview restoration and opt-in measured native Input wrapping. Existing four-entry formula/diagram cache already bounds retained rasters; no additional pruning mechanism was added.
- Unified Gallery compile initially failed while `input/wrapped.rs` was not yet written, then with E0446 because its layout-state type was private. Implementation now includes the module and matching internal visibility; integrated tests are running. These failures are not passing validation evidence.
- Follow-up automated PASS: actual Gallery root/card containment at both viewport sizes; 16 PDF tests (1 manual timing ignored); lazy-session rollback/serialization; actual preview-pointer/blur regression; wrapped End/pointer/visual-row/IME/no-redraw Unicode regression; unchanged-formula cache reuse; two shared Markdown demo regressions; `markdown_editing_smoke`; Docs compile-checked snippets.
- Public 10-page fontconfig fixture, debug CPU timing: preflight 0.965ms, lazy first-ready 218.493ms with exactly one rasterized page; full eager decode 2104.351ms. These are worker CPU timings, not native chooser/GPU timings or a reproduction of the unknown user PDF.
- Actual Gallery 3840×2114 native PDF capture exited 0, but screenshot `/tmp/liora-gallery-acceptance/pdf.png` shows loaded page-one thumbnail and a blank main page viewport despite the filled frame. Native acceptance FAIL; root/card geometry alone is insufficient. Continue fixing actual page visibility before commit/push.
- Blank-page root cause: the absolute viewport-measurement canvas lacked explicit dimensions, so FitPage retained its initial `(1,1)` measurement. Changed it to `.absolute().size_full()`; scoped review found no blocker. Strengthened actual-host regression now checks the page itself: 1568×863 host gives viewport 1050×470.5 and page 594.5×446.5; 3840×2114 host gives viewport 3322×1721.5 and page 2262.5×1697.5, contained in the viewport. Focused host test PASS.
- Repaired actual Gallery native 3840×2114 screenshot `/tmp/liora-gallery-acceptance/pdf-fixed.png` inspected: real text/RGB PDF page fills available height with proportional FitPage margins and loaded first-page thumbnail. Actual Markdown screenshot `/tmp/liora-gallery-acceptance/markdown.png` inspected: normal headings/inline formatting and bounded editor viewport; scroll and interaction behavior covered by the passing consumer/pointer tests.
- Final `pdf_reader_smoke` PASS: resize/sidebar/fit/rotation/zoom/continuous geometry and real PDF drawing, keyboard navigation, scroll and Find routing. Isolated Xwayland `markdown_editing_smoke` PASS with native clipboard, nested/table composition, consecutive edits and undo/redo. Focused source formatting and CTX validation PASS.
- Limits: no native file-chooser round-trip, unknown user PDF reproduction, complete workspace rerun, strict Clippy, Windows/macOS runtime, tag or publication in this follow-up. CPU fixture timings do not promise arbitrary-page latency. Full structural rich-text editing still requires explicit source mode.
