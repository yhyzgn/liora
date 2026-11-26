# Changelog

## Unreleased

- Bind Notification deadlines to their owning manager, cancel tasks on dismissal and preserve four seconds of unhovered display time across pointer hover and window closure. Add a native keyboard-accessible per-notification close action; share the executable semantic-type and stack example between Gallery and Docs.
- Make Carousel autoplay advance natively with cancellable deadlines, whole-carousel hover pause and retained uncontrolled selection. Preserve controlled `active_index` feedback through `on_change`; share stable, weakly owned Gallery/Docs examples and the executable Docs autoplay source.
- Reserve Carousel content space beside visible navigation arrows, preventing overlap with title-only slides and custom slots.

- Maintain an explicitly pinned Liora GPUI source derivative of official Zed `f0d896f2c18a543335eb155f729d36b624695591`. Distinguish pending Linux appearance from initialized Light/Dark, cache window readiness with its color, subscribe before the initial portal read, and resolve System mode from that state rather than synthetic notification history. Preserve same-color and decoration callbacks, fixed modes, zero-window transitions and native backends; keep old experiment manifests inert to avoid Git package collisions.

- Resolve Linux/FreeBSD GTK startup theme hints independently of configuration line order. Honor an explicit dark preference without letting a false preference override an explicitly dark theme name; retain fixed theme modes and live appearance observers.

- Correct Gallery's bilingual About badges to identify Liora-maintained GPUI and its official Zed provenance, matching the documented source boundary.

- Use the active theme's danger token for Gallery/Docs About update errors while preserving error details and all action guards. Give the Docs About panel its own native scroll viewport so narrow or short windows can reach wrapped update actions without changing ordinary virtualized pages.

- Resolve Docs theme choices and About/Updates text through existing typed locale keys instead of fixed English/Chinese captions. Keep retained control identity, selected values and update callbacks unchanged; authored documentation is not auto-translated.

- Use the existing protected OS-cache helper in both root README updater examples instead of a predictable shared temporary directory. Preserve fail-closed cache errors and the caller-owned install action.

- Distinguish manual installation instructions from launched installer commands in Gallery/Docs, without showing either terminal state as active update work. Resolve Docs update-status text in the current locale and preserve existing check/download coordination and caller-owned installer completion.

- Add opt-in content-driven Container header height and let Gallery/Docs toolbars wrap without covering the body or clipping translated theme, language and frame controls. Keep default and explicit fixed header heights unchanged; pair the Container examples and retain native resize geometry coverage.

- Improve default light/dark tertiary and placeholder contrast across neutral surfaces and hover/pressed states, including alpha compositing. Preserve disabled colors and avoid runtime work; share readable Theme samples and keep color-swatch labels off saturated fills.
- Handle Drawer Escape from retained trigger or underlying drawer focus through the app-wide stack policy; honor protected top drawers without dismissing lower ones. Install bindings once, including core-only hosts, and preserve stale-close identity guards without moving focus or adding a focus trap.

- Share the retained native spring interaction between Gallery Spinner and Docs, with retarget and remove/restore controls, stable animation identity, and a compile-checked example. Keep reduced-motion preference host-owned; do not add SDK motion policy or replace existing duration effects.
- Add a shared Gallery/Docs multilingual Typography sample and a real-display probe checking native glyph clusters and visible raster pixels with app-matched font settings. Document the pinned Linux/WGPU COLRv1 emoji blank-raster limitation and isolated CBDT font evidence; keep font installation/selection host-owned, without an SDK font policy or dependency change.
- Keep Container's main viewport within the width remaining beside its sidebar instead of expanding to wide children's intrinsic size. Add a native geometry regression for the bounded main region; preserve host-owned fixed-content overflow policy.
- Commit locale directory resources only after every file reads and parses successfully; preserve existing translations on failure so runtime reloads cannot leave partially updated text behind an unchanged locale version. Retain successful per-locale replacement and unrelated locales, with a failed-load/retry regression.
- Preserve package manifest, checksum and generated release-note entries across sequential font-variant/app builds; replace selected output scopes, retire missing files and reject changed retained bytes or symlink-parent escapes without relabeling their original provenance or signing state. Document building each variant immediately before `--skip-build` packaging.
- Follow painted Popover anchor bounds and retire its card/backdrop when the trigger unmounts, even if the popup view remains retained. Preserve same-ID replacement owners and existing focus/Escape policies. Add shared Gallery/Docs delayed move/remove/reset examples and Tooltip movement/remount regressions.
- Keep `Config::set_locales_config` version updates consistent with the free setter: advance the stored counter with saturation instead of adopting the replacement config's version.

- Expose Input's existing native root as a stable named text/password/multiline input, with localized `aria_label` naming and readable non-password text runs. Preserve focus, disabled state and native editing; omit password values and text runs even when revealed. This adds text-reading metadata, not synchronized AT caret/selection, text geometry or AT editing actions.
- Route Gallery/Docs system-menu Help → GitHub to the platform URL opener; allow only the canonical repository URL in Gallery's existing fallback while keeping documentation previews and shell layouts unchanged.
- Expose one localized Help → GitHub fallback in the Docs header on Linux/FreeBSD, where GPUI cannot activate system menus; dispatch the existing URL command without duplicating menus on macOS/Windows or enabling tutorial side effects.
- Preserve retained Timer start times on first render, take one elapsed snapshot, and skip unrepresentable refresh deadlines without displacing sibling wakes. Share retained Start / Pause / Resume / Reset examples and their actual host source in Gallery and Docs; add cached-root scheduling, teardown, delayed-countdown and extreme-interval regressions.
- Retire portaled Tooltip help when its trigger unmounts, without retaining the trigger through its own overlay. Let Escape pass inactive image Preview bindings so nested help, Dialog dismissal and unrelated key handlers keep their native priority.
- Show Tooltip on its original descendant focus as well as hover; dismiss visible help with Escape without moving focus or reopening during continuous focus/hover, and retain it across the trigger-to-content gap using native occlusion-aware hitboxes. Route Dialog Escape through its existing action path, with automatic core-only installation and top-dialog policy protection, so the first Escape can dismiss nested hover-only help without closing its container. Keep stable tooltip renders read-only to global observers and schedule a bounded follow-up frame for real content changes. Preserve chart tooltip APIs, Button/Input actions, immediate display and reduced motion; pair the actual Gallery/Docs examples and compile-checked snippet. Native Tooltip role/name does not claim a trigger description relationship. Keep clipped Tooltip trigger-to-panel bridges in the portal’s native hit-test scope. Give actual trigger focus or source/panel/bridge hover first Escape priority, including partly covered panels; retire unfocused help when native occlusion ends every hover source. Treat custom overlays that retain trigger focus conservatively (help first, then container), rather than claiming whole-panel visibility from point samples.
- Exit Gallery and Docs when tray residency is disabled with no remaining windows; preserve live windows when disabling residency.
- Keep Dropdown and DropdownButton long menus scrollable and reveal keyboard-selected items, with matching Gallery and Docs examples.
- Render and copy the authored CandidateBar/FindBar and ButtonGroup Rust examples in Docs.
- Retain Gallery/Docs root and child entities, navigation/search/demo state and current window geometry/display/rem-size zoom across creation-time frame changes. Open the replacement before removing the original and restore the frame selection on failure; keep Linux/FreeBSD live decoration switching. Add callback-free `Switch::set_checked` for retained-toggle synchronization.
- Make `Drawer::close` dismiss only the topmost drawer, preserving underlying drawers. Wire controlled sheet Cancel and Apply in Gallery and Docs to dismiss only their own sheet by ID.
- Repaint shared Drawer insertion, replacement, and removal without waiting for unrelated input. Keep no-op stack updates quiet and preserve underlying drawers on topmost close.
- Add `Dropdown::disabled_item(label)` and native keyboard navigation for Dropdown and DropdownButton. Focus the first enabled row on the next frame; navigation skips disabled rows and clicking one leaves focus unchanged. Tab/Shift+Tab closes the menu and moves focus through the host’s tab order. Activation closes the originating menu before invoking its callback, preserving any same-ID replacement it opens.
- Dismiss only the originating MenuBar popup after an enabled leaf command, before the selection callback; keep disabled rows, submenu parents, standalone Menu previews, and unrelated or callback-reopened popups intact. Use native Button entries for Tab focus and Enter/Space toggling; reuse shared Popover Escape and focus recovery.
- Open Button-triggered Popovers with Enter/Space, route Escape from trigger and interactive portal content according to the close policy, and restore originating focus on dismissal without adding wrapper tab stops or intercepting editable trigger input; retain split DropdownButton primary actions. Repaint shared Popover insertion, replacement, and dismissal immediately without dirtying windows for no-op clears.
- Wire Gallery/Docs platform-menu Zoom In, Zoom Out, and Reset Zoom, plus Gallery's visible fallback, to current-window rem-size behavior; request the next frame in the shared Zoom executor for all SDK callers. Leave other app-owned shell commands pending and documentation previews side-effect-free.
- Refresh Gallery's cached About label/search index and sidebar menu after locale changes while preserving the search query, active destination, and menu entity.
- Add manual keyboard activation and native role, name, orientation, and selection metadata to Tabs. Keep the first pane per business key, fall back to the first pane for an invalid initial key, and give editable Close and Add separate keyboard controls with focus recovery after removal.
- Preserve live Linux/FreeBSD window appearance when reselecting System or initializing another window, including notifications received in fixed modes; retain synchronous startup hints until the SDK observes an appearance notification.
- Resolve default code fonts from an available concrete family stack, including Linux monospace families, and cache the initialized SDK default until font registration or explicit refresh. Gallery/Docs inherit this policy without changing MiSans UI typography or explicit host font overrides; no font files are bundled by this fix.
- Add the sole application-root source-policy exception for official `accesskit_atspi_common 0.19.1` with upstream Busy/Disabled backports, preserving the exact official GPUI pin and SDK publication boundaries. Share executable metadata/source audit and negative controls between ordinary CI and release preflight, and explicitly test the excluded leaf. Downstream hosts must opt into their own root override; published SDK crates do not transmit the fix.

- Preserve complete checksum-manifest asset names, including spaces and literal leading asterisks, so a row for a longer filename cannot authorize a truncated asset name; retain text/binary marker support.
- Serialize Gallery and Docs update checks/downloads, disable conflicting update actions while busy, and revoke the previous install plan before a new download so failures cannot leave an old plan installable.
- Store Gallery and Docs updates in validated per-user caches, reject unsafe directory ownership/permissions and leaf aliases, and propagate cache errors without falling back to shared temporary paths.
- Give Buttons names and roles on their existing native root, with localized icon-action labels and a name-only `aria_label` override preserving visible `+` glyphs; retain unavailable interaction guards and report component disabled/busy flags separately from platform AT states.
- Paint Button keyboard focus as a hollow native outline without filling transparent text variants or changing root bounds, content geometry, or existing borders; reuse one retained native focus owner and preserve mouse/keyboard modality.
- Choose contrasting black or white label text for standard opaque semantic filled Buttons at rest, on hover, and while pressed, without changing fills or borders.
- Pair the five semantic Text Button foregrounds with normal, hover, and pressed backgrounds on known built-in light body/card surfaces without changing semantic tokens, filled colors, transparent resting backgrounds, or builder order; preserve already-readable built-in dark and explicit custom colors. Arbitrary host backgrounds and custom palettes remain outside this contrast guarantee.
- Keep text Button backgrounds transparent at rest and preserve semantic label colors regardless of builder order.
- Make Checkbox and Switch reachable in both Tab directions, preserve Enter/Space activation, and require an inside press/release for pointer toggles. Retain focus identity across disable/reenable, expose native root role/checked/disabled/name metadata, and add localized `aria_label` naming to Gallery/Docs examples without changing visible labels or explicit IDs.
- Require a matching primary press and release on the same CheckboxGroup option in button, styled, and custom layouts; dragging into or between options no longer changes selection.
- Make Dialog title close controls keyboard-reachable native buttons with a visible focus ring and opaque per-view accessibility IDs; preserve built-in versus business close callbacks and clarify Gallery/Docs examples.
- Prevent Dialog overlay clicks from reaching lower dialogs: disabled outside dismissal preserves both layers, while enabled outside dismissal closes only the top dialog.
- Fix Dialog and MessageBox manual close to dismiss only the topmost modal while preserving underlying dialogs.
- Notify searchable Select selection clears exactly once without treating search edits as selection changes; release Select when its window closes even if its Input remains retained.

- Give Alert close controls native button semantics and a visible keyboard focus ring; preserve Tab/Shift+Tab and key-up activation, with stable sibling IDs and parent-owned dismissal/restoration examples.
- Restore unhandled Tab/Shift+Tab traversal through system/custom window frames and enabled Input controls; reserve the Docs virtual scrollbar hit area so it cannot intercept live Alert close clicks.
- Fix Affix content alignment in nested layouts before pinning and after scrolling back.
- Release Autocomplete after its window closes by avoiding a strong reference cycle in the Input change callback.
- Display Avatar alt text when its image fails to load, with matching Gallery and Docs examples.
- Keep Autocomplete suggestions from blocking neighboring controls when outside-click dismissal is disabled.
- Clear Anchor target measurements and active highlighting when a target is removed.
- Fix icon-only Buttons to keep the configured icon size and show the loading indicator without changing the button bounds.
- Compare release tags by SemVer precedence so matching stable releases follow prereleases, numeric prerelease identifiers sort numerically, and build metadata is ignored.
- Follow event-updated GPUI appearance for live System theme changes without stale startup hints masking notifications; preserve initial startup synchronization and fixed Light/Dark modes.
- Show the Image fallback after a remote load fails. Use `Image::embedded` for local artwork and previews in standalone Gallery and Docs.
- Replace Markdown paragraph Input switching with a unified native rich editor, canonical source selection and styled variable-height virtual rows. Add source-preserving structural commands, slash commands, literal find/replace, move/fold and selection-aware history; retain native table inputs and host image authorization.
- Migrate rich-editor consumers from `rich_input_at()` / `rich_input()` to `focus_source_offset()`, `set_selection()` and `insert_text()`. Source-mode `input()` and table `table_input_at()` remain integration surfaces; Markdown remains the persistence format.
- Correct the native Markdown interaction contract to Typora-style direct syntax typing in one editable surface: retain `# `, `**strong**`, `- ` and code fences in canonical source while rendering in place. Remove the default formatting toolbar and shared Gallery/Docs append/reset controls and counter; retain the representative document and bounded viewport. Public `insert_text()` remains literal.
- Verify direct syntax typing through native input callbacks, source preservation, marker deletion and undo/redo, alongside shared-demo wheel scrolling. Focused Markdown tests, actual Linux/Xwayland rendering and real XIM checks passed; full Typora parity and cross-platform runtime remain unclaimed.
- Fix empty fenced-code pointer placement and Backspace at inline opening marks; preserve styled paragraph-join precedence and source/selection undo.
- Fix native Markdown blank-area pointer placement, visible-text arrow navigation across hidden delimiters, and find Enter/Escape routing with keyboard focus restoration. Let the actual Gallery Markdown editor fill the remaining pane height while retaining bounded Docs embedding.
- Retain Markdown task checkbox identity and pointer focus across redraws; preserve table-cell spaces and caret position during consecutive native typing after double-click selection.
- Restore native Markdown caret blinking and immediate whitespace geometry, including empty nested list/quote content. Preserve visible-row anchors during edits and syntax changes, use final painted pointer geometry after autoscroll, and recover vertical navigation after scrolling away from the caret. Preserve directed Source selections and immediate selection notifications.
- Refresh the registered Liora Docs system menu labels when the app language changes.

## 0.6.1 - 2026-11-16

- Make the actual Gallery PDF pane fill remaining window height instead of a fixed 640px strip; retain bounded Docs embedding. Add worker-owned `PdfDecoder`, visible-page requests and state-preserving page installation so opening no longer rasterizes every page. Preserve eager decode APIs and resource preflight.
- Replace the read-only unfinished Markdown demo with the existing native rich editor and complete supported content; preserve inactive block formatting and reuse unchanged formula/diagram rasters during prose edits. Fix doubled native anchor/preview offsets and bounded scrolling.
- Add native GPUI `PdfReader`, `FindBar`, and `CandidateBar`; decode real PDF bytes with pure-Rust Hayro 0.9.0, provide viewport fit, pixel rotation, continuous page navigation, and worker-based Gallery/Docs file loading. Search indexing, annotation persistence, and input-method engines remain host-owned.
- Add standalone opt-in `liora-webview` with GPUI child geometry, navigation, bounded page messaging, default-denied permissions/downloads/new windows, and window-close cleanup. Default SDK and native controls remain WRY-free; Linux X11/XWayland was tested, macOS/Windows native behavior was not.
- Precompute Docs live-demo prefix indices for virtualized blocks, preserving nested quote/list order while avoiding repeated prefix recounts during rendering.
- Fix native Mermaid contrast for nested translucent regions, percentage opacity, and self-message labels/loops.
- Investigate Gallery performance report [#2](https://github.com/yhyzgn/liora/issues/2): the issue has no platform or reproduction details, and no Gallery-specific root cause or fix is confirmed. The Docs optimization above is separate; keep #2 open pending reporter details.

## 0.6.0 - 2026-11-13

- Add reusable native `MarkdownDocument` / `MarkdownView` with batched cached parsing, streaming updates, rich and continuous-text selection, safe external links and host-approved image resources; keep Docs-only directives outside the SDK.
- Add keyed `Transcript` virtualization over official GPUI tail following, with prepend/reorder anchors, row-height invalidation and serializable session positions.
- Add Input-based `Composer` drafts, references, attachments, candidates, host clipboard payloads and Send/Stop events; preserve native composition and editable busy drafts.
- Add a real VT/ANSI `Terminal` backed by MIT-licensed `vt100 = 0.18.1`, with native cursor/cell rendering, selection, scrollback, input bytes, composition and resize events; no built-in PTY or process execution.
- Reduce Terminal rendering from one native element per cell to one fixed-width shaped canvas per row, and anchor its native text-input handler to the viewport so ordinary text and IME input reach the focused terminal.
- Add native `DiffDocument` / `DiffView` line and hunk review with file locations and host-owned accept/reject events; never modify files implicitly.
- Extend `Splitter` and `DockLayout` with persistent pointer/keyboard sizing, controlled bounds, grouped tabs/splits, host-owned content, and validated layout restoration.
- Add `CommandPalette` with stable command IDs, Unicode fuzzy ranking, dynamic availability, virtualized full-width row hit targets, focus restoration, and menu-compatible GPUI action dispatch.
- Make editor completions undoable and actionable through keyboard/pointer input, reject stale host results, add search navigation/replacement, and use shared render snapshots with delta-based history.
- Route editor Escape/F3 through context-scoped GPUI actions, keep unhandled searches available to the host, and redraw caret/IME state independently of blink timers. Targetless commands remain unavailable without panicking.
- Keep editor hit-test/IME geometry viewport-current, invalidate delta history after unrecorded text changes, preserve redo for identical-text operations, batch line-index rebuilds, and fix outdent caret mapping and initial diagnostics.
- Restore cold Transcript anchors using literal pixel offsets, cancel stale Input composition on draft replacement, make Markdown controls keyboard-operable, and align selection geometry with glyphs. Isolate Splitter/Dock drag owners, refresh measured bounds, retain controlled callbacks after resets, and reject exhausted Dock identities without partial mutation.
- Add Gallery/Docs examples, compile-checked snippets and assertion-driven native smoke scenarios; separate the public MarkdownView SDK page from the Docs-private renderer. Verify Linux/Xwayland rendering and Markdown clipboard selection without claiming Composer system paste, real IME candidate windows or cross-platform validation.
- Fill split Dock leaf drop regions so cross-group movement can collapse emptied groups while retaining host-owned content and layout restoration.

## 0.5.1 - 2026-11-11

- Sign tagged RPM releases with the existing native `cargo-generate-rpm` backend before checksums, verify both font variants against an isolated publisher keyring, and publish the public key/fingerprint after matching an independently pinned fingerprint. Missing RPM credentials or failed signatures block publication, including when unsigned releases are otherwise allowed. Strict non-tag builds use the same signing path; dry-run/Docs-only readiness does not require RPM secrets, and stale artifacts are not marked signed.
- Explain explicit RPM key trust and signature checks in updater/install plans for Fedora/RHEL and openSUSE/SLES; never auto-import keys, re-sign downloaded packages, or disable verification. Fedora's local `rpmbuild` autosign helper is not a release trust mechanism.
- Document one-time RSA-4096 publisher-key generation/export, repository Actions Secret/Variable setup, independent fingerprint publication, encrypted offline backup, renewal, and rotation in both READMEs, the signing policy, and Docs.
- Remove an obsolete README word blacklist that rejected valid publisher-trust terminology in the RPM signing documentation.

## 0.5.0 - 2026-11-10

- Update official Zed GPUI, platform backends, and collections to `f0d896f2c18a543335eb155f729d36b624695591`, including upstream text layout and focus navigation optimizations and a macOS pasteboard fix. GPUI remains at package version `0.2.2`.
- The updated GPUI raises the macOS runtime minimum to macOS 12; use the macOS 26 SDK for source builds.
- Build and runtime-verify macOS releases on the supported macOS 26 runner; select Xcode 26.0.1 with a macOS 12 deployment target.
- Make the Clipboard examples interactive: edit a source value, copy it to the native clipboard, then read it back to see the actual result.
- Add non-destructive filtering, stable display sorting, and contextual header/empty/footer slots to `TodoList`. Retain item identity and stop forwarding events from removed rows.
- Toggle TodoItem completion on primary press without waiting for a click release; keep trailing actions outside the toggle area.
- Disable text selection in Todo renderer examples so clicks on titles, descriptions, and check marks reach the item instead of being consumed by text selection.

## 0.4.0 - 2026-10-14

- Pin official Zed GPUI, platform backends, and collections to `1de16662ee0d704285c524a7753f0f8858e86abf`; update only the upstream dependency closure rather than unrelated workspace dependencies.
- Add opt-in `MotionSpring` Gentle/Snappy/Bouncy presets over GPUI's velocity-preserving spring API, numerical regression tests, a compile-tested rustdoc example, and an interruptible Gallery demonstration.
- Replace Switch's hand-rolled thumb interpolation with a stable-ID native GPUI spring so rapid toggles preserve position and velocity; retain the old public motion helpers for compatibility.
- Correct custom macOS frame setup to use app-owned titlebar dragging without disabling native window movement/tiling; reset the flag when returning to system frames. This fixes a previously missed official API integration, not a newly introduced upstream field.
- Inherit upstream virtualized-list, wrapped-text hit-testing, SVG rasterization, dynamic/fallback font, IME, Wayland/X11, Windows and macOS fixes. Document touch/headless/visibility/display/profiler capabilities without adding web or mobile runtime paths.
- Refresh bilingual README, Docs adoption manifests, facade rustdoc, release checklist, and canonical CTX pointers. Remove the unused workspace `unexpected_cfgs` warning suppression.
- Platform runtime coverage and compatibility risks are recorded in `docs/gpui-latest-upgrade.md`; release metadata and validation are now prepared for v0.4.0.

## 0.3.0 - 2026-08-20

- Upgraded the workspace to the official Zed GPUI revision `492acd6c815cbe8c7366d54e6092341340afa6c7`, including the new animation, accessibility, and native platform API integrations documented in `docs/gpui-latest-upgrade.md`.
- Added the reusable `Accessible` API and synchronized icon rotation behavior for native loading indicators.
- Fixed nested navigation scrolling so Gallery and Docs sidebars keep smooth, bounded scrolling without competing scroll containers.
- Fixed Docs icon-library catalog pages so their virtualized grids fill the current window viewport and follow window resizing.

## 0.2.9 - 2026-07-06

- Added Docs navigation search and kept the Gallery navigation search constrained to a single line.
- Improved CodeEditor configuration demos and cursor/selection behavior, including tab rendering and explicit config application.
- Fixed stale selectable CodeBlock ranges across Docs so normal text/code selection clears without requiring multi-select modifiers.
- Fixed Gallery and Docs Windows executable resources to embed their app-specific icons instead of the generic repository icon.
- Consolidated design documents under `design/` and refreshed the icon system design document for all bundled icon libraries and the icon optimizer.

## 0.1.13 - 2026-06-28

- Renamed the old navigation/sidebar `Menu` API to `NavigationMenu`, `NavigationMenuItem`, `NavigationMenuNode`, `NavigationMenuGroup`, `NavigationMenuMode`, and `NavigationSubMenu`. This is the required migration path for apps that used the previous Menu navigation component.
- Reclaimed `Menu`, `MenuItem`, `MenuAction`, and `MenuBar` for GPUI-aligned platform menu descriptors and in-window fallback menu bars. `Menu::register(cx, menus)` now delegates to GPUI's official `App::set_menus` platform menu API.
- Added real GPUI platform menu registration to Gallery and Docs while keeping a visible Gallery header fallback menu bar for Linux/Windows/system-frame/custom-frame environments where platform menus are not automatically shown inside the window.
- Updated README and Docs with a detailed platform-menu versus in-window `MenuBar` behavior matrix and migration guidance.

## 0.1.12 - 2026-06-28

- Enforced Docs component examples as strict one-effect / one-code pairs for every component use case.
- Split repeated Docs live demos for Calendar, Carousel, InputTag, Mention, TreeSelect, and Watermark into precise per-example renderers.
- Added missing live effect previews for CodeBlock, QrCode decode, Timer result snapshots, Tour variants, Tray integration patterns, and Typography paragraph snippets.
- Added regression tests so component docs cannot batch multiple effects before code or ship code-only examples.

## 0.1.11 - 2026-06-27

- Added the high-level `Shell` component with customizable titlebar, header, left/right sidebars, main content, footer, and positioned overlay slots.
- Expanded Shell docs, README examples, and Gallery demos to show custom/system frame, inspector, embedded, themed, scrollable, and overlay layouts without raw GPUI layout in app examples.
- Replaced single font-family configuration with ordered UI/code fallback lists through `FontConfig::with_ui_families` and `FontConfig::with_code_families`.
- Updated Gallery and Docs font configuration to use ordered platform fallback stacks.

## 0.1.10 - 2026-06-27

- Fix Windows release packaging by avoiding duplicate MANIFEST resources when GPUI already provides the Windows application manifest.
- Keep Windows app resource scripts focused on icons and file metadata.

## 0.1.9 - 2026-06-27

- Publish all Liora SDK crates to crates.io with Cargo gpui registry fallback and downstream official Zed GPUI patch guidance.
- Update release-sdk workflow to publish SDK crates in dependency order and verify a patched downstream consumer.
- Refresh README, Docs, and release checklist for the crates.io + official Zed GPUI patch policy.

## 0.1.8 - 2026-06-26

- Embedded Lucide SVG icon payloads into `liora-icons-lucide` virtual asset paths so raw Gallery/Docs executables render icons on Windows and Linux without access to the build machine source tree.
- Kept `IconAssetSource` compatible with explicit filesystem SVG paths while adding embedded SVG payload loading for release binaries.
- Added regression coverage ensuring Lucide `IconName` paths do not contain `CARGO_MANIFEST_DIR` and can be loaded through `IconAssetSource`.

## 0.1.7 - 2026-06-26

- Hardened the Windows CI packager test again for TOML-escaped Windows backslashes before re-running the release pipeline.

## 0.1.6 - 2026-06-26

- Embedded a Windows Common Controls v6 manifest into both Gallery and Docs executables so GPUI's `TaskDialogIndirect` dependency resolves on Windows 11 raw `.exe` and debug builds.
- Added packaging validation coverage for the shared Windows manifest and app `build.rs` embedding paths.
- Fixed the packager config unit test to normalize path separators, matching Windows runner output.

## 0.1.5 - 2026-06-19

- Removed Gallery/Docs-specific bundled tray icon assets from the reusable `liora-tray` SDK crate.
- Moved runtime tray icons into app-owned `apps/liora-gallery/assets/tray-icons/` and `apps/liora-docs/assets/tray-icons/` assets.
- Redesigned Gallery and Docs desktop/taskbar package icons as distinct Liora-branded app marks.
- Registered Gallery and Docs user-scoped Linux desktop identities at startup so Wayland titlebar/taskbar icons resolve correctly during `cargo run`, not only after installing packages.
- Standardized runtime app icons as 8-bit RGBA hicolor PNGs across 16/24/32/48/64/128/256/512 sizes, install a user hicolor index, and refresh KDE/desktop service caches after registration.
- Extended installer packaging resources so Linux packages ship multi-size hicolor icons, Windows NSIS/WiX packages rely on the official cargo-packager shortcut templates, Windows app binaries embed their `.ico` resources, and portable Linux archives include desktop install/uninstall scripts.
- Added separate app-owned window header icons under `assets/status-icons/` and status-bar state icons under `assets/status-bar-icons/`, then rendered them in Gallery/Docs shells.
- Updated tray documentation and compile-checked snippets to demonstrate app-owned icon loading with `icon_from_png_bytes`.

## 0.1.4 - 2026-06-19

- Refactored `liora-updater` into a reusable GitHub Release updater SDK for any native app, with generic `AssetSelector`, `UpdateRequest`, and `PreparedUpdate` APIs.
- Kept Liora Gallery/Docs release selection as small presets on top of the generic updater instead of hard-coding those apps into the core flow.
- Added configurable checksum asset names and documented custom updater integration for downstream applications.
- Updated Gallery and Docs About/update flows to use the generic updater request pipeline.

## 0.1.3 - 2026-06-19

- Added publishable `liora-updater` crate and default `liora::updater` facade export for GitHub Release checks, platform asset selection, cached downloads, SHA-256 verification, and explicit install plans.
- Added About / update panels to Liora Gallery and Liora Docs with startup auto-check/download behavior and user-visible install actions.
- Updated all workspace package versions and internal Liora dependency versions to `0.1.3`.
- Updated release/CI workflows to Node 24 action generations and included `liora-updater` in crates.io utility-crate publishing.
- Cleaned Rust warnings in component/app async update paths and form-control listeners.

## Unreleased

### Fixed

- 0.1.x dependency correction: switched every maintained Liora crate and app from non-official `open-gpui` packages / stale crates.io-only GPUI usage to official `zed-industries/zed` git dependencies pinned at `2c346f60a76fe3f0367ef924927f50a6efdf5718`; `gpui` and `gpui_platform` now share the same upstream revision, and app bootstraps use the official `gpui_platform::application()` entry point.
- Release and documentation guardrails now assert that SDK packages use official Zed GPUI only, with no renamed fork dependency, no `[patch]` section, and no bundled `third_party/zed` path dependency.

### Added

- 0.1.1 release refinement: added the one-stop `liora` facade crate, made `liora-packager` publishable for reusable packaging helpers, kept `xtask` repository-local, and shortened GitHub Release assets to distributable files plus one `SHA256SUMS.txt`.
- Release pipeline split: dedicated `.github/workflows/release-sdk.yml` audits git-only GPUI SDK metadata and publishes only crates.io utility crates (`liora-packager`, `liora-updater`) using `CRATES_IO_TOKEN`; native app packaging releases Docs as cross-platform raw executables and Gallery as raw executables plus planned installer formats.
- P21 release-candidate readiness: `docs/release-candidate-checklist.md`, explicit package metadata audit coverage, refreshed README/CHANGELOG/prompt/memory state, and regression tests for current release boundaries.
- P20 theme and interaction polish: System/Light/Dark theme mode, tokenized overlay/mask paths, custom window-frame controls, Theme Gallery page, and native Docs Theme page.
- P19 dashboard state/data-flow guidance: app-layer state, filter, refresh, empty/degraded/loading branches, and Dashboard State docs folded into Gallery/Docs.
- P18 dashboard polish/API ergonomics: Gallery shell dogfooding for search/filtering, theme switching, toast feedback, and Dashboard Patterns docs without exporting sample-only helpers.
- P17 dashboard dogfooding: realistic composition lessons were folded into Gallery and Docs; standalone dashboard app was intentionally removed.
- P16 adoption readiness: root README, CONTRIBUTING, crate-level Rustdoc entrypoints, native Docs Adoption Guide, and compile-checked adoption examples. Minimal app guidance is folded into Gallery/Docs instead of a standalone app.
- P15 quality hardening: workspace CI gates, docs/snippet checks, API consistency, theme/overlay cleanup, keyboard behavior, and CodeBlock/cache performance hardening.
- P14 deferred advanced controls: Carousel, Calendar, InputTag, Mention, Watermark, TreeSelect, Tour, VirtualizedTable, and VirtualizedTree.
- P13 component expansion: QR code, CodeEditor, SignalMeter, HeatBar, SegmentRatioBar, HorizontalList drag behavior, Timer, Label, Operation, and in-place enhancements for charts, Progress, Button, Tag, Radio, and Checkbox.
- P12 packaging/release readiness: `liora-packager`, `xtask package`, platform packaging resources, CI installer pipeline, grouped changelog generation, raw binary upload policy, signing policy, and release-readiness gate.

### Changed

- Preferred Liora app initialization now follows the operating system by default with `init_liora_with_mode(cx, ThemeMode::System)`. Fixed explicit themes such as `init_liora(cx, Theme::light())` remain supported for compatibility and product-specific policies.
- Gallery and Docs are the canonical maintained app surfaces. `examples/minimal-app`, `examples/dashboard-app`, `liora-minimal-app`, and `liora-dashboard-app` must not be reintroduced as separate products.

### Notes

- Liora remains pure Rust + GPUI native.
- GPUI-dependent SDK crates use the repository license file but remain git-only with `publish = false`; only `liora-packager` and `liora-updater` publish to crates.io. Installer/app metadata continues to use `LicenseRef-Liora` until the owner replaces the policy in `LICENSE.md` and package metadata.
- Formal app releases remain gated by owner-controlled signing/notarization credentials, protected release environments, real system-level install/uninstall smoke tests, and a matching `vX.Y.Z` GitHub Release tag.
