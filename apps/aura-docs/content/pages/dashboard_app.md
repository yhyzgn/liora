# Dashboard App

`examples/dashboard-app` is the P17 dogfooding example. It is not another isolated component demo; it is a realistic native GPUI screen that composes many Aura primitives together.

Run it with:

```bash
cargo run -p aura-dashboard-app
```

## What it covers

- Native GPUI window bootstrapping with `init_aura` and `Theme::light()`.
- Global message/toast setup through `MessageManager::init`.
- Key binding registration for inputs, select, switch, code blocks, text, and titles.
- Dashboard layout with header, filters, metric cards, charts, progress panels, table, and runbook code block.
- Real composition of `Card`, `Space`, `Statistic`, `Tag`, `Input`, `Select`, `Switch`, `Button`, `LineChart`, `BarChart`, `Progress`, `Table`, `CodeBlock`, `Text`, and `Title`.

## Dogfooding checklist

When changing Aura components, use this example as a smoke screen:

```bash
cargo check -p aura-dashboard-app
timeout 10s cargo run -p aura-dashboard-app
```

The app should compile, start a native window, and remain responsive long enough for manual inspection. If a component API change makes this app awkward to update, record that as API friction rather than hiding it in the example.

## Current observations

The first dogfooding pass validated that Aura can already build a management-dashboard style screen from existing components. The most useful future improvements are API ergonomics rather than missing primitives:

- layout helpers could expose more responsive grid presets;
- table cells are flexible but verbose for common text/tag rows;
- charts are usable in cards, but dashboard presets could reduce repeated options;
- app startup still requires manual key binding registration, so docs must keep that list visible.
