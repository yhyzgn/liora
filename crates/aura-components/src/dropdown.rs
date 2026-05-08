use crate::Popover;
use aura_core::{Config, Placement, clear_popover};
use gpui::{
    AnyElement, App, Component, IntoElement, RenderOnce, SharedString, Window, div, prelude::*, px,
};
use std::sync::Arc;

pub struct DropdownItem {
    pub label: SharedString,
    pub on_click: Arc<dyn Fn(&mut Window, &mut App) + 'static>,
}

pub struct Dropdown {
    trigger: AnyElement,
    items: Vec<DropdownItem>,
    placement: Placement,
    id: SharedString,
}

impl Dropdown {
    #[track_caller]
    pub fn new(trigger: impl IntoElement) -> Self {
        let caller = std::panic::Location::caller();
        Self {
            trigger: trigger.into_any_element(),
            items: vec![],
            placement: Placement::BottomStart,
            id: format!("dropdown-{}", caller).into(),
        }
    }

    pub fn item(
        mut self,
        label: impl Into<SharedString>,
        on_click: impl Fn(&mut Window, &mut App) + 'static,
    ) -> Self {
        self.items.push(DropdownItem {
            label: label.into(),
            on_click: Arc::new(on_click),
        });
        self
    }

    pub fn placement(mut self, p: Placement) -> Self {
        self.placement = p;
        self
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }
}

impl RenderOnce for Dropdown {
    #[track_caller]
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items = self.items;
        let dropdown_id = self.id.clone();
        let caller = std::panic::Location::caller();

        Popover::new(self.trigger)
            .id(dropdown_id.clone())
            .placement(self.placement)
            .offset(px(4.0))
            .content(move |_window, _cx| {
                let theme = theme.clone();
                div()
                    .id(format!("{}-menu", dropdown_id))
                    .cursor_default()
                    .occlude()
                    .flex()
                    .flex_col()
                    .py_1()
                    .min_w(px(168.0))
                    .max_h(px(200.0))
                    .children(items.iter().enumerate().map(|(i, item)| {
                        let on_click = item.on_click.clone();
                        let label = item.label.clone();
                        let dropdown_id = dropdown_id.clone();
                        let item_id = format!(
                            "{}-item-{}-{}-{}",
                            dropdown_id,
                            caller.line(),
                            caller.column(),
                            i
                        );

                        div()
                            .id(item_id)
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .min_h(px(34.0))
                            .px_3()
                            .py_2()
                            .text_size(px(theme.font_size.md))
                            .text_color(theme.neutral.text_1)
                            .hover(|s| {
                                s.cursor_pointer()
                                    .bg(theme.neutral.hover)
                                    .text_color(theme.primary.base)
                            })
                            .on_click(move |_, window, cx| {
                                on_click(window, cx);
                                clear_popover(&dropdown_id, cx);
                            })
                            .child(div().text_sm().child(label))
                    }))
            })
    }
}

impl IntoElement for Dropdown {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
