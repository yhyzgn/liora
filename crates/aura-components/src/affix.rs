use gpui::{
    AnyElement, App, Bounds, Context, ElementId, GlobalElementId, InspectorElementId, IntoElement,
    LayoutId, Pixels, Render, Window, div, prelude::*, px,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AffixPosition {
    #[default]
    Top,
    Bottom,
}

pub struct Affix {
    offset: Pixels,
    position: AffixPosition,
    is_fixed: bool,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    content: Arc<dyn Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static>,
}

use std::sync::Arc;

impl Affix {
    pub fn new() -> Self {
        Self {
            offset: px(0.0),
            position: AffixPosition::Top,
            is_fixed: false,
            on_change: None,
            content: Arc::new(|_, _| div().into_any_element()),
        }
    }

    pub fn offset(mut self, offset: impl Into<Pixels>) -> Self {
        self.offset = offset.into();
        self
    }

    pub fn position(mut self, pos: AffixPosition) -> Self {
        self.position = pos;
        self
    }

    pub fn on_change(mut self, f: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Affix>) -> AnyElement + 'static,
    {
        self.content = Arc::new(f);
        self
    }
}

impl Render for Affix {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_fixed = self.is_fixed;
        let offset = self.offset;
        let content_fn = self.content.clone();
        let affix_handle = cx.entity().clone();

        div()
            .relative()
            .child(div().id("affix-placeholder").child(BoundsTracker {
                on_bounds_change: Box::new(move |bounds, window, cx| {
                    let (offset, position, current_fixed) = affix_handle
                        .update(cx, |this, _| (this.offset, this.position, this.is_fixed));

                    let should_be_fixed = match position {
                        AffixPosition::Top => bounds.top() <= offset,
                        AffixPosition::Bottom => {
                            let viewport_h = window.viewport_size().height;
                            bounds.bottom() >= viewport_h - offset
                        }
                    };

                    if should_be_fixed != current_fixed {
                        affix_handle.update(cx, |this, cx| {
                            this.is_fixed = should_be_fixed;
                            if let Some(ref on_change) = this.on_change {
                                (on_change)(should_be_fixed, window, cx);
                            }
                            cx.notify();
                        });
                    }
                }),
            }))
            .when(is_fixed, |s| {
                s.child(
                    div()
                        .absolute()
                        .top(offset)
                        .left_0()
                        .w_full()
                        .child(content_fn(_window, cx)),
                )
            })
            .when(!is_fixed, |s| s.child(content_fn(_window, cx)))
    }
}

struct BoundsTracker {
    on_bounds_change: Box<dyn Fn(Bounds<Pixels>, &mut Window, &mut App)>,
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
        (window.request_layout(gpui::Style::default(), [], cx), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _id2: Option<&InspectorElementId>,
        _bounds: Bounds<Pixels>,
        _rl: &mut (),
        _window: &mut Window,
        _cx: &mut App,
    ) -> () {
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
        (self.on_bounds_change)(bounds, window, cx);
    }
}
