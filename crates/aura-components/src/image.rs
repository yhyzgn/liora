use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Component, IntoElement, ObjectFit, Pixels, RenderOnce, SharedString, Window,
    div, img, prelude::*, px,
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageFit {
    Fill,
    #[default]
    Contain,
    Cover,
    ScaleDown,
    None,
}

impl ImageFit {
    pub fn as_object_fit(self) -> ObjectFit {
        match self {
            ImageFit::Fill => ObjectFit::Fill,
            ImageFit::Contain => ObjectFit::Contain,
            ImageFit::Cover => ObjectFit::Cover,
            ImageFit::ScaleDown => ObjectFit::ScaleDown,
            ImageFit::None => ObjectFit::None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ImageRadius {
    None,
    Small,
    #[default]
    Medium,
    Large,
    Round,
}

pub struct Image {
    src: Option<SharedString>,
    alt: Option<SharedString>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    fit: ImageFit,
    radius: ImageRadius,
    bordered: bool,
    shadow: bool,
    grayscale: bool,
    preview: bool,
    placeholder: Option<Arc<dyn Fn() -> AnyElement + 'static>>,
    fallback: Option<Arc<dyn Fn() -> AnyElement + 'static>>,
}

impl Image {
    pub fn new(src: impl Into<SharedString>) -> Self {
        Self {
            src: Some(src.into()),
            alt: None,
            width: None,
            height: None,
            fit: ImageFit::Contain,
            radius: ImageRadius::Medium,
            bordered: true,
            shadow: false,
            grayscale: false,
            preview: false,
            placeholder: None,
            fallback: None,
        }
    }

    pub fn empty() -> Self {
        Self {
            src: None,
            alt: None,
            width: None,
            height: None,
            fit: ImageFit::Contain,
            radius: ImageRadius::Medium,
            bordered: true,
            shadow: false,
            grayscale: false,
            preview: false,
            placeholder: None,
            fallback: None,
        }
    }

    pub fn src(mut self, src: impl Into<SharedString>) -> Self {
        self.src = Some(src.into());
        self
    }

    pub fn alt(mut self, alt: impl Into<SharedString>) -> Self {
        self.alt = Some(alt.into());
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = Some(height.into());
        self
    }

    pub fn size(mut self, width: impl Into<Pixels>, height: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self.height = Some(height.into());
        self
    }

    pub fn square(mut self, size: impl Into<Pixels>) -> Self {
        let size = size.into();
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    pub fn fill(mut self) -> Self {
        self.fit = ImageFit::Fill;
        self
    }

    pub fn contain(mut self) -> Self {
        self.fit = ImageFit::Contain;
        self
    }

    pub fn cover(mut self) -> Self {
        self.fit = ImageFit::Cover;
        self
    }

    pub fn scale_down(mut self) -> Self {
        self.fit = ImageFit::ScaleDown;
        self
    }

    pub fn radius(mut self, radius: ImageRadius) -> Self {
        self.radius = radius;
        self
    }

    pub fn no_radius(mut self) -> Self {
        self.radius = ImageRadius::None;
        self
    }

    pub fn round(mut self) -> Self {
        self.radius = ImageRadius::Round;
        self
    }

    pub fn bordered(mut self, bordered: bool) -> Self {
        self.bordered = bordered;
        self
    }

    pub fn no_border(mut self) -> Self {
        self.bordered = false;
        self
    }

    pub fn shadow(mut self, shadow: bool) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }

    pub fn preview(mut self, preview: bool) -> Self {
        self.preview = preview;
        self
    }

    pub fn placeholder<E>(mut self, placeholder: impl Fn() -> E + 'static) -> Self
    where
        E: IntoElement,
    {
        self.placeholder = Some(Arc::new(move || placeholder().into_any_element()));
        self
    }

    pub fn fallback<E>(mut self, fallback: impl Fn() -> E + 'static) -> Self
    where
        E: IntoElement,
    {
        self.fallback = Some(Arc::new(move || fallback().into_any_element()));
        self
    }

    pub fn fit_kind(&self) -> ImageFit {
        self.fit
    }

    pub fn dimensions(&self) -> (Option<Pixels>, Option<Pixels>) {
        (self.width, self.height)
    }
}

impl RenderOnce for Image {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let radius = match self.radius {
            ImageRadius::None => px(0.0),
            ImageRadius::Small => px(theme.radius.sm),
            ImageRadius::Medium => px(theme.radius.md),
            ImageRadius::Large => px(theme.radius.lg),
            ImageRadius::Round => px(999.0),
        };
        let alt = self.alt.clone().unwrap_or_else(|| "Image".into());

        let mut frame = div()
            .relative()
            .flex()
            .items_center()
            .justify_center()
            .overflow_hidden()
            .rounded(radius)
            .bg(theme.neutral.hover);

        if let Some(width) = self.width {
            frame = frame.w(width);
        }
        if let Some(height) = self.height {
            frame = frame.h(height);
        }
        if self.width.is_none() && self.height.is_none() {
            frame = frame.w(px(160.0)).h(px(100.0));
        }
        if self.bordered {
            frame = frame.border_1().border_color(theme.neutral.border);
        }
        if self.shadow {
            frame = frame.shadow_md();
        }

        if let Some(src) = self.src {
            let loading = self.placeholder.unwrap_or_else({
                let theme = theme.clone();
                || Arc::new(move || default_loading(&theme))
            });
            let fallback = self.fallback.unwrap_or_else({
                let theme = theme.clone();
                || Arc::new(move || default_fallback(&theme, alt.clone()))
            });
            frame = frame.child(
                img(src)
                    .size_full()
                    .object_fit(self.fit.as_object_fit())
                    .grayscale(self.grayscale)
                    .with_loading(move || loading())
                    .with_fallback(move || fallback()),
            );
        } else {
            frame = frame.child(
                self.fallback
                    .map(|fallback| fallback())
                    .unwrap_or_else(|| default_empty(&theme)),
            );
        }

        if self.preview {
            frame = frame
                .cursor_pointer()
                .hover(|s| s.border_color(theme.primary.base).shadow_lg())
                .child(
                    div()
                        .absolute()
                        .right(px(6.0))
                        .bottom(px(6.0))
                        .px_2()
                        .py_1()
                        .rounded(px(theme.radius.sm))
                        .bg(theme.neutral.card.opacity(0.86))
                        .text_xs()
                        .text_color(theme.neutral.text_1)
                        .child("Preview"),
                );
        }

        frame
    }
}

impl IntoElement for Image {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}

fn default_loading(theme: &aura_theme::Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .size_full()
        .text_color(theme.neutral.text_3)
        .child(
            Icon::new(IconName::LoaderCircle)
                .size(px(20.0))
                .color(theme.neutral.icon),
        )
        .child(div().text_xs().child("Loading"))
        .into_any_element()
}

fn default_fallback(theme: &aura_theme::Theme, alt: SharedString) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .size_full()
        .text_color(theme.neutral.text_3)
        .child(
            Icon::new(IconName::ImageOff)
                .size(px(24.0))
                .color(theme.neutral.icon),
        )
        .child(div().text_xs().child(alt))
        .into_any_element()
}

fn default_empty(theme: &aura_theme::Theme) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
        .gap_2()
        .size_full()
        .text_color(theme.neutral.text_3)
        .child(
            Icon::new(IconName::Image)
                .size(px(24.0))
                .color(theme.neutral.icon),
        )
        .child(div().text_xs().child("No image"))
        .into_any_element()
}
