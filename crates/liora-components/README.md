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

## Cards

`Card::full_width()` fills its available width without overriding `.no_shrink()`, including in flex layouts. Optional titles take precedence over custom headers; body and footer retain theme-aware text and surface colors. `.hoverable()` changes the border and shadow without making the whole card an action; compose real buttons or links for actions.

Card identity defaults to its constructor call site and survives redraws. Give cards built by the same factory or loop distinct stable `.id(...)` values so their interactive children remain independent. Rendering copies only the required theme colors and radius, not the complete theme.

Gallery and Docs execute the same authored basic, footer-action and narrow scrolling examples. Footer messages report demo requests only; they do not save or undo data. Shared showcase wrappers use their static titles as identities, so sibling showcase titles must be distinct within their parent scope.

`Text::document` and `SelectableTextGroup` own their shared native selection, independently of nested Card/container identities. Mouse release retains the selected word or dragged range; Select All and Copy span the owning document, not an unrelated document with the same local ID under a different parent. Nonselectable documents do not allocate a selection owner.

## Cascader

Enabled Cascader triggers participate in Tab/Shift+Tab traversal. Enter/Space release toggles the popup; Escape closes it unless `close_on_escape(false)` is configured. The trigger exposes a native Button role, its displayed label, expanded state and disabled state. Disabled triggers are skipped and cannot activate, including when they retain focus from an earlier enabled frame. Hierarchical columns remain pointer-selected; filtered-result navigation below is not full hierarchical keyboard navigation or a complete keyboard combobox or screen-reader certification.

Disabled or loading nodes block selection through their descendants, including precomputed search results whose ancestors change before activation. Search excludes those subtrees; `is_selectable_path` also checks every ancestor. Lazy-data callbacks can still update loading branches through the mutable data APIs.

Option values retain Unicode, punctuation and path boundaries in popup identities. Treat `Cascader::popup_item_id` results as opaque and derive them through the helper rather than concatenating a string; callbacks still receive the original values unchanged.

Open popups borrow the current option columns during one-shot portal rendering instead of recursively copying their subtrees. Rows retain owned labels and click paths; host updates completed before the portal runs are reflected together in its columns, selection and geometry. This reduces CPU-side data copying, not a measured GPU or frame-latency guarantee.

Gallery and Docs execute the same five authored constructors with actual selection/clear feedback. The searchable example seeds `search_query("杭州")` and focuses a retained house Input when opened. Edit or clear the query to filter label paths or restore columns; a nonblank query with no matches shows inert `No results`. With stock Input key bindings and matching results, unmodified Down/Up highlight paths without moving the editor caret or changing the selected value: first Down highlights the first result, first Up the last, and navigation clamps at either end. Enter selects only an explicitly highlighted, still-valid result; otherwise it remains inert and never inserts a newline. Query/data changes and reopening reset the highlight. With those defaults, modified keys and active IME composition remain Input-owned. Consumer rebindings or direct dispatch of InputUp/InputDown/Enter follow normal GPUI action semantics; action capture does not recover the originating key modifiers. Escape respects active composition; otherwise it closes and restores trigger focus unless disabled by policy. Tab/Shift+Tab close and traverse relative to the trigger. Programmatic `set_search_query` also updates the retained editor while closed. The lazy example synchronously attaches fixed data without network I/O. The host owns persistence and asynchronous data loading.

Outside dismissal preserves an existing external focus destination; the active portal layer still occludes underlying controls, so it does not provide first-click-through. A short popup reserves its fixed search header before budgeting scrollable results. If the header itself cannot fit, existing clipping applies and results get zero height; this is not an always-visible-header guarantee.

## Calendar

`Calendar::show_adjacent_months(false)` preserves the six-week grid with same-height blank slots for adjacent dates. Those slots render no date or event content, have no hover feedback and cannot select; disabled adjacent dates remain blank too. Visible enabled dates invoke `on_select`, including when the surface redraws between pointer press and release; the host owns selected-date state and month navigation. Gallery and Docs execute the same authored event and range examples with date feedback.

Calendar identity defaults to its constructor call site and survives redraws. Use distinct stable `.id(...)` values for instances built at the same call site, such as a loop or factory. The authored examples assign independent event/range identities.

## Links

`Link` uses GPUI's native click lifecycle and platform URL opener. An enabled destination is reachable by Tab/Shift+Tab; an unmodified Enter press/release opens it, while Space does not. A keyboard-only focus outline preserves layout and survives redraws. Disabled and destinationless links are not tab stops and cannot activate. Destination-bearing links expose a native Link role, label, target URL and disabled state; this is not a complete cross-platform screen-reader certification. Gallery and Docs execute the same keyboard example.

Losing focus cancels a pending Enter activation. Disabling or destination removal cancels pending input; re-enabling cannot replay a stale pointer press. Handled Enter activation does not propagate into parent keyboard handlers.

A Space keydown cannot arm a later, unpaired Enter release.

## In-page anchors

`Anchor` uses retained native focus, Tab/Shift+Tab traversal and unmodified Enter press/release to scroll to painted targets. Space does not activate a target. Links expose a native Link role and name with a layout-neutral keyboard focus outline; `AnchorLink::href` names an `AnchorTarget` in the bound scroll handle, not an external URL. Missing targets leave the scroll position unchanged. Gallery and Docs execute the authored nested-navigation snippet; this is not complete screen-reader or cross-platform certification.

## Back to top

`Backtop` becomes visible when its bound `ScrollHandle` reaches `visibility_height` (200 by default, or 100 with `.visibility_height_sm()`). Its native Button exposes the name `Back to top`, Tab/Shift+Tab traversal and a layout-neutral 2px keyboard focus ring. A complete pointer click or unmodified Enter/Space press and release resets only that handle and requests a redraw; hidden buttons leave the tab order. Retain the entity in its host view. Gallery and Docs execute the authored basic/custom constructors, and Docs mounts the displayed container snippet. This is not full screen-reader or cross-platform certification.

`Flex::overflow_y_scroll()` contains wheel input, including at its scroll boundaries, so scrolling a nested viewport does not also move an ancestor list. Move the pointer outside that viewport to scroll the outer page.

## Badges

`Badge::value` accepts text or integer-count strings. `max(i32)` caps valid integers of any length at `max+`; noninteger text and uncapped labels are preserved. Zero remains visible. Dot mode skips count parsing, and `hidden(true)` keeps the child and its layout while skipping theme/count/marker work. Visible badges borrow the configured theme rather than cloning it. Gallery and Docs execute the same authored examples for all five semantic colors, counts/text, limits, dots and host-owned visibility toggling.

Badge text uses the shared `liora_theme::filled_foreground` black/white contrast policy, including for custom opaque semantic fills. Default light/dark semantic fills meet 4.5:1 text contrast. Translucent or gradient backgrounds require a host-owned compositing/contrast policy.

## Breadcrumb navigation

Ancestor items with `on_click` expose native Link names, retained Tab/Shift+Tab focus and Enter-only release-time activation with a layout-neutral keyboard focus outline. Space does not activate or arm Enter. The final/current item is inert even if a callback is supplied. Native pointer activation still requires a matching press/release. The SDK invokes host callbacks; it does not own navigation or open URLs implicitly.

`Breadcrumb::id` supplies a stable namespace for repeated identical trails. Default namespaces derive from labels and separator; item IDs include their label and position. Rendering copies only the needed theme colors and avoids per-item formatted ID strings. Gallery and Docs execute all five authored examples, including visible message feedback for activation.

## Affixed content

Retain `Affix` as an `Entity<Affix>` in its host view. Top/bottom offsets are relative to the window viewport, not the surrounding scroll container. Fixed/unfixed transitions schedule their functional follow-up frame even with decorative motion disabled; a stable Affix does not continuously request frames. Gallery and Docs execute the displayed 80px top / 20px bottom constructors. Docs also executes the complete container snippet with the retained entity as its real child.

While fixed, the portal follows horizontal anchor movement and the container's available width in both resize directions, preserving explicit content width constraints. Its measured height includes child margins and updates both the placeholder and bottom-edge placement. Correction frames settle once geometry is stable; ordinary vertical scrolling while fixed does not request a redundant geometry frame. `on_change` reports only fixed/unfixed transitions.

## Container headers

`Container` keeps a fixed 48px header by default. Use `.header_height_auto()` with wrapping header content to let it grow naturally while the body keeps the remaining height. A subsequent `.header_height(...)` or `.header_height_lg()` returns to fixed sizing; apply any minimum height to the header content itself. Gallery and Docs use this opt-in for their translated theme/language/frame toolbars and pair the layout example in their Container pages.

## Dropdown menus

`Dropdown::item(label, callback)` adds an executable menu item. Use `Dropdown::disabled_item(label)` for a non-executable item with no callback. Arrow/Home/End navigation skips it, and its native `MenuItem` node exposes the disabled state. Open menus focus the first enabled row on the next frame. A complete pointer click or Enter/Space press and release activates an enabled row once. Tab/Shift+Tab closes the menu and moves focus through the host’s normal tab order. This contract covers ordinary short menus; long-menu scrolling and revealing remain separate concerns.

## Carousel

`Carousel::autoplay(true)` schedules native slide changes without a host timer: 3000ms by default, with `interval_ms` clamped to 250–9,223,372,036,854ms for native scheduler compatibility. Hover pauses the whole carousel, including outside indicators; leaving starts a fresh interval. Without `.active_index(...)`, the component retains its own selection. Supplying that index makes selection controlled: handle `on_change`, retain the requested index and notify the host. Arrows, indicators and autoplay use that same callback. Give dynamically arranged instances distinct stable `.id(...)` values. Gallery and Docs retain their demo hosts; Docs displays the actual executable autoplay snippet.

Visible navigation arrows reserve horizontal content space so they do not cover titles, descriptions or custom slots. Hidden arrows and single-item carousels retain the normal content padding.

## Hover previews

`HoverCard` uses Popover's pointer-safe bridge between trigger and card, closing when the pointer leaves that region without scheduling delayed tasks. Use a focusable trigger such as `Button` for Tab and Enter/Space access. Pointer-opened previews do not move focus; unmodified, unconsumed Escape in their originating window dismisses them even while an unrelated editor retains focus. Raw keys consumed by IME/input handlers remain with those handlers. Unmounting the trigger retires the popup and backdrop. Gallery and Docs execute the same displayed two-trigger example.

## Messages / toast

`show_message`, the `toast_*` functions and the `toast_*!` macros use the same app-wide Message manager. Initialize Liora before macro use; call `message::render_messages(cx)` from the host root render and mount `liora_components::PortalLayer`. Each message expires independently three seconds after insertion, without hover pause or a close button. Its cancellable task belongs to the originating manager entity, so an old deadline cannot remove a replacement manager's new messages. Gallery and Docs execute the same displayed type/stack and positional/named-formatting snippets.

## Notifications

`show_notification` displays each app-wide notification for four seconds of unhovered time. Hover preserves its remaining time; leaving resumes that deadline. If the same notification appears in multiple windows, it stays paused until none is hovered; closing a hovered window releases its pause. Each native, keyboard-accessible “Close notification” button dismisses only its own notification. Pending tasks belong to their manager entity, so replacing or dropping that manager cannot remove a successor's notifications. Gallery and Docs execute the same displayed snippet, including a three-notification stack action.

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

## Loading

`Loading` is a visual spinner with optional text. `.full_screen()` absolutely fills its positioning container; it does not block input or move/trap focus. To cover a window's content and occlude background pointer input, queue it with `liora_core::push_portal` during active renders and mount `liora_components::PortalLayer` last at the window root. Stop queuing when loading ends. Use `Dialog` when keyboard-modal focus isolation is required.

Gallery and Docs execute the same authored basic/full-screen snippets. Retain the full-screen example's returned view across renders: its three-second dismissal task is canceled on manual dismissal or entity release, and cancellation cannot close a subsequent preview early. The demonstration covers window content, not system decorations or other windows.

## GPUI dependency note

Liora uses the repository-maintained GPUI derivative described in [the source maintenance boundary](../../third_party/gpui/README.md), based on official `zed-industries/zed`. Follow the root README for the exact immutable Liora Git revision and application-root patch. Do not mix it with official GPUI or other forks; SDK manifests must not contain developer-local paths.

## Documentation

- Repository: <https://github.com/yhyzgn/liora>
- Main README: <https://github.com/yhyzgn/liora#readme>
- Chinese README: <https://github.com/yhyzgn/liora/blob/main/README.zh-CN.md>

## License

This crate uses the repository license declared in `LICENSE.md`.

