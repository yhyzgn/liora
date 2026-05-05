use aura_core::{Config, push_portal};
use gpui::{
    prelude::*, px, App, Context, IntoElement, Render, Window,
    div, AnyElement, MouseButton, SharedString, Pixels,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPlacement {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

pub struct Drawer {
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DrawerView>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
}

pub struct DrawerView {
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl DrawerView {
    fn new(
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        placement: DrawerPlacement,
        width: Pixels,
        height: Pixels,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            title,
            content,
            placement,
            width,
            height,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DrawerView {
    #[track_caller]
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let placement = self.placement;
        let width = self.width;
        let height = self.height;

        let caller = std::panic::Location::caller();
        let id = format!("drawer-overlay-{}", caller);

        let mut container = div()
            .id(id)
            .absolute()
            .size_full()
            .bg(gpui::rgba(0x00000066))
            .on_mouse_down(MouseButton::Left, {
                let on_close = on_close.clone();
                move |_, window, cx| {
                    on_close(window, cx);
                }
            });

        let mut panel = div()
            .bg(theme.neutral.card)
            .shadow_xl()
            .on_mouse_down(MouseButton::Left, |_, _, _| {
                // Consume
            });

        match placement {
            DrawerPlacement::Left => {
                container = container.flex().flex_row().justify_start();
                panel = panel.h_full().w(width).border_r_1().border_color(theme.neutral.border);
            }
            DrawerPlacement::Right => {
                container = container.flex().flex_row().justify_end();
                panel = panel.h_full().w(width).border_l_1().border_color(theme.neutral.border);
            }
            DrawerPlacement::Top => {
                container = container.flex().flex_col().justify_start();
                panel = panel.w_full().h(height).border_b_1().border_color(theme.neutral.border);
            }
            DrawerPlacement::Bottom => {
                container = container.flex().flex_col().justify_end();
                panel = panel.w_full().h(height).border_t_1().border_color(theme.neutral.border);
            }
        }

        container.child(
            panel
                .child(
                    div().p_4().border_b_1().border_color(theme.neutral.border).flex().justify_between().items_center()
                        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
                        .child(
                            div().id("close-btn").cursor_pointer().child(Icon::new(IconName::X).size(px(16.0)).color(theme.neutral.icon))
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    on_close(window, cx);
                                })
                        )
                )
                .child(div().flex_1().p_4().child(content_fn(_window, cx)))
        )
    }
}

impl Drawer {
    pub fn new() -> Self {
        Self {
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Drawer Content").into_any_element()),
            placement: DrawerPlacement::Right,
            width: px(300.0),
            height: px(300.0),
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn placement(mut self, p: DrawerPlacement) -> Self {
        self.placement = p;
        self
    }

    pub fn width(mut self, w: impl Into<Pixels>) -> Self {
        self.width = w.into();
        self
    }

    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = h.into();
        self
    }

    pub fn content<F, E>(mut self, f: F) -> Self 
    where 
        F: Fn(&mut Window, &mut Context<DrawerView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    pub fn show(self, cx: &mut App) {
        let title = self.title;
        let content = self.content;
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        
        push_portal(move |_window, cx| {
            cx.new(|_| DrawerView::new(
                title,
                content,
                placement,
                width,
                height,
                |_window, _cx| {
                    aura_core::popper::clear_portals(_cx);
                }
            )).into_any_element()
        }, cx);
    }
}
