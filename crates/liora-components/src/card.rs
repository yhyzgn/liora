use gpui::{
    AnyElement, App, Component, IntoElement, Pixels, RenderOnce, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, stable_unique_id};

pub struct Card {
    title: Option<SharedString>,
    header: Option<AnyElement>,
    footer: Option<AnyElement>,
    body: AnyElement,
    hoverable: bool,
    shadow: bool,
    width: Option<Pixels>,
    shrink: bool,
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
            width: None,
            shrink: true,
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

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn width_md(self) -> Self {
        self.width(px(300.0))
    }

    pub fn width_lg(self) -> Self {
        self.width(px(400.0))
    }

    pub fn no_shrink(mut self) -> Self {
        self.shrink = false;
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
            .overflow_hidden()
            .when(!self.shrink, |s| s.flex_none())
            .when_some(self.width, |s, width| s.w(width));

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn card_width_helpers_set_demo_widths() {
        assert_eq!(Card::new("body").width_md().width, Some(px(300.0)));
        assert_eq!(Card::new("body").width_lg().width, Some(px(400.0)));
    }

    #[test]
    fn card_no_shrink_tracks_scroll_container_usage() {
        assert!(!Card::new("body").no_shrink().shrink);
    }
}
