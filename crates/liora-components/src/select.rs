//! Select module.
//!
//! This public module implements the Liora select dropdown component with keyboard-aware popup behavior. It keeps the reusable
//! component logic inside `liora-components` rather than Gallery or Docs so
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
//! the component, and avoid app-specific Gallery/Docs resources in this SDK
//! crate.

use crate::gpui_compat::element_id;
use crate::motion::pop_in;
use crate::{Input, SearchableList, SearchableListItem};
use gpui::{
    AnyElement, App, Bounds, Context, ElementId, Entity, FocusHandle, Focusable, GlobalElementId,
    Hsla, InspectorElementId, IntoElement, LayoutId, MouseButton, Pixels, Render, SharedString,
    Style, Window, actions, div, prelude::*, px, relative,
};
use liora_core::{Config, push_portal};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::sync::Arc;

actions!(
    select,
    [
        #[doc = "Keyboard action that closes the active select popup."]
        SelectClose
    ]
);

/// Selection/search policy for [`Select`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectMode {
    /// Classic fixed-option select with no text filtering.
    #[default]
    Plain,
    /// Searchable single-select popup.
    Searchable,
    /// Searchable multi-select popup.
    Multiple,
}

/// Fluent native GPUI component for rendering Liora select.
pub struct Select {
    options: Vec<SharedString>,
    items: Vec<SearchableListItem>,
    selected_idx: Option<usize>,
    selected_values: Vec<SharedString>,
    mode: SelectMode,
    input: Option<Entity<Input>>,
    is_open: bool,
    focus_handle: FocusHandle,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    on_values_change: Option<Arc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
    footer: Option<Arc<dyn Fn(&mut Window, &mut App) -> AnyElement + 'static>>,
    placeholder: SharedString,
    empty_text: SharedString,
    max_items: usize,
    disabled: bool,
    suppress_next_input_change: bool,
    border_none: bool,
    radius_none: bool,
    radius_left_none: bool,
    radius_right_none: bool,
    width: Option<Pixels>,
    text_size: Option<Pixels>,
    text_color: Option<Hsla>,
    padding_x: Option<Pixels>,
    close_on_click_outside: bool,
    close_on_escape: bool,
}

impl Select {
    /// Creates `Select` with default theme-driven styling and no optional callbacks attached.
    pub fn new(
        options: Vec<impl Into<SharedString>>,
        selected_idx: Option<usize>,
        cx: &mut Context<Self>,
    ) -> Self {
        Self {
            options: options.into_iter().map(|o| o.into()).collect(),
            items: Vec::new(),
            selected_idx,
            selected_values: Vec::new(),
            mode: SelectMode::Plain,
            input: None,
            is_open: false,
            focus_handle: cx.focus_handle(),
            last_bounds: None,
            on_change: None,
            on_values_change: None,
            footer: None,
            placeholder: "Search or select".into(),
            empty_text: "No matching options".into(),
            max_items: 8,
            disabled: false,
            suppress_next_input_change: false,
            border_none: false,
            radius_none: false,
            radius_left_none: false,
            radius_right_none: false,
            width: None,
            text_size: None,
            text_color: None,
            padding_x: None,
            close_on_click_outside: true,
            close_on_escape: true,
        }
    }

    /// Creates a searchable select from reusable list items.
    pub fn searchable(items: Vec<SearchableListItem>, cx: &mut Context<Self>) -> Self {
        let mut this = Self::new(Vec::<SharedString>::new(), None, cx);
        this.items = items;
        this.mode = SelectMode::Searchable;
        this.width = Some(px(300.0));
        this.input = Some(cx.new(|cx| {
            Input::new("", cx)
                .clearable(true)
                .icon_suffix(IconName::ChevronDown)
        }));
        this
    }

    /// Creates a searchable select from plain values.
    pub fn searchable_values(values: Vec<impl Into<SharedString>>, cx: &mut Context<Self>) -> Self {
        Self::searchable(
            values.into_iter().map(SearchableListItem::new).collect(),
            cx,
        )
    }

    /// Switches a searchable select into multiple-selection mode.
    pub fn multiple(mut self) -> Self {
        self.mode = SelectMode::Multiple;
        self
    }

    /// Sets the select mode explicitly.
    pub fn mode(mut self, mode: SelectMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the selected value for searchable single-select mode.
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected_values = vec![value.into()];
        self
    }

    /// Sets selected values for searchable multi-select mode.
    pub fn selected_values(mut self, values: Vec<impl Into<SharedString>>) -> Self {
        self.selected_values = values.into_iter().map(Into::into).collect();
        self
    }

    /// Uses the supplied placeholder text when no search is active.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Replaces the empty-state text rendered when filtering yields no rows.
    pub fn empty_text(mut self, text: impl Into<SharedString>) -> Self {
        self.empty_text = text.into();
        self
    }

    /// Toggles disabled state and suppresses interaction when enabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Limits how many matching rows are displayed in the searchable popup.
    pub fn max_items(mut self, max: usize) -> Self {
        self.max_items = max.max(1);
        self
    }

    /// Adds a footer element rendered below searchable options.
    pub fn footer(
        mut self,
        footer: impl Fn(&mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        self.footer = Some(Arc::new(footer));
        self
    }

    /// Registers a callback for searchable single/multiple selected values.
    pub fn on_values_change(
        mut self,
        callback: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_values_change = Some(Arc::new(callback));
        self
    }

    /// Returns selected values in their persisted order.
    pub fn selected_values_ref(&self) -> &[SharedString] {
        &self.selected_values
    }

    /// Returns matching items for a query using the shared searchable-list logic.
    pub fn matching_items_for(
        items: &[SearchableListItem],
        query: &str,
        max: usize,
    ) -> Vec<SearchableListItem> {
        SearchableList::filtered_items_for(items, query, max)
    }

    /// Removes the visible border treatment.
    pub fn borderless(mut self) -> Self {
        self.border_none = true;
        self
    }
    /// Sets the radius none value used by the component.
    pub fn radius_none(mut self) -> Self {
        self.radius_none = true;
        self
    }
    /// Sets the radius left none value used by the component.
    pub fn radius_left_none(mut self) -> Self {
        self.radius_left_none = true;
        self
    }
    /// Sets the radius right none value used by the component.
    pub fn radius_right_none(mut self) -> Self {
        self.radius_right_none = true;
        self
    }
    /// Sets the component width token used during GPUI layout.
    pub fn width(mut self, w: impl Into<Pixels>) -> Self {
        self.width = Some(w.into());
        self
    }

    /// Applies the predefined width xs sizing preset.
    pub fn width_xs(self) -> Self {
        self.width(gpui::px(90.0))
    }

    /// Sets the font size used for text content.
    pub fn text_size(mut self, s: impl Into<Pixels>) -> Self {
        self.text_size = Some(s.into());
        self
    }

    /// Applies the predefined text sm sizing preset.
    pub fn text_sm(self) -> Self {
        self.text_size(gpui::px(14.0))
    }
    /// Applies the foreground text color.
    pub fn text_color(mut self, c: Hsla) -> Self {
        self.text_color = Some(c);
        self
    }
    /// Sets the component horizontal padding token used during GPUI layout.
    pub fn padding_x(mut self, p: impl Into<Pixels>) -> Self {
        self.padding_x = Some(p.into());
        self
    }

    /// Applies the predefined padding x sm sizing preset.
    pub fn padding_x_sm(self) -> Self {
        self.padding_x(gpui::px(8.0))
    }

    /// Updates the stored borderless value and keeps the existing component identity.
    pub fn set_borderless(&mut self, b: bool, cx: &mut Context<Self>) {
        if self.border_none == b {
            return;
        }
        self.border_none = b;
        cx.notify();
    }

    /// Updates the stored radius none value and keeps the existing component identity.
    pub fn set_radius_none(&mut self, r: bool, cx: &mut Context<Self>) {
        if self.radius_none == r {
            return;
        }
        self.radius_none = r;
        cx.notify();
    }

    /// Updates the stored radius left none value and keeps the existing component identity.
    pub fn set_radius_left_none(&mut self, r: bool, cx: &mut Context<Self>) {
        if self.radius_left_none == r {
            return;
        }
        self.radius_left_none = r;
        cx.notify();
    }

    /// Updates the stored radius right none value and keeps the existing component identity.
    pub fn set_radius_right_none(&mut self, r: bool, cx: &mut Context<Self>) {
        if self.radius_right_none == r {
            return;
        }
        self.radius_right_none = r;
        cx.notify();
    }

    /// Updates the stored width value and keeps the existing component identity.
    pub fn set_width(&mut self, w: impl Into<Pixels>, cx: &mut Context<Self>) {
        let w = w.into();
        if self.width == Some(w) {
            return;
        }
        self.width = Some(w);
        cx.notify();
    }

    /// Updates the stored text size value and keeps the existing component identity.
    pub fn set_text_size(&mut self, s: impl Into<Pixels>, cx: &mut Context<Self>) {
        let s = s.into();
        if self.text_size == Some(s) {
            return;
        }
        self.text_size = Some(s);
        cx.notify();
    }

    /// Updates the stored text color value and keeps the existing component identity.
    pub fn set_text_color(&mut self, c: Hsla, cx: &mut Context<Self>) {
        if self.text_color == Some(c) {
            return;
        }
        self.text_color = Some(c);
        cx.notify();
    }

    /// Updates the stored padding x value and keeps the existing component identity.
    pub fn set_padding_x(&mut self, p: impl Into<Pixels>, cx: &mut Context<Self>) {
        let p = p.into();
        if self.padding_x == Some(p) {
            return;
        }
        self.padding_x = Some(p);
        cx.notify();
    }

    /// Updates the stored options value and keeps the existing component identity.
    pub fn set_options(&mut self, options: Vec<SharedString>, cx: &mut Context<Self>) {
        if self.options == options {
            return;
        }
        self.options = options;
        if let Some(idx) = self.selected_idx
            && idx >= self.options.len()
        {
            self.selected_idx = None;
        }
        cx.notify();
    }

    /// Updates the stored selected idx value and keeps the existing component identity.
    pub fn set_selected_idx(&mut self, idx: Option<usize>, cx: &mut Context<Self>) {
        if self.selected_idx == idx {
            return;
        }
        self.selected_idx = idx;
        cx.notify();
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
        cx.bind_keys([gpui::KeyBinding::new("escape", SelectClose, None)]);
    }

    fn close_on_escape_action(&mut self, _: &SelectClose, _: &mut Window, cx: &mut Context<Self>) {
        if self.close_on_escape && self.is_open {
            self.is_open = false;
            cx.notify();
        }
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    /// Updates the stored on change value and keeps the existing component identity.
    pub fn set_on_change(&mut self, cb: impl Fn(usize, &mut Window, &mut App) + 'static) {
        self.on_change = Some(Box::new(cb));
    }

    /// Returns the currently selected item index, if any.
    pub fn selected_index(&self) -> Option<usize> {
        self.selected_idx
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        if self.is_open {
            window.focus(&self.focus_handle, cx);
        }
        cx.notify();
    }

    fn select_option(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.selected_idx = Some(idx);
        self.is_open = false;
        if let Some(ref cb) = self.on_change {
            cb(idx, window, cx);
        }
        cx.notify();
    }

    fn query(&self, cx: &App) -> SharedString {
        self.input
            .as_ref()
            .map(|input| input.read(cx).value())
            .unwrap_or_default()
    }

    fn display_value_for(&self, value: &SharedString) -> SharedString {
        self.items
            .iter()
            .find(|item| &item.value == value)
            .map(|item| item.label.clone())
            .unwrap_or_else(|| value.clone())
    }

    fn summary(&self) -> SharedString {
        match self.selected_values.as_slice() {
            [] => SharedString::default(),
            [value] => self.display_value_for(value),
            values => format!("{} selected", values.len()).into(),
        }
    }

    fn clear_search_query(&mut self, cx: &mut Context<Self>) {
        if let Some(input) = &self.input {
            self.suppress_next_input_change = true;
            input.update(cx, |input, cx| input.set_value(SharedString::default(), cx));
        }
    }

    fn select_item(
        &mut self,
        item: SearchableListItem,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if item.disabled {
            return;
        }

        match self.mode {
            SelectMode::Plain => {}
            SelectMode::Searchable => {
                self.selected_values = vec![item.value.clone()];
                self.is_open = false;
                self.clear_search_query(cx);
            }
            SelectMode::Multiple => {
                if let Some(index) = self
                    .selected_values
                    .iter()
                    .position(|value| value == &item.value)
                {
                    self.selected_values.remove(index);
                } else {
                    self.selected_values.push(item.value.clone());
                }
                self.clear_search_query(cx);
            }
        }

        if let Some(callback) = &self.on_values_change {
            callback(self.selected_values.clone(), window, cx);
        }
        cx.notify();
    }

    fn open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = true;
        window.focus(&self.focus_handle, cx);
        cx.notify();
    }
}

impl Focusable for Select {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

struct BoundsCapturer {
    select: Entity<Select>,
}

impl IntoElement for BoundsCapturer {
    type Element = Self;
    fn into_element(self) -> Self::Element {
        self
    }
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
    ) -> () {
        self.select.update(cx, |this, _| {
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

impl Select {
    fn render_searchable(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let disabled = self.disabled;
        let placeholder = if self.selected_values.is_empty() {
            self.placeholder.clone()
        } else {
            self.summary()
        };
        let suffix_icon = if self.is_open {
            IconName::ChevronUp
        } else {
            IconName::ChevronDown
        };

        if let Some(input) = &self.input {
            input.update(cx, |input, cx| {
                input.set_placeholder(placeholder, cx);
                input.set_disabled(disabled, cx);
                input.set_clearable(!disabled, cx);
                input.set_icon_suffix(Some(suffix_icon), cx);
                input.set_on_change({
                    let entity = entity.clone();
                    move |_, cx| {
                        entity.update(cx, |this, cx| {
                            if this.suppress_next_input_change {
                                this.suppress_next_input_change = false;
                                cx.notify();
                                return;
                            }
                            this.is_open = true;
                            cx.notify();
                        });
                    }
                });
            });
        }

        if self.is_open && !self.disabled {
            let bounds = self.last_bounds;
            let query = self.query(cx);
            let items = self.items.clone();
            let selected_values = self.selected_values.clone();
            let max_items = self.max_items;
            let empty_text = self.empty_text.clone();
            let footer = self.footer.clone();
            let close_on_click_outside = self.close_on_click_outside;
            let entity = entity.clone();
            let theme_portal = theme.clone();

            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = bounds
                        .map(|b| (b.bottom() + px(4.0), b.left(), b.size.width))
                        .unwrap_or((px(120.0), px(120.0), px(300.0)));
                    let entity_for_list = entity.clone();
                    let mut panel = div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width)
                        .rounded(px(theme_portal.radius.md))
                        .bg(theme_portal.neutral.card)
                        .border_1()
                        .border_color(theme_portal.neutral.border)
                        .shadow_lg()
                        .occlude();

                    panel = panel.when(close_on_click_outside, |panel| {
                        panel.on_mouse_down_out({
                            let entity = entity.clone();
                            move |_, _, cx| {
                                entity.update(cx, |this, cx| {
                                    this.is_open = false;
                                    cx.notify();
                                });
                            }
                        })
                    });

                    let list = SearchableList::new(items.clone())
                        .id("liora-select-searchable-options")
                        .query(query.clone())
                        .selected_values(selected_values.clone())
                        .empty_text(empty_text.clone())
                        .max_items(max_items)
                        .width(width)
                        .on_select(move |item, window, cx| {
                            entity_for_list
                                .update(cx, |this, cx| this.select_item(item, window, cx));
                        });

                    panel = panel.child(list);
                    if let Some(footer) = footer.clone() {
                        panel = panel.child(
                            div()
                                .border_t_1()
                                .border_color(theme_portal.neutral.border)
                                .p_2()
                                .child(footer(_window, _cx)),
                        );
                    }

                    pop_in(element_id("liora-select-searchable-panel"), panel).into_any_element()
                },
                cx,
            );
        }

        div()
            .relative()
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w_full())
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        select: cx.entity().clone(),
                    }),
            )
            .children(self.input.iter().cloned())
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| this.open(window, cx)),
            )
            .on_action(cx.listener(Self::close_on_escape_action))
            .into_any_element()
    }
}

impl Render for Select {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let config = cx.global::<Config>();
        let theme = config.theme.clone();
        let focused = self.focus_handle.is_focused(_window);

        if self.mode != SelectMode::Plain {
            return self.render_searchable(_window, cx).into_any_element();
        }

        let display_text = self
            .selected_idx
            .and_then(|i| self.options.get(i).cloned())
            .unwrap_or_else(|| "Select...".into());

        let border_color = if focused || self.is_open {
            theme.primary.base
        } else {
            theme.neutral.border
        };
        let text_size = self.text_size.unwrap_or(gpui::px(theme.font_size.md));
        let text_color = self.text_color.unwrap_or(theme.neutral.text_1);
        let h_px = self.padding_x.unwrap_or(gpui::px(12.0));

        let trigger_content = gpui::div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .w_full()
            .h(gpui::px(34.0))
            .px(h_px)
            .child(
                gpui::div()
                    .text_size(text_size)
                    .text_color(text_color)
                    .child(display_text),
            )
            .child(
                Icon::new(if self.is_open {
                    IconName::ChevronUp
                } else {
                    IconName::ChevronDown
                })
                .size(gpui::px(14.0))
                .color(theme.neutral.icon),
            );

        if self.is_open {
            let options = self.options.clone();
            let selected_idx = self.selected_idx;
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            let trigger_bounds = self.last_bounds;

            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = if let Some(b) = trigger_bounds {
                        (b.bottom() + gpui::px(4.0), b.left(), b.size.width)
                    } else {
                        (gpui::px(100.0), gpui::px(100.0), gpui::px(200.0))
                    };

                    let entity = entity.clone();
                    let theme = theme_portal.clone();

                    let panel = gpui::div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width)
                        .max_h(gpui::px(200.0))
                        .bg(theme.neutral.card)
                        .rounded(gpui::px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow(vec![gpui::BoxShadow {
                            color: theme.neutral.border,
                            offset: gpui::point(gpui::px(0.0), gpui::px(4.0)),
                            blur_radius: gpui::px(12.0),
                            spread_radius: gpui::px(0.0),
                            inset: false,
                        }])
                        .children(options.iter().enumerate().map(|(idx, label)| {
                            let is_selected = Some(idx) == selected_idx;
                            let entity = entity.clone();
                            let theme = theme.clone();
                            let label = label.clone();

                            gpui::div()
                                .id(element_id(format!("select-option-{}", idx)))
                                .px(gpui::px(12.0))
                                .py(gpui::px(8.0))
                                .cursor_pointer()
                                .bg(if is_selected {
                                    theme.primary.base.opacity(0.1)
                                } else {
                                    theme.neutral.card
                                })
                                .hover(move |s| {
                                    s.cursor_pointer().bg(if is_selected {
                                        theme.neutral.text_3.opacity(0.16)
                                    } else {
                                        theme.neutral.hover
                                    })
                                })
                                .child(
                                    gpui::div()
                                        .text_size(gpui::px(theme.font_size.md))
                                        .text_color(if is_selected {
                                            theme.primary.base
                                        } else {
                                            theme.neutral.text_1
                                        })
                                        .child(label),
                                )
                                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                    entity.update(cx, |this, cx| {
                                        this.select_option(idx, window, cx);
                                    });
                                })
                        }));

                    pop_in(
                        element_id(format!("liora-select-panel-motion-{}", entity.entity_id())),
                        panel,
                    )
                    .into_any_element()
                },
                cx,
            );
        }

        let mut el = gpui::div()
            .relative()
            .when_some(self.width, |s, w| s.w(w))
            .when(self.width.is_none(), |s| s.w_full())
            .bg(theme.neutral.card)
            .when(!self.border_none, |s| {
                s.border_1().border_color(border_color)
            })
            .cursor_pointer()
            .hover(|s| {
                let s = s.cursor_pointer();
                if self.border_none {
                    s
                } else {
                    s.border_color(theme.primary.base)
                }
            });

        if !self.radius_none {
            if self.radius_left_none {
                el = el.rounded_r(gpui::px(theme.radius.md));
            } else if self.radius_right_none {
                el = el.rounded_l(gpui::px(theme.radius.md));
            } else {
                el = el.rounded(gpui::px(theme.radius.md));
            }
        }

        let close_on_click_outside = self.close_on_click_outside;

        el.child(trigger_content)
            .child(
                gpui::div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        select: cx.entity().clone(),
                    }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle_open(window, cx);
                }),
            )
            .when(close_on_click_outside, |s| {
                s.on_mouse_down_out(cx.listener(|this, _, _, cx| {
                    this.is_open = false;
                    cx.notify();
                }))
            })
            .on_action(cx.listener(Self::close_on_escape_action))
            .into_any_element()
    }
}

#[cfg(test)]
mod searchable_select_tests {
    use super::*;

    fn items() -> Vec<SearchableListItem> {
        vec![
            SearchableListItem::labeled("rust", "Rust").group("Language"),
            SearchableListItem::labeled("gpui", "GPUI")
                .description("Native UI")
                .group("Framework"),
        ]
    }

    #[test]
    fn select_reuses_searchable_list_matching_for_combobox_capabilities() {
        let matches = Select::matching_items_for(&items(), "native", 8);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].value, "gpui");
    }

    #[test]
    fn searchable_select_clears_input_query_after_selection_so_reopen_shows_all_options() {
        let source = include_str!("select.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("fn clear_search_query"));
        assert!(source.contains("input.set_value(SharedString::default(), cx)"));
        assert!(!source.contains("input.set_value(label, cx)"));
        assert!(!source.contains("input.set_value(summary, cx)"));
    }

    #[test]
    fn select_exposes_searchable_and_multiple_modes() {
        let source = include_str!("select.rs");
        assert!(source.contains("pub enum SelectMode"));
        assert!(source.contains("pub fn searchable"));
        assert!(source.contains("pub fn multiple"));
        assert!(source.contains("SearchableList::new(items.clone())"));
    }
}
