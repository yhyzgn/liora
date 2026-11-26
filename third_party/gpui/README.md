# Liora-maintained GPUI source

Upstream: [zed-industries/zed](https://github.com/zed-industries/zed/tree/f0d896f2c18a543335eb155f729d36b624695591), exact revision `f0d896f2c18a543335eb155f729d36b624695591`. This is a Liora-owned derivative of that source, not a newer official Zed revision. The application owner explicitly authorized this maintenance boundary for the Linux appearance initialization repair.

## Source and dependency boundary

- The nine complete upstream crate trees are `gpui`, `gpui_platform`, `gpui_linux`, `gpui_macos`, `gpui_ios`, `gpui_apple`, `gpui_windows`, `gpui_web`, and `gpui_wgpu`. Their mutual path dependencies preserve one GPUI type identity, including Apple backends. Source, resources, examples, tests, package metadata, and Apache-2.0 licensing are retained.
- Upstream root `assets/fonts/ibm-plex-sans` and `assets/fonts/lilex` are retained with their SIL OFL licenses for compile-time test/example includes. The standalone web example pins its two external utilities to the official baseline; the iOS helper resolves its plist and workspace relative to its script instead of assuming the original repository root. These relocation repairs do not claim either platform was run.
- The nested workspace reproduces the upstream package metadata, dependency settings and lint policy. Other Zed utility crates remain Git dependencies pinned to the exact upstream revision above. Unrelated upstream root patches are not imported; Cargo does not inherit dependency workspaces' root patches.
- Liora's root workspace excludes this nested workspace. Published SDK manifests must select `gpui` and `gpui_platform` from the same immutable Liora repository Git revision, with the existing `gpui 0.2.2` registry fallback. They must not depend on a developer-local path.
- Registry consumers still need their own root `[patch.crates-io]` for `gpui` to that same Liora Git revision. The AccessKit application-root backport remains a separate, non-inherited exception.
- `../zed` is historical reference material, not this source. Its three manifests are archived as `Cargo.toml.reference` to prevent duplicate package discovery when Cargo searches this Git repository.

## Appearance repair

`App::is_window_appearance_initialized()` and `Window::is_appearance_initialized()` distinguish a backend's pending fallback from its initialized appearance. Window readiness is cached together with its existing appearance snapshot; callers must not combine a live backend readiness bit with an older window color. The default platform trait policy remains initialized for synchronous and test backends.

Linux common/X11/Wayland appearance is `None` until a successful initial or live portal result, including initialized Light. New windows inherit that state, and the common state survives a zero-window interval. A successful new windowed connection resets readiness; a failed connection does not. Headless behavior stays initialized under its existing policy.

The existing portal stream is established before reading the initial appearance. This closes the missing-subscription interval without creating another listener; it is not an atomic snapshot-and-subscription protocol. A successful initial read remains useful if subscribing fails. Synthetic startup/decoration callbacks and actual equal-color notifications are preserved. Liora consults explicit readiness rather than interpreting a callback's occurrence as proof of a portal result.

## Updating and rollback

1. Compare against the exact upstream revision; keep the behavioral delta bounded to the appearance APIs, Linux state/delivery, and deterministic regressions.
2. Validate the focused GPUI/Core tests, actual private-portal scenarios and Gallery/Docs startup before publishing a source commit.
3. Publish the source commit first; then pin consuming manifests and examples to its full SHA in a separate commit. A commit cannot contain its own SHA.
4. Validate the Git-source dependency graph and a downstream consumer, not just local path builds. Record platform/runtime limits instead of claiming universal verification.
5. To return to official GPUI, first migrate Liora away from the readiness extension; then change both GPUI pins and the lockfile together. Do not substitute the old `../zed` experiment.

No signing, tag release, crates.io publication, or installer execution is implied by publishing a source revision. Verification evidence belongs in Liora's audit task ledger; copying unmodified platform source does not establish macOS, Windows, iOS, or web runtime support.
