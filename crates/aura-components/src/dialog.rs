use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, IntoElement, KeyBinding, MouseButton, Render, SharedString, Window,
    actions, div, prelude::*, px,
};
use std::sync::Arc;

actions!(dialog, [DialogClose]);

pub struct Dialog {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<DialogView>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

pub struct DialogView {
    id: SharedString,
    title: SharedString,
    content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    on_close: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

impl DialogView {
    fn new(
        id: SharedString,
        title: SharedString,
        content: Arc<dyn Fn(&mut Window, &mut Context<Self>) -> AnyElement + 'static>,
        close_on_click_outside: bool,
        close_on_escape: bool,
        on_close: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id,
            title,
            content,
            close_on_click_outside,
            close_on_escape,
            on_close: Arc::new(on_close),
        }
    }
}

impl Render for DialogView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = self.id.clone();
        let title = self.title.clone();
        let content_fn = self.content.clone();
        let on_close = self.on_close.clone();
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        div()
            .id(id.clone())
            .absolute()
            .size_full()
            .bg(gpui::rgba(0x00000066))
            .flex()
            .items_center()
            .justify_center()
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
                    move |_, _action: &DialogClose, window, cx| {
                        on_close(window, cx);
                    }
                }))
            })
            .child(
                div()
                    .w(px(400.0))
                    .bg(theme.neutral.card)
                    .rounded(px(theme.radius.md))
                    .shadow_xl()
                    .on_mouse_move(|_, _, cx| {
                        cx.stop_propagation();
                    })
                    .on_mouse_down(MouseButton::Left, |_, _, cx| {
                        cx.stop_propagation();
                    }) // Consume click so it doesn't trigger the background
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
                    .child(div().p_4().child(content_fn(_window, cx))),
            )
    }
}

impl Dialog {
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([KeyBinding::new("escape", DialogClose, None)]);
    }

    #[track_caller]
    pub fn new() -> Self {
        let caller = std::panic::Location::caller();
        Self {
            id: format!("dialog-{}", caller).into(),
            title: SharedString::default(),
            content: Arc::new(|_, _| div().child("Dialog Content").into_any_element()),
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
        F: Fn(&mut Window, &mut Context<DialogView>) -> E + 'static,
        E: IntoElement,
    {
        self.content = Arc::new(move |window, cx| f(window, cx).into_any_element());
        self
    }

    pub fn show(self, cx: &mut App) {
        let id = self.id;
        let title = self.title;
        let content = self.content;
        let close_on_click_outside = self.close_on_click_outside;
        let close_on_escape = self.close_on_escape;

        let id_for_close = id.clone();
        let view = cx.new(|_cx| {
            DialogView::new(
                id.clone(),
                title,
                content,
                close_on_click_outside,
                close_on_escape,
                move |_window, _cx| {
                    aura_core::clear_modal(&id_for_close, _cx);
                },
            )
        });

        aura_core::set_active_modal(id, view.into(), cx);
    }

    pub fn close(cx: &mut App) {
        aura_core::clear_active_modal(cx);
    }

    pub fn close_id(id: impl Into<SharedString>, cx: &mut App) {
        let id = id.into();
        aura_core::clear_modal(&id, cx);
    }
}
