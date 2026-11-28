//! Autocomplete module.
//!
//! This public module implements the Liora text input suggestion popup component with keyboard-aware selection. It keeps the reusable
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

use std::rc::Rc;

use crate::Input;
use gpui::{
    App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable, GlobalElementId,
    InspectorElementId, InteractiveElement, IntoElement, KeyBinding, LayoutId, MouseButton, Pixels,
    Render, ScrollHandle, SharedString, Style, Window, actions, prelude::*, px, relative,
};
use liora_core::{Config, push_passive_portal};
use liora_icons_lucide::IconName;

actions!(
    autocomplete,
    [
        #[doc = "Keyboard action that closes the active autocomplete popup."]
        AutocompleteClose,
        #[doc = "Keyboard action that moves to the previous autocomplete suggestion."]
        AutocompleteUp,
        #[doc = "Keyboard action that moves to the next autocomplete suggestion."]
        AutocompleteDown,
        #[doc = "Keyboard action that selects the active autocomplete suggestion."]
        AutocompleteSelect,
        #[doc = "Keyboard action that leaves autocomplete without selecting a suggestion."]
        AutocompleteTab
    ]
);

#[derive(Debug, Clone, PartialEq, Eq)]
/// Data model used by autocomplete item rendering.
pub struct AutocompleteItem {
    /// Machine-readable value represented by this item.
    pub value: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
}

impl AutocompleteItem {
    /// Creates `AutocompleteItem` initialized from the supplied value.
    pub fn new(value: impl Into<SharedString>) -> Self {
        let value = value.into();
        Self {
            label: value.clone(),
            value,
        }
    }

    /// Marks the autocomplete item as a labeled suggestion.
    pub fn labeled(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

/// Fluent native GPUI component for rendering Liora autocomplete.
pub struct Autocomplete {
    input: Entity<Input>,
    items: Vec<AutocompleteItem>,
    is_open: bool,
    disabled: bool,
    clearable: bool,
    suffix_icon: Option<IconName>,
    placeholder: SharedString,
    width: Option<Pixels>,
    max_suggestions: usize,
    trigger_on_focus: bool,
    last_bounds: Option<Bounds<Pixels>>,
    focus_handle: FocusHandle,
    focus_subscription: Option<gpui::Subscription>,
    on_select: Option<Rc<dyn Fn(AutocompleteItem, &mut Window, &mut App) + 'static>>,
    close_on_click_outside: bool,
    close_on_escape: bool,
    active_index: Option<usize>,
    suggestion_scroll: ScrollHandle,
}

impl Autocomplete {
    /// Creates `Autocomplete` that renders the supplied items collection.
    pub fn new(items: Vec<AutocompleteItem>, cx: &mut Context<Self>) -> Self {
        Self {
            input: cx.new(|cx| {
                Input::new("", cx)
                    .clearable(true)
                    .icon_suffix(IconName::Search)
            }),
            items,
            is_open: false,
            disabled: false,
            clearable: true,
            suffix_icon: Some(IconName::Search),
            placeholder: "Type to search".into(),
            width: Some(px(280.0)),
            max_suggestions: 8,
            trigger_on_focus: true,
            last_bounds: None,
            focus_handle: cx.focus_handle(),
            focus_subscription: None,
            on_select: None,
            close_on_click_outside: true,
            close_on_escape: true,
            active_index: None,
            suggestion_scroll: ScrollHandle::new(),
        }
    }

    /// Creates this value from values.
    pub fn from_values(values: Vec<impl Into<SharedString>>, cx: &mut Context<Self>) -> Self {
        Self::new(values.into_iter().map(AutocompleteItem::new).collect(), cx)
    }

    /// Uses the supplied placeholder text when the value is empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Toggles the disabled state and suppresses user interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Toggles whether the component renders a clear affordance.
    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    /// Sets the trailing icon shown inside the input-like control.
    pub fn suffix_icon(mut self, icon: IconName) -> Self {
        self.suffix_icon = Some(icon);
        self
    }

    /// Hides the trailing suffix icon.
    pub fn no_suffix_icon(mut self) -> Self {
        self.suffix_icon = None;
        self
    }

    /// Performs the suffix icon value operation used by this component.
    pub fn suffix_icon_value(&self) -> Option<IconName> {
        self.suffix_icon
    }

    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    /// Applies the predefined width lg sizing preset.
    pub fn width_lg(self) -> Self {
        self.width(px(320.0))
    }

    /// Limits how many suggestions are displayed in the popup.
    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max.max(1);
        self
    }

    /// Toggles whether focusing the field opens suggestions immediately.
    pub fn trigger_on_focus(mut self, trigger: bool) -> Self {
        self.trigger_on_focus = trigger;
        self
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Toggles whether the popup closes when click outside occurs.
    pub fn close_on_click_outside(mut self, close: bool) -> Self {
        self.close_on_click_outside = close;
        self
    }

    /// Registers GPUI key bindings required for keyboard interaction.
    pub fn register_key_bindings(cx: &mut App) {
        cx.bind_keys([
            KeyBinding::new("escape", AutocompleteClose, None),
            KeyBinding::new("up", AutocompleteUp, Some("AutocompleteSuggestions")),
            KeyBinding::new("down", AutocompleteDown, Some("AutocompleteSuggestions")),
            KeyBinding::new("enter", AutocompleteSelect, Some("AutocompleteSuggestions")),
            KeyBinding::new("tab", AutocompleteTab, Some("Autocomplete")),
            KeyBinding::new("shift-tab", AutocompleteTab, Some("Autocomplete")),
        ]);
    }

    fn close_on_escape_action(
        &mut self,
        _: &AutocompleteClose,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.close_on_escape && self.is_open {
            self.is_open = false;
            self.active_index = None;
            cx.notify();
        }
    }

    fn move_active_suggestion(&mut self, direction: isize, cx: &mut Context<Self>) {
        if !self.is_open || self.disabled {
            return;
        }
        let count = self.matching_items(cx).len();
        if count == 0 {
            self.active_index = None;
            return;
        }
        let index = match (self.active_index, direction < 0) {
            (Some(index), true) => index.saturating_sub(1),
            (Some(index), false) => (index + 1).min(count - 1),
            (None, true) => count - 1,
            (None, false) => 0,
        };
        self.active_index = Some(index);
        self.suggestion_scroll.scroll_to_item(index);
        cx.notify();
    }

    fn move_suggestion_up(&mut self, _: &AutocompleteUp, _: &mut Window, cx: &mut Context<Self>) {
        self.move_active_suggestion(-1, cx);
    }

    fn move_suggestion_down(
        &mut self,
        _: &AutocompleteDown,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.move_active_suggestion(1, cx);
    }

    fn select_active_suggestion(
        &mut self,
        _: &AutocompleteSelect,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_open || self.disabled {
            return;
        }
        let Some(item) = self
            .active_index
            .and_then(|index| self.matching_items(cx).get(index).cloned())
        else {
            return;
        };
        self.select_item(item, window, cx);
    }

    fn leave_without_selecting(
        &mut self,
        _: &AutocompleteTab,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_open = false;
        self.active_index = None;
        cx.notify();
        if window.modifiers().shift {
            window.focus_prev(cx);
        } else {
            window.focus_next(cx);
        }
    }

    /// Runs after filling the input and closing the popup, outside the entity update.
    pub fn on_select(
        mut self,
        cb: impl Fn(AutocompleteItem, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Rc::new(cb));
        self
    }

    /// Returns the serialized value used by forms, configuration, or persistence.
    pub fn value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    /// Updates the stored items value and keeps the existing component identity.
    pub fn set_items(&mut self, items: Vec<AutocompleteItem>, cx: &mut Context<Self>) {
        if self.items == items {
            return;
        }
        self.items = items;
        self.active_index = None;
        cx.notify();
    }

    /// Restricts asset selection to releases whose names match the supplied items for.
    pub fn matching_items_for(
        items: &[AutocompleteItem],
        query: &str,
        max: usize,
    ) -> Vec<AutocompleteItem> {
        let query = query.trim().to_lowercase();
        items
            .iter()
            .filter(|item| {
                query.is_empty()
                    || item.value.to_string().to_lowercase().contains(&query)
                    || item.label.to_string().to_lowercase().contains(&query)
            })
            .take(max.max(1))
            .cloned()
            .collect()
    }

    fn matching_items(&self, cx: &App) -> Vec<AutocompleteItem> {
        Self::matching_items_for(
            &self.items,
            self.input.read(cx).value().as_ref(),
            self.max_suggestions,
        )
    }

    fn select_item(&mut self, item: AutocompleteItem, window: &mut Window, cx: &mut Context<Self>) {
        self.input.update(cx, |input, cx| {
            input.set_value(item.value.clone(), cx);
        });
        self.is_open = false;
        self.active_index = None;
        cx.notify();
        if let Some(cb) = self.on_select.clone() {
            window.defer(cx, move |window, cx| cb(item, window, cx));
        }
    }
}

impl Focusable for Autocomplete {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct BoundsCapturer {
    autocomplete: Entity<Autocomplete>,
}

impl Element for BoundsCapturer {
    type RequestLayoutState = ();
    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, ()) {
        let mut style = Style::default();
        style.size.width = relative(1.0).into();
        style.size.height = relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.autocomplete.update(cx, |this, _| {
            this.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&GlobalElementId>,
        _: Option<&InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
}

impl Render for Autocomplete {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.weak_entity();
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        let clearable = self.clearable;
        let suffix_icon = self.suffix_icon;

        if self.focus_subscription.is_none() {
            let focus = self.input.read(cx).focus_handle(cx);
            self.focus_subscription = Some(cx.on_focus(&focus, window, |this, _, cx| {
                if this.trigger_on_focus && !this.disabled && !this.is_open {
                    this.is_open = true;
                    this.active_index = None;
                    cx.notify();
                }
            }));
        }

        self.input.update(cx, |input, _| {
            input.set_on_change({
                let entity = entity.clone();
                move |value, cx| {
                    let _ = entity.update(cx, |this, cx| {
                        this.is_open = !value.is_empty();
                        this.active_index = None;
                        cx.notify();
                    });
                }
            });
        });
        self.input.update(cx, |input, cx| {
            input.set_placeholder(placeholder, cx);
            input.set_disabled(disabled, cx);
            input.set_clearable(clearable && !disabled, cx);
            input.set_icon_suffix(suffix_icon, cx);
        });

        let matches = self.matching_items(cx);
        let composing = self.input.read(cx).is_composing();
        let suggestions_open = self.is_open && !disabled && !matches.is_empty() && !composing;
        if self.is_open && !disabled {
            let trigger_bounds = self.last_bounds;
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            let close_on_click_outside = self.close_on_click_outside;
            let active_index = self.active_index;
            let suggestion_scroll = self.suggestion_scroll.clone();
            push_passive_portal(
                move |_window, _cx| {
                    let (top, left, width) = trigger_bounds
                        .map(|b| (b.bottom() + px(4.0), b.left(), b.size.width))
                        .unwrap_or((px(120.0), px(120.0), px(280.0)));
                    let entity = entity.clone();
                    let theme = theme_portal.clone();
                    let mut panel = gpui::div()
                        .id(("autocomplete-suggestions", entity.entity_id()))
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width)
                        .max_h(px(240.0))
                        .overflow_y_scroll()
                        .track_scroll(&suggestion_scroll)
                        .bg(theme.neutral.card)
                        .rounded(px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow_lg();
                    panel = panel.when(close_on_click_outside, |panel| {
                        panel.on_mouse_down_out({
                            let entity = entity.clone();
                            move |_, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.is_open = false;
                                    this.active_index = None;
                                    cx.notify();
                                });
                            }
                        })
                    });

                    if matches.is_empty() {
                        panel = panel.child(
                            gpui::div()
                                .px(px(12.0))
                                .py(px(10.0))
                                .text_size(px(theme.font_size.sm))
                                .text_color(theme.neutral.text_3)
                                .child("No matching suggestions"),
                        );
                    } else {
                        panel = panel.children(matches.iter().enumerate().map(|(index, item)| {
                            let item = item.clone();
                            let entity = entity.clone();
                            let theme = theme.clone();
                            let item_row = gpui::div();
                            let active = active_index == Some(index);
                            #[cfg(test)]
                            let item_row = item_row
                                .debug_selector(move || format!("autocomplete-option-{index}"));
                            item_row
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_3()
                                .px(px(12.0))
                                .py(px(8.0))
                                .cursor_pointer()
                                .when(active, |row| row.bg(theme.primary.base.opacity(0.1)))
                                .hover(|s| s.cursor_pointer().bg(theme.neutral.hover))
                                .child(
                                    gpui::div()
                                        .text_size(px(theme.font_size.md))
                                        .text_color(theme.neutral.text_1)
                                        .child(item.label.clone()),
                                )
                                .child(
                                    gpui::div()
                                        .text_xs()
                                        .text_color(theme.neutral.text_3)
                                        .child(item.value.clone()),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    let item = item.clone();
                                    entity.update(cx, |this, cx| {
                                        this.select_item(item, window, cx);
                                    });
                                    cx.stop_propagation();
                                })
                        }));
                    }
                    #[cfg(test)]
                    let panel = panel.debug_selector(|| "autocomplete-suggestions".to_string());
                    panel.into_any_element()
                },
                cx,
            );
        }

        let frame = gpui::div()
            .relative()
            .when(!composing, |frame| frame.key_context("Autocomplete"))
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w_full())
            .child(
                gpui::div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        autocomplete: cx.entity().clone(),
                    }),
            )
            .child(
                gpui::div()
                    .when(suggestions_open, |input| {
                        input.key_context("AutocompleteSuggestions")
                    })
                    .child(self.input.clone()),
            );

        frame
            .on_action(cx.listener(Self::close_on_escape_action))
            .on_action(cx.listener(Self::move_suggestion_up))
            .on_action(cx.listener(Self::move_suggestion_down))
            .on_action(cx.listener(Self::select_active_suggestion))
            .on_action(cx.listener(Self::leave_without_selecting))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct LifetimeTestSurface(Entity<Autocomplete>);

    impl Render for LifetimeTestSurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            gpui::div().size_full().child(self.0.clone())
        }
    }

    #[test]
    fn autocomplete_releases_after_window_removal() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let autocomplete = app.new(|cx| Autocomplete::from_values(vec!["rust"], cx));
        let weak = autocomplete.downgrade();
        let handle = app.add_window(move |_, _| LifetimeTestSurface(autocomplete));
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(
            weak.read_with(&app, |this, _| this.last_bounds.is_some())
                .unwrap()
        );

        window.update(|window, _| window.remove_window());
        drop(window);
        app.run_until_parked();

        assert!(
            weak.upgrade().is_none(),
            "Autocomplete retained after window removal"
        );
        app.quit();
    }

    struct PopupPolicySurface {
        autocomplete: Entity<Autocomplete>,
        neighbor_clicks: std::rc::Rc<std::cell::Cell<usize>>,
    }

    impl Render for PopupPolicySurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let clicks = self.neighbor_clicks.clone();
            gpui::div()
                .relative()
                .size_full()
                .child(self.autocomplete.clone())
                .child(
                    gpui::div()
                        .absolute()
                        .left(px(320.0))
                        .top_0()
                        .w(px(120.0))
                        .h(px(32.0))
                        .on_mouse_down(MouseButton::Left, move |_, _, _| {
                            clicks.set(clicks.get() + 1);
                        }),
                )
                .child(crate::PortalLayer)
        }
    }

    struct AutocompleteKeyboardSurface {
        controls: Vec<Entity<Autocomplete>>,
        after: Option<FocusHandle>,
    }

    impl Render for AutocompleteKeyboardSurface {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let mut surface = gpui::div().relative().size_full();
            for (index, autocomplete) in self.controls.iter().enumerate() {
                surface = surface.child(
                    gpui::div()
                        .absolute()
                        .left(px(index as f32 * 320.0))
                        .top_0()
                        .child(autocomplete.clone()),
                );
            }
            if let Some(after) = &self.after {
                surface = surface.child(
                    gpui::div()
                        .absolute()
                        .left(px(self.controls.len() as f32 * 320.0))
                        .top_0()
                        .w(px(80.0))
                        .h(px(34.0))
                        .track_focus(after),
                );
            }
            surface.child(crate::PortalLayer)
        }
    }

    #[test]
    fn trigger_on_focus_opens_only_enabled_opted_in_inputs() {
        for (trigger, disabled) in [(None, false), (Some(false), false), (Some(true), true)] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let autocomplete = app.new(|cx| {
                let autocomplete =
                    Autocomplete::from_values(vec!["rust", "gpui"], cx).disabled(disabled);
                match trigger {
                    Some(trigger) => autocomplete.trigger_on_focus(trigger),
                    None => autocomplete,
                }
            });
            let root = autocomplete.clone();
            let handle = app.add_window(move |_, _| PopupPolicySurface {
                autocomplete: root,
                neighbor_clicks: std::rc::Rc::new(std::cell::Cell::new(0)),
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            window.update(|window, _| window.activate_window());
            window.run_until_parked();
            window.update(|window, cx| window.draw(cx).clear(cx));
            window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
            assert_eq!(
                autocomplete.read_with(&app, |this, _| this.is_open),
                trigger.unwrap_or(true) && !disabled,
                "focus policy mismatch: trigger={trigger:?}, disabled={disabled}"
            );
            window.simulate_keystrokes("escape");
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(!autocomplete.read_with(&app, |this, _| this.is_open));
            window.update(|window, cx| window.blur(cx));
            window.update(|window, cx| window.draw(cx).clear(cx));
            window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
            assert_eq!(
                autocomplete.read_with(&app, |this, _| this.is_open),
                trigger.unwrap_or(true) && !disabled,
                "refocusing must preserve the policy"
            );
            window.simulate_keystrokes("r");
            assert_eq!(
                autocomplete.read_with(&app, |this, _| this.is_open),
                !disabled,
                "opting out of focus opening must not disable query opening"
            );
            assert_eq!(
                autocomplete.read_with(&app, |this, cx| this.value(cx)),
                if disabled { "" } else { "r" }
            );
            app.quit();
        }
    }

    #[test]
    fn selection_callback_can_read_and_update_its_autocomplete() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let selections = std::rc::Rc::new(std::cell::Cell::new(0));
        let selected = selections.clone();
        let autocomplete = app.new(|cx| {
            let owner = cx.weak_entity();
            Autocomplete::from_values(vec!["rust", "gpui"], cx).on_select(move |item, _, cx| {
                owner
                    .update(cx, |this, cx| {
                        assert_eq!(this.value(cx), item.value);
                        assert!(!this.is_open);
                        this.set_items(vec![AutocompleteItem::new("replacement")], cx);
                    })
                    .unwrap();
                selected.set(selected.get() + 1);
            })
        });
        let root = autocomplete.clone();
        let handle = app.add_window(move |_, _| PopupPolicySurface {
            autocomplete: root,
            neighbor_clicks: std::rc::Rc::new(std::cell::Cell::new(0)),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, _| window.activate_window());
        window.run_until_parked();
        window.update(|window, cx| window.draw(cx).clear(cx));
        window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
        window.simulate_keystrokes("r");
        window.update(|window, cx| window.draw(cx).clear(cx));
        let bounds = autocomplete.read_with(&app, |this, _| this.last_bounds.unwrap());
        window.simulate_click(
            gpui::point(bounds.left() + px(20.0), bounds.bottom() + px(20.0)),
            gpui::Modifiers::none(),
        );
        assert_eq!(selections.get(), 1);
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.items.clone()),
            vec![AutocompleteItem::new("replacement")]
        );
        app.quit();
    }

    #[test]
    fn dispatched_keys_select_and_scroll_the_active_suggestion_into_view() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let selections = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let selected = selections.clone();
        let autocomplete = app.new(|cx| {
            Autocomplete::new(
                (0..12)
                    .map(|index| AutocompleteItem::new(format!("item-{index}")))
                    .collect(),
                cx,
            )
            .max_suggestions(12)
            .on_select(move |item, _, _| selected.borrow_mut().push(item.value.to_string()))
        });
        let root = autocomplete.clone();
        let handle = app.add_window(move |_, _| PopupPolicySurface {
            autocomplete: root,
            neighbor_clicks: std::rc::Rc::new(std::cell::Cell::new(0)),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, _| window.activate_window());
        window.run_until_parked();
        window.update(|window, cx| window.draw(cx).clear(cx));
        window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));

        for _ in 0..9 {
            window.simulate_keystrokes("down");
            window.update(|window, cx| window.draw(cx).clear(cx));
        }

        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            Some(8)
        );
        for key in ["ctrl-down", "shift-down"] {
            window.simulate_keystrokes(key);
            window.update(|window, cx| window.draw(cx).clear(cx));
        }
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            Some(8),
            "modified arrows must not change the active suggestion"
        );
        let input_focus =
            autocomplete.read_with(&app, |this, cx| this.input.read(cx).focus_handle(cx));
        assert_eq!(
            window.update(|window, cx| window.focused(cx)),
            Some(input_focus)
        );

        let viewport = window.debug_bounds("autocomplete-suggestions").unwrap();
        let active_row = window.debug_bounds("autocomplete-option-8").unwrap();
        assert!(
            active_row.top() >= viewport.top() && active_row.bottom() <= viewport.bottom(),
            "active suggestion must remain visible in the bounded popup: {active_row:?} outside {viewport:?}"
        );

        window.simulate_keystrokes("enter");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            autocomplete.read_with(&app, |this, cx| this.value(cx)),
            "item-8"
        );
        assert_eq!(*selections.borrow(), ["item-8"]);
        assert!(!autocomplete.read_with(&app, |this, _| this.is_open));
        app.quit();
    }

    #[test]
    fn escape_and_tab_clear_highlight_without_committing() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let selections = std::rc::Rc::new(std::cell::Cell::new(0));
        let selected = selections.clone();
        let autocomplete = app.new(|cx| {
            Autocomplete::from_values(vec!["rust", "go"], cx)
                .on_select(move |_, _, _| selected.set(selected.get() + 1))
        });
        let after = app.update(|cx| cx.focus_handle().tab_stop(true));
        let after_for_window = after.clone();
        let root = autocomplete.clone();
        let handle = app.add_window(move |_, _| AutocompleteKeyboardSurface {
            controls: vec![root],
            after: Some(after_for_window.clone()),
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, _| window.activate_window());
        window.run_until_parked();
        window.update(|window, cx| window.draw(cx).clear(cx));
        window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        let input_focus =
            autocomplete.read_with(&app, |this, cx| this.input.read(cx).focus_handle(cx));

        window.simulate_keystrokes("down");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            Some(0)
        );
        window.simulate_keystrokes("escape");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(!autocomplete.read_with(&app, |this, _| this.is_open));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            None
        );
        assert_eq!(
            window.update(|window, cx| window.focused(cx)),
            Some(input_focus)
        );
        assert_eq!(selections.get(), 0);

        window.simulate_keystrokes("g");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(autocomplete.read_with(&app, |this, _| this.is_open));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            None
        );
        window.simulate_keystrokes("down");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            Some(0)
        );

        app.update(|cx| {
            autocomplete.update(cx, |this, cx| {
                this.set_items(vec![AutocompleteItem::new("gamma")], cx);
            });
        });
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            None
        );
        window.simulate_keystrokes("down");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            Some(0)
        );
        window.simulate_keystrokes("tab");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(!autocomplete.read_with(&app, |this, _| this.is_open));
        assert_eq!(
            autocomplete.read_with(&app, |this, _| this.active_index),
            None
        );
        assert_eq!(autocomplete.read_with(&app, |this, cx| this.value(cx)), "g");
        assert_eq!(selections.get(), 0);
        assert_eq!(window.update(|window, cx| window.focused(cx)), Some(after));
        app.quit();
    }

    #[test]
    fn composing_input_and_sibling_autocompletes_keep_navigation_isolated() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);
        let first = app.new(|cx| {
            Autocomplete::from_values(vec!["rust", "go"], cx).close_on_click_outside(false)
        });
        let second = app.new(|cx| {
            Autocomplete::from_values(vec!["rust", "go"], cx).close_on_click_outside(false)
        });
        let root_first = first.clone();
        let root_second = second.clone();
        let handle = app.add_window(move |_, _| AutocompleteKeyboardSurface {
            controls: vec![root_first, root_second],
            after: None,
        });
        let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
        window.update(|window, _| window.activate_window());
        window.run_until_parked();
        window.update(|window, cx| window.draw(cx).clear(cx));
        window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        for _ in 0..2 {
            window.simulate_keystrokes("down");
            window.update(|window, cx| window.draw(cx).clear(cx));
        }
        assert_eq!(first.read_with(&app, |this, _| this.active_index), Some(1));

        window.simulate_click(gpui::point(px(332.0), px(12.0)), gpui::Modifiers::none());
        window.update(|window, cx| window.draw(cx).clear(cx));
        window.simulate_keystrokes("down");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(first.read_with(&app, |this, _| this.active_index), Some(1));
        assert_eq!(second.read_with(&app, |this, _| this.active_index), Some(0));

        let input = second.read_with(&app, |this, _| this.input.clone());
        window.update(|window, cx| {
            input.update(cx, |input, cx| {
                gpui::EntityInputHandler::replace_and_mark_text_in_range(
                    input,
                    None,
                    "r",
                    Some(1..1),
                    window,
                    cx,
                );
            });
        });
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert!(input.read_with(&app, |input, _| input.is_composing()));
        assert_eq!(second.read_with(&app, |this, _| this.active_index), None);
        window.simulate_keystrokes("down");
        window.update(|window, cx| window.draw(cx).clear(cx));
        assert_eq!(second.read_with(&app, |this, _| this.active_index), None);
        assert_eq!(first.read_with(&app, |this, _| this.active_index), Some(1));
        app.quit();
    }
    #[test]
    fn popup_policy_keeps_neighbors_clickable_and_suggestions_selectable() {
        for close_outside in [false, true] {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let selections = std::rc::Rc::new(std::cell::Cell::new(0));
            let selected = selections.clone();
            let autocomplete = app.new(|cx| {
                Autocomplete::from_values(vec!["rust", "gpui"], cx)
                    .trigger_on_focus(false)
                    .close_on_click_outside(close_outside)
                    .close_on_escape(false)
                    .on_select(move |item, _, _| {
                        assert_eq!(item.value, "rust");
                        selected.set(selected.get() + 1);
                    })
            });
            let neighbor_clicks = std::rc::Rc::new(std::cell::Cell::new(0));
            let root = autocomplete.clone();
            let clicks = neighbor_clicks.clone();
            let handle = app.add_window(move |_, _| PopupPolicySurface {
                autocomplete: root,
                neighbor_clicks: clicks,
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            window.update(|window, _| window.activate_window());
            window.run_until_parked();
            window.update(|window, cx| window.draw(cx).clear(cx));
            window.simulate_click(gpui::point(px(12.0), px(12.0)), gpui::Modifiers::none());
            window.simulate_keystrokes("r");
            window.update(|window, cx| window.draw(cx).clear(cx));
            assert!(autocomplete.read_with(&app, |this, _| this.is_open));
            window.simulate_keystrokes("escape");
            assert!(autocomplete.read_with(&app, |this, _| this.is_open));

            window.simulate_click(gpui::point(px(340.0), px(16.0)), gpui::Modifiers::none());
            assert_eq!(
                neighbor_clicks.get(),
                1,
                "open popup blocked a neighboring control"
            );
            assert_eq!(
                autocomplete.read_with(&app, |this, _| this.is_open),
                !close_outside
            );
            window.update(|window, cx| window.draw(cx).clear(cx));

            if close_outside {
                window.simulate_keystrokes("u");
                window.update(|window, cx| window.draw(cx).clear(cx));
            }
            let bounds = autocomplete.read_with(&app, |this, _| this.last_bounds.unwrap());
            window.simulate_click(
                gpui::point(bounds.left() + px(20.0), bounds.bottom() + px(20.0)),
                gpui::Modifiers::none(),
            );
            assert_eq!(
                autocomplete.read_with(&app, |this, cx| this.value(cx)),
                "rust"
            );
            assert_eq!(
                selections.get(),
                1,
                "suggestion must receive the native click"
            );
            assert!(!autocomplete.read_with(&app, |this, _| this.is_open));
            app.quit();
        }
    }
}
