use aura_core::{Config};
use gpui::{
    prelude::*, px, App, IntoElement, Window,
    div, SharedString, Component, RenderOnce,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertType {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

pub struct Alert {
    title: SharedString,
    description: Option<SharedString>,
    alert_type: AlertType,
    closable: bool,
    show_icon: bool,
    on_close: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Alert {
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            description: None,
            alert_type: AlertType::Info,
            closable: false,
            show_icon: true,
            on_close: None,
        }
    }

    pub fn description(mut self, desc: impl Into<SharedString>) -> Self {
        self.description = Some(desc.into());
        self
    }

    pub fn alert_type(mut self, t: AlertType) -> Self {
        self.alert_type = t;
        self
    }

    pub fn closable(mut self, c: bool) -> Self {
        self.closable = c;
        self
    }

    pub fn show_icon(mut self, s: bool) -> Self {
        self.show_icon = s;
        self
    }

    pub fn on_close(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_close = Some(Box::new(f));
        self
    }
}

impl RenderOnce for Alert {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        
        let (color, icon_name) = match self.alert_type {
            AlertType::Info => (theme.primary.base, IconName::Info),
            AlertType::Success => (theme.success.base, IconName::Check),
            AlertType::Warning => (theme.warning.base, IconName::TriangleAlert),
            AlertType::Error => (theme.danger.base, IconName::CircleX),
        };

        let bg = color.opacity(0.1);

        div()
            .flex().flex_row().items_center().gap_3().p_3()
            .bg(bg).border_1().border_color(color).rounded(px(theme.radius.md))
            .child(
                div().flex().items_center().when(self.show_icon, |s| s.child(Icon::new(icon_name).size(px(20.0)).color(color)))
            )
            .child(
                div().flex_1().flex().flex_col().gap_1()
                    .child(div().flex().items_center().min_h(px(20.0)).font_weight(gpui::FontWeight::BOLD).text_color(color).child(self.title))
                    .when_some(self.description, |s, d| s.child(div().text_sm().text_color(color).child(d)))
            )
            .child(
                div().flex().items_center().when(self.closable, |s| s.child(
                    div().id("close-btn").cursor_pointer().child(Icon::new(IconName::X).size(px(14.0)).color(color))
                        .on_click(|_, _window, _cx| {
                            // Notify parent
                        })
                ))
            )
    }
}

impl IntoElement for Alert {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
