use aura_core::{Config};
use gpui::{
    prelude::*, px, App, IntoElement, Window,
    div, SharedString, Component, RenderOnce,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;

pub struct Loading {
    text: Option<SharedString>,
    full_screen: bool,
}

impl Loading {
    pub fn new() -> Self {
        Self {
            text: None,
            full_screen: false,
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = Some(text.into());
        self
    }

    pub fn full_screen(mut self) -> Self {
        self.full_screen = true;
        self
    }
}

impl RenderOnce for Loading {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        
        let spinner = div().flex().flex_col().items_center().gap_2()
            .child(Icon::new(IconName::LoaderCircle).size(px(32.0)).color(theme.primary.base))
            .when_some(self.text, |s, t| s.child(div().text_sm().text_color(theme.primary.base).child(t)));

        if self.full_screen {
            div()
                .absolute()
                .size_full()
                .bg(gpui::rgba(0xFFFFFF99))
                .flex().items_center().justify_center()
                .child(spinner)
        } else {
            spinner
        }
    }
}

impl IntoElement for Loading {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
