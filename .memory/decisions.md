# Architecture Decisions

## ADR-009: No `Liora` Prefix on Any Public Type

**Decision**: Remove `Liora` prefix from ALL public types. Components: Button, Icon, Link, Text.
Theme/config: Theme, Config, ContextExt, ElementExt, ColorPalette, Spacing, Radius, FontSize.

**Rationale**:
- `liora_theme::Theme` is already namespaced by the crate
- No conflict with GPUI types (verified at compile time)
- Consistent with component naming decision

## ADR-008: Component API — RenderOnce + IntoElement (codex Paradigm)

**Decision**: All components implement `RenderOnce` + `IntoElement` (via `Component::new(self)`). Theme is read from `cx.global::<Config>().theme` inside `render()`, never passed as a parameter.

**Rationale**:
- Eliminates `.build(theme)` anti-pattern
- Theme is global state — no reason to thread it through every component
- Component::new() wrapper allows direct use in .child() and vec![]
- Matches GPUI's native rendering lifecycle

## ADR-001: Builder Pattern for Component API (superseded by ADR-008)

**Decision**: All components follow `Builder::new().method1().method2().build(&theme)` pattern.

**Rationale**:
- Matches Element-Plus's chainable attribute API
- Type-safe at compile time (no runtime prop validation needed)
- Theme passed explicitly to avoid implicit coupling
- Components are stateless builders, state managed by parent views

## ADR-002: Global Theme via GPUI Global

**Decision**: Theme injected via `cx.set_global(Config{})` and read via `cx.global::<Config>()`.

**Rationale**:
- GPUI's Global mechanism is O(1) and thread-safe
- Similar to Vue's provide/inject pattern
- No need for context threading through component trees
- App-level init once, available everywhere

## ADR-003: Feature Strategy for GPUI Dependencies

**Decision**: Workspace defines `gpui` with `default-features = false`. Library crates inherit (no platform features). App crates explicitly enable `wayland/x11/font-kit`.

**Rationale**:
- Library crates only need type definitions
- App crate controls which platform backend to use
- Avoids unnecessary platform-specific compilation in libraries

## ADR-004: Demo Registration System

**Decision**: Gallery uses a static `registry() -> Vec<DemoEntry>` pattern with function pointers.

**Rationale**:
- Adding a component demo = 2 file edits (demo file + registry)
- Auto-organized by Category enum
- No runtime registration overhead
- Type-safe function pointer dispatch

## ADR-005: Monorepo Workspace Structure

**Decision**: Single Cargo workspace with `crates/` (libraries) and `apps/` (applications).

**Rationale**:
- Shared dependency management
- Cross-crate refactoring is trivial
- Single `cargo check/build/test` for all crates
- Standard Rust ecosystem pattern (matches Zed's structure)

## ADR-006: Component ↔ Theme Decoupling

**Decision**: Components receive `&Theme` via `.build(&theme)` parameter, not via implicit Global read.

**Rationale**:
- Pure function: same theme + same props = same output
- Testable without GPUI context
- Allows theme override per component instance
- No hidden dependency on global state in component code

## ADR-007: `.into_any_element()` Return Type for Gallery Demos

**Decision**: Demo functions return `gpui::AnyElement` for type-erased registry storage.

**Rationale**:
- `impl IntoElement` is opaque and can't be stored in function pointers
- `AnyElement` provides uniform type for demo registry dispatch
- Small overhead acceptable (demos are dev-only, not production)

## ADR-010: Popper Foundation — Placement, Flip, and Portal

**Decision**:
- Implement 12 standard placements (Top/Bottom/Left/Right + Start/End/Center).
- Implement `calculate_position_with_flip` which prioritizes flipped position if original overflows viewport.
- Clamping is used as a final fallback to keep elements within viewport.
- Portals are rendered via a global `Portal` stack and a top-level `PortalLayer` in the main view.

**Rationale**:
- 12 placements match standard UI libraries (Popper.js, Element Plus).
- Flipping is the most intuitive overflow behavior for tooltips and menus.
- Global Portal stack allows components to "teleport" content to the top layer regardless of parent nesting or overflow constraints.
- `PortalLayer` at the end of root render ensures portals are always on top without complex Z-index management for simple cases.

## ADR-011: Built-in Unique ID for Every Control

**Decision**: Every Liora component MUST generate a default globally-unique Element ID at construction time. Users MAY override via `.id(...)` but MUST NOT be required to set one for correct behavior. The unique ID is generated from a global atomic counter (`AtomicU64`).

**Rationale**:
- GPUI `InteractiveElement` requires unique `.id()` for hit-testing, event dispatch, and state management.
- Multiple instances of the same component type with identical IDs (e.g. from a loop or helper function) cause event conflicts, silent failures, and lost hover/cursor feedback.
- Sessions 24-51 repeatedly encountered ID-conflict bugs (Rate hover, Menu/Tabs/Pagination/Segmented/Dropdown multi-instance).
- The library should guarantee correctness by default — not push the burden onto the consumer.

**Implementation**:
- `liora-core` provides `next_unique_id() -> u64` backed by a static `AtomicU64`.
- Components call `unique_id("component-type")` → returns `SharedString` like `"button-42"`.
- Internal interactive sub-elements use `"{component-id}-{role}"` (e.g. `"button-42-icon-start"`).
- Users can override: `Button::new("X").id("my-custom-id")`.

## ADR-012: Gallery Demo Must Be Self-Contained (No Raw GPUI Primitives in Demos)

**Decision**: Gallery demo files (`apps/liora-gallery/src/demos/*_demo.rs`) and gallery framework files (`main.rs`, `category.rs`) must use only Liora library controls for layout and styling. Direct use of GPUI primitives (`div()`, `px()`, `rgb()`, etc.) in demo code is forbidden. If a needed layout/style pattern has no corresponding Liora control, the control should be added to the library.

**Rationale**:
- Demos serve as the canonical usage reference — they should demonstrate Liora's API, not GPUI's.
- Eating our own dog food surfaces missing controls and API pain points.
- Consistent visual appearance across all demos.
- Lowers the barrier for new users: copy-paste from demos and it works.

## ADR-008: Components Read Theme from GPUI Global (duplicate — kept for reference)

**Decision**: Liora components implement GPUI `IntoElement` + `RenderOnce` and read `Config.theme` from `App` during render. Business usage is `Button::new("Save").primary()` without `.build(&theme)`.

**Supersedes**: ADR-001's `.build(&theme)` conversion detail and ADR-006's explicit theme-parameter policy.

**Rationale**:
- Preserves the global theme model established by `init_liora(cx, Theme::...)`.
- Avoids threading `&Theme` through every component call and demo registry.
- Matches GPUI/Zed component patterns where `RenderOnce::render(..., cx: &mut App)` resolves theme and style.
- Keeps chainable builder API while making components directly usable as `IntoElement` children.

**Escape hatch**: Low-level private helpers may accept `&Theme` for tests or style extraction, but public component use should not require theme parameters.

## ADR-013: P8 Documentation Runs Inside Native Liora Gallery

**Decision**: P8 abandons the previous VitePress/Web documentation plan. Liora's official documentation now runs in a separate `liora-docs` GPUI native application, while `liora-gallery` remains the component showcase. Markdown is an input format only; rendering output must be Liora/GPUI native elements.

**Rationale**:
- The project goal is a native Rust/GPUI component library; the official documentation surface should dogfood Liora rather than fork into a Web stack.
- Rich text rendering should validate and improve Liora's own Typography/Layout primitives.
- Live component examples must remain real GPUI/Liora view nodes with native hover/click behavior.

**Implementation constraints**:
- Use `pulldown-cmark` for Markdown AST/Event parsing.
- Use Liora Typography/Layout components for all layout, styling, wrapping, and scrolling.
- Implement Markdown rendering as a stack-based state machine in `liora-docs`.
- Store authored documentation as one Markdown file per page/component under `apps/liora-docs/content/pages/`.
- Store code examples outside Markdown under `apps/liora-docs/content/snippets/<page>/<case>.rs`; Markdown code fences reference them with `src="..."`.
- Support Live Demo injection syntax such as `::LioraDemo{component="Button"}::` by inserting real Liora components.
- Do not introduce a VitePress app, Web documentation runtime, or cross-runtime rendering path.

**Edition note**: The new architecture can be understood as Rust 2021-compatible in language semantics, but the existing workspace remains edition 2024 unless a separate migration decision changes it.

## ADR-014: P10 Charts Use Native GPUI Canvas and Path Pipeline

**Decision**: Liora chart components must be implemented as native Rust/GPUI components using GPUI `canvas`, `PathBuilder`, `Window::paint_path`, `Window::paint_quad`, and Liora/GPUI text primitives. Web chart engines, WebView, DOM/SVG DOM, WASM, and remote image rendering are forbidden.

**Rationale**:
- Liora is a native GPUI component library; charts must validate the same rendering stack as the rest of the library.
- Native charts can share Liora theme tokens, interaction patterns, docs snippets, and Gallery self-bootstrapping.
- GPUI already exposes enough low-level drawing primitives for line/area/bar/pie/ring/sparkline MVPs.

**Reference policy**:
- GPUI official/local source is the primary reference.
- `https://github.com/vicanso/zedis` may be used as a case study because its Metrics page draws Area/Line/Bar charts through GPUI `canvas` and separates scale, axis/grid, and shape layers.
- Liora must not copy zedis' public API or depend on its component crate; implement Liora-owned chart primitives and tests.

**Implementation constraints**:
- First-class components: `LineChart`, `AreaChart`, `BarChart`, `PieChart`, `RingChart`, `Sparkline`.
- Shared infrastructure should cover linear/band/point scales, axis/grid/ticks, legend, tooltip/hover hit testing, and theme palette selection.
- Every chart gets Gallery demo, Docs page, external `.rs` snippets, and scale/shape/builder tests.

## ADR-015: P11 Tray Uses an Liora Facade over tray-icon/muda

**Decision**: Implement system tray support in a new `crates/liora-tray` crate rather than exposing `tray-icon`/`muda` directly from apps or vendoring their source.

**Rationale**:
- Keeps GPUI apps focused on window lifecycle commands while isolating platform-specific tray/menu APIs.
- Allows Liora to provide stable `TrayCommand` routing for Show/Hide/Toggle/Quit/SetIcon/Custom actions.
- Supports future customization behind one facade without copying Tauri-maintained source today.

**Implementation constraints**:
- Use `tray-icon` and its `tray_icon::menu` re-export for `muda` menu types.
- Support runtime icon changes, tooltip/visibility updates, checkbox state sync, separators, and recursive 2nd/3rd/N-level submenus.
- Tray-enabled GPUI apps must use `QuitMode::Explicit` and retain the `LioraTray` handle for process lifetime.
- Normal Gallery/Docs examples should preview config and command behavior without creating intrusive OS tray icons.

## ADR — Root `assets/` for README-facing static resources (2026-06-19)

README-facing static resources such as logos, social/SEO metadata notes, and other project presentation attachments live under the repository root `assets/` directory. Do not place these assets under `docs/`, because `docs/` may be cleaned as an AI-agent working/documentation area in future maintenance passes. README links should use stable `assets/...` paths.

## ADR — Unified high-level Liora application initialization (2026-06-19)

Downstream apps should use `liora_components::init_liora(cx)` as the default one-line setup. It initializes core/theme state with `ThemeMode::System`, component global services such as `MessageManager`, and all app-level component key bindings. `liora_components::init_liora_with_mode(cx, mode)` remains available for explicit `System`/`Light`/`Dark` startup mode selection. Lower-level `liora_core::init_liora(...)` and `liora_core::init_liora_with_mode(...)` remain core/theme-only APIs for advanced crate-local use.
