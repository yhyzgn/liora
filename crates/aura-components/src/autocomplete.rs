use crate::Input;
use aura_core::{Config, push_portal};
use aura_icons_lucide::IconName;
use gpui::{
    App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable, GlobalElementId,
    InspectorElementId, IntoElement, LayoutId, MouseButton, Pixels, Render, SharedString, Style,
    Window, prelude::*, px, relative,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AutocompleteItem {
    pub value: SharedString,
    pub label: SharedString,
}

impl AutocompleteItem {
    pub fn new(value: impl Into<SharedString>) -> Self {
        let value = value.into();
        Self {
            label: value.clone(),
            value,
        }
    }

    pub fn labeled(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

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
    on_select: Option<Box<dyn Fn(AutocompleteItem, &mut Window, &mut App) + 'static>>,
}

impl Autocomplete {
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
            on_select: None,
        }
    }

    pub fn from_values(values: Vec<impl Into<SharedString>>, cx: &mut Context<Self>) -> Self {
        Self::new(values.into_iter().map(AutocompleteItem::new).collect(), cx)
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn clearable(mut self, clearable: bool) -> Self {
        self.clearable = clearable;
        self
    }

    pub fn suffix_icon(mut self, icon: IconName) -> Self {
        self.suffix_icon = Some(icon);
        self
    }

    pub fn no_suffix_icon(mut self) -> Self {
        self.suffix_icon = None;
        self
    }

    pub fn suffix_icon_value(&self) -> Option<IconName> {
        self.suffix_icon
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn max_suggestions(mut self, max: usize) -> Self {
        self.max_suggestions = max.max(1);
        self
    }

    pub fn trigger_on_focus(mut self, trigger: bool) -> Self {
        self.trigger_on_focus = trigger;
        self
    }

    pub fn on_select(
        mut self,
        cb: impl Fn(AutocompleteItem, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_select = Some(Box::new(cb));
        self
    }

    pub fn value(&self, cx: &App) -> SharedString {
        self.input.read(cx).value()
    }

    pub fn set_items(&mut self, items: Vec<AutocompleteItem>, cx: &mut Context<Self>) {
        if self.items == items {
            return;
        }
        self.items = items;
        cx.notify();
    }

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
        if let Some(ref cb) = self.on_select {
            cb(item, window, cx);
        }
        cx.notify();
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
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let entity = cx.entity().clone();
        let disabled = self.disabled;
        let placeholder = self.placeholder.clone();
        let clearable = self.clearable;
        let suffix_icon = self.suffix_icon;

        self.input.update(cx, |input, _| {
            input.set_on_change({
                let entity = entity.clone();
                move |_, cx| {
                    entity.update(cx, |this, cx| {
                        this.is_open = true;
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
        if self.is_open && !disabled {
            let trigger_bounds = self.last_bounds;
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            push_portal(
                move |_window, _cx| {
                    let (top, left, width) = trigger_bounds
                        .map(|b| (b.bottom() + px(4.0), b.left(), b.size.width))
                        .unwrap_or((px(120.0), px(120.0), px(280.0)));
                    let entity = entity.clone();
                    let theme = theme_portal.clone();
                    let mut panel = gpui::div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(width)
                        .max_h(px(240.0))
                        .bg(theme.neutral.card)
                        .rounded(px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow_lg();

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
                        panel = panel.children(matches.iter().map(|item| {
                            let item = item.clone();
                            let entity = entity.clone();
                            let theme = theme.clone();
                            gpui::div()
                                .flex()
                                .items_center()
                                .justify_between()
                                .gap_3()
                                .px(px(12.0))
                                .py(px(8.0))
                                .cursor_pointer()
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
                    panel.into_any_element()
                },
                cx,
            );
        }

        let frame = gpui::div()
            .relative()
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
            .child(self.input.clone());

        frame
    }
}
