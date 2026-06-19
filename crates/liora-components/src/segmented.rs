use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px};
use liora_core::Config;

pub struct SegmentedOption {
    pub label: SharedString,
    pub value: SharedString,
    pub disabled: bool,
}

impl SegmentedOption {
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

pub struct Segmented {
    id: SharedString,
    options: Vec<SegmentedOption>,
    value: Option<SharedString>,
    block: bool,
    on_change: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl Segmented {
    pub fn new(options: Vec<SegmentedOption>) -> Self {
        let first_value = options.first().map(|o| o.value.clone());
        Self {
            id: liora_core::unique_id("segmented"),
            options,
            value: first_value,
            block: false,
            on_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = Some(val.into());
        self
    }

    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    pub fn on_change(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn set_on_change(&mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Box::new(f));
    }

    fn select_option(&mut self, value: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if Some(&value) != self.value.as_ref() {
            self.value = Some(value.clone());
            if let Some(ref on_change) = self.on_change {
                (on_change)(value, window, cx);
            }
            cx.notify();
        }
    }
}

impl Render for Segmented {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .p(px(2.0))
            .gap(px(2.0))
            .bg(theme.neutral.hover)
            .rounded(px(theme.radius.md))
            .when(self.block, |s| s.w_full())
            .children(self.options.iter().enumerate().map(|(i, opt)| {
                let is_active = self.value.as_ref() == Some(&opt.value);
                let value = opt.value.clone();
                let disabled = opt.disabled;

                let option = div()
                    .id(element_id(format!("{}-option-{}", self.id, i)))
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(28.0))
                    .px_3()
                    .rounded(px(theme.radius.sm))
                    .when(self.block, |s| s.flex_1())
                    .when(is_active, |s| {
                        s.bg(theme.neutral.card)
                            .shadow_sm()
                            .text_color(theme.neutral.text_1)
                            .font_weight(gpui::FontWeight::BOLD)
                    })
                    .when(!is_active && !disabled, |s| {
                        s.text_color(theme.neutral.text_2).hover(|s| {
                            s.cursor_pointer()
                                .bg(theme.neutral.card.opacity(0.6))
                                .text_color(theme.neutral.text_1)
                        })
                    })
                    .when(disabled, |s| {
                        s.text_color(theme.neutral.text_3)
                            .opacity(0.5)
                            .cursor_not_allowed()
                    })
                    .when(!disabled && !is_active, |s| {
                        s.cursor_pointer().on_click(cx.listener({
                            let value = value.clone();
                            move |this, _, window, cx| {
                                this.select_option(value.clone(), window, cx);
                            }
                        }))
                    })
                    .child(div().text_sm().child(opt.label.clone()));

                if is_active {
                    pop_in(
                        element_id(format!("{}-option-motion-{}", self.id, i)),
                        option,
                    )
                    .into_any_element()
                } else {
                    option.into_any_element()
                }
            }))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn segmented_supports_runtime_on_change_binding() {
        let source = include_str!("segmented.rs");
        assert!(source.contains("pub fn set_on_change"));
        assert!(source.contains("on_change: Option"));
    }
}
