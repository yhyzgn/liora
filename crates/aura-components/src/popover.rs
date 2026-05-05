use aura_core::{Config, Placement, set_active_popover, clear_active_popover};
use gpui::{
    prelude::*, px, App, Component, Context, IntoElement, Render, Window,
    Bounds, Pixels, div, AnyElement, MouseButton, RenderOnce, ElementId, LayoutId, GlobalElementId, InspectorElementId,
    SharedString,
};
use std::sync::Arc;
use std::rc::Rc;
use std::cell::Cell;

pub struct Popover {
    trigger: AnyElement,
    content: Arc<dyn Fn(&mut Window, &mut Context<PopoverView>) -> AnyElement + 'static>,
    placement: Placement,
    offset: Pixels,
}

pub struct PopoverView {
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    anchor_bounds: Bounds<Pixels>,
    placement: Placement,
    offset: Pixels,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl PopoverView {
    pub fn new(
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        anchor_bounds: Bounds<Pixels>,
        placement: Placement,
        offset: Pixels,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            content,
            anchor_bounds,
            placement,
            offset,
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
        
        let content = (self.content)(_window, cx);
        let viewport_size = _window.viewport_size();
        let viewport = Bounds {
            origin: gpui::Point::default(),
            size: viewport_size,
        };

        // Use Popper just for flip logic
        let popper = aura_core::Popper {
            anchor_bounds,
            placement,
            offset,
        };
        // Reference size for flipping and clamping
        let reference_size = gpui::Size { width: px(400.0), height: px(300.0) };
        let (_pos, final_placement) = popper.calculate_position_with_flip(reference_size, viewport);

        let mut pivot_container = div().absolute().flex();

        match final_placement {
            Placement::Top | Placement::Bottom | Placement::TopStart | Placement::BottomStart | Placement::TopEnd | Placement::BottomEnd => {
                let container_width = px(2000.0);
                let ideal_center_x = anchor_bounds.left() + anchor_bounds.size.width / 2.0;
                
                // Clamping for horizontal centering
                let half_content_width = reference_size.width / 2.0;
                let clamped_center_x = ideal_center_x
                    .max(half_content_width)
                    .min(viewport_size.width - half_content_width);

                pivot_container = pivot_container
                    .w(container_width)
                    .h(px(0.0))
                    .left(clamped_center_x - container_width / 2.0);

                if final_placement == Placement::Top || final_placement == Placement::TopStart || final_placement == Placement::TopEnd {
                    let bottom_offset = viewport_size.height - anchor_bounds.top() + offset;
                    pivot_container = pivot_container.bottom(bottom_offset).flex_col_reverse();
                } else {
                    pivot_container = pivot_container.top(anchor_bounds.bottom() + offset).flex_col();
                }

                match final_placement {
                    Placement::Top | Placement::Bottom => {
                        pivot_container = pivot_container.items_center();
                    }
                    Placement::TopStart | Placement::BottomStart => {
                        pivot_container = pivot_container.items_start().pl(container_width / 2.0 - (clamped_center_x - anchor_bounds.left()));
                    }
                    Placement::TopEnd | Placement::BottomEnd => {
                        pivot_container = pivot_container.items_end().pr(container_width / 2.0 - (anchor_bounds.right() - clamped_center_x));
                    }
                    _ => unreachable!()
                }
            }
            Placement::Left | Placement::Right | Placement::LeftStart | Placement::RightStart | Placement::LeftEnd | Placement::RightEnd => {
                let container_height = px(2000.0);
                let ideal_center_y = anchor_bounds.top() + anchor_bounds.size.height / 2.0;

                // Clamping for vertical centering
                let half_content_height = reference_size.height / 2.0;
                let clamped_center_y = ideal_center_y
                    .max(half_content_height)
                    .min(viewport_size.height - half_content_height);

                pivot_container = pivot_container
                    .h(container_height)
                    .w(px(0.0))
                    .top(clamped_center_y - container_height / 2.0);

                if final_placement == Placement::Left || final_placement == Placement::LeftStart || final_placement == Placement::LeftEnd {
                    let dist_from_right = viewport_size.width - anchor_bounds.left() + offset;
                    pivot_container = pivot_container.right(dist_from_right).flex_row_reverse();
                } else {
                    pivot_container = pivot_container.left(anchor_bounds.right() + offset).flex_row();
                }

                match final_placement {
                    Placement::Left | Placement::Right => {
                        pivot_container = pivot_container.items_center();
                    }
                    Placement::LeftStart | Placement::RightStart => {
                        pivot_container = pivot_container.items_start().pt(container_height / 2.0 - (clamped_center_y - anchor_bounds.top()));
                    }
                    Placement::LeftEnd | Placement::RightEnd => {
                        pivot_container = pivot_container.items_end().pb(container_height / 2.0 - (anchor_bounds.bottom() - clamped_center_y));
                    }
                    _ => unreachable!()
                }
            }
        }

        div()
            .absolute()
            .size_full()
            .on_mouse_down(MouseButton::Left, cx.listener(move |_, _, window, cx| {
                on_close(window, cx);
            }))
            .child(
                pivot_container
                    .child(
                        div()
                            .on_mouse_down(MouseButton::Left, |_, _, _| {}) // Consume click
                            .bg(theme.neutral.card)
                            .border_1().border_color(theme.neutral.border)
                            .rounded(px(theme.radius.md))
                            .shadow_lg()
                            .child(content)
                    )
            )
    }
}

impl Popover {
    #[track_caller]
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            content: Arc::new(|_, _| div().child("Popover Content").into_any_element()),
            placement: Placement::Bottom,
            offset: px(8.0),
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
}

impl RenderOnce for Popover {
    #[track_caller]
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let placement = self.placement;
        let offset = self.offset;
        let content = self.content.clone();
        
        let bounds_cell = Rc::new(Cell::new(None));
        let bounds_cell_clone = bounds_cell.clone();

        // Generate a stable ID based on caller location
        let caller = std::panic::Location::caller();
        let id = ElementId::from(SharedString::from(format!("popover-trigger-{}", caller)));

        div()
            .id(id)
            .child(
                BoundsTracker {
                    trigger: self.trigger,
                    bounds: bounds_cell,
                }
            )
            .on_click(move |_event, _window, cx| {
                if let Some(anchor_bounds) = bounds_cell_clone.get() {
                    let content = content.clone();
                    let view = cx.new(|_cx| {
                        PopoverView::new(
                            content,
                            anchor_bounds,
                            placement,
                            offset,
                            |_window, _cx| {
                                clear_active_popover(_cx);
                            }
                        )
                    });
                    set_active_popover(view.into(), cx);
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
    fn into_element(self) -> Self::Element { self }
}

impl gpui::Element for BoundsTracker {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> { None }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

    fn request_layout(&mut self, _id: Option<&GlobalElementId>, _id2: Option<&InspectorElementId>, window: &mut Window, cx: &mut App) -> (LayoutId, ()) {
        (self.trigger.request_layout(window, cx), ())
    }

    fn prepaint(&mut self, _id: Option<&GlobalElementId>, _id2: Option<&InspectorElementId>, _bounds: Bounds<Pixels>, _rl: &mut (), window: &mut Window, cx: &mut App) -> () {
        self.trigger.prepaint(window, cx);
    }

    fn paint(&mut self, _id: Option<&GlobalElementId>, _id2: Option<&InspectorElementId>, bounds: Bounds<Pixels>, _rl: &mut (), _ps: &mut (), window: &mut Window, cx: &mut App) {
        self.bounds.set(Some(bounds));
        self.trigger.paint(window, cx);
    }
}
