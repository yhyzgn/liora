use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, Context, IntoElement, Pixels, Render, ScrollHandle, Window, div, point, prelude::*,
    px,
};

pub struct Backtop {
    scroll_handle: ScrollHandle,
    visibility_height: Pixels,
    right: Pixels,
    bottom: Pixels,
    content: Option<Box<dyn Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static>>,
}

impl Backtop {
    pub fn new(scroll_handle: ScrollHandle) -> Self {
        Self {
            scroll_handle,
            visibility_height: px(200.0),
            right: px(40.0),
            bottom: px(40.0),
            content: None,
        }
    }

    pub fn visibility_height(mut self, h: impl Into<Pixels>) -> Self {
        self.visibility_height = h.into();
        self
    }

    pub fn right(mut self, r: impl Into<Pixels>) -> Self {
        self.right = r.into();
        self
    }

    pub fn bottom(mut self, b: impl Into<Pixels>) -> Self {
        self.bottom = b.into();
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut Context<Backtop>) -> AnyElement + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }
}

impl Render for Backtop {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let scroll_offset = self.scroll_handle.offset();
        let is_visible = -scroll_offset.y >= self.visibility_height;

        let scroll_handle = self.scroll_handle.clone();

        div().when(is_visible, |s| {
            s.child(
                div()
                    .id("backtop-btn")
                    .absolute()
                    .bottom(self.bottom)
                    .right(self.right)
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(40.0))
                    .h(px(40.0))
                    .rounded_full()
                    .bg(theme.neutral.card)
                    .border_1()
                    .border_color(theme.neutral.border)
                    .shadow_lg()
                    .hover(|s| s.bg(theme.neutral.hover))
                    .on_click(move |_, _, _| {
                        scroll_handle.set_offset(point(px(0.0), px(0.0)));
                    })
                    .child(match &self.content {
                        Some(content_fn) => (content_fn)(_window, cx),
                        None => Icon::new(IconName::ChevronUp)
                            .size(px(20.0))
                            .color(theme.primary.base)
                            .into_any_element(),
                    }),
            )
        })
    }
}
