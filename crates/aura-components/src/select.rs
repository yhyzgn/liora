use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    prelude::*, px, App, Render, Window, Context, Focusable, FocusHandle,
    SharedString, MouseButton, ElementId, Bounds, Pixels, Entity
};

pub struct Select {
    options: Vec<SharedString>,
    selected_idx: Option<usize>,
    is_open: bool,
    focus_handle: FocusHandle,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    border_none: bool,
    radius_none: bool,
}

impl Select {
    pub fn new(options: Vec<impl Into<SharedString>>, selected_idx: Option<usize>, cx: &mut Context<Self>) -> Self {
        Self {
            options: options.into_iter().map(|o| o.into()).collect(),
            selected_idx,
            is_open: false,
            focus_handle: cx.focus_handle(),
            last_bounds: None,
            on_change: None,
            border_none: false,
            radius_none: false,
        }
    }

    pub fn borderless(mut self) -> Self { self.border_none = true; self }
    pub fn radius_none(mut self) -> Self { self.radius_none = true; self }

    pub fn set_borderless(&mut self, b: bool, cx: &mut Context<Self>) {
        self.border_none = b;
        cx.notify();
    }

    pub fn set_radius_none(&mut self, r: bool, cx: &mut Context<Self>) {
        self.radius_none = r;
        cx.notify();
    }

    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        if self.is_open {
            window.focus(&self.focus_handle, cx);
        }
        cx.notify();
    }

    fn select_option(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_idx = Some(idx);
        self.is_open = false;
        if let Some(ref cb) = self.on_change {
            cb(idx, window, cx);
        }
        cx.notify();
    }
}

impl Focusable for Select {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

struct BoundsCapturer {
    select: Entity<Select>,
}

impl IntoElement for BoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element { self }
}

impl Element for BoundsCapturer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> { None }
    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> { None }

    fn request_layout(&mut self, _: Option<&gpui::GlobalElementId>, _: Option<&gpui::InspectorElementId>, window: &mut Window, cx: &mut App) -> (gpui::LayoutId, ()) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(&mut self, _: Option<&gpui::GlobalElementId>, _: Option<&gpui::InspectorElementId>, bounds: Bounds<Pixels>, _: &mut (), _window: &mut Window, cx: &mut App) -> () {
        self.select.update(cx, |this, _| {
            this.last_bounds = Some(bounds);
        });
    }

    fn paint(&mut self, _: Option<&gpui::GlobalElementId>, _: Option<&gpui::InspectorElementId>, _: Bounds<Pixels>, _: &mut (), _: &mut (), _window: &mut Window, _: &mut App) {}
}

impl Render for Select {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let theme = config.theme.clone();
        let focused = self.focus_handle.is_focused(_window);
        
        let display_text = self.selected_idx
            .map(|i| self.options[i].clone())
            .unwrap_or_else(|| "Select...".into());

        let border_color = if focused || self.is_open { theme.primary.base } else { theme.neutral.border };

        let trigger_content = gpui::div()
            .flex().flex_row().items_center().justify_between()
            .w_full().h(px(34.0)).px(px(12.0))
            .child(gpui::div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_1).child(display_text))
            .child(Icon::new(if self.is_open { IconName::ChevronUp } else { IconName::ChevronDown }).size(px(16.0)).color(theme.neutral.icon));

        if self.is_open {
            let options = self.options.clone();
            let selected_idx = self.selected_idx;
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            let trigger_bounds = self.last_bounds;

            push_portal(move |_window, _cx| {
                let (top, left, width) = if let Some(b) = trigger_bounds {
                    (b.bottom() + px(4.0), b.left(), b.size.width)
                } else {
                    (px(100.0), px(100.0), px(200.0))
                };

                let entity = entity.clone();
                let theme = theme_portal.clone();

                gpui::div()
                    .absolute()
                    .top(top)
                    .left(left)
                    .w(width)
                    .max_h(px(200.0))
                    .bg(theme.neutral.card).rounded(px(theme.radius.md)).border_1().border_color(theme.neutral.border)
                    .shadow(vec![gpui::BoxShadow {
                        color: theme.neutral.border,
                        offset: gpui::point(px(0.0), px(4.0)),
                        blur_radius: px(12.0),
                        spread_radius: px(0.0),
                    }])
                    .children(options.iter().enumerate().map(|(idx, label)| {
                        let is_selected = Some(idx) == selected_idx;
                        let entity = entity.clone();
                        let theme = theme.clone();
                        let label = label.clone();
                        
                        gpui::div()
                            .px(px(12.0)).py(px(8.0)).cursor_pointer()
                            .bg(if is_selected { theme.primary.base.opacity(0.1) } else { theme.neutral.card })
                            .hover(|s| s.bg(theme.neutral.hover))
                            .child(gpui::div()
                                .text_size(px(theme.font_size.md))
                                .text_color(if is_selected { theme.primary.base } else { theme.neutral.text_1 })
                                .child(label))
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                entity.update(cx, |this, cx| {
                                    this.select_option(idx, window, cx);
                                });
                            })
                    })).into_any_element()
            }, cx);
        }

        gpui::div()
            .relative()
            .w_full()
            .when(!self.radius_none, |s| s.rounded(px(theme.radius.md)))
            .bg(theme.neutral.card)
            .when(!self.border_none, |s| s.border_1().border_color(border_color))
            .cursor_pointer()
            .child(trigger_content)
            .child(
                gpui::div()
                    .absolute()
                    .top_0().left_0().size_full()
                    .child(BoundsCapturer { select: cx.entity().clone() })
            )
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                this.toggle_open(window, cx);
            }))
    }
}
