# Dashboard State Patterns

Dashboard-like state belongs in the app layer, not in `aura-components` as a sample screen. Aura components should make that state easy to render.

## Model first

Keep dashboard data in explicit Rust structs before rendering components:

- model data owns metrics, chart series, service rows, progress values, and a revision number;
- filter state owns search query, region, and alerts-only toggles;
- status enums model loading, ready, empty, and degraded branches.

This keeps rendering code focused on Aura composition and makes filtering/refresh logic unit-testable.

```rust
let visible = services
    .iter()
    .filter(|service| filters.matches(service))
    .collect::<Vec<_>>();
```

## Control wiring

Long-lived controls remain `Entity<T>` fields. Their callbacks update the parent view and call `cx.notify()`:

```rust
input.set_on_change(move |value, cx| {
    view.update(cx, |screen, cx| {
        screen.query = value.to_string();
        cx.notify();
    });
});
```

Gallery uses the same pattern for shell search/filtering and theme switching. Docs explains the pattern; neither requires a separate dashboard binary.

## State branches

Use ordinary Aura components for state branches:

- loading: simple text or `Loading`;
- ready: summary text and content;
- empty: `Empty` component;
- degraded: warning text plus table/details.

No special dashboard runtime is needed. State is normal Rust data; UI is normal Aura composition.

## Verification

```bash
cargo check -p aura-gallery
cargo test -p aura-gallery
cargo check -p aura-docs
```
