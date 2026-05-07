use aura_core::{Config, Placement, clear_popover, is_popover_active, set_active_popover};
use gpui::{
    AnyElement, App, Bounds, Component, Context, ElementId, GlobalElementId, InspectorElementId,
    IntoElement, LayoutId, MouseButton, Pixels, Render, RenderOnce, SharedString, Window, div,
    point, prelude::*, px,
};
use std::cell::Cell;
use std::rc::Rc;
use std::sync::Arc;

pub struct Popover {
    trigger: AnyElement,
    content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement + 'static>,
    placement: Placement,
    offset: Pixels,
    close_on_click_outside: bool,
    trigger_id: ElementId,
}

pub struct PopoverView {
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
    close_on_click_outside: bool,
    id: SharedString,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl PopoverView {
    pub fn new(
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        anchor_bounds: Bounds<Pixels>,
        placement: Placement,
        offset: Pixels,
        close_on_click_outside: bool,
        id: SharedString,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            content,
            anchor_bounds,
            placement,
            offset,
            close_on_click_outside,
            id,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for PopoverView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let anchor_bounds = self.anchor_bounds;
        let placement = self.placement;
        let offset = self.offset;
        let on_close = self.on_close.clone();
        let close_on_click_outside = self.close_on_click_outside;
        let id = self.id.clone();

        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let popover_anchor = popover_anchor_point(anchor_bounds, placement, offset);
        let popover_anchor_corner = popover_anchor_corner(placement);
        let viewport_margin = px(4.0);
        let max_w = (viewport_size.width - viewport_margin * 2.0).max(px(0.0));

        div()
            .id(id)
            .absolute()
            .size_full()
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .when(close_on_click_outside, |s| {
                s.on_mouse_down(
                    MouseButton::Left,
                    cx.listener(move |_, _, window, cx| {
                        on_close(window, cx);
                    }),
                )
            })
            .child(
                gpui::anchored()
                    .position(popover_anchor)
                    .anchor(popover_anchor_corner)
                    .snap_to_window_with_margin(viewport_margin)
                    .child(
                        div()
                            .flex_shrink_0() // Ensure content is not squeezed by flex layout
                            .max_w(max_w)
                            .on_mouse_move(|_, _, cx| {
                                cx.stop_propagation();
                            })
                            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                                cx.stop_propagation();
                            }) // Consume click so it doesn't trigger the background
                            .bg(theme.neutral.card)
                            .border_1()
                            .border_color(theme.neutral.border)
                            .rounded(px(theme.radius.md))
                            .shadow_lg()
                            .child(content),
                    ),
            )
    }
}

fn popover_anchor_point(
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
) -> gpui::Point<Pixels> {
    match placement {
        Placement::Top => point(
            anchor_bounds.left() + anchor_bounds.size.width / 2.0,
            anchor_bounds.top() - offset,
        ),
        Placement::TopStart => point(anchor_bounds.left(), anchor_bounds.top() - offset),
        Placement::TopEnd => point(anchor_bounds.right(), anchor_bounds.top() - offset),
        Placement::Bottom => point(
            anchor_bounds.left() + anchor_bounds.size.width / 2.0,
            anchor_bounds.bottom() + offset,
        ),
        Placement::BottomStart => point(anchor_bounds.left(), anchor_bounds.bottom() + offset),
        Placement::BottomEnd => point(anchor_bounds.right(), anchor_bounds.bottom() + offset),
        Placement::Left => point(
            anchor_bounds.left() - offset,
            anchor_bounds.top() + anchor_bounds.size.height / 2.0,
        ),
        Placement::LeftStart => point(anchor_bounds.left() - offset, anchor_bounds.top()),
        Placement::LeftEnd => point(anchor_bounds.left() - offset, anchor_bounds.bottom()),
        Placement::Right => point(
            anchor_bounds.right() + offset,
            anchor_bounds.top() + anchor_bounds.size.height / 2.0,
        ),
        Placement::RightStart => point(anchor_bounds.right() + offset, anchor_bounds.top()),
        Placement::RightEnd => point(anchor_bounds.right() + offset, anchor_bounds.bottom()),
    }
}

fn popover_anchor_corner(placement: Placement) -> gpui::Anchor {
    match placement {
        Placement::Top => gpui::Anchor::BottomCenter,
        Placement::TopStart => gpui::Anchor::BottomLeft,
        Placement::TopEnd => gpui::Anchor::BottomRight,
        Placement::Bottom => gpui::Anchor::TopCenter,
        Placement::BottomStart => gpui::Anchor::TopLeft,
        Placement::BottomEnd => gpui::Anchor::TopRight,
        Placement::Left => gpui::Anchor::RightCenter,
        Placement::LeftStart => gpui::Anchor::TopRight,
        Placement::LeftEnd => gpui::Anchor::BottomRight,
        Placement::Right => gpui::Anchor::LeftCenter,
        Placement::RightStart => gpui::Anchor::TopLeft,
        Placement::RightEnd => gpui::Anchor::BottomLeft,
    }
}

impl Popover {
    #[track_caller]
    pub fn new(trigger: impl IntoElement) -> Self {
        let caller = std::panic::Location::caller();
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Popover Content").into_any_element()),
            placement: Placement::Bottom,
            offset: px(8.0),
            close_on_click_outside: true,
            trigger_id: ElementId::from(SharedString::from(format!("popover-trigger-{}", caller))),
        }
    }

    pub fn content<F, E>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<PopoverView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }

    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.trigger_id = ElementId::from(id.into());
        self
    }
}

impl RenderOnce for Popover {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let close_on_click_outside = self.close_on_click_outside;
        let content = self.content.clone();
        let trigger_id = self.trigger_id;
        let popover_id = match &trigger_id {
            ElementId::Name(name) => name.clone(),
            _ => SharedString::from(format!("popover-{:?}", trigger_id)),
        };

        let bounds_cell = Rc::new(Cell::new(None));
        let bounds_cell_clone = bounds_cell.clone();

        div()
            .id(trigger_id)
            .child(BoundsTracker {
                trigger: self.trigger,
                bounds: bounds_cell,
            })
            .on_click(move |_event, _window, cx| {
                if is_popover_active(&popover_id, cx) {
                    clear_popover(&popover_id, cx);
                    return;
                }

                if let Some(anchor_bounds) = bounds_cell_clone.get() {
                    let content = content.clone();
                    let popover_id_for_close = popover_id.clone();
                    let popover_id_for_view = popover_id.clone();
                    let view = cx.new(|_cx| {
                        PopoverView::new(
                            content,
                            anchor_bounds,
                            placement,
                            offset,
                            close_on_click_outside,
                            popover_id_for_view,
                            move |_window, _cx| {
                                clear_popover(&popover_id_for_close, _cx);
                            },
                        )
                    });
                    set_active_popover(popover_id.clone(), view.into(), cx);
                }
            })
    }
}

impl IntoElement for Popover {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

struct BoundsTracker {
    trigger: AnyElement,
    bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl IntoElement for BoundsTracker {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl gpui::Element for BoundsTracker {
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
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        (self.trigger.request_layout(window, cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.trigger.prepaint(window, cx);
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _rl: &mut (),
        _ps: &mut (),
        window: &mut Window,
        cx: &mut App,
    ) {
        self.bounds.set(Some(bounds));
        self.trigger.paint(window, cx);
    }
}
