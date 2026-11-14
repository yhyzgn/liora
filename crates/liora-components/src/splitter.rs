//! Native, keyboard-accessible resizable two-pane layout.

use std::sync::Arc;

use gpui::{
    App, Context, Entity, FocusHandle, IntoElement, KeyDownEvent, Pixels, Render, RenderOnce,
    SharedString, ViewElement as Component, Window, div, prelude::*, px,
};

const HANDLE_SIZE: f32 = 6.0;
const MIN_OTHER_PANE: f32 = 80.0;
const DEFAULT_SIZE: f32 = 300.0;

/// The direction in which the two panes are arranged.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SplitterOrientation {
    /// Panes are side-by-side; the divider moves horizontally.
    #[default]
    Horizontal,
    /// Panes are stacked; the divider moves vertically.
    Vertical,
}

/// Fluent native GPUI component for a persistent, resizable two-pane splitter.
pub struct Splitter {
    id: SharedString,
    left: Option<gpui::AnyElement>,
    right: Option<gpui::AnyElement>,
    height: Option<Pixels>,
    bordered: bool,
    orientation: SplitterOrientation,
    default_size: Pixels,
    controlled_size: Option<Pixels>,
    min_size: Pixels,
    max_size: Option<Pixels>,
    on_change: Option<Arc<dyn Fn(Pixels) + 'static>>,
}

impl Splitter {
    /// Creates a horizontal splitter. The default identity is stable at this call site;
    /// use [`Self::id`] when constructing repeated splitters from a loop.
    #[track_caller]
    pub fn new() -> Self {
        let caller = std::panic::Location::caller();
        Self {
            id: format!(
                "splitter:{}:{}:{}",
                caller.file(),
                caller.line(),
                caller.column()
            )
            .into(),
            left: None,
            right: None,
            height: None,
            bordered: false,
            orientation: SplitterOrientation::Horizontal,
            default_size: px(DEFAULT_SIZE),
            controlled_size: None,
            min_size: px(80.0),
            max_size: None,
            on_change: None,
        }
    }

    /// Sets the stable state identity. Required for multiple instances created at one call site.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Applies the left/top pane content.
    pub fn left(mut self, el: impl IntoElement) -> Self {
        self.left = Some(el.into_any_element());
        self
    }

    /// Applies the right/bottom pane content.
    pub fn right(mut self, el: impl IntoElement) -> Self {
        self.right = Some(el.into_any_element());
        self
    }

    /// Sets the component height, used as its cross-axis size.
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    /// Applies the predefined medium height.
    pub fn height_md(self) -> Self {
        self.height(px(200.0))
    }

    /// Applies a border around the splitter.
    pub fn bordered(mut self) -> Self {
        self.bordered = true;
        self
    }

    /// Selects the split direction.
    pub fn orientation(mut self, orientation: SplitterOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Sets the initial pane size for an uncontrolled splitter.
    pub fn default_size(mut self, size: impl Into<Pixels>) -> Self {
        self.default_size = size.into().max(self.min_size);
        self
    }

    /// Sets a controlled pane size. The host should pass the updated value back after [`Self::on_change`].
    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.controlled_size = Some(size.into());
        self
    }

    /// Sets the minimum size of the first pane. The second pane retains at least 80px when space permits.
    pub fn min_size(mut self, size: impl Into<Pixels>) -> Self {
        self.min_size = size.into().max(px(0.0));
        self.default_size = self.default_size.max(self.min_size);
        self.max_size = self.max_size.map(|max| max.max(self.min_size));
        self
    }

    /// Sets the maximum size of the first pane.
    pub fn max_size(mut self, size: impl Into<Pixels>) -> Self {
        self.max_size = Some(size.into().max(self.min_size));
        self
    }

    /// Called after pointer or keyboard resizing; controlled splitters report changes without mutating `size`.
    pub fn on_change(mut self, callback: impl Fn(Pixels) + 'static) -> Self {
        self.on_change = Some(Arc::new(callback));
        self
    }
}

struct SplitterState {
    size: Pixels,
    available: Option<Pixels>,
    last_reported: Option<Pixels>,
    controlled_size: Option<Pixels>,
    focus_handle: FocusHandle,
}

struct SplitterDrag {
    source: gpui::EntityId,
}

struct SplitterDragPreview;

impl Render for SplitterDragPreview {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().w(px(6.0)).h(px(36.0)).rounded(px(3.0)).bg(cx
            .global::<liora_core::Config>()
            .theme
            .primary
            .base)
    }
}

impl RenderOnce for Splitter {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<liora_core::Config>().theme.clone();
        let initial_size = self.controlled_size.unwrap_or(self.default_size);
        let state = window.use_keyed_state(self.id.clone(), cx, move |_, cx| SplitterState {
            size: initial_size,
            available: None,
            last_reported: None,
            controlled_size: self.controlled_size,
            focus_handle: cx.focus_handle(),
        });
        let focus_handle = state.read(cx).focus_handle.clone();
        let controlled_size = self.controlled_size;
        state.update(cx, |state, _| {
            if state.controlled_size != controlled_size {
                state.controlled_size = controlled_size;
                state.last_reported = None;
            }
        });
        let orientation = self.orientation;
        let min_size = self.min_size;
        let max_size = self.max_size;
        let on_change = self.on_change.clone();
        let displayed_size = clamp_size(
            controlled_size.unwrap_or(state.read(cx).size),
            min_size,
            max_size,
            state.read(cx).available,
        );
        let first = self.left.unwrap_or_else(|| div().into_any_element());
        let second = self.right.unwrap_or_else(|| div().into_any_element());
        let horizontal = orientation == SplitterOrientation::Horizontal;

        let handle = div()
            .id(format!("{}-divider", self.id))
            .flex_none()
            .w(if horizontal { px(HANDLE_SIZE) } else { px(0.0) })
            .h(if horizontal { px(0.0) } else { px(HANDLE_SIZE) })
            .when(horizontal, |this| this.h_full().cursor_ew_resize())
            .when(!horizontal, |this| this.w_full().cursor_ns_resize())
            .flex()
            .items_center()
            .justify_center()
            .track_focus(&focus_handle)
            .tab_index(0)
            .on_mouse_down(gpui::MouseButton::Left, {
                let focus = focus_handle.clone();
                move |_, window, cx| focus.focus(window, cx)
            })
            .child(
                div()
                    .when(horizontal, |this| this.w(px(2.0)).h_full())
                    .when(!horizontal, |this| this.h(px(2.0)).w_full())
                    .bg(theme.neutral.border),
            )
            .on_drag(
                SplitterDrag {
                    source: state.entity_id(),
                },
                {
                    let state = state.clone();
                    move |_, _, _, cx| {
                        state.update(cx, |state, cx| {
                            state.last_reported = None;
                            cx.notify();
                        });
                        cx.new(|_| SplitterDragPreview)
                    }
                },
            )
            .on_key_down({
                let state = state.clone();
                let on_change = on_change.clone();
                move |event: &KeyDownEvent, window, cx| {
                    let key = event.keystroke.key.as_str();
                    let step = if event.keystroke.modifiers.shift {
                        px(50.0)
                    } else {
                        px(10.0)
                    };
                    let current = controlled_size.unwrap_or(state.read(cx).size);
                    let requested = match (orientation, key) {
                        (SplitterOrientation::Horizontal, "left")
                        | (SplitterOrientation::Vertical, "up") => {
                            Some((current - step).max(px(0.0)))
                        }
                        (SplitterOrientation::Horizontal, "right")
                        | (SplitterOrientation::Vertical, "down") => Some(current + step),
                        (_, "home") => Some(min_size),
                        (_, "end") => Some(
                            max_size
                                .or_else(|| {
                                    state.read(cx).available.map(|available| {
                                        (available - px(MIN_OTHER_PANE)).max(min_size)
                                    })
                                })
                                .unwrap_or(current + step),
                        ),
                        _ => None,
                    };
                    if let Some(requested) = requested {
                        request_size(
                            &state,
                            requested,
                            controlled_size,
                            min_size,
                            max_size,
                            on_change.as_ref(),
                            window,
                            cx,
                        );
                        cx.stop_propagation();
                    }
                }
            });

        let state_for_drag = state.clone();
        let callback_for_drag = on_change.clone();
        let root = div()
            .id(format!("{}-root", self.id))
            .flex()
            .relative()
            .when(horizontal, |this| this.flex_row())
            .when(!horizontal, |this| this.flex_col())
            .w_full()
            .h_full()
            .min_w(px(0.0))
            .min_h(px(0.0))
            .when_some(self.height, |this, height| this.h(height))
            .when(self.bordered, |this| {
                this.border_1()
                    .border_color(theme.neutral.border)
                    .rounded(px(theme.radius.sm))
            })
            .child(
                gpui::canvas(
                    {
                        let state = state.clone();
                        move |bounds, window, cx| {
                            let total = if horizontal {
                                bounds.size.width
                            } else {
                                bounds.size.height
                            };
                            let available = (total - px(HANDLE_SIZE)).max(px(0.0));
                            let changed = state.update(cx, |state, _| {
                                let changed = state.available != Some(available);
                                state.available = Some(available);
                                changed
                            });
                            if changed {
                                // Measurement runs during draw; invalidate only after draw completes.
                                let state = state.clone();
                                window.defer(cx, move |window, cx| {
                                    state.update(cx, |_, cx| cx.notify());
                                    window.refresh();
                                });
                            }
                        }
                    },
                    |_, _, _, _| {},
                )
                .absolute()
                .size_full(),
            )
            .child(
                div()
                    .flex_none()
                    .min_w(px(0.0))
                    .min_h(px(0.0))
                    .when(horizontal, |this| this.w(displayed_size).h_full())
                    .when(!horizontal, |this| this.h(displayed_size).w_full())
                    .child(first),
            )
            .child(handle)
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .min_h(px(0.0))
                    .when(horizontal, |this| this.h_full())
                    .when(!horizontal, |this| this.w_full())
                    .child(second),
            )
            .on_drag_move::<SplitterDrag>(move |event, window, cx| {
                if event.drag(cx).source != state_for_drag.entity_id() {
                    return;
                }
                let (origin, position, total) = if horizontal {
                    (
                        event.bounds.origin.x,
                        event.event.position.x,
                        event.bounds.size.width,
                    )
                } else {
                    (
                        event.bounds.origin.y,
                        event.event.position.y,
                        event.bounds.size.height,
                    )
                };
                let available = (total - px(HANDLE_SIZE)).max(px(0.0));
                state_for_drag.update(cx, |state, _| state.available = Some(available));
                request_size(
                    &state_for_drag,
                    position - origin - px(HANDLE_SIZE / 2.0),
                    controlled_size,
                    min_size,
                    max_size,
                    callback_for_drag.as_ref(),
                    window,
                    cx,
                );
            });
        root
    }
}

fn clamp_size(
    requested: Pixels,
    min_size: Pixels,
    max_size: Option<Pixels>,
    available: Option<Pixels>,
) -> Pixels {
    let available_limit = available.map(|available| (available - px(MIN_OTHER_PANE)).max(px(0.0)));
    let upper = match (max_size, available_limit) {
        (Some(max_size), Some(available_limit)) => Some(max_size.min(available_limit)),
        (Some(max_size), None) => Some(max_size),
        (None, Some(available_limit)) => Some(available_limit),
        (None, None) => None,
    };
    upper.map_or(requested.max(min_size), |upper| {
        requested.max(min_size.min(upper)).min(upper)
    })
}

fn request_size(
    state: &Entity<SplitterState>,
    requested: Pixels,
    controlled_size: Option<Pixels>,
    min_size: Pixels,
    max_size: Option<Pixels>,
    on_change: Option<&Arc<dyn Fn(Pixels) + 'static>>,
    window: &mut Window,
    cx: &mut App,
) {
    let state_snapshot = state.read(cx);
    let current = controlled_size.unwrap_or(state_snapshot.size);
    let last_reported = state_snapshot.last_reported;
    let value = clamp_size(requested, min_size, max_size, state_snapshot.available);
    if value == current || last_reported == Some(value) {
        return;
    }
    state.update(cx, |state, cx| {
        state.last_reported = Some(value);
        if controlled_size.is_none() {
            state.size = value;
        }
        cx.notify();
    });
    if let Some(callback) = on_change {
        callback(value);
    }
    window.refresh();
}

impl IntoElement for Splitter {
    type Element = Component<Self>;

    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn splitter_defaults_and_limits_are_applied_to_builder_values() {
        let splitter = Splitter::new()
            .default_size(px(120.0))
            .min_size(px(90.0))
            .max_size(px(300.0))
            .orientation(SplitterOrientation::Vertical)
            .bordered();

        assert_eq!(splitter.default_size, px(120.0));
        assert_eq!(splitter.min_size, px(90.0));
        assert_eq!(splitter.max_size, Some(px(300.0)));
        assert_eq!(splitter.orientation, SplitterOrientation::Vertical);
        assert!(splitter.bordered);
    }
    #[test]
    fn controlled_and_available_bounds_are_clamped() {
        assert_eq!(
            clamp_size(px(20.0), px(80.0), Some(px(300.0)), None),
            px(80.0)
        );
        assert_eq!(
            clamp_size(px(420.0), px(80.0), Some(px(300.0)), None),
            px(300.0)
        );
        assert_eq!(
            clamp_size(px(420.0), px(80.0), None, Some(px(360.0))),
            px(280.0)
        );
    }
}
