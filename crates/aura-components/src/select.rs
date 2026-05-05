use aura_core::{Config, push_portal, clear_portals};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    prelude::*, px, App, Render, Window, Context, Focusable, FocusHandle,
    SharedString, MouseButton, ElementId, Entity
};

pub struct Select {
    options: Vec<SharedString>,
    selected_idx: Option<usize>,
    is_open: bool,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Select {
    pub fn new(options: Vec<impl Into<SharedString>>, selected_idx: Option<usize>, cx: &mut Context<Self>) -> Self {
        Self {
            options: options.into_iter().map(|o| o.into()).collect(),
            selected_idx,
            is_open: false,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        if self.is_open {
            window.focus(&self.focus_handle, cx);
        } else {
            clear_portals(cx);
        }
        cx.notify();
    }

    fn select_option(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_idx = Some(idx);
        self.is_open = false;
        clear_portals(cx);
        if let Some(ref cb) = self.on_change {
            cb(idx, window, cx);
        }
        cx.notify();
    }
}

impl Focusable for Select {
    fn focus_handle(&self, _cx: &App) -> FocusHandle { self.focus_handle.clone() }
}

impl Render for Select {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        
        let display_text = self.selected_idx
            .map(|i| self.options[i].clone())
            .unwrap_or_else(|| "Select...".into());

        let border_color = if focused || self.is_open { theme.primary.base } else { theme.neutral.border };

        let trigger = gpui::div()
            .flex().flex_row().items_center().justify_between()
            .w_full().h(px(34.0)).px(px(12.0)).rounded(px(theme.radius.md))
            .bg(theme.neutral.card).border_1().border_color(border_color)
            .child(gpui::div().text_size(px(theme.font_size.md)).text_color(theme.neutral.text_1).child(display_text))
            .child(Icon::new(if self.is_open { IconName::ChevronUp } else { IconName::ChevronDown }).size(px(16.0)).color(theme.neutral.icon))
            .cursor_pointer()
            .on_mouse_down(MouseButton::Left, cx.listener(|this, _, window, cx| {
                this.toggle_open(window, cx);
            }));

        if self.is_open {
            let options = self.options.clone();
            let selected_idx = self.selected_idx;
            let entity = cx.entity().clone();
            let theme = theme.clone();

            push_portal(
                gpui::div()
                    .absolute()
                    .top(px(40.0)) 
                    .left(px(0.0)) 
                    .w_full()
                    .max_h(px(200.0))
                    .bg(theme.neutral.card).rounded(px(theme.radius.md)).border_1().border_color(theme.neutral.border)
                    .shadow(vec![gpui::BoxShadow {
                        color: theme.neutral.border,
                        offset: gpui::point(px(0.0), px(4.0)),
                        blur_radius: px(12.0),
                        spread_radius: px(0.0),
                    }])
                    .children(options.into_iter().enumerate().map(|(idx, label)| {
                        let is_selected = Some(idx) == selected_idx;
                        let entity = entity.clone();
                        let theme = theme.clone();
                        
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
                    })),
                cx
            );
        }

        trigger
    }
}
