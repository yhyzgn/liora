# Architecture Decisions

## ADR-009: No `Aura` Prefix on Any Public Type

**Decision**: Remove `Aura` prefix from ALL public types. Components: Button, Icon, Link, Text.
Theme/config: Theme, Config, ContextExt, ElementExt, ColorPalette, Spacing, Radius, FontSize.

**Rationale**:
- `aura_theme::Theme` is already namespaced by the crate
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

**Decision**: Every Aura component MUST generate a default globally-unique Element ID at construction time. Users MAY override via `.id(...)` but MUST NOT be required to set one for correct behavior. The unique ID is generated from a global atomic counter (`AtomicU64`).

**Rationale**:
- GPUI `InteractiveElement` requires unique `.id()` for hit-testing, event dispatch, and state management.
- Multiple instances of the same component type with identical IDs (e.g. from a loop or helper function) cause event conflicts, silent failures, and lost hover/cursor feedback.
- Sessions 24-51 repeatedly encountered ID-conflict bugs (Rate hover, Menu/Tabs/Pagination/Segmented/Dropdown multi-instance).
- The library should guarantee correctness by default — not push the burden onto the consumer.

**Implementation**:
- `aura-core` provides `next_unique_id() -> u64` backed by a static `AtomicU64`.
- Components call `unique_id("component-type")` → returns `SharedString` like `"button-42"`.
- Internal interactive sub-elements use `"{component-id}-{role}"` (e.g. `"button-42-icon-start"`).
- Users can override: `Button::new("X").id("my-custom-id")`.

## ADR-012: Gallery Demo Must Be Self-Contained (No Raw GPUI Primitives in Demos)

**Decision**: Gallery demo files (`apps/aura-gallery/src/demos/*_demo.rs`) and gallery framework files (`main.rs`, `category.rs`) must use only Aura library controls for layout and styling. Direct use of GPUI primitives (`div()`, `px()`, `rgb()`, etc.) in demo code is forbidden. If a needed layout/style pattern has no corresponding Aura control, the control should be added to the library.

**Rationale**:
- Demos serve as the canonical usage reference — they should demonstrate Aura's API, not GPUI's.
- Eating our own dog food surfaces missing controls and API pain points.
- Consistent visual appearance across all demos.
- Lowers the barrier for new users: copy-paste from demos and it works.

## ADR-008: Components Read Theme from GPUI Global (duplicate — kept for reference)

**Decision**: Aura components implement GPUI `IntoElement` + `RenderOnce` and read `Config.theme` from `App` during render. Business usage is `Button::new("Save").primary()` without `.build(&theme)`.

**Supersedes**: ADR-001's `.build(&theme)` conversion detail and ADR-006's explicit theme-parameter policy.

**Rationale**:
- Preserves the global theme model established by `init_aura(cx, Theme::...)`.
- Avoids threading `&Theme` through every component call and demo registry.
- Matches GPUI/Zed component patterns where `RenderOnce::render(..., cx: &mut App)` resolves theme and style.
- Keeps chainable builder API while making components directly usable as `IntoElement` children.

**Escape hatch**: Low-level private helpers may accept `&Theme` for tests or style extraction, but public component use should not require theme parameters.

## ADR-013: P8 Documentation Runs Inside Native Aura Gallery

**Decision**: P8 abandons the previous VitePress/Web documentation plan. Aura's official documentation and component showcase will run entirely inside `aura-gallery`, a GPUI native application. Markdown is an input format only; rendering output must be Aura/GPUI native elements.

**Rationale**:
- The project goal is a native Rust/GPUI component library; the official documentation surface should dogfood Aura rather than fork into a Web stack.
- Rich text rendering should validate and improve Aura's own Typography/Layout primitives.
- Live component examples must remain real GPUI/Aura view nodes with native hover/click behavior.

**Implementation constraints**:
- Use `pulldown-cmark` for Markdown AST/Event parsing.
- Use Aura Typography/Layout components for all layout, styling, wrapping, and scrolling.
- Implement Markdown rendering as a stack-based state machine in `aura-gallery`.
- Support Live Demo injection syntax such as `::AuraDemo{component="Button"}::` by inserting real Aura components.
- Do not introduce a VitePress app, Web documentation runtime, or cross-runtime rendering path.

**Edition note**: The new architecture can be understood as Rust 2021-compatible in language semantics, but the existing workspace remains edition 2024 unless a separate migration decision changes it.
