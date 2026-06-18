# Gallery Dogfooding

Aura does not maintain a separate dashboard sample app. The dashboard-app experiment was folded back into the maintained Gallery and Docs surfaces.

## What moved into Gallery

Gallery now carries the useful app-level behaviors that were validated during dogfooding:

- shell-level menu search/filtering;
- light/dark theme switching through the global Aura `Config`;
- refresh status and toast feedback;
- tray residency and close-to-tray flows;
- large component composition through existing component pages rather than a one-off sample binary.

The goal is that Gallery itself exposes adoption friction. If a real product behavior requires too much raw GPUI glue inside Gallery or Docs, improve Aura's reusable components/helpers instead of adding another sample app.

## Verification

```bash
cargo check -p aura-gallery
cargo run -p aura-gallery
cargo check -p aura-docs
cargo run -p aura-docs
```

Use Gallery for visual behavior and Docs for setup/reference. Standalone `minimal-app` and `dashboard-app` binaries are intentionally removed.
