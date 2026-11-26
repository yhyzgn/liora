# Liora Components

Native GPUI component library for desktop Rust applications.

## What it provides

It provides reusable native controls such as Button, Input, Select, Table, Grid, Shell, Sidebar, TitleBar, charts, overlays, menus, virtualized data views, and the `MarkdownDocument` / `MarkdownView` / `MarkdownEditor` and PDF/input controls. These are Rust + GPUI components, not WebView/Tauri/HTML runtime widgets.

## Quick example

```rust
use liora_components::{Button, Input, init_liora};

fn setup(cx: &mut gpui::App) {
    init_liora(cx);
    let _button = Button::new("Save").primary();
    let _input = Input::new().placeholder("Search");
}
```

## Recommended usage

- Prefer the top-level `liora` crate for ordinary applications.
- Depend on `liora-components` directly when you need fine-grained dependency control or are building another Liora crate.
- Keep application UI pure Rust + native GPUI; Liora crates do not require WebView, Tauri, HTML, CSS, or DOM runtime paths.

## Container headers

`Container` keeps a fixed 48px header by default. Use `.header_height_auto()` with wrapping header content to let it grow naturally while the body keeps the remaining height. A subsequent `.header_height(...)` or `.header_height_lg()` returns to fixed sizing; apply any minimum height to the header content itself. Gallery and Docs use this opt-in for their translated theme/language/frame toolbars and pair the layout example in their Container pages.

## Dropdown menus

`Dropdown::item(label, callback)` adds an executable menu item. Use `Dropdown::disabled_item(label)` for a non-executable item with no callback. Arrow/Home/End navigation skips it, and its native `MenuItem` node exposes the disabled state. Open menus focus the first enabled row on the next frame. A complete pointer click or Enter/Space press and release activates an enabled row once. Tab/Shift+Tab closes the menu and moves focus through the host’s normal tab order. This contract covers ordinary short menus; long-menu scrolling and revealing remain separate concerns.

## Carousel

`Carousel::autoplay(true)` schedules native slide changes without a host timer: 3000ms by default, with `interval_ms` clamped to 250–9,223,372,036,854ms for native scheduler compatibility. Hover pauses the whole carousel, including outside indicators; leaving starts a fresh interval. Without `.active_index(...)`, the component retains its own selection. Supplying that index makes selection controlled: handle `on_change`, retain the requested index and notify the host. Arrows, indicators and autoplay use that same callback. Give dynamically arranged instances distinct stable `.id(...)` values. Gallery and Docs retain their demo hosts; Docs displays the actual executable autoplay snippet.

Visible navigation arrows reserve horizontal content space so they do not cover titles, descriptions or custom slots. Hidden arrows and single-item carousels retain the normal content padding.

## Images

- Use `Image::embedded(include_bytes!("asset.png"))` for app-bundled artwork. The byte slice must be `'static`; decoded rasters are cached and shared with the preview overlay. Embedded images have no `ImageSource`, so `source()` returns `None` and the public `ImageSource::Url` / `File` variants remain unchanged.
- Use `Image::local(path)` for files resolved from the runtime filesystem. Remote `Image::new(url)` displays its loading placeholder while fetching, then its fallback if fetching or decoding fails.

## Alerts

- Give sibling `Alert`s distinct stable `.id(...)` values when they share an ancestor scope, especially when they are closable; the ID scopes their close controls. The parent owns dismissal and restoration through `on_close`.
- `AppWindowFrame` provides unhandled Tab / Shift+Tab traversal in both system and custom frame modes; enabled `Input` controls participate, disabled inputs are skipped, and child-consumed or modified Tab shortcuts remain with the child.


## PDF reader and reusable input controls

- `Input::soft_wrap(true)` opts multiline text into native width-dependent wrapping, with shared caret, pointer, visual-row navigation and IME geometry. Ordinary inputs retain their existing unwrapped behavior.
- `Input` exposes its existing native root with a stable entity-based ID (or your explicit `.id(...)`), text/password/multiline role, disabled state, and localized placeholder. `.aria_label(...)` supplies a separate localized name; without it, the placeholder is the name. Give inputs a meaningful name even when the value is empty.
- Non-password inputs supply native readable text runs, including empty and disabled values, with Unicode byte lengths, CRLF handling, hard-line boundaries and bounded word offsets. Password contents are omitted from both root values and text runs, even while visually revealed. This is text-reading metadata, not a complete assistive-technology editing contract: synchronized caret/selection, per-character geometry, soft-wrapped visual-line boundaries and AT-driven editing/selection/scroll actions are not supplied by this change. Native keyboard/IME editing remains unchanged.
- `PdfDocument::from_bytes(title, bytes: Vec<u8>) -> Result<PdfDocument, PdfDecodeError>` parses and rasterizes real PDFs through Hayro, a pure-Rust CPU backend. Call it on a background worker, then create `PdfReader` or replace its document on the UI thread. Input is limited to 64 MiB, retained page/thumbnail rasters to 128 MiB, and the longest page raster side to 2048 pixels. This is eager rendering, not a total-process memory cap or an on-demand page cache.
- `PdfDocument::from_bytes_with_preview(title, bytes, on_preview)` retains the eager decode contract with an early first-page callback on the decoding thread. For demand-driven loading, keep `PdfDecoder::new(title, bytes)` on a worker, obtain unloaded metadata with `document()`, and call `render_page(one_based)` only for needed pages. The same input, geometry and whole-document retained-raster budget preflight applies. Gallery/Docs use this path; `PdfReaderEvent::PagesRequested` drives visible-page requests and `set_decoded_page` installs page/thumbnail images without resetting zoom, rotation or scroll.
- `PdfReader` provides native page navigation, layouts, zoom, rotation, thumbnails and search/tool events. The host owns file I/O, downloads, search indexing, text selection and annotation persistence. Tools are state/events only, not a drawing or PDF-writeback engine. Hosts may also supply their own `PdfPage` images.
- `FindBar` owns native query input and `QueryChanged`/`Previous`/`Next`/`Closed` events; the host computes matches and supplies match status. Focused Enter/Shift-Enter/Escape request navigation/close while active IME composition and modified shortcuts remain with the input.
- `CandidateBar` displays host-supplied candidates (default 8 per page, configurable from 1 to 9). Explicit focus enables arrows, PageUp/PageDown, 1–9, Space for candidate commit, Enter for literal commit, and Escape for cancel. Selection/page events use zero-based indices; the host performs text mutation and clears composition. No input-method engine, dictionary, language model or platform IME integration is bundled.

Create the reader only after a successful decode; surface errors in the host UI and retain the previous document on failure. For integration examples, see the repository documentation linked below.

## Markdown editing

`MarkdownDocument` / `MarkdownView` provide cached native Markdown rendering, headings and footnote navigation, task-list checkboxes, highlighted spans, math, and restricted Mermaid diagrams. `MarkdownEditor` keeps canonical Markdown while owning a single native rich selection/input handler and virtual scroll viewport. The host supplies a bounded height. The syntax-driven contract is to type `# `, `**strong**`, `- ` and code fences directly and render immediately in the same editable surface without a formatting toolbar, external program call or mode switch. The shared Gallery/Docs demo retains representative content without append/reset controls; table cells retain native inputs and literal-text source edits.

Use `focus_source_offset(offset, window, cx)` and `set_selection(range, cx)` with canonical UTF-8 source offsets. Programmatic `insert_text(text, cx)` remains a literal-text transaction; native committed input accepts Markdown syntax. The former paragraph `rich_input_at() -> Entity<Input>` API has been removed. `input()` remains optional source-mode integration; `table_input_at()` is restricted to table cells. Retain a subscription to `MarkdownEditorChanged` for committed source; IME preedit is not a committed change.

The Quiet.md-informed migration and exact runtime acceptance are tracked in `.context/tasks/011_quiet_markdown_native_20261117.md`. Syntax typing passed 112 focused Markdown tests, shared demo typing/scrolling tests, native Xwayland captures and real Linux XIM checks. This is not a claim of full Typora parity or Windows/macOS validation. Standard Markdown does not persist merged cells, columns, colors or whiteboards without an explicit extension format.

## Timers

`Timer::tick_interval` controls refresh scheduling for running, unfinished timers: the default is 250ms and requested intervals are clamped to at least 16ms. Timers in one window share its earliest pending wake, so a faster sibling may refresh slower timers sooner. Active timers schedule the next wake when rendered; paused, finished or removed timers do not renew it. A pending wake can still retire after unmount or window close; other active timers in the same window keep running.
Intervals beyond the platform clock range do not schedule a new wake or displace an existing sibling wake. A retained Timer's first render preserves its original start time, even when its countdown has already finished.

The Gallery/Docs Timer examples share a retained native host with Start / Pause / Resume / Reset. The host saves `snapshot().elapsed` before pausing and creates a fresh retained Timer on resume, preserving the countdown duration. `.paused()` only configures the display; it does not save elapsed time. Retain the returned view across renders, and reset a completed countdown before restarting it.

Stable Timer IDs reuse app-global start epochs. Pending refresh work is bounded by the windows with an outstanding wake, but epoch storage is not reclaimed by this scheduling change; creating unbounded timer identities can still grow that existing cache.

## GPUI dependency note

Liora uses the repository-maintained GPUI derivative described in [the source maintenance boundary](../../third_party/gpui/README.md), based on official `zed-industries/zed`. Follow the root README for the exact immutable Liora Git revision and application-root patch. Do not mix it with official GPUI or other forks; SDK manifests must not contain developer-local paths.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

