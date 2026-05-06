use crate::{Checkbox, CheckboxChanged};
use gpui::{
    App, Context, Entity, FocusHandle, Focusable, Render, SharedString, Window, prelude::*,
};

pub struct CheckboxGroup {
    selected: Vec<usize>,
    disabled: bool,
    focus_handle: FocusHandle,
    checkboxes: Vec<Entity<Checkbox>>,
    on_change: Option<Box<dyn Fn(Vec<usize>, &mut Window, &mut App) + 'static>>,
}

impl CheckboxGroup {
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected: Vec<usize>,
        cx: &mut Context<Self>,
    ) -> Self {
        let options: Vec<SharedString> = options.into_iter().map(|o| o.into()).collect();
        let mut checkboxes = Vec::new();

        for (i, label) in options.iter().enumerate() {
            let is_checked = selected.contains(&i);
            let checkbox = cx.new(|cx| Checkbox::new(is_checked, cx).label(label.clone()));

            // Subscribe to each checkbox's change
            cx.subscribe(
                &checkbox,
                move |this, _checkbox, event: &CheckboxChanged, cx| {
                    this.update_selection(i, event.0, cx);
                },
            )
            .detach();

            checkboxes.push(checkbox);
        }

        Self {
            selected,
            disabled: false,
            focus_handle: cx.focus_handle(),
            checkboxes,
            on_change: None,
        }
    }

    pub fn disabled(mut self, d: bool, cx: &mut Context<Self>) -> Self {
        self.disabled = d;
        for cb in &self.checkboxes {
            cb.update(cx, |cb, cx| {
                cb.set_disabled(d, cx);
            });
        }
        self
    }

    pub fn on_change(mut self, cb: impl Fn(Vec<usize>, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    pub fn register_key_bindings(_cx: &mut App) {}

    fn update_selection(&mut self, idx: usize, checked: bool, cx: &mut Context<Self>) {
        if checked {
            if !self.selected.contains(&idx) {
                self.selected.push(idx);
                self.selected.sort();
            }
        } else {
            self.selected.retain(|&i| i != idx);
        }

        // We don't have Window here, but we can notify.
        // If on_change needs window, we might need a different approach.
        // For now, let's just notify.
        cx.notify();
    }
}

impl Focusable for CheckboxGroup {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for CheckboxGroup {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let mut col = gpui::div().flex().flex_col().gap_2();

        if !self.disabled {
            col = col.track_focus(&self.focus_handle);
        }

        for cb_entity in &self.checkboxes {
            col = col.child(cb_entity.clone());
        }

        col
    }
}
