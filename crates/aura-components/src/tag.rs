use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagType {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagSize {
    Small,
    #[default]
    Default,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TagEffect {
    Dark,
    #[default]
    Light,
    Plain,
}

pub struct Tag {
    label: SharedString,
    tag_type: TagType,
    size: TagSize,
    effect: TagEffect,
    closable: bool,
    round: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Tag {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            tag_type: TagType::Info,
            size: TagSize::Default,
            effect: TagEffect::Light,
            closable: false,
            round: false,
            on_close: None,
        }
    }

    pub fn tag_type(mut self, t: TagType) -> Self {
        self.tag_type = t;
        self
    }

    pub fn success(mut self) -> Self {
        self.tag_type = TagType::Success;
        self
    }

    pub fn warning(mut self) -> Self {
        self.tag_type = TagType::Warning;
        self
    }

    pub fn danger(mut self) -> Self {
        self.tag_type = TagType::Danger;
        self
    }

    pub fn info(mut self) -> Self {
        self.tag_type = TagType::Info;
        self
    }

    pub fn size(mut self, s: TagSize) -> Self {
        self.size = s;
        self
    }

    pub fn small(mut self) -> Self {
        self.size = TagSize::Small;
        self
    }

    pub fn large(mut self) -> Self {
        self.size = TagSize::Large;
        self
    }

    pub fn effect(mut self, e: TagEffect) -> Self {
        self.effect = e;
        self
    }

    pub fn dark(mut self) -> Self {
        self.effect = TagEffect::Dark;
        self
    }

    pub fn plain(mut self) -> Self {
        self.effect = TagEffect::Plain;
        self
    }

    pub fn closable(mut self, c: bool) -> Self {
        self.closable = c;
        self
    }

    pub fn round(mut self, r: bool) -> Self {
        self.round = r;
        self
    }

    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Tag {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let on_close = self.on_close;

        let color = match self.tag_type {
            TagType::Info => theme.primary.base,
            TagType::Success => theme.success.base,
            TagType::Warning => theme.warning.base,
            TagType::Danger => theme.danger.base,
        };

        let (bg, border, text_color) = match self.effect {
            TagEffect::Light => (color.opacity(0.1), color.opacity(0.2), color),
            TagEffect::Dark => (color, color, theme.neutral.text_1.opacity(1.0)),
            TagEffect::Plain => (theme.neutral.body, color.opacity(0.4), color),
        };

        // If it's dark effect, we might want a different text color if theme text primary is too dark.
        // But for simplicity, let's assume the user wants light text on dark backgrounds.
        let actual_text_color = if self.effect == TagEffect::Dark {
            gpui::white()
        } else {
            text_color
        };

        let (padding_x, height, text_size) = match self.size {
            TagSize::Small => (px(8.0), px(20.0), px(11.0)),
            TagSize::Default => (px(10.0), px(24.0), px(12.0)),
            TagSize::Large => (px(12.0), px(32.0), px(14.0)),
        };

        let radius = if self.round {
            height / 2.0
        } else {
            px(theme.radius.sm)
        };

        div()
            .flex()
            .items_center()
            .justify_center()
            .h(height)
            .px(padding_x)
            .bg(bg)
            .border_1()
            .border_color(border)
            .rounded(radius)
            .text_size(text_size)
            .text_color(actual_text_color)
            .child(div().child(self.label.clone()))
            .when(self.closable, |s| {
                let label = self.label.clone();
                s.child(
                    div()
                        .id(format!("close-btn-{}", label))
                        .ml_1()
                        .flex()
                        .items_center()
                        .justify_center()
                        .cursor_pointer()
                        .child(
                            Icon::new(IconName::X)
                                .size(px(10.0))
                                .color(actual_text_color),
                        )
                        .hover(|s| s.bg(actual_text_color.opacity(0.2)).rounded(px(2.0)))
                        .on_click(move |_, window, cx| {
                            if let Some(ref f) = on_close {
                                f(window, cx);
                            }
                        }),
                )
            })
    }
}

impl IntoElement for Tag {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
