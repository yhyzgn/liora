use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyElement, App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResultStatus {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

pub struct Result {
    status: ResultStatus,
    title: SharedString,
    sub_title: Option<SharedString>,
    icon: Option<AnyElement>,
    extra: Option<Box<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
}

impl Result {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            status: ResultStatus::Info,
            title: title.into(),
            sub_title: None,
            icon: None,
            extra: None,
        }
    }

    pub fn status(mut self, s: ResultStatus) -> Self {
        self.status = s;
        self
    }

    pub fn sub_title(mut self, sub: impl Into<SharedString>) -> Self {
        self.sub_title = Some(sub.into());
        self
    }

    pub fn icon(mut self, icon: impl IntoElement) -> Self {
        self.icon = Some(icon.into_any_element());
        self
    }

    pub fn extra<F>(mut self, f: F) -> Self
    where
        F: Fn(&mut Window, &mut App) -> AnyElement + 'static,
    {
        self.extra = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Result {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        let (icon_name, color) = match self.status {
            ResultStatus::Success => (IconName::CircleCheckBig, theme.success.base),
            ResultStatus::Warning => (IconName::TriangleAlert, theme.warning.base),
            ResultStatus::Error => (IconName::CircleX, theme.danger.base),
            ResultStatus::Info => (IconName::Info, theme.primary.base),
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .w_full()
            .p_12()
            .gap_4()
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(72.0))
                    .h(px(72.0))
                    .child(match self.icon {
                        Some(icon) => icon,
                        None => Icon::new(icon_name)
                            .size(px(72.0))
                            .color(color)
                            .into_any_element(),
                    }),
            )
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(theme.neutral.text_1)
                    .child(self.title),
            )
            .when_some(self.sub_title, |s, sub| {
                s.child(div().text_sm().text_color(theme.neutral.text_3).child(sub))
            })
            .when_some(self.extra, |s, extra| {
                s.child(div().mt_4().child((extra)(window, cx)))
            })
    }
}

impl IntoElement for Result {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
