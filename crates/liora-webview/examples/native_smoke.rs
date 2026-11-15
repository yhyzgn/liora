#[cfg(target_os = "linux")]
mod linux {
    use gpui::{
        AppContext, Context, Entity, IntoElement, Render, Window, WindowOptions, prelude::*,
    };
    use liora_webview::{Error, Event, WebView};
    use std::{
        cell::Cell,
        io::{BufRead, BufReader, Write},
        net::TcpListener,
        rc::Rc,
        time::Duration,
    };
    use wry::raw_window_handle::{HasWindowHandle, RawWindowHandle};

    const PAYLOAD: &str = "quote');window.ipc.postMessage('injected')//";

    struct Smoke {
        child: Entity<WebView>,
        other: Entity<WebView>,
        _events: gpui::Subscription,
    }

    impl Render for Smoke {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div()
                .size_full()
                .flex()
                .child(gpui::div().flex_1().h_full().child(self.child.clone()))
                .child(gpui::div().flex_1().h_full().child(self.other.clone()))
        }
    }

    pub fn run() {
        let server = TcpListener::bind("127.0.0.1:0").expect("local HTTP server");
        let url = format!("http://{}/", server.local_addr().expect("bound address"));
        std::thread::spawn(move || {
            let (mut stream, _) = server.accept().expect("navigation request");
            {
                let mut reader = BufReader::new(&mut stream);
                loop {
                    let mut line = String::new();
                    assert!(reader.read_line(&mut line).expect("request headers") > 0);
                    if line == "\r\n" {
                        break;
                    }
                }
            }
            let html = "<body style='background:#047;color:white'><h1>LOCAL NAVIGATION</h1><script>window.ipc.postMessage('navigated')</script></body>";
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{html}",
                html.len()
            );
            stream
                .write_all(response.as_bytes())
                .expect("page response");
        });
        gpui_platform::application().run(|cx| {
            let ready = Rc::new(Cell::new(false));
            let echoed = Rc::new(Cell::new(false));
            let navigated = Rc::new(Cell::new(false));
            let handle = cx.open_window(WindowOptions::default(), |window, cx| {
                cx.new(|cx| {
                    let child = cx.new(|cx| WebView::new(window, cx).expect("X11 child must attach"));
                    let other = cx.new(|cx| WebView::new(window, cx).expect("second X11 child must attach"));
                    let (r, e, n) = (ready.clone(), echoed.clone(), navigated.clone());
                    let events = cx.subscribe(&child, move |_, _, event, _| {
                        if let Event::Message(body) = event {
                            if body == "ready" { r.set(true); }
                            if body == &format!("echo:{PAYLOAD}") { e.set(true); }
                            if body == "navigated" { n.set(true); }
                            assert_ne!(body, "injected", "host message must not run as JS source");
                        }
                        assert!(!matches!(event, Event::NativeError(_)), "native bounds must update");
                    });
                    child.update(cx, |child, _| {
                        child.load_html("<body style='background:#135;color:white'><h1>GPUI + WRY CHILD</h1><script>window.addEventListener('liora-message',e=>window.ipc.postMessage('echo:'+e.data));window.ipc.postMessage('ready')</script></body>").expect("initial HTML");
                    });
                    other.update(cx, |other, _| {
                        other.load_html("<body style='background:#940;color:white'><h1>SECOND CHILD</h1></body>").expect("other HTML");
                    });
                    Smoke { child, other, _events: events }
                })
            }).expect("GPUI window must open");
            let executor = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                for _ in 0..100 {
                    executor.timer(Duration::from_millis(50)).await;
                    if ready.get() { break }
                }
                cx.update(|cx| {
                    assert!(ready.get(), "GTK must deliver page IPC through GPUI");
                    handle.update(cx, |root, _, cx| root.child.update(cx, |child, _| {
                        assert!(matches!(child.navigate("file:///etc/passwd"), Err(Error::InvalidUrl)));
                        child.post_message(PAYLOAD).expect("host message delivery");
                    })).expect("window alive");
                });
                for _ in 0..100 {
                    executor.timer(Duration::from_millis(50)).await;
                    if echoed.get() { break }
                }
                cx.update(|cx| {
                    assert!(echoed.get(), "quoted host data must round-trip as data");
                    handle.update(cx, |root, _, cx| root.child.update(cx, |child, _| child.navigate(&url).expect("local navigation"))).expect("window alive");
                });
                for _ in 0..100 {
                    executor.timer(Duration::from_millis(50)).await;
                    if navigated.get() { break }
                }
                executor.timer(Duration::from_millis(250)).await;
                cx.update(|cx| {
                    if let Ok(path) = std::env::var("LIORA_WEBVIEW_SCREENSHOT") {
                        let (id, size) = handle.update(cx, |_, window, _| {
                            let raw = HasWindowHandle::window_handle(window).expect("X11 handle").as_raw();
                            let RawWindowHandle::Xcb(handle) = raw else { panic!("expected XCB") };
                            let bounds = window.bounds();
                            (handle.window.get().to_string(), format!("{}x{}", f32::from(bounds.size.width) as u32, f32::from(bounds.size.height) as u32))
                        }).expect("window alive");
                        assert!(std::process::Command::new("ffmpeg").args(["-hide_banner", "-loglevel", "error", "-y", "-f", "x11grab", "-window_id", &id, "-video_size", &size, "-i", &std::env::var("DISPLAY").expect("isolated X11 display"), "-frames:v", "1", &path]).status().expect("capture native window").success());
                    }
                    assert!(navigated.get(), "local HTTP page must load and send IPC");
                    let other = handle.update(cx, |root, window, cx| {
                        root.child.update(cx, |child, _| {
                            assert!(child.url().expect("current URL").starts_with(&url));
                            child.close();
                            assert!(child.is_closed());
                            assert!(matches!(child.reload(), Err(Error::Closed)));
                        });
                        let other = root.other.clone();
                        window.remove_window();
                        other
                    }).expect("parent close");
                    assert!(other.read(cx).is_closed(), "parent close must destroy retained native child");
                    println!("PASS: native child layout, GTK IPC, navigation, safe bridge, explicit and parent teardown");
                    cx.quit();
                });
            }).detach();
        });
    }
}

fn main() {
    #[cfg(target_os = "linux")]
    linux::run();
}
