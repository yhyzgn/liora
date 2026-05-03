# Architecture Decisions

## ADR-008: Component API — RenderOnce + IntoElement (codex Paradigm)

**Decision**: All components implement `RenderOnce` + `IntoElement` (via `Component::new(self)`). Theme is read from `cx.global::<AuraConfig>().theme` inside `render()`, never passed as a parameter.

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

**Decision**: Theme injected via `cx.set_global(AuraConfig{})` and read via `cx.global::<AuraConfig>()`.

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

**Decision**: Components receive `&AuraTheme` via `.build(&theme)` parameter, not via implicit Global read.

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

## ADR-008: Components Read Theme from GPUI Global

**Decision**: Aura components implement GPUI `IntoElement` + `RenderOnce` and read `AuraConfig.theme` from `App` during render. Business usage is `AuraButton::new("Save").primary()` without `.build(&theme)`.

**Supersedes**: ADR-001's `.build(&theme)` conversion detail and ADR-006's explicit theme-parameter policy.

**Rationale**:
- Preserves the global theme model established by `init_aura(cx, AuraTheme::...)`.
- Avoids threading `&AuraTheme` through every component call and demo registry.
- Matches GPUI/Zed component patterns where `RenderOnce::render(..., cx: &mut App)` resolves theme and style.
- Keeps chainable builder API while making components directly usable as `IntoElement` children.

**Escape hatch**: Low-level private helpers may accept `&AuraTheme` for tests or style extraction, but public component use should not require theme parameters.
