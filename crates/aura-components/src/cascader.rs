use aura_core::{Config, push_portal};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Bounds, Context, Element, ElementId, Entity, FocusHandle, Focusable,
    IntoElement, MouseButton, Pixels, Render, SharedString, Window, div, prelude::*, px,
};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CascaderOption {
    pub value: SharedString,
    pub label: SharedString,
    pub children: Vec<CascaderOption>,
    pub disabled: bool,
    pub loading: bool,
}

pub struct Cascader {
    options: Vec<CascaderOption>,
    selected_path: Vec<SharedString>,
    active_path: Vec<SharedString>,
    placeholder: SharedString,
    separator: SharedString,
    is_open: bool,
    disabled: bool,
    clearable: bool,
    filterable: bool,
    search_query: SharedString,
    width: Option<Pixels>,
    focus_handle: FocusHandle,
    last_bounds: Option<Bounds<Pixels>>,
    on_change: Option<Arc<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
}

impl CascaderOption {
    pub fn new(value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            children: vec![],
            disabled: false,
            loading: false,
        }
    }

    pub fn child(mut self, child: CascaderOption) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = CascaderOption>) -> Self {
        self.children.extend(children);
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn loading(mut self, loading: bool) -> Self {
        self.loading = loading;
        self
    }

    fn find_child(&self, value: &SharedString) -> Option<&CascaderOption> {
        self.children.iter().find(|child| &child.value == value)
    }
}

impl Cascader {
    pub fn new(options: Vec<CascaderOption>, cx: &mut Context<Self>) -> Self {
        Self {
            options,
            selected_path: vec![],
            active_path: vec![],
            placeholder: "请选择".into(),
            separator: " / ".into(),
            is_open: false,
            disabled: false,
            clearable: false,
            filterable: false,
            search_query: SharedString::default(),
            width: None,
            focus_handle: cx.focus_handle(),
            last_bounds: None,
            on_change: None,
        }
    }

    pub fn selected_path(
        mut self,
        path: impl IntoIterator<Item = impl Into<SharedString>>,
    ) -> Self {
        self.selected_path = path.into_iter().map(Into::into).collect();
        self.active_path = self.selected_path.clone();
        self
    }

    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    pub fn separator(mut self, separator: impl Into<SharedString>) -> Self {
        self.separator = separator.into();
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

    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    pub fn search_query(mut self, query: impl Into<SharedString>) -> Self {
        self.search_query = query.into();
        self
    }

    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = Some(width.into());
        self
    }

    pub fn on_change(
        mut self,
        f: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Arc::new(f));
        self
    }

    pub fn set_options(&mut self, options: Vec<CascaderOption>, cx: &mut Context<Self>) {
        self.options = options;
        if !Self::is_selectable_path(&self.options, &self.selected_path) {
            self.selected_path.clear();
            self.active_path.clear();
        }
        cx.notify();
    }

    pub fn set_selected_path(
        &mut self,
        path: impl IntoIterator<Item = impl Into<SharedString>>,
        cx: &mut Context<Self>,
    ) {
        self.selected_path = path.into_iter().map(Into::into).collect();
        self.active_path = self.selected_path.clone();
        cx.notify();
    }

    pub fn set_search_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        self.search_query = query.into();
        cx.notify();
    }

    pub fn labels_for_path(options: &[CascaderOption], path: &[SharedString]) -> Vec<SharedString> {
        let mut labels = Vec::new();
        let mut siblings = options;

        for value in path {
            let Some(option) = siblings.iter().find(|option| &option.value == value) else {
                return vec![];
            };
            labels.push(option.label.clone());
            siblings = &option.children;
        }

        labels
    }

    pub fn is_selectable_path(options: &[CascaderOption], path: &[SharedString]) -> bool {
        let Some(option) = Self::option_for_path(options, path) else {
            return false;
        };
        !option.disabled && !option.loading && option.children.is_empty()
    }

    fn option_for_path<'a>(
        options: &'a [CascaderOption],
        path: &[SharedString],
    ) -> Option<&'a CascaderOption> {
        let (first, rest) = path.split_first()?;
        let mut option = options.iter().find(|option| &option.value == first)?;

        for value in rest {
            option = option.find_child(value)?;
        }

        Some(option)
    }

    fn columns_for_active_path(&self) -> Vec<Vec<CascaderOption>> {
        let mut columns = vec![self.options.clone()];
        let mut siblings = self.options.as_slice();

        for value in &self.active_path {
            let Some(option) = siblings.iter().find(|option| &option.value == value) else {
                break;
            };
            if option.children.is_empty() {
                break;
            }
            columns.push(option.children.clone());
            siblings = &option.children;
        }

        columns
    }

    fn matching_leaf_paths(&self) -> Vec<(Vec<SharedString>, Vec<SharedString>)> {
        let query = self.search_query.to_string().to_lowercase();
        if query.trim().is_empty() {
            return vec![];
        }

        let mut matches = Vec::new();
        collect_leaf_matches(
            &self.options,
            &query,
            &mut vec![],
            &mut vec![],
            &mut matches,
        );
        matches
    }

    fn display_text(&self) -> SharedString {
        let labels = Self::labels_for_path(&self.options, &self.selected_path);
        if labels.is_empty() {
            return self.placeholder.clone();
        }
        labels
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(self.separator.as_ref())
            .into()
    }

    fn toggle_open(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled {
            return;
        }
        self.is_open = !self.is_open;
        if self.is_open {
            self.active_path = self.selected_path.clone();
            window.focus(&self.focus_handle, cx);
        }
        cx.notify();
    }

    fn choose_path(
        &mut self,
        path: Vec<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !Self::is_selectable_path(&self.options, &path) {
            self.active_path = path;
            cx.notify();
            return;
        }

        self.selected_path = path.clone();
        self.active_path = path.clone();
        self.is_open = false;
        if let Some(on_change) = &self.on_change {
            on_change(path, window, cx);
        }
        cx.notify();
    }

    fn clear(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.selected_path.is_empty() {
            return;
        }
        self.selected_path.clear();
        self.active_path.clear();
        if let Some(on_change) = &self.on_change {
            on_change(vec![], window, cx);
        }
        cx.notify();
    }
}

impl Focusable for Cascader {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for Cascader {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let focused = self.focus_handle.is_focused(_window);
        let display_text = self.display_text();
        let has_value = !self.selected_path.is_empty();
        let text_color = if has_value {
            theme.neutral.text_1
        } else {
            theme.neutral.text_3
        };
        let border_color = if focused || self.is_open {
            theme.primary.base
        } else {
            theme.neutral.border
        };

        if self.is_open {
            let entity = cx.entity().clone();
            let theme_portal = theme.clone();
            let trigger_bounds = self.last_bounds;
            let columns = self.columns_for_active_path();
            let active_path = self.active_path.clone();
            let selected_path = self.selected_path.clone();
            let filterable = self.filterable;
            let matches = self.matching_leaf_paths();

            push_portal(
                move |_window, _cx| {
                    let (top, left, trigger_width) = if let Some(bounds) = trigger_bounds {
                        (bounds.bottom() + px(4.0), bounds.left(), bounds.size.width)
                    } else {
                        (px(100.0), px(100.0), px(240.0))
                    };
                    let panel_width =
                        px((columns.len().max(1) as f32 * 180.0).max(f32::from(trigger_width)));
                    let theme = theme_portal.clone();
                    let entity = entity.clone();

                    div()
                        .absolute()
                        .top(top)
                        .left(left)
                        .w(panel_width)
                        .max_h(px(280.0))
                        .overflow_hidden()
                        .bg(theme.neutral.card)
                        .rounded(px(theme.radius.md))
                        .border_1()
                        .border_color(theme.neutral.border)
                        .shadow(vec![gpui::BoxShadow {
                            color: theme.neutral.border,
                            offset: gpui::point(px(0.0), px(4.0)),
                            blur_radius: px(14.0),
                            spread_radius: px(0.0),
                        }])
                        .when(filterable && !matches.is_empty(), |panel| {
                            panel.child(render_match_list(
                                matches.clone(),
                                entity.clone(),
                                theme.clone(),
                            ))
                        })
                        .when(!filterable || matches.is_empty(), |panel| {
                            panel.child(render_columns(
                                columns.clone(),
                                active_path.clone(),
                                selected_path.clone(),
                                entity.clone(),
                                theme.clone(),
                            ))
                        })
                        .into_any_element()
                },
                cx,
            );
        }

        div()
            .relative()
            .when_some(self.width, |s, width| s.w(width))
            .when(self.width.is_none(), |s| s.w_full())
            .h(px(34.0))
            .flex()
            .items_center()
            .justify_between()
            .gap_2()
            .px_3()
            .bg(if self.disabled {
                theme.neutral.hover
            } else {
                theme.neutral.card
            })
            .border_1()
            .border_color(border_color)
            .rounded(px(theme.radius.md))
            .text_size(px(theme.font_size.md))
            .text_color(text_color)
            .when(!self.disabled, |s| {
                s.cursor_pointer()
                    .hover(|s| s.border_color(theme.primary.base).cursor_pointer())
            })
            .child(div().flex_1().overflow_hidden().child(display_text))
            .when(self.clearable && has_value && !self.disabled, |s| {
                s.child(
                    div()
                        .id("cascader-clear")
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_full()
                        .hover(|s| s.bg(theme.neutral.hover))
                        .child(
                            Icon::new(IconName::X)
                                .size(px(14.0))
                                .color(theme.neutral.icon),
                        )
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _, window, cx| {
                                this.clear(window, cx);
                                cx.stop_propagation();
                            }),
                        ),
                )
            })
            .child(
                Icon::new(if self.is_open {
                    IconName::ChevronUp
                } else {
                    IconName::ChevronDown
                })
                .size(px(14.0))
                .color(theme.neutral.icon),
            )
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .child(BoundsCapturer {
                        cascader: cx.entity().clone(),
                    }),
            )
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _, window, cx| {
                    this.toggle_open(window, cx);
                }),
            )
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.is_open = false;
                cx.notify();
            }))
    }
}

fn render_columns(
    columns: Vec<Vec<CascaderOption>>,
    active_path: Vec<SharedString>,
    selected_path: Vec<SharedString>,
    entity: Entity<Cascader>,
    theme: aura_theme::Theme,
) -> AnyElement {
    div()
        .flex()
        .flex_row()
        .children(columns.into_iter().enumerate().map(|(depth, column)| {
            let active_value = active_path.get(depth).cloned();
            let selected_value = selected_path.get(depth).cloned();
            let path_prefix = active_path.iter().take(depth).cloned().collect::<Vec<_>>();
            div()
                .id(format!("cascader-column-{depth}"))
                .w(px(180.0))
                .max_h(px(280.0))
                .overflow_y_scroll()
                .border_r_1()
                .border_color(theme.neutral.divider)
                .children(column.into_iter().map({
                    let entity = entity.clone();
                    let theme = theme.clone();
                    move |option| {
                        let mut path = path_prefix.clone();
                        path.push(option.value.clone());
                        let is_active = Some(option.value.clone()) == active_value;
                        let is_selected = Some(option.value.clone()) == selected_value;
                        let has_children = !option.children.is_empty();
                        let disabled = option.disabled || option.loading;
                        let entity = entity.clone();
                        let theme = theme.clone();

                        div()
                            .h(px(34.0))
                            .px_3()
                            .flex()
                            .items_center()
                            .justify_between()
                            .gap_2()
                            .bg(if is_active || is_selected {
                                theme.primary.base.opacity(0.1)
                            } else {
                                theme.neutral.card
                            })
                            .text_color(if disabled {
                                theme.neutral.text_3
                            } else if is_active || is_selected {
                                theme.primary.base
                            } else {
                                theme.neutral.text_1
                            })
                            .when(!disabled, |s| {
                                s.cursor_pointer()
                                    .hover(|s| s.bg(theme.neutral.hover).cursor_pointer())
                            })
                            .child(div().flex_1().text_sm().child(option.label.clone()))
                            .when(option.loading, |s| {
                                s.child(
                                    Icon::new(IconName::LoaderCircle)
                                        .size(px(14.0))
                                        .color(theme.neutral.icon),
                                )
                            })
                            .when(has_children && !option.loading, |s| {
                                s.child(
                                    Icon::new(IconName::ChevronRight)
                                        .size(px(14.0))
                                        .color(theme.neutral.icon),
                                )
                            })
                            .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                                if disabled {
                                    return;
                                }
                                entity.update(cx, |this, cx| {
                                    this.choose_path(path.clone(), window, cx);
                                });
                            })
                    }
                }))
        }))
        .into_any_element()
}

fn render_match_list(
    matches: Vec<(Vec<SharedString>, Vec<SharedString>)>,
    entity: Entity<Cascader>,
    theme: aura_theme::Theme,
) -> AnyElement {
    div()
        .id("cascader-search-results")
        .flex()
        .flex_col()
        .max_h(px(280.0))
        .overflow_y_scroll()
        .children(matches.into_iter().map(move |(path, labels)| {
            let entity = entity.clone();
            let theme = theme.clone();
            let text = labels
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" / ");
            div()
                .h(px(34.0))
                .px_3()
                .flex()
                .items_center()
                .cursor_pointer()
                .hover(|s| s.bg(theme.neutral.hover).cursor_pointer())
                .child(div().text_sm().text_color(theme.neutral.text_1).child(text))
                .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                    entity.update(cx, |this, cx| {
                        this.choose_path(path.clone(), window, cx);
                    });
                })
        }))
        .into_any_element()
}

fn collect_leaf_matches(
    options: &[CascaderOption],
    query: &str,
    path: &mut Vec<SharedString>,
    labels: &mut Vec<SharedString>,
    matches: &mut Vec<(Vec<SharedString>, Vec<SharedString>)>,
) {
    for option in options {
        path.push(option.value.clone());
        labels.push(option.label.clone());

        if option.children.is_empty() {
            let haystack = labels
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(" /")
                .to_lowercase();
            if !option.disabled && !option.loading && haystack.contains(query) {
                matches.push((path.clone(), labels.clone()));
            }
        } else {
            collect_leaf_matches(&option.children, query, path, labels, matches);
        }

        path.pop();
        labels.pop();
    }
}

struct BoundsCapturer {
    cascader: Entity<Cascader>,
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
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (gpui::LayoutId, ()) {
        let mut style = gpui::Style::default();
        style.size.width = gpui::relative(1.0).into();
        style.size.height = gpui::relative(1.0).into();
        (window.request_layout(style, [], cx), ())
    }

    fn prepaint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut (),
        _window: &mut Window,
        cx: &mut App,
    ) {
        self.cascader.update(cx, |this, _| {
            this.last_bounds = Some(bounds);
        });
    }

    fn paint(
        &mut self,
        _: Option<&gpui::GlobalElementId>,
        _: Option<&gpui::InspectorElementId>,
        _: Bounds<Pixels>,
        _: &mut (),
        _: &mut (),
        _window: &mut Window,
        _: &mut App,
    ) {
    }
}
