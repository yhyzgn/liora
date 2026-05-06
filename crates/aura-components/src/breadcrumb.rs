use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{App, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreadcrumbSeparator {
    String(SharedString),
    Icon(IconName),
}

pub struct BreadcrumbItem {
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub on_click: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

pub struct Breadcrumb {
    separator: BreadcrumbSeparator,
    items: Vec<BreadcrumbItem>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            icon: None,
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: IconName) -> Self {
        self.icon = Some(icon);
        self
    }

    pub fn on_click(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Breadcrumb {
    pub fn new() -> Self {
        Self {
            separator: BreadcrumbSeparator::String("/".into()),
            items: vec![],
        }
    }

    pub fn separator(mut self, s: impl Into<SharedString>) -> Self {
        self.separator = BreadcrumbSeparator::String(s.into());
        self
    }

    pub fn separator_icon(mut self, icon: IconName) -> Self {
        self.separator = BreadcrumbSeparator::Icon(icon);
        self
    }

    pub fn item(mut self, item: BreadcrumbItem) -> Self {
        self.items.push(item);
        self
    }
}

impl RenderOnce for Breadcrumb {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items_count = self.items.len();
        let separator = self.separator;

        div().flex().flex_row().items_center().gap_1().children(
            self.items.into_iter().enumerate().map(|(i, item)| {
                let is_last = i == items_count - 1;
                let has_click = item.on_click.is_some();
                let on_click = item.on_click;

                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .child(
                        div()
                            .id(format!("breadcrumb-item-{}", i))
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .text_color(if is_last {
                                theme.neutral.text_1
                            } else {
                                theme.neutral.text_2
                            })
                            .when(is_last, |s| s.font_weight(gpui::FontWeight::BOLD))
                            .when(!is_last && has_click, |s| {
                                s.cursor_pointer()
                                    .hover(|s| s.text_color(theme.primary.base))
                                    .on_click(move |_, window, cx| {
                                        if let Some(ref f) = on_click {
                                            (f)(window, cx);
                                        }
                                    })
                            })
                            .when_some(item.icon, |s, icon| {
                                s.child(Icon::new(icon).size(px(14.0)).color(theme.neutral.icon))
                            })
                            .child(div().text_sm().child(item.label)),
                    )
                    .when(!is_last, |s| {
                        s.child(
                            div().px_2().flex().items_center().justify_center().child(
                                match separator.clone() {
                                    BreadcrumbSeparator::String(sep) => div()
                                        .text_sm()
                                        .text_color(theme.neutral.text_3)
                                        .child(sep)
                                        .into_any_element(),
                                    BreadcrumbSeparator::Icon(icon) => Icon::new(icon)
                                        .size(px(12.0))
                                        .color(theme.neutral.icon)
                                        .into_any_element(),
                                },
                            ),
                        )
                    })
            }),
        )
    }
}

impl IntoElement for Breadcrumb {
    type Element = gpui::Component<Self>;
    fn into_element(self) -> Self::Element {
        gpui::Component::new(self)
    }
}
