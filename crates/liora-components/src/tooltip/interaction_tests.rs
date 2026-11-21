use super::Tooltip;
use crate::{AppWindowFrame, Button, Input, PortalLayer};
use gpui::{
    AppContext, Context, Entity, Focusable, IntoElement, Modifiers, Render, TestAppContext,
    VisualTestContext, Window, WindowHandle, div, point, prelude::*, px,
};
use liora_core::{ActiveTooltip, Placement};
use std::{cell::Cell, rc::Rc, time::Duration};

struct Surface {
    input: Entity<Input>,
    clicks: Rc<Cell<usize>>,
    escapes: Rc<Cell<usize>>,
    content: &'static str,
    button_offset: Option<gpui::Pixels>,
    cover: bool,
}

impl Render for Surface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        liora_core::render_active_tooltip_in_window(window, cx);
        if self.cover {
            liora_core::push_portal(
                |_, _| {
                    gpui::deferred(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .size_full()
                            .occlude()
                            .bg(gpui::rgb(0x151515))
                            .debug_selector(|| "tooltip-occluding-portal".into()),
                    )
                    .with_priority(1)
                    .into_any_element()
                },
                cx,
            );
        }
        let clicks = self.clicks.clone();
        let escapes = self.escapes.clone();
        let content = div()
            .relative()
            .size_full()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_start()
                    .gap_4()
                    .p_8()
                    .on_key_down(move |event, _, _| {
                        if event.keystroke.key == "escape" {
                            escapes.set(escapes.get() + 1);
                        }
                    })
                    .child(Button::new("Before").id("tooltip-before"))
                    .when_some(self.button_offset, |column, offset| {
                        column.child(
                            div().ml(offset).child(
                                Tooltip::new(
                                    Button::new("Help")
                                        .id("tooltip-button")
                                        .on_click(move |_, _, _| clicks.set(clicks.get() + 1)),
                                )
                                .id("button-tip")
                                .content(self.content)
                                .placement(Placement::Bottom),
                            ),
                        )
                    })
                    .child(
                        Tooltip::new(self.input.clone())
                            .id("input-tip")
                            .content("Input help")
                            .placement(Placement::Right),
                    )
                    .child(Button::new("After").id("tooltip-after")),
            )
            .child(PortalLayer);
        // Input delegates Tab traversal to the host, unlike Button's own Tab handler.
        AppWindowFrame::new("Tooltip interactions", content)
    }
}

fn setup() -> (TestAppContext, WindowHandle<Surface>, VisualTestContext) {
    let mut app = TestAppContext::single();
    app.update(crate::init_liora);
    app.update(|cx| cx.set_reduce_motion(true));
    let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, cx| Surface {
        input: cx.new(|cx| Input::new("", cx).id("tooltip-input")),
        clicks: Rc::new(Cell::new(0)),
        escapes: Rc::new(Cell::new(0)),
        content: "Button help",
        button_offset: Some(px(0.0)),
        cover: false,
    });
    let mut window = VisualTestContext::from_window(handle.into(), &app);
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    (app, handle, window)
}

fn draw(window: &mut VisualTestContext) {
    window.update(|window, cx| window.draw(cx).clear(cx));
    // Deliver only callbacks actually requested by GPUI; never force a second draw.
    for _ in 0..4 {
        let pending = window.update(|window, cx| window.simulate_next_frame(cx));
        if pending == 0 {
            return;
        }
        window.update(|window, cx| window.draw(cx).clear(cx));
    }
    panic!("Tooltip scheduled an unbounded idle repaint loop");
}

fn active(window: &mut VisualTestContext) -> Vec<(String, String)> {
    window.update(|_, cx| {
        cx.global::<ActiveTooltip>()
            .0
            .iter()
            .map(|tip| (tip.id.to_string(), tip.content.to_string()))
            .collect()
    })
}

fn press(window: &mut VisualTestContext, key: &str) {
    let keystroke =
        window.update(|_, cx| gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap());
    // Physical key presses must not fall back to IME insertion of a literal tab.
    window.simulate_event(gpui::KeyDownEvent {
        keystroke: keystroke.clone(),
        is_held: false,
        prefer_character_input: false,
    });
    window.simulate_event(gpui::KeyUpEvent { keystroke });
    draw(window);
}

#[test]
fn keyboard_focus_escape_and_reentry_preserve_original_button_actions() {
    let (mut app, handle, mut window) = setup();
    window.update(|window, cx| window.focus_next(cx));
    press(&mut window, "tab");
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    let origin = window.update(|window, cx| window.focused(cx).unwrap());
    press(&mut window, "escape");
    assert!(active(&mut window).is_empty());
    assert!(window.update(|window, _| origin.is_focused(window)));
    handle.update(&mut app, |_, _, cx| cx.notify()).unwrap();
    app.executor().advance_clock(Duration::from_secs(1));
    window.run_until_parked();
    draw(&mut window);
    assert!(
        active(&mut window).is_empty(),
        "continuous focus must not reopen on render or timer"
    );
    press(&mut window, "enter");
    press(&mut window, "space");
    assert_eq!(
        handle.read_with(&app, |view, _| view.clicks.get()).unwrap(),
        2
    );
    assert!(window.update(|window, _| origin.is_focused(window)));
    press(&mut window, "tab");
    assert_eq!(
        active(&mut window),
        [("input-tip".to_string(), "Input help".to_string())]
    );
    press(&mut window, "shift-tab");
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    assert!(
        window.update(|window, _| origin.is_focused(window)),
        "no extra wrapper tab stop"
    );
    app.quit();
}

#[test]
fn hover_crosses_offset_to_content_and_escape_stays_dismissed_until_exit() {
    let (app, handle, mut window) = setup();
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    let gap = point(trigger.center().x, trigger.bottom() + px(4.0));
    let content = point(trigger.center().x, trigger.bottom() + px(12.0));
    for position in [gap, content] {
        window.simulate_mouse_move(position, None, Modifiers::none());
        draw(&mut window);
        assert_eq!(
            active(&mut window),
            [("button-tip".to_string(), "Button help".to_string())]
        );
    }
    // A hover tooltip must dismiss even when focus is on an unrelated control.
    window.update(|window, cx| window.focus_next(cx));
    let origin = window.update(|window, cx| window.focused(cx).unwrap());
    press(&mut window, "escape");
    assert!(active(&mut window).is_empty());
    assert!(window.update(|window, _| origin.is_focused(window)));
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert!(
        active(&mut window).is_empty(),
        "the same continuous hover session stays dismissed"
    );
    app.executor().advance_clock(Duration::from_secs(1));
    window.run_until_parked();
    draw(&mut window);
    assert!(active(&mut window).is_empty());
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    window.simulate_click(trigger.center(), Modifiers::none());
    draw(&mut window);
    assert_eq!(
        handle.read_with(&app, |view, _| view.clicks.get()).unwrap(),
        1
    );
    app.quit();
}

#[test]
fn focused_input_keeps_native_typing_and_unrelated_escape_propagates() {
    let (app, handle, mut window) = setup();
    window.update(|window, cx| window.focus_next(cx));
    press(&mut window, "tab");
    press(&mut window, "tab");
    assert_eq!(
        active(&mut window),
        [("input-tip".to_string(), "Input help".to_string())]
    );
    let origin = window.update(|window, cx| window.focused(cx).unwrap());
    window.simulate_input("typed help");
    draw(&mut window);
    let input = handle
        .read_with(&app, |view, _| view.input.clone())
        .unwrap();
    assert_eq!(
        input.read_with(&app, |input, _| input.value()),
        "typed help"
    );
    press(&mut window, "escape");
    assert!(active(&mut window).is_empty());
    assert!(window.update(|window, _| origin.is_focused(window)));
    press(&mut window, "tab");
    assert!(
        !window.update(|window, _| origin.is_focused(window)),
        "Tab must leave Input"
    );
    assert_eq!(
        input.read_with(&app, |input, _| input.value()),
        "typed help"
    );
    assert!(active(&mut window).is_empty());
    let escapes = handle
        .read_with(&app, |view, _| view.escapes.get())
        .unwrap();
    press(&mut window, "escape");
    assert_eq!(
        handle
            .read_with(&app, |view, _| view.escapes.get())
            .unwrap(),
        escapes + 1
    );
    app.quit();
}

#[test]
fn stationary_pointer_refreshes_reused_trigger_content() {
    let (mut app, handle, mut window) = setup();
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    handle
        .update(&mut app, |view, _, cx| {
            view.content = "Recycled help";
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Recycled help".to_string())]
    );
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    assert!(active(&mut window).is_empty());
    app.quit();
}

#[test]
fn stationary_hover_tracks_a_moving_anchor_and_releases_unmounted_help() {
    let (mut app, handle, mut window) = setup();
    window.update(|window, cx| window.focus_next(cx));
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    let panel = window.debug_bounds("liora-tooltip-panel").unwrap();

    handle
        .update(&mut app, |view, _, cx| {
            view.button_offset = Some(px(12.0));
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    let moved_trigger = window.debug_bounds("tooltip-button").unwrap();
    let moved_panel = window.debug_bounds("liora-tooltip-panel").unwrap();
    assert!(
        moved_trigger.contains(&trigger.center()),
        "the pointer must still be over the moved trigger"
    );
    assert_eq!(moved_trigger.left() - trigger.left(), px(12.0));
    assert_eq!(moved_panel.left() - panel.left(), px(12.0));

    window.simulate_mouse_move(moved_panel.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        window.debug_bounds("liora-tooltip-panel"),
        Some(moved_panel)
    );
    handle
        .update(&mut app, |view, _, cx| {
            view.button_offset = None;
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    assert!(active(&mut window).is_empty());
    assert!(window.debug_bounds("liora-tooltip-panel").is_none());
    press(&mut window, "escape");
    assert_eq!(
        handle
            .read_with(&app, |view, _| view.escapes.get())
            .unwrap(),
        1
    );
    app.quit();
}

#[test]
fn escape_rearms_only_after_both_focus_and_hover_leave() {
    let (app, _, mut window) = setup();
    window.update(|window, cx| window.focus_next(cx));
    press(&mut window, "tab");
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    press(&mut window, "escape");
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    assert!(active(&mut window).is_empty(), "focus is still continuous");
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert!(active(&mut window).is_empty());
    press(&mut window, "tab");
    assert_eq!(
        active(&mut window),
        [("input-tip".to_string(), "Input help".to_string())]
    );
    press(&mut window, "shift-tab");
    assert!(active(&mut window).is_empty(), "hover is still continuous");
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    press(&mut window, "tab");
    press(&mut window, "shift-tab");
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    app.quit();
}

#[test]
fn native_window_exit_hides_hover_help_and_reentry_rearms_it() {
    let (app, _, mut window) = setup();
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    window.simulate_event(gpui::MouseExitEvent {
        position: point(px(-1.0), px(-1.0)),
        ..Default::default()
    });
    draw(&mut window);
    assert!(
        active(&mut window).is_empty(),
        "MouseExited need not include a final MouseMove"
    );
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    app.quit();
}

// These panel-selector assertions belong to the product delta, not the baseline RED patch.
#[test]
fn rendered_panel_keeps_multiline_content_bounds_and_reduced_motion_hover() {
    let (mut app, handle, mut window) = setup();
    app.update(|cx| cx.set_reduce_motion(true));
    handle
        .update(&mut app, |view, _, cx| {
            view.content = "First line\nSecond line";
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [(
            "button-tip".to_string(),
            "First line\nSecond line".to_string()
        )]
    );
    let panel = window.debug_bounds("liora-tooltip-panel").unwrap();
    assert_eq!(panel.top(), trigger.bottom() + px(8.0));
    assert!(panel.size.height > px(12.0));
    window.simulate_mouse_move(panel.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(window.debug_bounds("liora-tooltip-panel"), Some(panel));
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    assert!(active(&mut window).is_empty());
    assert!(window.debug_bounds("liora-tooltip-panel").is_none());
    app.quit();
}

#[test]
fn escape_in_another_window_does_not_dismiss_the_visible_help_owner() {
    let (mut app, _, mut window) = setup();
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    let other = app.open_window(gpui::size(px(640.0), px(480.0)), |_, cx| Surface {
        input: cx.new(|cx| Input::new("", cx).id("tooltip-input")),
        clicks: Rc::new(Cell::new(0)),
        escapes: Rc::new(Cell::new(0)),
        content: "Other window help",
        button_offset: Some(px(0.0)),
        cover: false,
    });
    let mut other_window = VisualTestContext::from_window(other.into(), &app);
    other_window.update(|window, _| window.activate_window());
    draw(&mut other_window);
    other_window.update(|window, cx| window.focus_next(cx));
    press(&mut other_window, "escape");
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    return_to_hover(&mut window, trigger.center());
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    press(&mut window, "escape");
    assert!(active(&mut window).is_empty());
    app.quit();
}

#[test]
fn closing_the_owner_retires_help_without_another_render() {
    let (mut app, _, mut owner) = setup();
    let other = app.add_empty_window();
    let trigger = owner.debug_bounds("tooltip-button").unwrap();
    owner.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut owner);
    assert!(owner.debug_bounds("liora-tooltip-panel").is_some());
    assert_eq!(
        active(other),
        [("button-tip".to_string(), "Button help".to_string())]
    );

    owner.update(|window, _| window.remove_window());

    assert!(
        active(other).is_empty(),
        "the last owner frame cannot clean up after its window closes"
    );
    app.quit();
}

#[test]
fn stable_renders_do_not_notify_tooltip_global_observers() {
    let (mut app, handle, mut window) = setup();
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    let notices = Rc::new(Cell::new(0));
    let counted = notices.clone();
    let _observer = window.update(|window, cx| {
        window.observe_global::<ActiveTooltip>(cx, move |window, _| {
            counted.set(counted.get() + 1);
            window.refresh();
        })
    });
    for _ in 0..3 {
        handle.update(&mut app, |_, _, cx| cx.notify()).unwrap();
        draw(&mut window);
    }
    assert_eq!(
        notices.get(),
        0,
        "read-only rendering must not publish NotifyGlobalObservers"
    );
    handle
        .update(&mut app, |view, _, cx| {
            view.content = "Changed help";
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    assert_eq!(
        notices.get(),
        1,
        "a real content change must still notify observers"
    );
    press(&mut window, "escape");
    let after_close = notices.get();
    for _ in 0..3 {
        draw(&mut window);
    }
    assert_eq!(
        notices.get(),
        after_close,
        "empty stable rendering must also be read-only"
    );
    app.quit();
}

#[test]
fn occluding_portal_releases_hover_help_without_eating_escape() {
    let (mut app, handle, mut window) = setup();
    window.update(|window, cx| window.focus_next(cx));
    let trigger = window.debug_bounds("tooltip-button").unwrap();
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    window.simulate_mouse_move(
        point(trigger.center().x, trigger.bottom() + px(4.0)),
        None,
        Modifiers::none(),
    );
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    handle
        .update(&mut app, |view, _, cx| {
            view.cover = true;
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    assert!(window.debug_bounds("tooltip-occluding-portal").is_some());
    assert!(
        active(&mut window).is_empty(),
        "a real blocking portal must end the covered hover session"
    );
    let before = handle
        .read_with(&app, |view, _| view.escapes.get())
        .unwrap();
    press(&mut window, "escape");
    assert_eq!(
        handle
            .read_with(&app, |view, _| view.escapes.get())
            .unwrap(),
        before + 1
    );
    handle
        .update(&mut app, |view, _, cx| {
            view.cover = false;
            cx.notify();
        })
        .unwrap();
    draw(&mut window);
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("button-tip".to_string(), "Button help".to_string())]
    );
    press(&mut window, "escape");
    assert!(active(&mut window).is_empty());
    assert_eq!(
        handle
            .read_with(&app, |view, _| view.escapes.get())
            .unwrap(),
        before + 1,
        "uncovered positive control must still consume Tooltip Escape"
    );
    app.quit();
}

struct OverlaySurface {
    input: Entity<Input>,
    leaked_escape: Rc<Cell<usize>>,
    popup_escape: bool,
    blocker: Option<gpui::Bounds<gpui::Pixels>>,
}

fn overlay_content(input: Entity<Input>) -> impl IntoElement {
    div().flex().flex_col().gap_4().child(input).child(
        Tooltip::new(Button::new("Hover help").id("nested-help"))
            .id("nested-tip")
            .content("Nested help")
            .placement(Placement::Right),
    )
}

impl Render for OverlaySurface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        liora_core::render_active_modal_in_window(window, cx);
        liora_core::render_active_popover_in_window(window, cx);
        liora_core::render_active_tooltip_in_window(window, cx);
        if let Some(bounds) = self.blocker {
            liora_core::push_portal(
                move |_, _| {
                    gpui::deferred(
                        div()
                            .absolute()
                            .top(bounds.top())
                            .left(bounds.left())
                            .w(bounds.size.width)
                            .h(bounds.size.height)
                            .occlude()
                            .bg(gpui::rgb(0x151515))
                            .debug_selector(|| "tooltip-partial-blocker".into()),
                    )
                    .with_priority(1)
                    .into_any_element()
                },
                cx,
            );
        }
        let input = self.input.clone();
        let leaked = self.leaked_escape.clone();
        let content = div()
            .relative()
            .size_full()
            .on_key_down(move |event, _, _| {
                if event.keystroke.key == "escape" {
                    leaked.set(leaked.get() + 1);
                }
            })
            .child(
                crate::Popover::new(Button::new("Open").id("nested-open"))
                    .id("nested-popup")
                    .close_on_escape(self.popup_escape)
                    .content(move |_, _| overlay_content(input.clone())),
            )
            .child(PortalLayer);
        AppWindowFrame::new("Tooltip overlays", content)
    }
}

fn overlay_setup(
    core_only: bool,
    popup_escape: bool,
) -> (
    TestAppContext,
    WindowHandle<OverlaySurface>,
    VisualTestContext,
) {
    let mut app = TestAppContext::single();
    app.update(|cx| {
        if core_only {
            liora_core::init_liora(cx, liora_theme::Theme::light());
            Input::register_key_bindings(cx);
        } else {
            crate::init_liora(cx);
        }
        cx.set_reduce_motion(true);
    });
    let handle = app.open_window(gpui::size(px(640.0), px(480.0)), |_, cx| OverlaySurface {
        input: cx.new(|cx| Input::new("keep", cx).id("nested-input")),
        leaked_escape: Rc::new(Cell::new(0)),
        popup_escape,
        blocker: None,
    });
    let mut window = VisualTestContext::from_window(handle.into(), &app);
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    (app, handle, window)
}

struct OtherInput(Entity<Input>);
impl Render for OtherInput {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(self.0.clone())
    }
}

fn unrelated_active_input(
    app: &mut TestAppContext,
) -> (WindowHandle<OtherInput>, VisualTestContext) {
    let other = app.open_window(gpui::size(px(320.0), px(200.0)), |_, cx| {
        OtherInput(cx.new(|cx| Input::new("other", cx)))
    });
    let mut window = VisualTestContext::from_window(other.into(), app);
    window.update(|window, _| window.activate_window());
    draw(&mut window);
    window.update(|window, cx| window.focus_next(cx));
    (other, window)
}

fn unrelated_window_escape(
    app: &TestAppContext,
    other: WindowHandle<OtherInput>,
    window: &mut VisualTestContext,
) {
    assert!(window.update(|window, cx| cx.active_window() == Some(window.window_handle())));
    press(window, "escape");
    assert_eq!(
        other
            .read_with(app, |view, cx| view.0.read(cx).value())
            .unwrap(),
        "other"
    );
}

fn return_to_hover(window: &mut VisualTestContext, position: gpui::Point<gpui::Pixels>) {
    window.update(|window, _| window.activate_window());
    window.run_until_parked();
    assert!(window.update(|window, cx| cx.active_window() == Some(window.window_handle())));
    // Activation resamples the cursor, but TestWindow::mouse_position always returns (0, 0).
    window.simulate_mouse_move(position, None, Modifiers::none());
    draw(window);
}

#[test]
fn hover_help_precedes_dialog_escape_with_core_only_and_full_init() {
    for core_only in [true, false] {
        for dismiss in [true, false] {
            let (mut app, handle, mut window) = overlay_setup(core_only, true);
            let input = handle
                .read_with(&app, |view, _| view.input.clone())
                .unwrap();
            let calls = Rc::new(Cell::new(0));
            window.update(|window, cx| {
                let input = input.clone();
                let counted = calls.clone();
                crate::Dialog::new()
                    .id("tooltip-dialog")
                    .animated(false)
                    .close_on_escape(dismiss)
                    .on_close(move |_, _| counted.set(counted.get() + 1))
                    .content(move |_, _| overlay_content(input.clone()))
                    .show_in_window(window, cx);
                // Public installation remains idempotent after show auto-installs it.
                crate::Dialog::register_key_bindings(cx);
                crate::Dialog::register_key_bindings(cx);
            });
            draw(&mut window);
            window.update(|window, cx| window.focus(&input.read(cx).focus_handle(cx), cx));
            draw(&mut window);
            let focus = window.update(|window, cx| window.focused(cx).unwrap());
            let dialog = window.update(|_, cx| {
                cx.global::<liora_core::ActiveModal>()
                    .0
                    .last()
                    .unwrap()
                    .view
                    .entity_id()
            });
            // Finish any activation-driven cursor reset before establishing hover.
            let (other, mut other_window) = unrelated_active_input(&mut app);
            let trigger = window.debug_bounds("nested-help").unwrap();
            window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
            draw(&mut window);
            assert_eq!(
                active(&mut window),
                [("nested-tip".to_string(), "Nested help".to_string())]
            );
            unrelated_window_escape(&app, other, &mut other_window);
            assert_eq!(
                active(&mut window),
                [("nested-tip".to_string(), "Nested help".to_string())]
            );
            window.update(|_, cx| {
                assert_eq!(
                    cx.global::<liora_core::ActiveModal>()
                        .0
                        .last()
                        .unwrap()
                        .view
                        .entity_id(),
                    dialog
                )
            });
            return_to_hover(&mut window, trigger.center());
            assert_eq!(
                active(&mut window),
                [("nested-tip".to_string(), "Nested help".to_string())]
            );
            press(&mut window, "escape");
            assert!(active(&mut window).is_empty());
            assert!(window.update(|window, _| focus.is_focused(window)));
            window.update(|_, cx| {
                assert_eq!(
                    cx.global::<liora_core::ActiveModal>()
                        .0
                        .last()
                        .unwrap()
                        .view
                        .entity_id(),
                    dialog
                )
            });
            assert_eq!(calls.get(), 0);
            draw(&mut window);
            assert!(
                active(&mut window).is_empty(),
                "continuous hover remains latched"
            );
            press(&mut window, "escape");
            window.update(|_, cx| {
                assert_eq!(cx.global::<liora_core::ActiveModal>().0.is_empty(), dismiss)
            });
            assert_eq!(calls.get(), usize::from(dismiss));
            assert_eq!(input.read_with(&app, |input, _| input.value()), "keep");
            assert_eq!(
                handle
                    .read_with(&app, |view, _| view.leaked_escape.get())
                    .unwrap(),
                0,
                "modal Escape must not reach Input/raw background key handlers"
            );
            if !dismiss {
                assert!(window.update(|window, _| focus.is_focused(window)));
            }
            app.quit();
        }
    }
}

#[test]
fn hover_help_precedes_real_popover_escape_under_each_policy() {
    for dismiss in [true, false] {
        let (mut app, handle, mut window) = overlay_setup(false, dismiss);
        let open = window.debug_bounds("nested-open").unwrap();
        window.simulate_click(open.center(), Modifiers::none());
        draw(&mut window);
        let popover = window.update(|_, cx| {
            cx.global::<liora_core::ActivePopover>()
                .0
                .last()
                .unwrap()
                .view
                .entity_id()
        });
        let input = handle
            .read_with(&app, |view, _| view.input.clone())
            .unwrap();
        window.update(|window, cx| window.focus(&input.read(cx).focus_handle(cx), cx));
        draw(&mut window);
        let focus = window.update(|window, cx| window.focused(cx).unwrap());
        let (other, mut other_window) = unrelated_active_input(&mut app);
        let trigger = window.debug_bounds("nested-help").unwrap();
        window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
        draw(&mut window);
        assert_eq!(
            active(&mut window),
            [("nested-tip".to_string(), "Nested help".to_string())]
        );
        unrelated_window_escape(&app, other, &mut other_window);
        assert_eq!(
            active(&mut window),
            [("nested-tip".to_string(), "Nested help".to_string())]
        );
        window.update(|_, cx| {
            assert_eq!(
                cx.global::<liora_core::ActivePopover>()
                    .0
                    .last()
                    .unwrap()
                    .view
                    .entity_id(),
                popover
            )
        });
        return_to_hover(&mut window, trigger.center());
        assert_eq!(
            active(&mut window),
            [("nested-tip".to_string(), "Nested help".to_string())]
        );
        press(&mut window, "escape");
        assert!(active(&mut window).is_empty());
        assert!(window.update(|window, _| focus.is_focused(window)));
        window.update(|_, cx| {
            assert_eq!(
                cx.global::<liora_core::ActivePopover>()
                    .0
                    .last()
                    .unwrap()
                    .view
                    .entity_id(),
                popover
            )
        });
        assert_eq!(
            handle
                .read_with(&app, |view, _| view.leaked_escape.get())
                .unwrap(),
            0
        );
        draw(&mut window);
        assert!(active(&mut window).is_empty());
        press(&mut window, "escape");
        window.update(|_, cx| {
            assert_eq!(
                cx.global::<liora_core::ActivePopover>().0.is_empty(),
                dismiss
            )
        });
        assert_eq!(input.read_with(&app, |input, _| input.value()), "keep");
        if !dismiss {
            assert!(window.update(|window, _| focus.is_focused(window)));
            window.update(|_, cx| {
                assert_eq!(
                    cx.global::<liora_core::ActivePopover>()
                        .0
                        .last()
                        .unwrap()
                        .view
                        .entity_id(),
                    popover
                )
            });
        }
        app.quit();
    }
}

#[test]
fn stationary_parent_update_requests_one_frame_and_replaces_rendered_text_and_size() {
    for reduced in [true, false] {
        let (mut app, handle, mut window) = setup();
        app.update(|cx| cx.set_reduce_motion(reduced));
        let trigger = window.debug_bounds("tooltip-button").unwrap();
        window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        app.executor().advance_clock(Duration::from_secs(1));
        window.run_until_parked();
        draw(&mut window);
        assert_eq!(
            window.update(|window, cx| window.simulate_next_frame(cx)),
            0
        );
        let old_panel = window.debug_bounds("liora-tooltip-panel").unwrap();
        assert!(
            window
                .debug_bounds("liora-tooltip-text:Button help")
                .is_some()
        );
        handle
            .update(&mut app, |view, _, cx| {
                view.content = "Stationary parent changed this help\nSecond line";
                cx.notify();
            })
            .unwrap();
        // Exactly one parent update/initial draw. A follow-up is permitted only
        // if the SDK really scheduled it; v1's dropped paint refresh schedules none.
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            window.update(|window, cx| window.simulate_next_frame(cx)),
            1,
            "one real mutation needs one bounded native follow-up frame"
        );
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            window
                .debug_bounds("liora-tooltip-text:Button help")
                .is_none()
        );
        assert!(
            window
                .debug_bounds("liora-tooltip-text:Stationary parent changed this help")
                .is_some()
        );
        assert!(
            window
                .debug_bounds("liora-tooltip-text:Second line")
                .is_some()
        );
        let panel = window.debug_bounds("liora-tooltip-panel").unwrap();
        assert!(panel.size.width > old_panel.size.width);
        assert!(panel.size.height > old_panel.size.height);
        assert_eq!(
            window.update(|window, cx| window.simulate_next_frame(cx)),
            0,
            "the settled tooltip must not request another idle frame"
        );
        app.quit();
    }
}

#[test]
fn clipped_dialog_trigger_keeps_help_across_the_actual_portaled_gap() {
    let (app, handle, mut window) = overlay_setup(false, true);
    let input = handle
        .read_with(&app, |view, _| view.input.clone())
        .unwrap();
    let closed = Rc::new(Cell::new(0));
    window.update(|window, cx| {
        let input = input.clone();
        let closed = closed.clone();
        crate::Dialog::new()
            .id("clipped-tooltip-dialog")
            .animated(false)
            .on_close(move |_, _| closed.set(closed.get() + 1))
            .content(move |_, _| {
                div().flex().flex_col().gap_4().child(input.clone()).child(
                    div()
                        .w(px(220.0))
                        .h(px(48.0))
                        .overflow_hidden()
                        .debug_selector(|| "tooltip-trigger-clip".into())
                        .flex()
                        .justify_end()
                        .items_center()
                        .child(
                            Tooltip::new(Button::new("Clipped help").id("clipped-help"))
                                .id("clipped-tip")
                                .content("Clipped help")
                                .placement(Placement::Right),
                        ),
                )
            })
            .show_in_window(window, cx);
    });
    draw(&mut window);
    window.update(|window, cx| window.focus(&input.read(cx).focus_handle(cx), cx));
    draw(&mut window);
    let focus = window.update(|window, cx| window.focused(cx).unwrap());
    let dialog = window.update(|_, cx| {
        cx.global::<liora_core::ActiveModal>()
            .0
            .last()
            .unwrap()
            .view
            .entity_id()
    });
    let trigger = window.debug_bounds("clipped-help").unwrap();
    let clip = window.debug_bounds("tooltip-trigger-clip").unwrap();
    assert!(
        (clip.right() - trigger.right()).abs() < px(1.0),
        "trigger must touch the real ancestor clip edge"
    );
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("clipped-tip".to_string(), "Clipped help".to_string())]
    );
    let panel = window.debug_bounds("liora-tooltip-panel").unwrap();
    assert!(
        panel.left() > clip.right(),
        "the measured panel must be beyond the trigger clip"
    );
    let gap = point((trigger.right() + panel.left()) / 2.0, trigger.center().y);
    assert!(
        gap.x > clip.right() && gap.x < panel.left(),
        "exercise the real open offset, not either endpoint"
    );
    for position in [gap, panel.center()] {
        window.simulate_mouse_move(position, None, Modifiers::none());
        draw(&mut window);
        assert_eq!(
            active(&mut window),
            [("clipped-tip".to_string(), "Clipped help".to_string())]
        );
        assert!(window.debug_bounds("liora-tooltip-panel").is_some());
        assert!(window.update(|window, _| focus.is_focused(window)));
    }
    press(&mut window, "escape");
    assert!(active(&mut window).is_empty());
    assert!(window.update(|window, _| focus.is_focused(window)));
    assert_eq!(closed.get(), 0);
    window.update(|_, cx| {
        assert_eq!(
            cx.global::<liora_core::ActiveModal>()
                .0
                .last()
                .unwrap()
                .view
                .entity_id(),
            dialog
        )
    });
    window.simulate_mouse_move(gap, None, Modifiers::none());
    draw(&mut window);
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert!(
        active(&mut window).is_empty(),
        "escaping while on the panel must not rearm inside the same portal bridge"
    );
    window.simulate_mouse_move(point(px(620.0), px(460.0)), None, Modifiers::none());
    draw(&mut window);
    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
    draw(&mut window);
    assert_eq!(
        active(&mut window),
        [("clipped-tip".to_string(), "Clipped help".to_string())]
    );
    assert_eq!(input.read_with(&app, |input, _| input.value()), "keep");
    assert_eq!(closed.get(), 0);
    app.quit();
}

#[test]
fn panel_occlusion_preserves_help_first_for_actual_focus_or_hover_sources() {
    for dialog in [true, false] {
        for owner_focus in [false, true] {
            for fully_covered in [false, true] {
                let (mut app, handle, mut window) = overlay_setup(false, true);
                let input = handle
                    .read_with(&app, |view, _| view.input.clone())
                    .unwrap();
                let closed = Rc::new(Cell::new(0));
                if dialog {
                    window.update(|window, cx| {
                        let input = input.clone();
                        let closed = closed.clone();
                        crate::Dialog::new()
                            .id("partly-covered-dialog")
                            .animated(false)
                            .on_close(move |_, _| closed.set(closed.get() + 1))
                            .content(move |_, _| overlay_content(input.clone()))
                            .show_in_window(window, cx);
                    });
                } else {
                    let open = window.debug_bounds("nested-open").unwrap();
                    window.simulate_click(open.center(), Modifiers::none());
                }
                draw(&mut window);
                let container = window.update(|_, cx| {
                    if dialog {
                        cx.global::<liora_core::ActiveModal>()
                            .0
                            .last()
                            .unwrap()
                            .view
                            .entity_id()
                    } else {
                        cx.global::<liora_core::ActivePopover>()
                            .0
                            .last()
                            .unwrap()
                            .view
                            .entity_id()
                    }
                });
                window.update(|window, cx| window.focus(&input.read(cx).focus_handle(cx), cx));
                draw(&mut window);
                if owner_focus {
                    press(&mut window, "tab");
                    assert_eq!(
                        active(&mut window),
                        [("nested-tip".to_string(), "Nested help".to_string())],
                        "actual keyboard focus must open this tooltip without a pointer hover"
                    );
                    assert!(
                        !window.update(|window, cx| input
                            .read(cx)
                            .focus_handle(cx)
                            .is_focused(window))
                    );
                }
                let focus = window.update(|window, cx| window.focused(cx).unwrap());
                let trigger = window.debug_bounds("nested-help").unwrap();
                if !owner_focus {
                    window.simulate_mouse_move(trigger.center(), None, Modifiers::none());
                    draw(&mut window);
                }
                let panel = window.debug_bounds("liora-tooltip-panel").unwrap();
                let edge = point(panel.left() + px(2.0), panel.center().y);
                if owner_focus {
                    let pointer = window.update(|window, _| window.mouse_position());
                    assert!(
                        !trigger.union(&panel).contains(&pointer),
                        "focus case must have no source, panel, or bridge hover"
                    );
                }
                let cover = if fully_covered {
                    gpui::Bounds {
                        origin: gpui::Point::default(),
                        size: window.update(|window, _| window.viewport_size()),
                    }
                } else {
                    gpui::Bounds {
                        origin: point(panel.left() + panel.size.width / 4.0, panel.top()),
                        size: gpui::size(panel.size.width / 2.0, panel.size.height),
                    }
                };
                handle
                    .update(&mut app, |view, _, cx| {
                        view.blocker = Some(cover);
                        cx.notify();
                    })
                    .unwrap();
                draw(&mut window);
                let blocker = window.debug_bounds("tooltip-partial-blocker").unwrap();
                assert!(
                    blocker.contains(&panel.center()),
                    "the real high-priority portal must cover the center"
                );
                if fully_covered {
                    assert!(
                        blocker.contains(&edge) && blocker.contains(&trigger.center()),
                        "full-cover negative control must actually occlude every hover source"
                    );
                } else {
                    assert!(
                        !blocker.contains(&edge),
                        "the measured panel edge must remain outside the blocker"
                    );
                    if !owner_focus {
                        window.simulate_mouse_move(edge, None, Modifiers::none());
                        draw(&mut window);
                    }
                }
                if !owner_focus && fully_covered {
                    assert!(
                        active(&mut window).is_empty(),
                        "unfocused help must retire when real occlusion removes all hover sources"
                    );
                } else {
                    assert_eq!(
                        active(&mut window),
                        [("nested-tip".to_string(), "Nested help".to_string())]
                    );
                    assert!(window.debug_bounds("liora-tooltip-panel").is_some());
                    press(&mut window, "escape");
                    assert!(active(&mut window).is_empty());
                    assert!(window.update(|window, _| focus.is_focused(window)));
                    window.update(|_, cx| {
                        let current = if dialog {
                            cx.global::<liora_core::ActiveModal>()
                                .0
                                .last()
                                .unwrap()
                                .view
                                .entity_id()
                        } else {
                            cx.global::<liora_core::ActivePopover>()
                                .0
                                .last()
                                .unwrap()
                                .view
                                .entity_id()
                        };
                        assert_eq!(
                            current, container,
                            "first Escape must retain the same real container"
                        );
                    });
                    assert_eq!(closed.get(), 0);
                    assert_eq!(input.read_with(&app, |input, _| input.value()), "keep");
                    draw(&mut window);
                    assert!(
                        active(&mut window).is_empty(),
                        "continuous focus or exposed-edge hover stays dismissed"
                    );
                }
                press(&mut window, "escape");
                window.update(|_, cx| {
                    if dialog {
                        assert!(cx.global::<liora_core::ActiveModal>().0.is_empty());
                    } else {
                        assert!(cx.global::<liora_core::ActivePopover>().0.is_empty());
                    }
                });
                assert_eq!(closed.get(), usize::from(dialog));
                assert_eq!(
                    handle
                        .read_with(&app, |view, _| view.leaked_escape.get())
                        .unwrap(),
                    0
                );
                app.quit();
            }
        }
    }
}
