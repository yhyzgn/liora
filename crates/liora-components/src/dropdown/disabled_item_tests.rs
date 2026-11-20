use crate::{Dropdown, PortalLayer};
use gpui::{
    App, Context, KeyDownEvent, KeyUpEvent, Keystroke, MouseButton, MouseDownEvent, MouseMoveEvent,
    MouseUpEvent, PlatformInput, Render, TestAppContext, Window, div, point, prelude::*, px, size,
};
use liora_core::{is_popover_active, render_active_popover_in_window};
use std::{cell::RefCell, rc::Rc};

const ID: &str = "dropdown-disabled-item-test";

struct Surface(Rc<RefCell<Vec<usize>>>);

impl Render for Surface {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        render_active_popover_in_window(window, cx);
        let first = self.0.clone();
        let last = self.0.clone();
        let menu = Dropdown::new(div().id("plain-trigger").size(px(80.)).child("Actions"))
            .id(ID)
            .close_on_click_outside(false)
            .item("First", move |_, _| first.borrow_mut().push(0))
            .disabled_item("No permission")
            .item("Last", move |_, _| last.borrow_mut().push(2));
        div()
            .size_full()
            .relative()
            .child(div().absolute().top(px(24.)).left(px(24.)).child(menu))
            .child(PortalLayer)
    }
}

fn draw(window: &mut Window, cx: &mut App) {
    window.draw(cx).clear(cx);
}

fn key(window: &mut Window, cx: &mut App, key: &str, down: bool) {
    let keystroke = Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
    let event = if down {
        PlatformInput::KeyDown(KeyDownEvent {
            keystroke,
            is_held: false,
            prefer_character_input: false,
        })
    } else {
        PlatformInput::KeyUp(KeyUpEvent { keystroke })
    };
    window.dispatch_event(event, cx);
}

fn press(window: &mut Window, cx: &mut App, stroke: &str) {
    key(window, cx, stroke, true);
    key(window, cx, stroke, false);
}

fn pointer(window: &mut Window, cx: &mut App, position: gpui::Point<gpui::Pixels>, down: bool) {
    window.dispatch_event(
        PlatformInput::MouseMove(MouseMoveEvent {
            position,
            ..Default::default()
        }),
        cx,
    );
    let event = if down {
        PlatformInput::MouseDown(MouseDownEvent {
            position,
            button: MouseButton::Left,
            click_count: 1,
            ..Default::default()
        })
    } else {
        PlatformInput::MouseUp(MouseUpEvent {
            position,
            button: MouseButton::Left,
            click_count: 1,
            ..Default::default()
        })
    };
    window.dispatch_event(event, cx);
}

#[test]
fn dropdown_disabled_item_is_inert_and_navigation_skips_it() {
    for pointer_only in [true, false] {
        let mut app = TestAppContext::single();
        app.update(crate::init_liora);
        let commands = Rc::new(RefCell::new(Vec::new()));
        let window = app.open_window(size(px(480.), px(360.)), {
            let commands = commands.clone();
            move |_, _| Surface(commands)
        });
        app.update_window(window.into(), |_, window, cx| {
            draw(window, cx);
            let trigger = point(px(42.), px(38.));
            pointer(window, cx, trigger, true);
            pointer(window, cx, trigger, false);
            assert!(
                is_popover_active(&ID.into(), cx),
                "real trigger must open the production menu"
            );
            draw(window, cx);
        })
        .unwrap();
        if pointer_only {
            let mut visual = gpui::VisualTestContext::from_window(window.into(), &app);
            let disabled = visual
                .debug_bounds("dropdown-disabled-item-test-item-1")
                .expect("painted disabled item")
                .center();
            app.update_window(window.into(), |_, window, cx| {
                pointer(window, cx, disabled, true);
                pointer(window, cx, disabled, false);
                assert!(
                    commands.borrow().is_empty(),
                    "disabled item has no command callback"
                );
                assert!(is_popover_active(&ID.into(), cx));
            })
            .unwrap();
        } else {
            app.update_window(window.into(), |_, window, cx| {
                window.simulate_next_frame(cx);
                press(window, cx, "down");
                draw(window, cx);
                press(window, cx, "enter");
                assert_eq!(*commands.borrow(), [2], "navigation skips the disabled row");
                assert!(!is_popover_active(&ID.into(), cx));
            })
            .unwrap();
        }
    }
}
