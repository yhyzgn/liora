use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub struct PageHeader {
    title: SharedString,
    sub_title: Option<SharedString>,
    back_icon: Option<IconName>,
    on_back: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    content: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    footer: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

impl PageHeader {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            sub_title: None,
            back_icon: Some(IconName::ArrowLeft),
            on_back: None,
            extra: None,
            content: None,
            footer: None,
        }
    }

    pub fn sub_title(mut self, sub_title: impl Into<SharedString>) -> Self {
        self.sub_title = Some(sub_title.into());
        self
    }

    pub fn back_icon(mut self, icon: IconName) -> Self {
        self.back_icon = Some(icon);
        self
    }

    pub fn on_back(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_back = Some(Box::new(f));
        self
    }

    pub fn extra<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.extra = Some(Box::new(f));
        self
    }

    pub fn content<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.content = Some(Box::new(f));
        self
    }

    pub fn footer<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.footer = Some(Box::new(f));
        self
    }
}

impl RenderOnce for PageHeader {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_col()
            .w_full()
            .p_4()
            .gap_4()
            .bg(theme.neutral.card)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            // Back Button
                            .when_some(self.back_icon, |s, icon| {
                                s.child(
                                    div()
                                        .id("back-btn")
                                        .cursor_pointer()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .w_8()
                                        .h_8()
                                        .rounded_full()
                                        .hover(|s| s.bg(theme.neutral.hover))
                                        .on_click(move |_, window, cx| {
                                            if let Some(ref f) = self.on_back {
                                                (f)(window, cx);
                                            }
                                        })
                                        .child(
                                            Icon::new(icon)
                                                .size(px(20.0))
                                                .color(theme.neutral.text_1),
                                        ),
                                )
                            })
                            // Title
                            .child(
                                div()
                                    .text_lg()
                                    .font_weight(gpui::FontWeight::BOLD)
                                    .text_color(theme.neutral.text_1)
                                    .child(self.title),
                            )
                            // Divider
                            .when(self.sub_title.is_some(), |s| {
                                s.child(div().w(px(1.0)).h(px(16.0)).bg(theme.neutral.border))
                            })
                            // Subtitle
                            .when_some(self.sub_title, |s, sub| {
                                s.child(div().text_sm().text_color(theme.neutral.text_3).child(sub))
                            }),
                    )
                    // Extra
                    .when_some(self.extra, |s, extra| {
                        s.child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .child((extra)(window, cx)),
                        )
                    }),
            )
            // Content
            .when_some(self.content, |s, content| {
                s.child(div().child((content)(window, cx)))
            })
            // Footer
            .when_some(self.footer, |s, footer| {
                s.child(
                    div()
                        .border_t_1()
                        .border_color(theme.neutral.border)
                        .pt_4()
                        .child((footer)(window, cx)),
                )
            })
    }
}

impl IntoElement for PageHeader {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
