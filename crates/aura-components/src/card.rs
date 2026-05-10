use aura_core::{Config, stable_unique_id};
use gpui::{
    AnyElement, App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};

pub struct Card {
    title: Option<SharedString>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    body: AnyElement,
    hoverable: bool,
    shadow: bool,
}

impl Card {
    pub fn new(body: impl IntoElement) -> Self {
        Self {
            title: None,
            header: None,
            footer: None,
            body: body.into_any_element(),
            hoverable: false,
            shadow: true,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn header(mut self, header: impl IntoElement) -> Self {
        self.header = Some(header.into_any_element());
        self
    }

    pub fn footer(mut self, footer: impl IntoElement) -> Self {
        self.footer = Some(footer.into_any_element());
        self
    }

    pub fn hoverable(mut self) -> Self {
        self.hoverable = true;
        self
    }

    pub fn no_shadow(mut self) -> Self {
        self.shadow = false;
        self
    }
}

impl RenderOnce for Card {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = stable_unique_id("card", "card", _window, cx);

        let mut el = div()
            .id(id)
            .bg(theme.neutral.card)
            .border_1()
            .border_color(theme.neutral.border)
            .rounded(px(theme.radius.md))
            .overflow_hidden();

        if self.shadow {
            el = el.shadow_md();
        }

        if self.hoverable {
            el = el.hover(|s| s.shadow_xl().border_color(theme.primary.base));
        }

        // We use on_click to ensure the ID-based hover and other interactions are registered correctly
        el = el.on_click(|_, _, _| {});

        // Header
        if let Some(title) = self.title {
            el = el.child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .child(div().font_weight(gpui::FontWeight::BOLD).child(title)),
            );
        } else if let Some(header) = self.header {
            el = el.child(
                div()
                    .p_4()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .child(header),
            );
        }

        // Body
        el = el.child(div().p_4().child(self.body));

        // Footer
        if let Some(footer) = self.footer {
            el = el.child(
                div()
                    .p_4()
                    .border_t_1()
                    .border_color(theme.neutral.border)
                    .bg(theme.neutral.hover)
                    .child(footer),
            );
        }

        el
    }
}

impl IntoElement for Card {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
