//! Shared trigger interaction and native portal rendering for help tooltips.

use crate::{
    ActiveTooltip, Config, Popper, TooltipData, clear_tooltip, push_passive_portal,
    set_exclusive_active_tooltip, tooltip_content_lines,
};
use gpui::{
    Animation, AnimationExt, AnyElement, AnyWindowHandle, App, Bounds, DispatchPhase, Element,
    ElementId, FocusHandle, Global, GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId,
    IntoElement, LayoutId, MouseExitEvent, MouseMoveEvent, Pixels, Subscription, TextRun, Window,
    div, prelude::*, px,
};
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
    time::Duration,
};

struct TriggerState {
    data: TooltipData,
    focus_scope: FocusHandle,
    hovered: bool,
    pointer_in_window: bool,
    focused: bool,
    dismissed: bool,
    panel_bounds: Option<Bounds<Pixels>>,
    panel_hitbox: Option<Hitbox>,
    bridge_hitbox: Option<Hitbox>,
    trigger_hitbox: Option<Hitbox>,
    _focus_subscriptions: Option<[Subscription; 2]>,
}

struct ActiveTrigger {
    state: Weak<RefCell<TriggerState>>,
    id: gpui::SharedString,
    window: AnyWindowHandle,
    _escape: Subscription,
    _window_closed: Subscription,
}
impl Global for ActiveTrigger {}

fn owns_trigger(state: &Rc<RefCell<TriggerState>>, cx: &App) -> bool {
    cx.try_global::<ActiveTrigger>()
        .is_some_and(|active| active.state.as_ptr() == Rc::as_ptr(state))
}

fn owns_active(state: &Rc<RefCell<TriggerState>>, cx: &App) -> bool {
    owns_trigger(state, cx)
        && cx
            .global::<ActiveTooltip>()
            .0
            .iter()
            .any(|tip| tip.id == state.borrow().data.id)
}

fn sync_sources(
    state: &Rc<RefCell<TriggerState>>,
    hovered: bool,
    focused: bool,
    window: &mut Window,
    cx: &mut App,
) {
    let current = owns_active(state, cx);
    let owner = owns_trigger(state, cx);
    let mut value = state.borrow_mut();
    let entered = (!value.hovered && hovered) || (!value.focused && focused);
    value.hovered = hovered;
    value.focused = focused;
    if !hovered && !focused {
        value.dismissed = false;
        value.panel_bounds = None;
        value.panel_hitbox = None;
        value.bridge_hitbox = None;
        if owner {
            if current {
                clear_tooltip(&value.data.id, cx);
            }
            cx.remove_global::<ActiveTrigger>();
            window.on_next_frame(|window, _| window.refresh());
        }
    } else if !value.dismissed
        && (entered || current)
        && value.data.anchor_bounds.size.width > px(0.0)
    {
        let changed = set_exclusive_active_tooltip(value.data.clone(), cx);
        if !current {
            let weak = Rc::downgrade(state);
            let owner_window = window.window_handle();
            let escape = cx.intercept_keystrokes(move |event, window, cx| {
                if window.window_handle() != owner_window
                    || event.keystroke.key != "escape"
                    || event.keystroke.modifiers != gpui::Modifiers::none()
                {
                    return;
                }
                if let Some(state) = weak.upgrade()
                    && owns_active(&state, cx)
                {
                    let focused = state.borrow().focus_scope.contains_focused(window, cx);
                    let hovered = pointer_present(&state, window, cx);
                    // shortcut: pinned GPUI exposes point hit-tests only; keep active-source help first until an area-visibility API exists.
                    if !focused && !hovered {
                        sync_sources(&state, false, false, window, cx);
                        return;
                    }
                    let mut value = state.borrow_mut();
                    value.focused = focused;
                    value.hovered = hovered;
                    value.dismissed = true;
                    value.panel_hitbox = None;
                    clear_tooltip(&value.data.id, cx);
                    window.refresh();
                    cx.stop_propagation();
                }
            });
            let window_closed = cx.on_window_closed(move |cx, closed| {
                if cx.try_global::<ActiveTrigger>().is_some_and(|owner| {
                    owner.window == owner_window && owner_window.window_id() == closed
                }) {
                    let owner = cx.remove_global::<ActiveTrigger>();
                    clear_tooltip(&owner.id, cx);
                }
            });
            cx.set_global(ActiveTrigger {
                state: Rc::downgrade(state),
                id: value.data.id.clone(),
                window: owner_window,
                _escape: escape,
                _window_closed: window_closed,
            });
        }
        if changed {
            window.on_next_frame(|window, _| window.refresh());
        }
    }
}

fn pointer_present(state: &Rc<RefCell<TriggerState>>, window: &Window, cx: &App) -> bool {
    let value = state.borrow();
    let hovered = |hitbox: &Hitbox| hitbox.is_hovered_at(window.mouse_position(), window);
    value.pointer_in_window
        && (value.trigger_hitbox.as_ref().is_some_and(hovered)
            || (owns_trigger(state, cx)
                && (value.dismissed || owns_active(state, cx))
                && (value.bridge_hitbox.as_ref().is_some_and(hovered)
                    || value.panel_hitbox.as_ref().is_some_and(hovered))))
}

/// Composes a help trigger without adding a tab stop or taking its child's focus.
///
/// Used by the component crate; chart/data tooltips retain their existing API.
pub fn render_trigger(
    trigger: AnyElement,
    data: TooltipData,
    window: &mut Window,
    cx: &mut App,
) -> impl IntoElement {
    let id = data.id.clone();
    let state = window.with_global_id(id.clone().into(), |global_id, window| {
        window.with_element_state(
            global_id,
            |previous: Option<Rc<RefCell<TriggerState>>>, window| {
                let state = previous.unwrap_or_else(|| {
                    let scope = cx.focus_handle().tab_stop(false);
                    let state = Rc::new(RefCell::new(TriggerState {
                        data: data.clone(),
                        focus_scope: scope.clone(),
                        hovered: false,
                        pointer_in_window: window.is_window_hovered(),
                        focused: false,
                        dismissed: false,
                        panel_bounds: None,
                        panel_hitbox: None,
                        bridge_hitbox: None,
                        trigger_hitbox: None,
                        _focus_subscriptions: None,
                    }));
                    let weak_in = Rc::downgrade(&state);
                    let focus_in = window.on_focus_in(&scope, cx, move |window, cx| {
                        if let Some(state) = weak_in.upgrade() {
                            let hovered = state.borrow().hovered;
                            sync_sources(&state, hovered, true, window, cx);
                        }
                    });
                    let weak_out = Rc::downgrade(&state);
                    let focus_out = window.on_focus_out(&scope, cx, move |_, window, cx| {
                        if let Some(state) = weak_out.upgrade() {
                            let hovered = state.borrow().hovered;
                            sync_sources(&state, hovered, false, window, cx);
                        }
                    });
                    state.borrow_mut()._focus_subscriptions = Some([focus_in, focus_out]);
                    state
                });
                {
                    let mut value = state.borrow_mut();
                    value.data.content = data.content;
                    value.data.placement = data.placement;
                    value.data.offset = data.offset;
                }
                (state.clone(), state)
            },
        )
    });
    div().id(id).child(TriggerAnchor { trigger, state })
}

struct TriggerAnchor {
    trigger: AnyElement,
    state: Rc<RefCell<TriggerState>>,
}

impl Element for TriggerAnchor {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.trigger.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
        // A dispatch-only scope observes descendants without Div's mouse-focus behavior.
        window.set_focus_handle(&self.state.borrow().focus_scope, cx);
        let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);
        self.state.borrow_mut().trigger_hitbox = Some(hitbox);
        self.trigger.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.state.borrow_mut().data.anchor_bounds = bounds;
        // Re-test against the completed native hit-test tree, including occluding portals.
        let state = self.state.clone();
        window.defer(cx, move |window, cx| {
            let hovered = pointer_present(&state, window, cx);
            let focused = state.borrow().focus_scope.contains_focused(window, cx);
            sync_sources(&state, hovered, focused, window, cx);
        });
        let state = self.state.clone();
        window.on_mouse_event(move |_: &MouseMoveEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                state.borrow_mut().pointer_in_window = true;
                let hovered = pointer_present(&state, window, cx);
                let focused = state.borrow().focused;
                sync_sources(&state, hovered, focused, window, cx);
            }
        });
        let state = self.state.clone();
        window.on_mouse_event(move |_: &MouseExitEvent, phase, window, cx| {
            if phase == DispatchPhase::Capture {
                state.borrow_mut().pointer_in_window = false;
                let focused = state.borrow().focused;
                sync_sources(&state, false, focused, window, cx);
            }
        });
        self.trigger.paint(window, cx);
    }
}

pub(crate) fn render_active(window: &mut Window, cx: &mut App) {
    let managed = cx
        .try_global::<ActiveTrigger>()
        .map(|owner| (owner.state.upgrade(), owner.window));
    if let Some(owner) = cx.try_global::<ActiveTrigger>()
        && owner.window == window.window_handle()
    {
        let state = owner.state.clone();
        // After GPUI drops the old frame, only a mounted trigger retains this state.
        window.defer(cx, move |window, cx| {
            if state.strong_count() == 0
                && cx
                    .try_global::<ActiveTrigger>()
                    .is_some_and(|owner| owner.state.ptr_eq(&state))
            {
                let owner = cx.remove_global::<ActiveTrigger>();
                clear_tooltip(&owner.id, cx);
                window.on_next_frame(|window, _| window.refresh());
            }
        });
    }
    let mouse = window.mouse_position();
    let keep = |data: &TooltipData| {
        if let Some((Some(state), owner_window)) = &managed {
            let value = state.borrow();
            if value.data.id == data.id {
                return *owner_window != window.window_handle()
                    || (!value.dismissed && (value.focused || value.hovered));
            }
        }
        data.anchor_bounds.contains(&mouse)
    };
    if cx
        .global::<ActiveTooltip>()
        .0
        .iter()
        .any(|data| !keep(data))
    {
        cx.global_mut::<ActiveTooltip>().0.retain(keep);
    }
    if let Some((Some(state), owner_window)) = &managed
        && *owner_window == window.window_handle()
        && state.borrow().dismissed
    {
        let state = Rc::downgrade(state);
        // Keep a dismissed hover session latched without recreating visible help.
        push_passive_portal(
            move |_, _| {
                gpui::deferred(
                    gpui::canvas(
                        move |_, window, _| {
                            let Some(state) = state.upgrade() else {
                                return;
                            };
                            let mut value = state.borrow_mut();
                            if let Some(panel) = value.panel_bounds {
                                value.bridge_hitbox = Some(window.insert_hitbox(
                                    value.data.anchor_bounds.union(&panel),
                                    HitboxBehavior::Normal,
                                ));
                            }
                        },
                        |_, _, _, _| {},
                    )
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full(),
                )
                .into_any_element()
            },
            cx,
        );
    }
    let active = cx.global::<ActiveTooltip>().0.clone();
    for (tooltip_index, data) in active.into_iter().enumerate() {
        let owner = managed.as_ref().and_then(|(state, owner_window)| {
            state
                .as_ref()
                .filter(|state| state.borrow().data.id == data.id)
                .map(|state| (Rc::downgrade(state), *owner_window))
        });
        if owner
            .as_ref()
            .is_some_and(|(_, owner_window)| *owner_window != window.window_handle())
        {
            continue;
        }
        let theme = cx.global::<Config>().theme.clone();
        let font_size = px(theme.font_size.sm);
        let text_style = window.text_style();
        let lines = tooltip_content_lines(&data.content);
        let max_line_width = lines
            .iter()
            .map(|line| {
                let run = TextRun {
                    len: line.len(),
                    font: text_style.font(),
                    color: theme.neutral.card,
                    background_color: None,
                    underline: None,
                    strikethrough: None,
                };
                window
                    .text_system()
                    .shape_line(line.clone(), font_size, &[run], None)
                    .width
            })
            .fold(px(0.0), |width, next| width.max(next));
        let padding_h = px(12.0);
        let padding_v = px(6.0);
        let content_size = gpui::Size {
            width: max_line_width + padding_h * 2.0,
            height: window.line_height() * lines.len() + padding_v * 2.0,
        };
        push_passive_portal(
            move |window, _cx| {
                let viewport = Bounds {
                    origin: gpui::Point::default(),
                    size: window.viewport_size(),
                };
                let popper = Popper {
                    anchor_bounds: data.anchor_bounds,
                    placement: data.placement,
                    offset: data.offset,
                };
                let (pos, _) = popper.calculate_position_with_flip(content_size, viewport);
                if let Some(state) = owner.as_ref().and_then(|(state, _)| state.upgrade()) {
                    state.borrow_mut().panel_bounds = Some(Bounds {
                        origin: pos,
                        size: content_size,
                    });
                }
                let managed = owner.is_some();
                let mut panel = div()
                    .id(("liora-tooltip-panel", tooltip_index))
                    .role(gpui::Role::Tooltip)
                    .aria_label(data.content)
                    .debug_selector(|| "liora-tooltip-panel".into())
                    .absolute()
                    .cursor_default()
                    .when(managed, |panel| panel.block_mouse_except_scroll())
                    .top(pos.y)
                    .left(pos.x)
                    .w(content_size.width)
                    .h(content_size.height)
                    .bg(theme.neutral.text_1)
                    .text_color(theme.neutral.card)
                    .px(padding_h)
                    .py(padding_v)
                    .flex()
                    .flex_col()
                    .items_start()
                    .justify_center()
                    .gap(px(2.0))
                    .rounded(px(theme.radius.sm))
                    .shadow_lg()
                    .text_size(font_size)
                    .children(lines.into_iter().map(|line| {
                        let label = div().whitespace_nowrap();
                        #[cfg(debug_assertions)]
                        let label = label.debug_selector({
                            let line = line.clone();
                            move || format!("liora-tooltip-text:{line}").into()
                        });
                        label.child(line)
                    }));
                if let Some((state, _)) = owner {
                    panel = panel.child(
                        gpui::canvas(
                            move |bounds, window, _| {
                                let Some(state) = state.upgrade() else {
                                    return;
                                };
                                let mut value = state.borrow_mut();
                                value.bridge_hitbox = Some(window.insert_hitbox(
                                    value.data.anchor_bounds.union(&bounds),
                                    HitboxBehavior::Normal,
                                ));
                                value.panel_hitbox =
                                    Some(window.insert_hitbox(bounds, HitboxBehavior::Normal));
                            },
                            |_, _, _, _| {},
                        )
                        .absolute()
                        .top_0()
                        .left_0()
                        .size_full(),
                    );
                }
                let panel = panel.with_animation(
                    ("liora-tooltip-motion", tooltip_index),
                    Animation::new(Duration::from_millis(220)).with_easing(gpui::ease_out_quint()),
                    |tooltip, delta| tooltip.opacity(delta),
                );
                if managed {
                    // Official deferred drawing keeps nested help above its containing portal.
                    gpui::deferred(panel).into_any_element()
                } else {
                    panel.into_any_element()
                }
            },
            cx,
        );
    }
}
