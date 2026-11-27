//! Segmented module.
//!
//! This public module implements the Liora segmented control for compact mutually exclusive choices. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
//! downstream GPUI applications can compose the same behavior with their own
//! app state, assets, and release policy.
//!
//! ## Usage model
//!
//! Components in this module render native GPUI element trees. Stateless builder
//! values can be constructed inline, while controls with focus, selection,
//! popup, drag, or editing state should be stored as `gpui::Entity<T>` fields in
//! the parent view so state survives GPUI render passes.
//!
//! ## Design contract
//!
//! The implementation should use Liora theme tokens from `liora-core` and
//! `liora-theme`, keep accessibility-oriented keyboard/pointer behavior close to
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::motion::pop_in;
use gpui::{
    App, Context, FocusHandle, IntoElement, Render, Role, SharedString, Window, div, prelude::*, px,
};
use liora_core::{Config, LocalizedText};
use std::collections::{HashMap, VecDeque};

/// Fluent native GPUI component for rendering Liora segmented option.
pub struct SegmentedOption {
    /// User-facing label rendered for this item.
    pub label: LocalizedText,
    /// Machine-readable value represented by this item.
    pub value: SharedString,
    /// Whether user interaction is disabled for this item.
    pub disabled: bool,
}

impl SegmentedOption {
    /// Creates `SegmentedOption` initialized from the supplied label, and value.
    pub fn new(label: impl Into<LocalizedText>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            disabled: false,
        }
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

/// Fluent native GPUI component for rendering Liora segmented.
pub struct Segmented {
    id: SharedString,
    options: Vec<SegmentedOption>,
    value: Option<SharedString>,
    selected_index: Option<usize>,
    focus: Vec<Option<FocusHandle>>,
    block: bool,
    on_change: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl Segmented {
    /// Creates `Segmented` that renders the supplied options collection.
    pub fn new(options: Vec<SegmentedOption>) -> Self {
        let first_value = options.first().map(|o| o.value.clone());
        Self {
            id: liora_core::unique_id("segmented"),
            selected_index: (!options.is_empty()).then_some(0),
            focus: Vec::new(),
            options,
            value: first_value,
            block: false,
            on_change: None,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(mut self, val: impl Into<SharedString>) -> Self {
        self.value = Some(val.into());
        self.selected_index = self
            .options
            .iter()
            .position(|option| Some(&option.value) == self.value.as_ref());
        self
    }

    /// Makes the component occupy the available inline width.
    pub fn block(mut self, block: bool) -> Self {
        self.block = block;
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(&mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Box::new(f));
    }

    /// Replaces options while preserving the selected value when possible.
    pub fn set_options(&mut self, options: Vec<SegmentedOption>) {
        let current = self.value.clone();
        self.value = current
            .filter(|value| options.iter().any(|option| &option.value == value))
            .or_else(|| options.first().map(|option| option.value.clone()));

        // Queues match value + occurrence only when options change, including duplicates.
        let mut previous: HashMap<_, VecDeque<_>> = HashMap::new();
        let mut handles = self.focus.drain(..);
        for (index, option) in self.options.iter().enumerate() {
            previous
                .entry(&option.value)
                .or_default()
                .push_back((index, handles.next().flatten()));
        }
        drop(handles);
        self.focus.reserve(options.len());
        let mut selected_index = None;
        for (index, option) in options.iter().enumerate() {
            let focus = previous
                .get_mut(&option.value)
                .and_then(VecDeque::pop_front)
                .and_then(|(old_index, focus)| {
                    if self.selected_index == Some(old_index) {
                        selected_index = Some(index);
                    }
                    focus
                });
            self.focus.push(focus);
        }
        self.selected_index = selected_index.or_else(|| {
            options
                .iter()
                .position(|option| Some(&option.value) == self.value.as_ref())
        });
        drop(previous);
        self.options = options;
    }

    fn select_option(&mut self, index: usize, window: &mut Window, cx: &mut Context<Self>) {
        let Some(option) = self.options.get(index).filter(|option| !option.disabled) else {
            return;
        };
        if self.selected_index == Some(index) {
            return;
        }
        let value = option.value.clone();
        let changed_value = self.value.as_ref() != Some(&value);
        self.selected_index = Some(index);
        self.value = Some(value.clone());
        if changed_value && let Some(on_change) = &self.on_change {
            on_change(value, window, cx);
        }
        cx.notify();
    }

    fn move_focus(
        &mut self,
        focus: &FocusHandle,
        key: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(index) = self
            .focus
            .iter()
            .position(|handle| handle.as_ref() == Some(focus))
            .filter(|&index| !self.options[index].disabled)
        else {
            return;
        };
        let len = self.options.len();
        let target = match key {
            "left" | "up" => (1..=len)
                .map(|offset| (index + len - offset) % len)
                .find(|&index| !self.options[index].disabled),
            "right" | "down" => (1..=len)
                .map(|offset| (index + offset) % len)
                .find(|&index| !self.options[index].disabled),
            "home" => self.options.iter().position(|option| !option.disabled),
            "end" => self.options.iter().rposition(|option| !option.disabled),
            _ => return,
        };
        if let Some(target) = target {
            if let Some(focus) = &self.focus[target] {
                focus.focus(window, cx);
            }
            self.select_option(target, window, cx);
        }
        cx.stop_propagation();
    }
}

impl Render for Segmented {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        self.focus.resize_with(self.options.len(), || None);
        for focus in &mut self.focus {
            focus.get_or_insert_with(|| cx.focus_handle());
        }
        let entry = self
            .focus
            .iter()
            .enumerate()
            .find(|(index, focus)| {
                !self.options[*index].disabled
                    && focus.as_ref().is_some_and(|focus| focus.is_focused(window))
            })
            .map(|(index, _)| index)
            .or_else(|| self.selected_index.filter(|&index| !self.options[index].disabled))
            .or_else(|| self.options.iter().position(|option| !option.disabled));

        div()
            .id(self.id.clone())
            .role(Role::RadioGroup)
            .flex()
            .flex_row()
            .items_center()
            .p(px(2.0))
            .gap(px(2.0))
            .bg(theme.neutral.hover)
            .rounded(px(theme.radius.md))
            .when(self.block, |s| s.w_full())
            .children(self.options.iter().enumerate().map(|(i, opt)| {
                let is_active = self.selected_index == Some(i);
                let label = opt.label.resolve(cx);
                let focus = self.focus[i]
                    .as_ref()
                    .expect("render initialized every option focus")
                    .clone()
                    .tab_stop(entry == Some(i));
                let disabled = opt.disabled;

                let option = div()
                    .id(&focus)
                    .role(Role::RadioButton)
                    .aria_label(label.clone())
                    .aria_toggled(is_active.into())
                    .when(disabled, |option| {
                        option.a11y_synthetic_children(|tree| tree.parent_node().set_disabled())
                    })
                    .when(!disabled, |option| {
                        option.track_focus(&focus).on_key_down(cx.listener({
                            let focus = focus.clone();
                            move |this, event: &gpui::KeyDownEvent, window, cx| {
                                if !event.keystroke.modifiers.modified() {
                                    this.move_focus(&focus, &event.keystroke.key, window, cx);
                                }
                            }
                        }))
                    })
                    .relative()
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
                    .when(!disabled, |s| {
                        s.cursor_pointer().on_click(cx.listener({
                            let focus = focus.clone();
                            move |this, _, window, cx| {
                                if let Some(index) = this
                                    .focus
                                    .iter()
                                    .position(|handle| handle.as_ref() == Some(&focus))
                                {
                                    this.select_option(index, window, cx);
                                }
                            }
                        }))
                    })
                    .child(
                        gpui::canvas(|_, _, _| (), {
                            let focus = focus.clone();
                            let color = theme.primary.base;
                            move |bounds, (), window, _| {
                                if !disabled && focus.is_focused(window)
                                    && window.last_input_was_keyboard()
                                {
                                    window.paint_quad(gpui::outline(
                                        bounds, color, gpui::BorderStyle::default(),
                                    ));
                                }
                            }
                        })
                        .absolute()
                        .inset_0(),
                    )
                    .child({
                        let label = div().text_sm().child(label);
                        if is_active {
                            pop_in("selected-label-motion", label).into_any_element()
                        } else {
                            label.into_any_element()
                        }
                    });

                #[cfg(test)]
                let option = option
                    .debug_selector(move || format!("segmented-option-{i}-{is_active}"));

                option
            }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{AppContext, Element, FocusHandle, Modifiers};
    use std::{cell::RefCell, rc::Rc};

    struct Surface {
        segmented: gpui::Entity<Segmented>,
        before: FocusHandle,
        after: FocusHandle,
    }

    impl Render for Surface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            crate::AppWindowFrame::new(
                "Segmented test",
                div()
                    .flex()
                    .flex_col()
                    .child(div().id("before").track_focus(&self.before).child("Before"))
                    .child(self.segmented.clone())
                    .child(div().id("after").track_focus(&self.after).child("After")),
            )
        }
    }

    fn surface(
        app: &mut gpui::TestAppContext,
        segmented: Segmented,
    ) -> (
        gpui::VisualTestContext,
        gpui::Entity<Segmented>,
        FocusHandle,
        FocusHandle,
    ) {
        app.update(crate::init_liora);
        let (segmented, before, after) = app.update(|cx| {
            (
                cx.new(|_| segmented),
                cx.focus_handle().tab_stop(true),
                cx.focus_handle().tab_stop(true),
            )
        });
        let control = segmented.clone();
        let before_focus = before.clone();
        let after_focus = after.clone();
        let root = app.add_window(move |_, _| Surface {
            segmented,
            before,
            after,
        });
        let mut window = gpui::VisualTestContext::from_window(root.into(), app);
        window.update(|window, cx| {
            window.draw(cx).clear(cx);
            window.focus(&before_focus, cx);
        });
        (window, control, before_focus, after_focus)
    }

    fn press(window: &mut gpui::VisualTestContext, key: &str) {
        window.simulate_keystrokes(key);
        window.update(|window, cx| {
            let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
            window.dispatch_event(
                gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                cx,
            );
            window.draw(cx).clear(cx);
        });
    }

    fn focused(window: &mut gpui::VisualTestContext) -> FocusHandle {
        window.update(|window, cx| window.focused(cx).expect("a rendered choice must hold focus"))
    }

    #[test]
    fn retained_segmented_exposes_native_radio_group() {
        let mut app = gpui::TestAppContext::single();
        let (mut window, control, _, _) = surface(
            &mut app,
            Segmented::new(vec![SegmentedOption::new("Light", "light")]),
        );
        window.update(|window, cx| {
            control.update(cx, |control, cx| {
                assert_eq!(
                    control.render(window, cx).into_element().a11y_role(),
                    Some(gpui::Role::RadioGroup),
                );
            });
        });
    }

    #[test]
    fn retained_segmented_keyboard_skips_disabled_and_keeps_focus_when_labels_change() {
        let mut app = gpui::TestAppContext::single();
        let calls = Rc::new(RefCell::new(Vec::<SharedString>::new()));
        let (mut window, control, before, after) = surface(
            &mut app,
            Segmented::new(vec![
                SegmentedOption::new("Light", "light"),
                SegmentedOption::new("Dark", "dark").disabled(true),
                SegmentedOption::new("System", "system"),
            ]),
        );
        window.update(|_, cx| {
            control.update(cx, |control, _| {
                let calls = calls.clone();
                control.set_on_change(move |value, _, _| calls.borrow_mut().push(value));
            });
        });
        press(&mut window, "tab");
        let light = focused(&mut window);
        assert_ne!(light, after, "Tab must enter the retained choice group");
        assert_ne!(light, before);
        press(&mut window, "right");
        let system = focused(&mut window);
        assert_ne!(system, light, "Right must skip the disabled middle choice");
        assert_eq!(calls.borrow().as_slice(), &[SharedString::from("system")]);
        press(&mut window, "space");
        press(&mut window, "enter");
        assert_eq!(calls.borrow().len(), 1, "reselecting must not notify");
        let disabled = window.debug_bounds("segmented-option-1-false").unwrap();
        window.simulate_click(disabled.center(), Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(calls.borrow().len(), 1, "disabled pointer choice must not notify");
        assert_eq!(focused(&mut window), system);
        press(&mut window, "right");
        assert_eq!(focused(&mut window), light, "Right must wrap to the first choice");
        press(&mut window, "left");
        assert_eq!(focused(&mut window), system);
        press(&mut window, "home");
        assert_eq!(focused(&mut window), light);
        press(&mut window, "end");
        assert_eq!(focused(&mut window), system);
        let changes = calls.borrow().len();
        press(&mut window, "ctrl-left");
        assert_eq!(focused(&mut window), system);
        assert_eq!(calls.borrow().len(), changes);
        window.update(|window, cx| {
            control.update(cx, |control, cx| {
                control.set_options(vec![
                    SegmentedOption::new("浅色", "light"),
                    SegmentedOption::new("深色", "dark").disabled(true),
                    SegmentedOption::new("跟随系统", "system"),
                ]);
                cx.notify();
            });
            window.draw(cx).clear(cx);
        });
        assert_eq!(focused(&mut window), system, "label replacement must retain focus");
        assert_eq!(calls.borrow().len(), changes);
        press(&mut window, "tab");
        assert_eq!(focused(&mut window), after, "one Tab must exit the group");
        press(&mut window, "shift-tab");
        assert_eq!(focused(&mut window), system, "reverse entry must use the selected choice");
        press(&mut window, "shift-tab");
        assert_eq!(focused(&mut window), before);
        let selected = window.debug_bounds("segmented-option-2-true").unwrap();
        window.simulate_click(selected.center(), Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(focused(&mut window), system, "clicking the selected choice must focus it");
        assert_eq!(calls.borrow().len(), changes);
    }

    #[test]
    fn retained_segmented_duplicate_values_still_select_exactly_one_choice() {
        let mut app = gpui::TestAppContext::single();
        let calls = Rc::new(RefCell::new(Vec::<SharedString>::new()));
        let changes = calls.clone();
        let (mut window, control, _, _) = surface(
            &mut app,
            Segmented::new(vec![
                SegmentedOption::new("First alias", "same"),
                SegmentedOption::new("Second alias", "same"),
            ])
            .on_change(move |value, _, _| changes.borrow_mut().push(value)),
        );
        assert!(window.debug_bounds("segmented-option-0-true").is_some());
        assert!(
            window.debug_bounds("segmented-option-1-false").is_some(),
            "equal serialized values must not mark two radio choices selected",
        );
        press(&mut window, "tab");
        press(&mut window, "right");
        assert!(window.debug_bounds("segmented-option-0-false").is_some());
        assert!(window.debug_bounds("segmented-option-1-true").is_some());
        assert!(calls.borrow().is_empty(), "an equal serialized value must not notify");
        press(&mut window, "space");
        assert!(calls.borrow().is_empty(), "reselecting the same choice must not notify");
        window.update(|window, cx| {
            control.update(cx, |control, cx| {
                control.set_options(vec![
                    SegmentedOption::new("第一个", "same"),
                    SegmentedOption::new("第二个", "same"),
                ]);
                cx.notify();
            });
            window.draw(cx).clear(cx);
        });
        assert!(window.debug_bounds("segmented-option-0-false").is_some());
        assert!(window.debug_bounds("segmented-option-1-true").is_some());
        assert!(calls.borrow().is_empty());
    }

    #[test]
    fn retained_segmented_reordering_preserves_value_occurrence_focus() {
        let mut app = gpui::TestAppContext::single();
        let calls = Rc::new(RefCell::new(Vec::<SharedString>::new()));
        let changes = calls.clone();
        let (mut window, control, _, _) = surface(
            &mut app,
            Segmented::new(vec![
                SegmentedOption::new("First alias", "same"),
                SegmentedOption::new("Other", "other"),
                SegmentedOption::new("Second alias", "same"),
            ])
            .on_change(move |value, _, _| changes.borrow_mut().push(value)),
        );
        press(&mut window, "tab");
        let first = focused(&mut window);
        press(&mut window, "end");
        let second = focused(&mut window);
        assert_ne!(first, second);
        assert!(calls.borrow().is_empty());
        window.update(|window, cx| {
            control.update(cx, |control, cx| {
                control.set_options(vec![
                    SegmentedOption::new("第一个", "same"),
                    SegmentedOption::new("第二个", "same"),
                    SegmentedOption::new("其他", "other"),
                ]);
                cx.notify();
            });
            window.draw(cx).clear(cx);
        });
        assert_eq!(focused(&mut window), second, "the second occurrence must keep its own focus");
        assert!(window.debug_bounds("segmented-option-1-true").is_some());
        assert!(window.debug_bounds("segmented-option-2-false").is_some());
        assert!(calls.borrow().is_empty());
        press(&mut window, "right");
        assert_ne!(focused(&mut window), second, "Right must now reach Other at its new position");
        assert_eq!(calls.borrow().as_slice(), &[SharedString::from("other")]);
        press(&mut window, "left");
        assert_eq!(focused(&mut window), second);
        assert_eq!(
            calls.borrow().as_slice(),
            &[SharedString::from("other"), SharedString::from("same")],
        );
        press(&mut window, "home");
        assert_eq!(focused(&mut window), first);
        assert_eq!(calls.borrow().len(), 2, "moving between equal-valued choices is not a value change");
    }

    #[test]
    fn retained_segmented_disabled_selection_is_not_an_interactive_tab_stop() {
        let mut app = gpui::TestAppContext::single();
        let calls = Rc::new(RefCell::new(Vec::<SharedString>::new()));
        let changes = calls.clone();
        let (mut window, _, _, after) = surface(
            &mut app,
            Segmented::new(vec![
                SegmentedOption::new("Disabled selected", "disabled").disabled(true),
                SegmentedOption::new("Available", "available"),
            ])
            .value("disabled")
            .on_change(move |value, _, _| changes.borrow_mut().push(value)),
        );
        assert!(window.debug_bounds("segmented-option-0-true").is_some());
        press(&mut window, "tab");
        assert_ne!(focused(&mut window), after);
        assert!(calls.borrow().is_empty(), "focus entry must not change the selected value");
        assert!(window.debug_bounds("segmented-option-0-true").is_some());
        press(&mut window, "space");
        assert_eq!(calls.borrow().as_slice(), &[SharedString::from("available")]);
        assert!(window.debug_bounds("segmented-option-0-false").is_some());
        assert!(window.debug_bounds("segmented-option-1-true").is_some());
        let disabled = window.debug_bounds("segmented-option-0-false").unwrap();
        window.simulate_click(disabled.center(), Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(calls.borrow().len(), 1);
        press(&mut window, "tab");
        assert_eq!(focused(&mut window), after);
    }

    #[test]
    fn retained_segmented_empty_and_all_disabled_groups_do_not_capture_tab() {
        for options in [
            vec![],
            vec![
                SegmentedOption::new("First", "first").disabled(true),
                SegmentedOption::new("Second", "second").disabled(true),
            ],
        ] {
            let mut app = gpui::TestAppContext::single();
            let (mut window, _, before, after) = surface(&mut app, Segmented::new(options));
            press(&mut window, "tab");
            assert_eq!(focused(&mut window), after);
            press(&mut window, "shift-tab");
            assert_eq!(focused(&mut window), before);
        }
    }
}
