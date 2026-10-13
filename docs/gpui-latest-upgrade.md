# GPUI upstream refresh — 2026-10-13

## Verified source snapshot

- Official repository: <https://github.com/zed-industries/zed>.
- Previous pin: `492acd6c815cbe8c7366d54e6092341340afa6c7` (2026-08-10).
- Selected official `main`: `1de16662ee0d704285c524a7753f0f8858e86abf`
  (commit timestamp `2026-10-13T14:09:05Z`). This means latest at intake,
  not a promise that a moving branch remains unchanged after intake.
- Final upstream recheck observed main `d3267a39ff900f114711c26720c7cbf12c41f204`
  (`2026-10-13T15:07:06Z`). Changes after intake affect agent-server,
  agent UI and terminal-view code only. A source diff confirms no
  `crates/gpui*`, collections or workspace manifest/lock content change; retain the
  tested intake pin rather than endlessly chase unrelated Zed app commits.
- Source evidence: official `git ls-remote ... refs/heads/main`, the official
  clone's `git log old..new -- crates/gpui*`, and source diffs at those exact SHAs.
  The web search surface returned no usable page contents in this session;
  findings below are grounded in upstream git source, not guessed release notes.
- GPUI's package version remains `0.2.2`; its git SHA defines this compatibility
  baseline. The refresh is released as Liora 0.4.0; tag and registry publication remain separate protected actions.

## Liora changes actually made

1. Pin workspace `gpui`, `gpui_platform`, `collections` and the Zed transitive
   crate closure to the same official SHA. Use a targeted Cargo update, retaining
   unrelated locked packages. Required changes include futures `0.3.34`, taffy
   `0.13`, strum `0.28`, platform accessibility adapters, shared `gpui_apple`, and
   upstream's `unicode-properties = 0.1.3` requirement. Unused dependencies removed
   upstream disappear from the lock; no new independent Liora dependency is added.
2. Add `MotionSpring::{Gentle, Snappy, Bouncy}` presets over GPUI `SpringConfig`
   and `SpringAnimation`. `animation_to(target)` is opt-in and preserves existing
   duration-based motion APIs. Tests exercise finite settling, frame partition
   equivalence, damping/overshoot and zero-time retarget continuity. The rustdoc
   example compile-checks `AnimationExt::with_spring` with a Liora `Space`.
3. Replace Switch's hand-rolled `thumb_from_checked` + `slide_snap` interpolation
   with GPUI `with_spring` and a stable thumb element ID. Rapid toggles now retain
   native spring position/velocity instead of restarting a duration animation;
   the old public motion helpers remain available for compatibility.
4. Add an interactive spring example in Gallery's Spinner page. Repeated clicks
   reuse one element ID while changing gap targets; GPUI owns animation state and
   velocity. The demo uses Liora layout controls, not a new runtime path.
5. Add `cargo run -p liora-gallery --example gpui_upgrade_smoke --locked`: a display-free native Linux/FreeBSD text/layout and spring reduced-motion smoke. It uses official headless windows, clears draw arenas correctly, and exits through the platform event loop; it does not claim GPU screenshot coverage.
6. Correct custom macOS window frame initialization: set `app_owns_titlebar_drag`
   and keep `is_movable` enabled on macOS, then reset ownership for system frames.
   This avoids conflating custom titlebar drag with disabled native move/tiling.
   **This field already existed in the previous GPUI pin**; the upgrade audit
   found a missing Liora integration. Other platforms keep the old movement policy.
7. Update bilingual README, native Docs adoption/architecture manifests, facade
   rustdoc, release checklist and CTX pointers. Remove the unused workspace
   `unexpected_cfgs = allow` declaration rather than hiding warnings.

## Upstream features and disposition

The listed commits are relevant examples from the entire pinned range, not a
claim that every Zed application change is an SDK feature.

| Official change (commit / PR) | Liora disposition |
| --- | --- |
| Spring animation (`8b1497dbd2`, #62778) | Explicit `MotionSpring` preset integration and Gallery dogfooding. Stable IDs preserve state; `.from(...)` controls initial entrance values. |
| Animation max FPS (`dd04a229dd`, #62579) | Available through the existing public `motion_animation` result's `.with_max_fps(...)`; no duplicate wrapper. |
| Inactive animation frame throttling (`511ac17036`, #62628) | Inherited default `WindowOptions::inactive_frame_interval`; apps can opt out. Liora frame helpers preserve this option. |
| Dynamic fonts (`9e6e14160c`, #63498; `b68add5b37`, #64204) | Existing font loading helpers delegate to GPUI and inherit its font-system improvements. No new installation policy or side effects in SDK initialization. |
| Binary SVG data (`ff9f114cd3`, #52319), exact-size SVG rasterization (`03c9c4e707`, #62770) | Rendering fixes inherited. Liora's virtual asset-path icon loading API remains unchanged; no speculative raw-data icon abstraction. |
| LineLayout split/paint (`2936989f1b`, #60831), ShapedLineCursor (`f792c2d72d`, #64185) | Available to native custom painters. No unnecessary rewrite of the current input/editor selection and UTF-16 mapping. |
| Touch events (#63373), long press (#63561), touch drag (#63645), tooltip long press (#63948), dominant-axis touch scrolling (#63553), platform scroll physics (#63505) | GPUI event/gesture/scroll pipeline inherited. Liora's own Tooltip portal implementation is **not** GPUI's built-in tooltip and does not gain long-press activation automatically. No claim of touch-native support for every Liora control; hardware/platform follow-up required. |
| TextInputConfiguration (#63264), editable range (#63294), character input preference (#64859) | Default GPUI input-handler behavior inherited; desktop Input/CodeEditor APIs unchanged. Software keyboard hints are not exposed as a new mobile/web Liora contract. |
| Window visibility (#64107), connected displays/window display tracking (#65059), refresh-rate-aware frame pacing (#65351) | Inherited; applications use GPUI's window/display APIs directly. No app scheduling policy injected into SDK. |
| Cross-platform headless window API (#64969), Linux headless WGPU rendering (#64718), Windows render-to-image (#63012) | Native headless layout smoke is integrated as an opt-in example; GPU screenshot capture remains future visual QA. Not silently enabled for normal Gallery/Docs launches; compilation does not prove offscreen GPU capture works on all machines. |
| Profiler/frame histogram/journal/hang monitor (#61871, #62779, #64944), inspector cost reduction (#64309), element ID hashing (#64209), test-support safety (#65043) | Runtime fixes inherited; benchmark/profiler/test instrumentation remains opt-in upstream. No telemetry/hang-monitor service is enabled by Liora. |
| Shared Apple services/rendering (#62649, #64990, #65058) | Dependency graph follows official upstream `gpui_apple`; no iOS app/support claim. |
| Web Canvas font fallback, IME mirror, Fetch streaming, web-thread/executor updates | Upstream target-specific lock metadata only. No WebView/Tauri/HTML/CSS/DOM/browser runtime or WASM Liora app is introduced. |

GPUI animation wrappers support app-level `App::set_reduce_motion(true)`;
Liora's existing fades/pulses/spinners and new **running** springs inherit it.
This does not automatically read OS accessibility settings. Application code
still owns preference selection and spring playback policy.

## Important inherited bug fixes

| Fix | Official evidence | Relevant Liora surface / validation boundary |
| --- | --- | --- |
| Far-down uniform-list item layout | `679ad7c304`, #65288 (2026-10-09) | Virtualized list/table/tree layouts; no downstream workaround added. |
| Wrapped-line hit testing past line width panic | `e71963a599`, #64672 (2026-09-23) | Text/paragraph/input/editor; compile/unit tests do not substitute for pointer-selection UI checks. |
| Closing punctuation/slash line wrapping | `4278ff36ef`, #62743 (2026-08-26) | Native typography and Docs wrapping behavior changes intentionally. |
| Linux fallback fonts disappearing | `d6358b2d3c`, #65269 (2026-10-07) | Theme/font loading, CJK/emoji fallback; requires visual font verification. |
| Pending input on blur and IME races | `f66ed399cd`, #63271; `5218009a2d`, #63311 | Input/Textarea/CodeEditor inherit core fixes. Real IME/composition testing remains separate. |
| Wayland demand-driven rendering, stale file drag reads, no-handler IME | `eb354c8d50`, #60690; `87a1ea30e8`, #64122; `4a217d5316`, #63776 | Native Linux backend fixes inherited; do not assume X11 smoke validates Wayland. |
| X11 Expose after GPU recovery; close callback state; WM_CLASS terminator | `f25434f3c5`, #64570; `d9ad6aff67`, #63089; `5dd0666dfb`, #62848 | Native shell/startup stability inherited. Recovery path needs a real GPU-reset test. |
| Atlas texture released panic | `96e95edac4`, #64623 | Images/icons and GPU atlas rendering inherited. |
| Windows mixed-DPI placement, default ligatures, linear/sRGB shader | `1d7e5f1d01`, #62859; `ac6818fb2d`, #62338; `b9419ae728`, #57543 | Windows native windows/text/color correctness; not runtime-verified on Linux. |
| Windows embedded-NUL clipboard; credential null handling; Restart Manager shutdown | `decbf641b1`, #64911; `8720fd1d09`, #64735; `7f2a2c3c3e`, #62987 | Platform clipboard/security/lifecycle inherited; no credential policy exposed by Liora. |
| macOS background blur, synthetic mouse drag termination, accessibility adapter release | `90d8514059`, #65361; `382ff7a64b`, #64175; `0eda7703f6`, #64143 | macOS shell/interaction/cleanup fixes; native macOS verification still required. |
| macOS hover/Space/fullscreen behavior | #64301, #58886, #64339 | Native chrome behavior inherited; separate from the explicit Liora drag-ownership fix. |

Official PR evidence is available at
`https://github.com/zed-industries/zed/pull/<PR-number>`; the exact range can be
reproduced with:

```bash
git log --no-merges --format='%h %cs %s' \
  492acd6c815cbe8c7366d54e6092341340afa6c7..1de16662ee0d704285c524a7753f0f8858e86abf \
  -- crates/gpui crates/gpui_platform crates/gpui_linux crates/gpui_macos \
     crates/gpui_windows crates/gpui_wgpu crates/gpui_web crates/gpui_apple
```

## Risks and mitigations

- **High — pre-1.0 compatibility / downstream pin mismatch.** All Zed crates
  and published SDK fallback patches must use the new SHA together. A registry
  `gpui 0.2.2` dependency without the official patch is not this tested baseline.
  No large Render/View signature migration was required by this range.
- **High — platform runtime coverage.** Linux validation cannot prove Windows,
  macOS, physical touch input, mixed-DPI/refresh displays, GPU recovery or headless
  capture. No cross-platform runtime readiness claim is made. Run native CI and
  targeted manual checks before release.
- **Medium — rendering/interaction drift.** taffy, line wrapping, fallback fonts,
  atlas and scroll/frame pacing changes may alter geometry, text or motion. Check
  long virtualized lists, CJK/emoji, fractional scaling, selection/IME and overlays.
- **Medium — spring state/overshoot.** Reuse stable IDs across renders. Bouncy
  values may overshoot; do not feed them directly into bounded opacity/progress
  or sizes that must remain positive. Existing duration helpers are not replaced.
- **Medium — custom macOS frame correction.** Native tiling/move semantics are
  restored in options, but actual titlebar drag/double-click and system/custom
  frame reopening require macOS smoke checks. The helper preserves caller frame
  interval settings.
- **Medium — touch/tooltip gap.** Upstream gesture recognition does not add a
  complete touch UX to custom mouse-based Liora controls/portal tooltips.
- **Low — historical snapshots.** `third_party/zed` remains historical test
  material, not the dependency source. Tests reading that snapshot are not proof
  that the new pin implements the same runtime behavior.

## Validation

Refresh results are recorded in `.context/tasks/003_gpui_refresh_20261013.md`;
native replacement follow-up results are recorded in
`.context/tasks/004_gpui_native_replacement_20261013.md`.
No previous task's test result is reused as proof. Strict workspace Clippy is
**not green**: unchanged `liora-icons-optimizer` (collapsible-if), `liora-packager`
(argument count / derivable Default) and `liora-tray` (return-type complexity)
block `-D warnings`. An isolated original-HEAD worktree reproduces the optimizer/
packager failures, and all five reported source locations are unchanged. No
suppression or lint downgrade is added. These are explicit validation debt,
not a claim that the entire workspace passes lint. Fresh workspace all-target
check and tests passed: **668 passed, 0 failed, 1 pre-existing ignored**, across
44 test suites including rustdoc tests. Compile-checked Docs, warning-denied
rustdoc, formatting, CTX, whitespace and release/package policy gates passed.
The native Linux headless smoke passed and exited with code 0: it exercises
text/layout and reduced-motion spring retargeting, not GPU rendering or
exhaustive visual/cross-platform interactions. The shell has no display server,
so real Gallery/Docs window checks remain unavailable. Release-readiness is
a local policy check, not a signed release.

## Rollback

Keep changes on `chore/gpui-latest-upgrade-20261013`. Revert this upgrade's pin,
lock, spring/demo/frame changes and documentation together to the previous SHA.
Do not change dependency source to a fork or add a local runtime patch. Do not use
broad resets that erase unrelated subsequent work.
