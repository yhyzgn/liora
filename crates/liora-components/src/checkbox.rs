use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use gpui::{
    App, Context, EventEmitter, FocusHandle, Focusable, Hsla, KeyBinding, MouseButton, Render,
    Rgba, SharedString, Window, prelude::*, px,
};
use liora_icons::Icon;
use liora_icons_lucide::IconName;

fn rgba(r: u8, g: u8, b: u8, a: f32) -> Hsla {
    Rgba {
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
        a,
    }
    .into()
}

gpui::actions!(checkbox, [CheckboxToggle]);

pub struct Checkbox {
    checked: bool,
    disabled: bool,
    label: Option<SharedString>,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

#[derive(Clone, Copy)]
pub struct CheckboxChanged(pub bool);

impl EventEmitter<CheckboxChanged> for Checkbox {}

impl Checkbox {
    pub fn new(checked: bool, cx: &mut Context<Self>) -> Self {
        Self {
            checked,
            disabled: false,
            label: None,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn label(mut self, text: impl Into<SharedString>) -> Self {
        self.label = Some(text.into());
        self
    }
    pub fn on_change(mut self, cb: impl Fn(bool, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub fn set_disabled(&mut self, d: bool, cx: &mut Context<Self>) {
        self.disabled = d;
        cx.notify();
    }

    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("space", CheckboxToggle, None),
            KeyBinding::new("enter", CheckboxToggle, None),
        ]);
    }

    fn toggle(&mut self, _: &CheckboxToggle, window: &mut Window, cx: &mut Context<Self>) {
        if !self.disabled {
            self.checked = !self.checked;
            cx.emit(CheckboxChanged(self.checked));
            cx.notify();
            if let Some(ref cb) = self.on_change {
                cb(self.checked, window, cx);
            }
        }
    }
}

impl Focusable for Checkbox {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Checkbox {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<liora_core::Config>().theme;
        let focused = self.focus_handle.is_focused(_window);
        let sz = 16.0;

        let (bg, border, check_color) = if self.disabled {
            (
                theme.neutral.hover,
                theme.neutral.border,
                theme.neutral.text_disabled,
            )
        } else if self.checked {
            (
                theme.primary.base,
                theme.primary.base,
                rgba(255, 255, 255, 1.0),
            )
        } else {
            (
                rgba(0, 0, 0, 0.0),
                if focused {
                    theme.primary.base
                } else {
                    theme.neutral.border
                },
                rgba(0, 0, 0, 0.0),
            )
        };

        let mut row = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .on_action(cx.listener(Self::toggle));

        if !self.disabled {
            row = row.cursor_pointer().track_focus(&self.focus_handle);
            row = row.on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, _cx| {
                    window.focus(&this.focus_handle);
                }),
            );
            row = row.on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle(&CheckboxToggle, window, cx);
                }),
            );
        } else {
            row = row.cursor_not_allowed();
        }

        let mut box_el = gpui::div()
            .flex_none()
            .w(px(sz))
            .h(px(sz))
            .rounded(px(2.0))
            .bg(bg)
            .border_1()
            .border_color(border)
            .flex()
            .items_center()
            .justify_center();

        if self.checked {
            box_el = box_el.child(pop_in(
                element_id(format!(
                    "liora-checkbox-check-motion-{}",
                    cx.entity().entity_id()
                )),
                gpui::div()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::Check).size(px(12.0)).color(check_color)),
            ));
        }

        row = row.child(box_el);

        if let Some(ref label) = self.label {
            row = row.child(
                gpui::div()
                    .text_size(px(theme.font_size.md))
                    .text_color(theme.neutral.text_1)
                    .child(label.clone()),
            );
        }

        row
    }
}
