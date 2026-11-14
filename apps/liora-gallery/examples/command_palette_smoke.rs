//! Native event assertions for command ranking, availability, dispatch and focus restoration.
//! Run: cargo run -p liora-gallery --example command_palette_smoke --locked
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
mod smoke {
    use gpui::{
        App, Context, Entity, FocusHandle, KeyUpEvent, Keystroke, MouseButton, MouseDownEvent,
        MouseMoveEvent, MouseUpEvent, PlatformInput, Render, Window, WindowOptions, actions, div,
        point, prelude::*, px,
    };
    use liora_components::{CommandPalette, CommandPaletteCommand, PortalLayer, init_liora};
    use std::{cell::Cell, rc::Rc, time::Duration};

    actions!(command_palette_smoke, [SaveDocument]);

    struct Host {
        palette: Entity<CommandPalette>,
        focus: FocusHandle,
        actions: Rc<Cell<usize>>,
    }

    impl Render for Host {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let actions = self.actions.clone();
            div()
                .relative()
                .size_full()
                .track_focus(&self.focus)
                .on_action(move |_: &SaveDocument, _, _| actions.set(actions.get() + 1))
                .child(self.palette.clone())
                .child(PortalLayer)
        }
    }

    fn redraw(window: &mut Window, cx: &mut App) {
        window.refresh();
        window.draw(cx).clear(cx);
    }

    fn key(key: &'static str, window: &mut Window, cx: &mut App) {
        let keystroke = Keystroke::parse(key).unwrap();
        window.dispatch_keystroke(keystroke.clone(), cx);
        window.dispatch_event(PlatformInput::KeyUp(KeyUpEvent { keystroke }), cx);
    }

    fn click(y: f32, window: &mut Window, cx: &mut App) {
        let position = point(px(400.0), px(y));
        window.dispatch_event(
            PlatformInput::MouseMove(MouseMoveEvent {
                position,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            PlatformInput::MouseDown(MouseDownEvent {
                position,
                button: MouseButton::Left,
                ..Default::default()
            }),
            cx,
        );
        window.dispatch_event(
            PlatformInput::MouseUp(MouseUpEvent {
                position,
                button: MouseButton::Left,
                ..Default::default()
            }),
            cx,
        );
    }

    pub fn run() {
        gpui_platform::headless().run(|cx| {
            init_liora(cx);
            CommandPalette::register_key_bindings(cx);
            let actions = Rc::new(Cell::new(0));
            let callbacks = Rc::new(Cell::new(0));
            let allowed = Rc::new(Cell::new(true));
            let unbound = CommandPaletteCommand::new("unbound", "Metadata only");
            assert!(!unbound.is_available(cx), "a targetless command is unavailable, not a panic");
            let commands = vec![
                CommandPaletteCommand::new("save", "Save document").with_action(SaveDocument),
                CommandPaletteCommand::new("disabled", "Disabled command").disabled(true)
                    .on_execute({ let callbacks = callbacks.clone(); move |_, _, _| callbacks.set(callbacks.get() + 100) }),
                CommandPaletteCommand::new("callback", "Host callback")
                    .available_when({ let allowed = allowed.clone(); move |_| allowed.get() })
                    .on_execute({ let callbacks = callbacks.clone(); move |_, _, _| callbacks.set(callbacks.get() + 1) }),
                unbound,
            ];
            let palette = CommandPalette::entity(commands, cx);
            let host_focus = cx.focus_handle();
            let handle = cx.open_window(WindowOptions { show: false, focus: false, ..Default::default() }, |_, cx| {
                cx.new(|_| Host { palette: palette.clone(), focus: host_focus.clone(), actions: actions.clone() })
            }).expect("headless command host opens");
            cx.update_window(handle.into(), |_, window, cx| {
                window.resize(gpui::size(px(800.0), px(600.0)));
                window.bounds_changed(cx);
                window.focus(&host_focus, cx);
                redraw(window, cx);
                palette.update(cx, |palette, cx| palette.open(window, cx));
                redraw(window, cx);
            }).unwrap();

            let executor = cx.background_executor().clone();
            cx.spawn(async move |cx| {
                // Separate updates let GPUI drain deferred focus and input callbacks.
                for step in -1..15 {
                    executor.timer(Duration::from_millis(10)).await;
                    cx.update(|cx| {
                        cx.update_window(handle.into(), |_, window, cx| {
                            redraw(window, cx);
                            match step {
                                -1 => {
                                    for character in ["s", "v", "e"] { key(character, window, cx); }
                                }
                                0 => {
                                    assert_ne!(window.focused(cx), Some(host_focus.clone()));
                                    key("down", window, cx);
                                    assert_eq!(palette.read(cx).active_command_id().unwrap().as_str(), "save");
                                    redraw(window, cx);
                                    key("enter", window, cx);
                                }
                                1 => {
                                    assert_eq!(actions.get(), 1, "palette dispatches the host GPUI action exactly once");
                                    assert!(!palette.read(cx).is_open());
                                    assert_eq!(window.focused(cx), Some(host_focus.clone()));
                                    palette.update(cx, |palette, cx| palette.open(window, cx));
                                }
                                2 => {
                                    key("down", window, cx);
                                    assert_eq!(palette.read(cx).active_command_id().unwrap().as_str(), "callback", "down skips disabled");
                                    key("up", window, cx);
                                    assert_eq!(palette.read(cx).active_command_id().unwrap().as_str(), "save");
                                    palette.update(cx, |palette, cx| palette.set_query("disabled", cx));
                                    assert!(palette.read(cx).active_command_id().is_none());
                                    redraw(window, cx);
                                    key("enter", window, cx);
                                }
                                3 => {
                                    assert!(palette.read(cx).is_open());
                                    assert_eq!(callbacks.get(), 0, "disabled command cannot run through Enter");
                                    palette.update(cx, |palette, cx| palette.set_query("callback", cx));
                                    allowed.set(false);
                                    redraw(window, cx);
                                    key("enter", window, cx);
                                }
                                4 => {
                                    assert!(palette.read(cx).is_open());
                                    assert_eq!(callbacks.get(), 0, "availability is checked again at execution");
                                    allowed.set(true);
                                    palette.update(cx, |palette, cx| palette.refresh(cx));
                                    redraw(window, cx);
                                    key("enter", window, cx);
                                }
                                5 => {
                                    assert_eq!(callbacks.get(), 1, "host callback executes exactly once");
                                    assert!(!palette.read(cx).is_open());
                                    palette.update(cx, |palette, cx| palette.open(window, cx));
                                }
                                6 => key("escape", window, cx),
                                7 => {
                                    assert!(!palette.read(cx).is_open());
                                    assert_eq!(window.focused(cx), Some(host_focus.clone()));
                                    let commands = (0..2000).map(|index| CommandPaletteCommand::new(format!("item.{index}"), format!("Item {index}"))
                                        .on_execute({ let callbacks = callbacks.clone(); move |_, _, _| callbacks.set(callbacks.get() + 1) })).collect();
                                    palette.update(cx, |palette, cx| {
                                        palette.set_commands(commands, cx);
                                        palette.open(window, cx);
                                    });
                                }
                                8 => {
                                    for _ in 0..40 { key("down", window, cx); }
                                    assert_eq!(palette.read(cx).active_command_id().unwrap().as_str(), "item.40");
                                    redraw(window, cx);
                                    palette.update(cx, |palette, cx| palette.set_query("no such command", cx));
                                    assert!(palette.read(cx).active_command_id().is_none());
                                    redraw(window, cx);
                                    key("enter", window, cx);
                                }
                                9 => {
                                    assert!(palette.read(cx).is_open(), "empty results cannot execute stale active commands");
                                    key("escape", window, cx);
                                }
                                10 => {
                                    assert!(!palette.read(cx).is_open());
                                    assert_eq!(window.focused(cx), Some(host_focus.clone()));
                                    palette.update(cx, |palette, cx| {
                                        palette.set_commands(vec![
                                            CommandPaletteCommand::new("disabled", "Disabled").disabled(true)
                                                .on_execute({ let callbacks = callbacks.clone(); move |_, _, _| callbacks.set(callbacks.get() + 100) }),
                                            CommandPaletteCommand::new("pointer", "Pointer callback")
                                                .on_execute({ let callbacks = callbacks.clone(); move |_, _, _| callbacks.set(callbacks.get() + 1) }),
                                        ], cx);
                                        palette.open(window, cx);
                                    });
                                }
                                11 => {
                                    click(160.0, window, cx);
                                    assert!(palette.read(cx).is_open(), "disabled pointer row stays open");
                                    assert_eq!(callbacks.get(), 1, "disabled pointer row cannot execute");
                                    click(218.0, window, cx);
                                }
                                12 => {
                                    assert_eq!(callbacks.get(), 2, "enabled pointer row executes once from its blank area");
                                    assert!(!palette.read(cx).is_open());
                                    assert_eq!(window.focused(cx), Some(host_focus.clone()));
                                    palette.update(cx, |palette, cx| palette.open(window, cx));
                                }
                                13 => click(30.0, window, cx),
                                _ => {
                                    assert!(!palette.read(cx).is_open(), "outside click dismisses the palette");
                                    assert_eq!(callbacks.get(), 2, "outside click never executes a command");
                                    assert_eq!(window.focused(cx), Some(host_focus.clone()));
                                }
                            }
                        }).unwrap();
                    });
                }
                cx.update(|cx| { println!("PASS: native command palette ranking, navigation, disabled/availability guards, callbacks, GPUI dispatch, focus, and large candidate list"); cx.quit(); });
            }).detach();
        });
    }
}
#[cfg(any(target_os = "linux", target_os = "freebsd"))]
fn main() {
    smoke::run();
}
#[cfg(not(any(target_os = "linux", target_os = "freebsd")))]
fn main() {
    eprintln!("This smoke uses the official Linux/FreeBSD native headless backend.");
    std::process::exit(1);
}
