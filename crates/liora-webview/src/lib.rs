//! Opt-in WRY child webview hosted inside a GPUI window.

use gpui::{Context, IntoElement, Render, Window, canvas, prelude::*};
use std::{
    cell::{Cell, RefCell},
    rc::Rc,
};
use wry::{
    Rect, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
};

const MAX_MESSAGE_BYTES: usize = 64 * 1024;
const MAX_PENDING_EVENTS: usize = 128;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unsupported GPUI window handle for a child webview")]
    UnsupportedWindow,
    #[error("native window handle: {0}")]
    Window(#[from] wry::raw_window_handle::HandleError),
    #[error("GTK initialization: {0}")]
    #[cfg(target_os = "linux")]
    Gtk(String),
    #[error("only http and https URLs are allowed")]
    InvalidUrl,
    #[error("webview: {0}")]
    Wry(#[from] wry::Error),
    #[error("the child webview has been closed")]
    Closed,
    #[error("page message encoding: {0}")]
    Json(#[from] serde_json::Error),
}

/// Untrusted page events delivered to subscribers on the GPUI thread.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    LoadStarted(String),
    LoadFinished(String),
    Message(String),
    MessageTooLarge,
    EventsDropped,
    NativeError(String),
}

impl gpui::EventEmitter<Event> for WebView {}

/// A GPUI view that owns a WRY child; keep its entity alive while it is rendered.
pub struct WebView {
    native: Rc<RefCell<Option<wry::WebView>>>,
    events: Rc<RefCell<Vec<Event>>>,
    last_bounds: Rc<Cell<Option<Rect>>>,
    visible: Rc<Cell<bool>>,
    _on_window_closed: gpui::Subscription,
}

impl WebView {
    /// Create a hidden child in an existing GPUI window; show it when first laid out.
    /// On Linux, only a GPUI X11/XWayland window is supported.
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Result<Self, Error> {
        let handle = wry::raw_window_handle::HasWindowHandle::window_handle(window)?.as_raw();
        #[cfg(target_os = "linux")]
        let parent = match handle {
            wry::raw_window_handle::RawWindowHandle::Xcb(xcb) => {
                gtk::init().map_err(|error| Error::Gtk(error.to_string()))?;
                use gtk::glib::Cast;
                if gtk::gdk::Display::default()
                    .is_none_or(|display| display.downcast_ref::<gdkx11::X11Display>().is_none())
                {
                    return Err(Error::UnsupportedWindow);
                }
                XlibParent(xcb.window.get() as _)
            }
            _ => return Err(Error::UnsupportedWindow),
        };
        #[cfg(not(target_os = "linux"))]
        if !matches!(
            handle,
            wry::raw_window_handle::RawWindowHandle::AppKit(_)
                | wry::raw_window_handle::RawWindowHandle::Win32(_)
        ) {
            return Err(Error::UnsupportedWindow);
        }

        let events = Rc::new(RefCell::new(Vec::new()));
        let ipc_events = events.clone();
        let load_events = events.clone();
        let builder = WebViewBuilder::new()
            .with_visible(false)
            .with_incognito(true)
            .with_autoplay(false)
            .with_navigation_handler(|url| allowed_url(&url) || url == "about:blank")
            .with_clipboard(false)
            .with_ipc_handler(move |request| push_message(&ipc_events, request.into_body()))
            .with_on_page_load_handler(move |event, url| {
                push_event(
                    &load_events,
                    match event {
                        wry::PageLoadEvent::Started => Event::LoadStarted(url),
                        wry::PageLoadEvent::Finished => Event::LoadFinished(url),
                    },
                );
            })
            .with_permission_handler(|_| wry::PermissionResponse::Deny)
            .with_download_started_handler(|_, _| false)
            .with_new_window_req_handler(|_, _| wry::NewWindowResponse::Deny);
        #[cfg(target_os = "linux")]
        let webview = builder.build_as_child(&parent)?;
        #[cfg(not(target_os = "linux"))]
        let webview = builder.build_as_child(window)?;
        let native = Rc::new(RefCell::new(Some(webview)));
        let weak = Rc::downgrade(&native);
        let id = Window::window_handle(window).window_id();
        let closed = cx.on_window_closed(move |_, closed_id| {
            if closed_id == id
                && let Some(native) = weak.upgrade()
            {
                native.borrow_mut().take();
            }
        });
        let weak = Rc::downgrade(&native);
        let executor = cx.background_executor().clone();
        cx.spawn(async move |this, cx| {
            loop {
                executor.timer(std::time::Duration::from_millis(16)).await;
                let Some(native) = weak.upgrade() else { break };
                if native.borrow().is_none() {
                    break;
                }
                if cx
                    .update(|cx| {
                        this.update(cx, |this, cx| {
                            #[cfg(target_os = "linux")]
                            for _ in 0..64 {
                                if !gtk::events_pending() {
                                    break;
                                }
                                gtk::main_iteration_do(false);
                            }
                            for event in this.events.borrow_mut().drain(..) {
                                cx.emit(event);
                            }
                        })
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();
        Ok(Self {
            native,
            events,
            last_bounds: Rc::new(Cell::new(None)),
            visible: Rc::new(Cell::new(true)),
            _on_window_closed: closed,
        })
    }

    /// Load HTML supplied by the host.
    pub fn load_html(&self, html: &str) -> Result<(), Error> {
        self.with_native(|view| view.load_html(html))
    }

    /// Navigate to an HTTP(S) URL. Page redirects and links follow the same policy.
    pub fn navigate(&self, url: &str) -> Result<(), Error> {
        if !allowed_url(url) {
            return Err(Error::InvalidUrl);
        }
        self.with_native(|view| view.load_url(url))
    }

    /// Reload the current page.
    pub fn reload(&self) -> Result<(), Error> {
        self.with_native(wry::WebView::reload)
    }

    /// Navigate one entry back in the engine history.
    pub fn go_back(&self) -> Result<(), Error> {
        self.with_native(wry::WebView::go_back)
    }

    /// Navigate one entry forward in the engine history.
    pub fn go_forward(&self) -> Result<(), Error> {
        self.with_native(wry::WebView::go_forward)
    }

    /// Report whether engine history contains a previous entry.
    pub fn can_go_back(&self) -> Result<bool, Error> {
        self.with_native(wry::WebView::can_go_back)
    }

    /// Report whether engine history contains a next entry.
    pub fn can_go_forward(&self) -> Result<bool, Error> {
        self.with_native(wry::WebView::can_go_forward)
    }

    /// Return the currently committed URL as reported by the system engine.
    pub fn url(&self) -> Result<String, Error> {
        self.with_native(wry::WebView::url)
    }

    /// Deliver a string to `window` as a `liora-message` MessageEvent.
    pub fn post_message(&self, message: &str) -> Result<(), Error> {
        let mut script =
            b"window.dispatchEvent(new MessageEvent('liora-message', { data: ".to_vec();
        serde_json::to_writer(&mut script, message)?;
        script.extend_from_slice(b" }))");
        self.with_native(|view| {
            view.evaluate_script(std::str::from_utf8(&script).expect("JSON is UTF-8"))
        })
    }

    fn with_native<T>(
        &self,
        call: impl FnOnce(&wry::WebView) -> wry::Result<T>,
    ) -> Result<T, Error> {
        let native = self.native.borrow();
        Ok(call(native.as_ref().ok_or(Error::Closed)?)?)
    }

    /// Show or hide the native child when retaining its entity outside the GPUI tree.
    pub fn set_visible(&self, visible: bool) -> Result<(), Error> {
        if self.native.borrow().is_none() {
            return Err(Error::Closed);
        }
        if self.visible.get() != visible {
            if self.last_bounds.get().is_some() {
                self.with_native(|view| view.set_visible(visible))?;
            }
            self.visible.set(visible);
        }
        Ok(())
    }

    /// Destroy the native child before its GPUI parent is destroyed.
    pub fn close(&mut self) {
        self.native.borrow_mut().take();
    }

    /// Whether the native child has been destroyed, including by parent-window closure.
    pub fn is_closed(&self) -> bool {
        self.native.borrow().is_none()
    }
}

impl Render for WebView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let native = self.native.clone();
        let last_bounds = self.last_bounds.clone();
        let visible = self.visible.clone();
        let events = self.events.clone();
        canvas(
            |_, _, _| (),
            move |bounds, _, _window, _cx| {
                if let Some(view) = native.borrow().as_ref() {
                    let rect = Rect {
                        position: LogicalPosition::new(
                            f64::from(bounds.origin.x),
                            f64::from(bounds.origin.y),
                        )
                        .into(),
                        size: LogicalSize::new(
                            f64::from(bounds.size.width),
                            f64::from(bounds.size.height),
                        )
                        .into(),
                    };
                    let old = last_bounds.get();
                    let empty =
                        bounds.size.width <= gpui::px(0.0) || bounds.size.height <= gpui::px(0.0);
                    let updated = if empty {
                        if old.is_some() {
                            view.set_visible(false)
                        } else {
                            Ok(())
                        }
                    } else if old != Some(rect) {
                        view.set_bounds(rect).and_then(|_| {
                            if old.is_none() && visible.get() {
                                view.set_visible(true)?;
                            }
                            last_bounds.set(Some(rect));
                            Ok(())
                        })
                    } else {
                        Ok(())
                    };
                    if empty && updated.is_ok() {
                        last_bounds.set(None);
                    }
                    if let Err(error) = updated {
                        push_event(&events, Event::NativeError(error.to_string()));
                    }
                }
            },
        )
        .size_full()
    }
}

#[cfg(target_os = "linux")]
struct XlibParent(std::os::raw::c_ulong);

#[cfg(target_os = "linux")]
impl wry::raw_window_handle::HasWindowHandle for XlibParent {
    fn window_handle(
        &self,
    ) -> Result<wry::raw_window_handle::WindowHandle<'_>, wry::raw_window_handle::HandleError> {
        use wry::raw_window_handle::{RawWindowHandle, WindowHandle, XlibWindowHandle};
        // SAFETY: this X11 ID belongs to the live GPUI XCB window passed into WebView::new.
        Ok(unsafe {
            WindowHandle::borrow_raw(RawWindowHandle::Xlib(XlibWindowHandle::new(self.0)))
        })
    }
}

fn allowed_url(url: &str) -> bool {
    url::Url::parse(url).is_ok_and(|parsed| {
        matches!(parsed.scheme(), "http" | "https") && parsed.host_str().is_some()
    })
}

fn push_message(events: &RefCell<Vec<Event>>, body: String) {
    let event = if body.len() > MAX_MESSAGE_BYTES {
        Event::MessageTooLarge
    } else {
        Event::Message(body)
    };
    push_event(events, event);
}

fn push_event(events: &RefCell<Vec<Event>>, event: Event) {
    let mut events = events.borrow_mut();
    if events.len() < MAX_PENDING_EVENTS - 1 {
        events.push(event);
    } else if events.len() == MAX_PENDING_EVENTS - 1 {
        events.push(Event::EventsDropped);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigation_only_accepts_http_urls_with_hosts() {
        assert!(allowed_url("https://example.com/path"));
        assert!(allowed_url("http://127.0.0.1:3000/"));
        for url in [
            "file:///tmp/secret",
            "javascript:alert(1)",
            "data:text/html,hi",
            "https://",
            "about:blank",
        ] {
            assert!(!allowed_url(url), "{url}");
        }
    }

    #[test]
    fn page_messages_are_bounded_and_overflow_is_reported() {
        let events = RefCell::new(Vec::new());
        push_message(&events, "x".repeat(MAX_MESSAGE_BYTES));
        push_message(&events, "x".repeat(MAX_MESSAGE_BYTES + 1));
        assert!(matches!(events.borrow()[0], Event::Message(_)));
        assert_eq!(events.borrow()[1], Event::MessageTooLarge);
        for _ in 0..MAX_PENDING_EVENTS {
            push_event(&events, Event::Message("x".into()));
        }
        let events = events.into_inner();
        assert_eq!(events.len(), MAX_PENDING_EVENTS);
        assert_eq!(events.last(), Some(&Event::EventsDropped));
    }
}
