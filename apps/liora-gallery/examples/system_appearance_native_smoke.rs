//! Private-portal regression: python3 scripts/system_appearance_native.py OUTPUT_DIR NATIVE_BINARY system
//! Build this example first; requires Python dbus-next, dbus-daemon, KWin Wayland,
//! Xwayland and software Vulkan. The runner creates its own bus, HOME/XDG and display.
use gpui::{App, AppContext, AsyncApp, AnyWindowHandle, Context, IntoElement, Render, Window,
    WindowAppearance, WindowOptions, div, prelude::*};
use liora_core::{Config, ThemeMode, apply_theme_mode, attach_system_theme_observer,
    init_liora_with_mode, sync_system_theme};
use std::{cell::Cell, path::{Path, PathBuf}, rc::Rc, time::{Duration, Instant}};

struct Surface;
impl Render for Surface {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().bg(cx.global::<Config>().theme.neutral.body)
    }
}

fn open(cx: &mut App, events: Rc<Cell<usize>>, expected: Option<WindowAppearance>) -> AnyWindowHandle {
    cx.open_window(WindowOptions::default(), |window, cx| {
        if let Some(expected) = expected {
            assert_eq!(window.appearance(), expected, "new platform window inherits common appearance");
        }
        attach_system_theme_observer(window, cx);
        window.observe_window_appearance(move |_, _| events.set(events.get() + 1)).detach();
        cx.new(|_| Surface)
    }).expect("open native window").into()
}

async fn portal(cx: &mut AsyncApp, root: &Path, seq: u32, dark: bool,
    window: Option<AnyWindowHandle>, events: &Cell<usize>) {
    let before = events.get();
    let expected = if dark { WindowAppearance::Dark } else { WindowAppearance::Light };
    std::fs::write(root.join("request.tmp"), format!("{seq} {}", if dark { 1 } else { 2 })).unwrap();
    std::fs::rename(root.join("request.tmp"), root.join("request")).unwrap();
    let deadline = Instant::now() + Duration::from_secs(8);
    loop {
        let acknowledged = std::fs::read_to_string(root.join("ack")).is_ok_and(|v| v == seq.to_string());
        let app_ready = cx.update(|cx| cx.window_appearance() == expected);
        let window_ready = window.map(|handle| cx.update_window(handle, |_, window, _| {
            window.appearance() == expected
        }).unwrap()).unwrap_or(true);
        if acknowledged && app_ready && window_ready && (window.is_none() || events.get() > before) {
            println!("PORTAL seq={seq} app={expected:?} window={window_ready} notifications={}", events.get());
            return;
        }
        assert!(Instant::now() < deadline, "portal delivery timeout seq={seq} ack={acknowledged} app={app_ready} window={window_ready} notifications={}/{}", events.get(), before);
        cx.background_executor().timer(Duration::from_millis(25)).await;
    }
}

fn assert_theme(cx: &mut App, expected: &str, stage: &str) {
    assert_eq!(cx.global::<Config>().theme.name, expected, "{stage}");
    println!("PASS {stage}: theme={expected}");
}

fn main() {
    let root = PathBuf::from(std::env::args_os().nth(1).expect("private control directory"));
    let case = std::env::args().nth(2).expect("system, fixed-dark, or fixed-light");
    gpui_platform::application().with_quit_mode(gpui::QuitMode::Explicit).run(move |cx| {
        let mode = match case.as_str() {
            "system" => ThemeMode::System,
            "fixed-dark" => ThemeMode::Dark,
            "fixed-light" => ThemeMode::Light,
            _ => panic!("unknown scenario"),
        };
        init_liora_with_mode(cx, mode);
        let events = Rc::new(Cell::new(0));
        let first = open(cx, events.clone(), None);
        assert_theme(cx, if mode == ThemeMode::Dark { "dark" } else { "light" }, "startup");
        cx.spawn(async move |cx| {
            let deadline = Instant::now() + Duration::from_secs(8);
            loop {
                let initial_light = cx.update_window(first, |_, window, cx| {
                    cx.window_appearance() == WindowAppearance::Light
                        && window.appearance() == WindowAppearance::Light
                        && events.get() > 0
                }).unwrap();
                if root.join("subscription-ready").exists() && initial_light {
                    println!("READY initial Light observed; PID-bound Read return and Settings AddMatch acknowledged; notifications={}", events.get());
                    break;
                }
                assert!(Instant::now() < deadline, "Settings subscription/initial delivery readiness timeout");
                cx.background_executor().timer(Duration::from_millis(25)).await;
            }
            let live_dark = mode != ThemeMode::Light;
            portal(cx, &root, 1, live_dark, Some(first), &events).await;
            let expected = if live_dark { "dark" } else { "light" };
            cx.update_window(first, |_, window, cx| {
                assert_theme(cx, expected, "observed or fixed theme");
                apply_theme_mode(window, cx, ThemeMode::Light);
                apply_theme_mode(window, cx, ThemeMode::System);
                assert_theme(cx, expected, "Light to System after live observation");
                apply_theme_mode(window, cx, ThemeMode::Dark);
                apply_theme_mode(window, cx, ThemeMode::System);
                assert_theme(cx, expected, "Dark to System after live observation");
                sync_system_theme(window, cx);
                assert_theme(cx, expected, "explicit sync");
            }).unwrap();
            let second = cx.update(|cx| open(cx, events.clone(), Some(if live_dark { WindowAppearance::Dark } else { WindowAppearance::Light })));
            cx.update(|cx| assert_theme(cx, expected, "replacement attach"));
            cx.update_window(first, |_, window, _| window.remove_window()).unwrap();
            cx.update_window(second, |_, window, _| window.remove_window()).unwrap();
            cx.update(|cx| assert!(cx.windows().is_empty(), "no-window phase"));
            portal(cx, &root, 2, !live_dark, None, &events).await;
            let third = cx.update(|cx| open(cx, events.clone(), Some(if live_dark { WindowAppearance::Light } else { WindowAppearance::Dark })));
            cx.update(|cx| assert_theme(cx, if live_dark { "light" } else { "dark" }, "reopen after no-window common change"));
            portal(cx, &root, 3, live_dark, Some(third), &events).await;
            cx.update(|cx| assert_theme(cx, expected, "reverse live transition"));
            cx.update(|cx| { println!("PASS native appearance scenario {case}"); cx.quit(); });
        }).detach();
    });
}
