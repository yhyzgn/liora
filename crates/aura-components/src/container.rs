use gpui::{App, Component, IntoElement, RenderOnce, Window, prelude::*, px};

pub struct Container {
    header: Option<gpui::AnyElement>,
    aside: Option<gpui::AnyElement>,
    aside_right: bool,
    footer: Option<gpui::AnyElement>,
    main: Vec<gpui::AnyElement>,
}

impl Container {
    pub fn new() -> Self {
        Self {
            header: None,
            aside: None,
            aside_right: false,
            footer: None,
            main: vec![],
        }
    }

    pub fn header(mut self, el: impl IntoElement) -> Self {
        self.header = Some(el.into_any_element());
        self
    }
    pub fn aside(mut self, el: impl IntoElement) -> Self {
        self.aside = Some(el.into_any_element());
        self
    }
    pub fn aside_right(mut self) -> Self {
        self.aside_right = true;
        self
    }
    pub fn footer(mut self, el: impl IntoElement) -> Self {
        self.footer = Some(el.into_any_element());
        self
    }
    pub fn child(mut self, el: impl IntoElement) -> Self {
        self.main.push(el.into_any_element());
        self
    }
}

impl RenderOnce for Container {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;
        let header_h = 48.0;
        let footer_h = 48.0;
        let aside_w = 200.0;

        let mut page = gpui::div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.neutral.body);

        // Header
        if let Some(h) = self.header {
            page = page.child(
                gpui::div()
                    .flex_none()
                    .h(px(header_h))
                    .w_full()
                    .border_b_1()
                    .border_color(theme.neutral.border)
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .child(h),
            );
        }

        // Body: aside + main
        let mut body = gpui::div().flex().flex_1().flex_row();
        if let Some(a) = self.aside {
            let aside_el = gpui::div()
                .flex_none()
                .w(px(aside_w))
                .h_full()
                .border_r_1()
                .border_color(theme.neutral.border)
                .child(a);
            if self.aside_right {
                body = body.child(gpui::div().flex_1().flex().flex_col().children(self.main));
                body = body.child(aside_el);
            } else {
                body = body.child(aside_el);
                body = body.child(gpui::div().flex_1().flex().flex_col().children(self.main));
            }
        } else {
            body = body.child(gpui::div().flex_1().flex().flex_col().children(self.main));
        }
        page = page.child(body);

        // Footer
        if let Some(f) = self.footer {
            page = page.child(
                gpui::div()
                    .flex_none()
                    .h(px(footer_h))
                    .w_full()
                    .border_t_1()
                    .border_color(theme.neutral.border)
                    .px(px(16.0))
                    .flex()
                    .items_center()
                    .child(f),
            );
        }

        page
    }
}

impl IntoElement for Container {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
