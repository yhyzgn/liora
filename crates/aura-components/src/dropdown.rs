use crate::{Popover};
use aura_core::{Config, Placement};
use gpui::{
    prelude::*, px, App, IntoElement, RenderOnce, Window,
    div, SharedString, AnyElement, MouseButton, Component,
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
}

impl Dropdown {
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            items: vec![],
            placement: Placement::BottomStart,
        }
    }

    pub fn item(mut self, label: impl Into<SharedString>, on_click: impl Fn(&mut Window, &mut App) + 'static) -> Self {
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
}

impl RenderOnce for Dropdown {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let items = self.items;

        Popover::new(self.trigger)
            .placement(self.placement)
            .offset(px(4.0))
            .content(move |_window, _cx| {
                let theme = theme.clone();
                div().flex().flex_col().py_1()
                    .children(items.iter().enumerate().map(|(i, item)| {
                        let on_click = item.on_click.clone();
                        let label = item.label.clone();
                        
                        div()
                            .id(("dropdown-item", i))
                            .cursor_pointer()
                            .px_4().py_2()
                            .text_size(px(theme.font_size.md))
                            .text_color(theme.neutral.text_2)
                            .hover(|s| s.bg(theme.neutral.hover).text_color(theme.primary.base))
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                on_click(window, cx);
                                aura_core::clear_active_popover(cx);
                            })
                            .child(label)
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
