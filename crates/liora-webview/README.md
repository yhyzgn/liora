# liora-webview

Standalone, opt-in WRY child webview hosted by a GPUI `Entity<WebView>`. Add `liora-webview` explicitly; neither `liora` nor `liora-components`, Gallery, or Docs depends on it. Other Liora controls remain native GPUI; this is not a Tauri or browser app shell.

```toml
[dependencies]
liora-webview = { path = "../liora/crates/liora-webview" } # adjust to your checkout; not released yet
```

Create the entity with the **live GPUI window**, keep it in the host view and render it as a child. Hold the returned GPUI `Subscription` while handling `Event`; page events are untrusted input. The runnable integration, including two child views, is [`examples/native_smoke.rs`](examples/native_smoke.rs).

```rust
let child = cx.new(|cx| WebView::new(window, cx).expect("supported native window"));
let events = cx.subscribe(&child, |_, _, event, _| {
    // Validate Event::Message(body) against your application's own protocol.
});
child.update(cx, |child, _| child.navigate("https://example.com").expect("navigation"));
// Retain child and events as fields of your host view; render child.clone().
```

`navigate` permits only HTTP(S) URLs with a host; this restricts **top-level navigation**, not subresource requests, JavaScript, or page-origin trust. Page redirects and links follow the same navigation policy. `load_html` accepts host-supplied HTML (do not pass untrusted markup). `reload`, `go_back`, `go_forward`, `can_go_back`, `can_go_forward`, and `url` delegate to the installed engine. `Event::LoadStarted`/`LoadFinished` report engine lifecycle, **not HTTP success**. The host-to-page `post_message` dispatches a `liora-message` DOM MessageEvent with JSON-escaped string `data`; pages send strings with `window.ipc.postMessage(...)`. Do not treat a message as an authenticated origin or execute it as a command without application-level validation. Messages over 64 KiB yield `MessageTooLarge`; a full 128-event queue yields `EventsDropped`.

The child follows GPUI canvas bounds; `set_visible(false)` hides it when its entity is retained outside the render tree. Call `close()` when removing a view or replacing its window; the parent window's close callback also destroys the native child, and subsequent operations return `Error::Closed`. GPUI overlays may not cover the native child uniformly across platforms; native child stacking/clipping is not a GPUI z-index contract.

Permissions, downloads, and new windows are denied by default; JavaScript clipboard access and autoplay are disabled; WRY uses an incognito context. This is **not** a sandbox for untrusted websites: pages may execute JavaScript and request network resources. The host owns URL selection, message authorization, any additional network restrictions, and engine updates.

Linux requires GTK3 and WebKitGTK 4.1 development/runtime packages plus a GPUI **X11/XWayland** window and GTK configured for X11 (`GDK_BACKEND=x11` where necessary). WRY has no GPUI-native Wayland child path here; an unsupported GPUI or GTK display returns `UnsupportedWindow`. GTK is advanced on the GPUI thread without a second event loop. The isolated X11 smoke is run with `env -u WAYLAND_DISPLAY DISPLAY=:2 GDK_BACKEND=x11 cargo run -p liora-webview --example native_smoke`; `LIORA_WEBVIEW_SCREENSHOT=/tmp/liora-webview.png` optionally captures the live window using ffmpeg. macOS and Windows have child-view code paths, but neither cross-target compilation nor native window/render/permission behavior has been verified here. Web compatibility depends on the installed WebKitGTK, WKWebView, or WebView2 engine; no cross-browser parity is promised.
