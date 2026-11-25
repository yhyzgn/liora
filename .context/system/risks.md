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

## R-006 — PDF decoding resources and untrusted content

- Severity: Medium for trusted documents; High for hostile, unbounded workloads.
- Evidence: Hayro 0.9.0 decodes pure Rust PDF bytes in-process. Liora preflights 64 MiB input and 128 MiB retained page/thumbnail rasters, not parser/transient/GPU allocations or execution time; rotation adds cached page copies.
- Failure mode: malformed or complex PDF content may exhaust worker memory/time; eager rendering is not a large-document cache or security sandbox. Raster zoom can blur above native resolution.
- Mitigation: decode on a background worker, surface explicit errors and preserve the previous document; applications accepting hostile documents need process-level resource isolation. No scripts, external resource fetches, implicit PDF writes or password UI are added.
- Rollback: revert the Hayro dependency/lockfile and `from_bytes`/backend/demo loading changes together; host-supplied image APIs remain a separate integration path.
- Status: Open for adversarial-document isolation, real system IME and Windows/macOS native runtime verification.

## R-007 — Opt-in system WebView behavior and untrusted pages

- Severity: High for applications loading untrusted sites.
- Evidence: WRY 0.57.2 uses GTK3/WebKitGTK 4.1 on Linux and requires an X11 GDK display for GPUI XCB child XIDs; WKWebView/WebView2 use platform engines. Isolated Linux X11 proof covered two live child surfaces, HTTP navigation, page IPC, geometry, explicit close and parent-close teardown.
- Failure mode: Wayland or non-X11 GDK embedding fails; page content may issue network/subresource requests and untrusted IPC can induce privileged host actions; engine permission/browser compatibility differs by platform. Loading completion is not HTTP success.
- Mitigation: keep WRY isolated outside default SDK; reject unsupported handles/GDK displays, constrain top-level navigation, deny permissions/downloads/new windows and clipboard/autoplay, bound incoming events, and require the host to authorize page URLs/messages. Use current patched system engines.
- Rollback: remove only `liora-webview` and its workspace member/lockfile changes; native components and facade remain untouched.
- Status: Open for macOS/Windows runtime tests, engine-specific page behavior and hostile-site/network isolation.

## R-008 — Linux color-emoji format compatibility

- Severity: Medium for native text fidelity.
- Evidence: the pinned official Linux/WGPU backend uses Swash color-layer parsing. A real-display probe with application-matched MiSans/Medium and 14px theme text shaped Noto COLRv1 emoji into nonzero glyph IDs but returned empty native raster bounds. Screenshot `/home/neo/liora-glyph-native-6rlqk4d5/native.png` shows blank emoji; the raster regression rejects it in `/home/neo/liora-glyph-native-ldwpubgs/app.log`.
- Compatible-font evidence: the same binary passed glyph/visible-raster assertions and displayed color emoji under an isolated font configuration using official Noto Emoji v2.051 `NotoColorEmoji.ttf` (CBDT/CBLC; SHA-256 `72a635cb3d2f3524c51620cdde406b217204e8a6a06c6a096ff8ed4b5fd6e27b`). Capture/log/identity: `/home/neo/liora-glyph-native-tkm57hkm/`. Font source/license: https://github.com/googlefonts/noto-emoji/tree/v2.051/fonts .
- Failure mode: installed family names and valid shaped glyph IDs can falsely suggest complete glyph coverage while displayed emoji are blank.
- Mitigation: host-owned compatible font distribution/selection, verified with `multilingual_font_native_smoke` and a real screenshot. Fontconfig reject rules alone were insufficient for this backend's directory-based font discovery; the passing fixture restricted private font directories. Do not modify desktop fontconfig, silently download SDK fonts, or treat an isolated compatible fixture as a default-system fix.
- Actual Gallery/Docs acceptance: rebuilt app binaries both presented the shared multilingual Paragraph at 1568px and 900px window widths under the isolated compatible-font configuration. Narrow screenshots show complete wrapping; native Select All/Copy preserved the exact same Unicode payload, including emoji and a combining accent, and both apps exited normally. Screenshots, clipboard payloads and executable identities: `/home/neo/liora-typography-host-0roe448f/`. The run also exposed Container's intrinsic-width overflow; a native geometry regression measured 680px instead of 300px before the owning main-region minimum-width repair, then all five Container tests passed.
- Status: Open for default COLRv1 rendering, Windows/macOS, and broader Unicode coverage. The isolated dual-application sample passed; it is not a default-font or whole-application responsive-layout certification. No GPUI fork, pin change, or universal color-font guarantee.
