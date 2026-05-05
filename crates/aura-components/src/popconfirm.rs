use crate::{Popover, Button};
use aura_core::{Config, Placement};
use gpui::{
    prelude::*, App, Component, IntoElement, RenderOnce, SharedString, Window,
    div, AnyElement,
};
use std::sync::Arc;

pub struct Popconfirm {
    trigger: AnyElement,
    title: SharedString,
    confirm_text: SharedString,
    cancel_text: SharedString,
    on_confirm: Option<Arc<dyn Fn(&mut Window, &mut App) + 'static>>,
    on_cancel: Option<Arc<dyn Fn(&mut Window, &mut App) + 'static>>,
    placement: Placement,
}

impl Popconfirm {
    pub fn new(trigger: impl IntoElement) -> Self {
        Self {
            trigger: trigger.into_any_element(),
            title: SharedString::default(),
            confirm_text: "Confirm".into(),
            cancel_text: "Cancel".into(),
            on_confirm: None,
            on_cancel: None,
            placement: Placement::Top,
        }
    }

    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    pub fn confirm_text(mut self, text: impl Into<SharedString>) -> Self {
        self.confirm_text = text.into();
        self
    }

    pub fn cancel_text(mut self, text: impl Into<SharedString>) -> Self {
        self.cancel_text = text.into();
        self
    }

    pub fn on_confirm(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_confirm = Some(Arc::new(f));
        self
    }

    pub fn on_cancel(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_cancel = Some(Arc::new(f));
        self
    }

    pub fn placement(mut self, placement: Placement) -> Self {
        self.placement = placement;
        self
    }
}

impl RenderOnce for Popconfirm {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let title = self.title.clone();
        let confirm_text = self.confirm_text.clone();
        let cancel_text = self.cancel_text.clone();
        let on_confirm = self.on_confirm.clone();
        let on_cancel = self.on_cancel.clone();
        let theme = cx.global::<Config>().theme.clone();

        Popover::new(self.trigger)
            .placement(self.placement)
            .content(move |_window, _cx| {
                let on_confirm = on_confirm.clone();
                let on_cancel = on_cancel.clone();
                let confirm_text = confirm_text.clone();
                let cancel_text = cancel_text.clone();
                
                div().p_4().flex().flex_col().gap_3()
                    .child(
                        div().flex().flex_row().items_center().gap_2()
                            .child(div().text_color(theme.warning.base).child("⚠️"))
                            .child(div().font_weight(gpui::FontWeight::BOLD).child(title.clone()))
                    )
                    .child(
                        div().flex().flex_row().justify_end().gap_2()
                            .child(
                                Button::new(cancel_text.clone()).small()
                                    .on_click(move |_event, _window, _cx| {
                                        if let Some(ref f) = on_cancel { f(_window, _cx); }
                                        aura_core::clear_active_popover(_cx);
                                    })
                            )
                            .child(
                                Button::new(confirm_text.clone()).primary().small()
                                    .on_click(move |_event, _window, _cx| {
                                        if let Some(ref f) = on_confirm { f(_window, _cx); }
                                        aura_core::clear_active_popover(_cx);
                                    })
                            )
                    )
            })
    }
}

impl IntoElement for Popconfirm {
    type Element = Component<Self>;
    fn into_element(self) -> Self::Element {
        Component::new(self)
    }
}
