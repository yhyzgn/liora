use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    App, Bounds, Context, ElementId, Entity, FocusHandle, Focusable, Hsla, MouseButton, Pixels,
    Render, SharedString, Window, actions, prelude::*,
};
use liora_core::{Config, push_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

actions!(select, [SelectClose]);

pub struct Select {
    options: Vec<SharedString>,
    selected_idx: Option<usize>,
    is_open: bool,
    focus_handle: FocusHandle,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    border_none: bool,
    radius_none: bool,
    radius_left_none: bool,
    radius_right_none: bool,
    width: Option<Pixels>,
    text_size: Option<Pixels>,
    text_color: Option<Hsla>,
    padding_x: Option<Pixels>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl Select {
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected_idx: Option<usize>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            options: options.into_iter().map(|o| o.into()).collect(),
            selected_idx,
            is_open: false,
            focus_handle: cx.focus_handle(),
            last_bounds: None,
            on_change: None,
            border_none: false,
            radius_none: false,
            radius_left_none: false,
            radius_right_none: false,
            width: None,
            text_size: None,
            text_color: None,
            padding_x: None,
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    pub fn borderless(mut self) -> Self {
        self.border_none = true;
        self
    }
    pub fn radius_none(mut self) -> Self {
        self.radius_none = true;
        self
    }
    pub fn radius_left_none(mut self) -> Self {
        self.radius_left_none = true;
        self
    }
    pub fn radius_right_none(mut self) -> Self {
        self.radius_right_none = true;
        self
    }
    pub fn width(mut self, w: impl Into<Pixels>) -> Self {
        self.width = Some(w.into());
        self
    }

    pub fn width_xs(self) -> Self {
        self.width(gpui::px(90.0))
    }

    pub fn text_size(mut self, s: impl Into<Pixels>) -> Self {
        self.text_size = Some(s.into());
        self
    }

    pub fn text_sm(self) -> Self {
        self.text_size(gpui::px(14.0))
    }
    pub fn text_color(mut self, c: Hsla) -> Self {
        self.text_color = Some(c);
        self
    }
    pub fn padding_x(mut self, p: impl Into<Pixels>) -> Self {
        self.padding_x = Some(p.into());
        self
    }

    pub fn padding_x_sm(self) -> Self {
        self.padding_x(gpui::px(8.0))
    }

    pub fn set_borderless(&mut self, b: bool, cx: &mut Context<Self>) {
        if self.border_none == b {
            return;
        }
        self.border_none = b;
        cx.notify();
    }

    pub fn set_radius_none(&mut self, r: bool, cx: &mut Context<Self>) {
        if self.radius_none == r {
            return;
        }
        self.radius_none = r;
        cx.notify();
    }

    pub fn set_radius_left_none(&mut self, r: bool, cx: &mut Context<Self>) {
        if self.radius_left_none == r {
            return;
        }
        self.radius_left_none = r;
        cx.notify();
    }

    pub fn set_radius_right_none(&mut self, r: bool, cx: &mut Context<Self>) {
        if self.radius_right_none == r {
            return;
        }
        self.radius_right_none = r;
        cx.notify();
    }

    pub fn set_width(&mut self, w: impl Into<Pixels>, cx: &mut Context<Self>) {
        let w = w.into();
        if self.width == Some(w) {
            return;
        }
        self.width = Some(w);
        cx.notify();
    }

    pub fn set_text_size(&mut self, s: impl Into<Pixels>, cx: &mut Context<Self>) {
        let s = s.into();
        if self.text_size == Some(s) {
            return;
        }
        self.text_size = Some(s);
        cx.notify();
    }

    pub fn set_text_color(&mut self, c: Hsla, cx: &mut Context<Self>) {
        if self.text_color == Some(c) {
            return;
        }
        self.text_color = Some(c);
        cx.notify();
    }

    pub fn set_padding_x(&mut self, p: impl Into<Pixels>, cx: &mut Context<Self>) {
        let p = p.into();
        if self.padding_x == Some(p) {
            return;
        }
        self.padding_x = Some(p);
        cx.notify();
    }

    pub fn set_options(&mut self, options: Vec<SharedString>, cx: &mut Context<Self>) {
        if self.options == options {
            return;
        }
        self.options = options;
        if let Some(idx) = self.selected_idx
            && idx >= self.options.len()
        {
            self.selected_idx = None;
        }
        cx.notify();
    }

    pub fn set_selected_idx(&mut self, idx: Option<usize>, cx: &mut Context<Self>) {
        if self.selected_idx == idx {
            return;
        }
        self.selected_idx = idx;
        cx.notify();
    }

    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([gpui::KeyBinding::new("escape", SelectClose, None)]);
    }

    fn close_on_escape_action(&mut self, _: &SelectClose, _: &mut Window, cx: &mut Context<Self>) {
        if self.close_on_escape && self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub fn set_on_change(&mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Box::new(cb));
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_idx
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        if self.is_open {
            window.focus(&self.focus_handle);
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
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct BoundsCapturer {
    select: Entity<Select>,
}

impl IntoElement for BoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
}

impl Element for BoundsCapturer {
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
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, ()) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) -> () {
        self.select.update(cx, |this, _| {
            this.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
}

impl Render for Select {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let theme = config.theme.clone();
        let focused = self.focus_handle.is_focused(_window);

        let display_text = self
            .selected_idx
            .map(|i| self.options[i].clone())
            .unwrap_or_else(|| "Select...".into());

        let border_color = if focused || self.is_open {
            theme.primary.base
        } else {
            theme.neutral.border
        };
        let text_size = self.text_size.unwrap_or(gpui::px(theme.font_size.md));
        let text_color = self.text_color.unwrap_or(theme.neutral.text_1);
        let h_px = self.padding_x.unwrap_or(gpui::px(12.0));

        let trigger_content = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(gpui::px(34.0))
            .px(h_px)
            .child(
                gpui::div()
                    .text_size(text_size)
                    .text_color(text_color)
                    .child(display_text),
            )
            .child(
                Icon::new(if self.is_open {
                    IconName::ChevronUp
                } else {
                    IconName::ChevronDown
                })
                .size(gpui::px(14.0))
                .color(theme.neutral.icon),
            );

        if self.is_open {
            let options = self.options.clone();
            let selected_idx = self.selected_idx;
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            let trigger_bounds = self.last_bounds;

            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = if let Some(b) = trigger_bounds {
                        (b.bottom() + gpui::px(4.0), b.left(), b.size.width)
                    } else {
                        (gpui::px(100.0), gpui::px(100.0), gpui::px(200.0))
                    };

                    let entity = entity.clone();
                    let theme = theme_portal.clone();

                    let panel = gpui::div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width)
                        .max_h(gpui::px(200.0))
                        .bg(theme.neutral.card)
                        .rounded(gpui::px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow(vec![gpui::BoxShadow {
                            color: theme.neutral.border,
                            offset: gpui::point(gpui::px(0.0), gpui::px(4.0)),
                            blur_radius: gpui::px(12.0),
                            spread_radius: gpui::px(0.0),
                        }])
                        .children(options.iter().enumerate().map(|(idx, label)| {
                            let is_selected = Some(idx) == selected_idx;
                            let entity = entity.clone();
                            let theme = theme.clone();
                            let label = label.clone();

                            gpui::div()
                                .id(element_id(format!("select-option-{}", idx)))
                                .px(gpui::px(12.0))
                                .py(gpui::px(8.0))
                                .cursor_pointer()
                                .bg(if is_selected {
                                    theme.primary.base.opacity(0.1)
                                } else {
                                    theme.neutral.card
                                })
                                .hover(move |s| {
                                    s.cursor_pointer().bg(if is_selected {
                                        theme.neutral.text_3.opacity(0.16)
                                    } else {
                                        theme.neutral.hover
                                    })
                                })
                                .child(
                                    gpui::div()
                                        .text_size(gpui::px(theme.font_size.md))
                                        .text_color(if is_selected {
                                            theme.primary.base
                                        } else {
                                            theme.neutral.text_1
                                        })
                                        .child(label),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.select_option(idx, window, cx);
                                    });
                                })
                        }));

                    pop_in(
                        element_id(format!("liora-select-panel-motion-{}", entity.entity_id())),
                        panel,
                    )
                    .into_any_element()
                },
                cx,
            );
        }

        let mut el = gpui::div()
            .relative()
            .when_some(self.width, |s, w| s.w(w))
            .when(self.width.is_none(), |s| s.w_full())
            .bg(theme.neutral.card)
            .when(!self.border_none, |s| {
                s.border_1().border_color(border_color)
            })
            .cursor_pointer()
            .hover(|s| {
                let s = s.cursor_pointer();
                if self.border_none {
                    s
                } else {
                    s.border_color(theme.primary.base)
                }
            });

        if !self.radius_none {
            if self.radius_left_none {
                el = el.rounded_r(gpui::px(theme.radius.md));
            } else if self.radius_right_none {
                el = el.rounded_l(gpui::px(theme.radius.md));
            } else {
                el = el.rounded(gpui::px(theme.radius.md));
            }
        }

        let close_on_click_outside = self.close_on_click_outside;

        el.child(trigger_content)
            .child(
                gpui::div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        select: cx.entity().clone(),
                    }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle_open(window, cx);
                }),
            )
            .when(close_on_click_outside, |s| {
                s.on_mouse_down_out(cx.listener(|this, _, _, cx| {
                    this.is_open = false;
                    cx.notify();
                }))
            })
            .on_action(cx.listener(Self::close_on_escape_action))
    }
}
