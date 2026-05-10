use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, IntoElement, KeyBinding, MouseButton, Pixels, Render, SharedString,
    Window, actions, div, prelude::*, px,
};
use std::sync::Arc;

actions!(drawer, [DrawerClose]);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DrawerPlacement {
    #[default]
    Right,
    Left,
    Top,
    Bottom,
}

pub struct Drawer {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DrawerView>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

pub struct DrawerView {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    placement: DrawerPlacement,
    width: Pixels,
    height: Pixels,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl DrawerView {
    fn new(
        id: SharedString,
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        placement: DrawerPlacement,
        width: Pixels,
        height: Pixels,
        close_on_click_outside: bool,
        close_on_escape: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id,
            title,
            content,
            placement,
            width,
            height,
            close_on_click_outside,
            close_on_escape,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DrawerView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let mut container = div()
            .id(id.clone())
            .absolute()
            .size_full()
            .cursor_default()
            .bg(gpui::rgba(0x00000066))
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .when(close_on_click_outside, |s| {
                s.on_mouse_down(MouseButton::Left, {
                    let on_close = on_close.clone();
                    move |_, window, cx| {
                        on_close(window, cx);
                    }
                })
            })
            .when(close_on_escape, |s| {
                s.on_action(cx.listener({
                    let on_close = on_close.clone();
                    move |_, _action: &DrawerClose, window, cx| {
                        on_close(window, cx);
                    }
                }))
            });

        let mut panel = div()
            .bg(theme.neutral.card)
            .cursor_default()
            .shadow_xl()
            // CONSUME mouse down inside the panel so it doesn't trigger the overlay close
            .on_mouse_move(|_, _, cx| {
                cx.stop_propagation();
            })
            .on_mouse_down(MouseButton::Left, |_, _, cx| {
                cx.stop_propagation();
            });

        match placement {
            DrawerPlacement::Left => {
                container = container.flex().flex_row().justify_start();
                panel = panel
                    .h_full()
                    .w(width)
                    .border_r_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Right => {
                container = container.flex().flex_row().justify_end();
                panel = panel
                    .h_full()
                    .w(width)
                    .border_l_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Top => {
                container = container.flex().flex_col().justify_start();
                panel = panel
                    .w_full()
                    .h(height)
                    .border_b_1()
                    .border_color(theme.neutral.border);
            }
            DrawerPlacement::Bottom => {
                container = container.flex().flex_col().justify_end();
                panel = panel
                    .w_full()
                    .h(height)
                    .border_t_1()
                    .border_color(theme.neutral.border);
            }
        }

        container.child(
            panel
                .child(
                    div()
                        .p_4()
                        .border_b_1()
                        .border_color(theme.neutral.border)
                        .flex()
                        .justify_between()
                        .items_center()
                        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
                        .child(
                            div()
                                .id(format!("{id}-close-btn"))
                                .cursor_pointer()
                                .child(
                                    Icon::new(IconName::X)
                                        .size(px(16.0))
                                        .color(theme.neutral.icon),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    on_close(window, cx);
                                }),
                        ),
                )
                .child(div().flex_1().p_4().child(content_fn(_window, cx))),
        )
    }
}

impl Drawer {
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", DrawerClose, None)]);
    }

    pub fn new() -> Self {
        Self {
            id: aura_core::unique_id("drawer"),
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Drawer Content").into_any_element()),
            placement: DrawerPlacement::Right,
            width: px(300.0),
            height: px(300.0),
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
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

    pub fn width_lg(self) -> Self {
        self.width(px(480.0))
    }

    pub fn height(mut self, h: impl Into<Pixels>) -> Self {
        self.height = h.into();
        self
    }

    pub fn height_sm(self) -> Self {
        self.height(px(200.0))
    }

    pub fn height_lg(self) -> Self {
        self.height(px(360.0))
    }

    pub fn close_on_click_outside(mut self, c: bool) -> Self {
        self.close_on_click_outside = c;
        self
    }

    pub fn close_on_escape(mut self, c: bool) -> Self {
        self.close_on_escape = c;
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
        let id = self.id;
        let title = self.title;
        let content = self.content;
        let placement = self.placement;
        let width = self.width;
        let height = self.height;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let id_for_close = id.clone();
        let view = cx.new(|_cx| {
            DrawerView::new(
                id.clone(),
                title,
                content,
                placement,
                width,
                height,
                close_on_click_outside,
                close_on_escape,
                move |_window, _cx| {
                    aura_core::clear_drawer(&id_for_close, _cx);
                },
            )
        });

        aura_core::set_active_drawer(id, view.into(), cx);
    }

    pub fn close(cx: &mut App) {
        aura_core::clear_active_drawer(cx);
    }

    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        aura_core::clear_drawer(&id, cx);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drawer_size_helpers_set_demo_sizes() {
        assert_eq!(Drawer::new().width_lg().width, px(480.0));
        assert_eq!(Drawer::new().height_sm().height, px(200.0));
        assert_eq!(Drawer::new().height_lg().height, px(360.0));
    }
}
