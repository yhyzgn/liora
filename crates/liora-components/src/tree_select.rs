//! Tree Select module.
//!
//! This public module implements the Liora tree-backed selection popup component. It keeps the reusable
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

use crate::Input;
use crate::gpui_compat::element_id;
use gpui::{App, Context, Entity, Render, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::{HashMap, HashSet};

/// Public builder and render state for the Liora tree select component.
pub struct TreeSelect {
    nodes: Vec<TreeSelectNode>,
    selected_keys: HashSet<SharedString>,
    disabled_keys: HashSet<SharedString>,
    multiple: bool,
    filterable: bool,
    filter_input: Entity<Input>,
    filter_query: SharedString,
    placeholder: SharedString,
    is_open: bool,
    max_panel_height: gpui::Pixels,
    on_change: Option<Box<dyn Fn(Vec<SharedString>, &mut Window, &mut App) + 'static>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
/// Public builder and render state for the Liora tree select node component.
pub struct TreeSelectNode {
    /// Stable identifier used to connect rendered UI, callbacks, and external state.
    pub id: SharedString,
    /// Human-readable label shown in the component UI.
    pub label: SharedString,
    /// Nested child items rendered beneath this item.
    pub children: Vec<TreeSelectNode>,
}

impl TreeSelectNode {
    /// Creates a new value with the required baseline configuration.
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: Vec::new(),
        }
    }

    /// Configures the child option.
    pub fn child(mut self, child: TreeSelectNode) -> Self {
        self.children.push(child);
        self
    }
}

impl TreeSelect {
    /// Creates a new value with the required baseline configuration.
    pub fn new(nodes: Vec<TreeSelectNode>, cx: &mut Context<Self>) -> Self {
        Self {
            nodes,
            selected_keys: HashSet::new(),
            disabled_keys: HashSet::new(),
            multiple: false,
            filterable: false,
            filter_input: cx.new(|cx| Input::new("", cx).placeholder("Search tree...")),
            filter_query: SharedString::default(),
            placeholder: "Select node".into(),
            is_open: false,
            max_panel_height: px(280.0),
            on_change: None,
        }
    }

    /// Creates a GPUI entity that owns this component state across render passes.
    pub fn entity(nodes: Vec<TreeSelectNode>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(nodes, cx))
    }

    /// Configures the selected option.
    pub fn selected(mut self, ids: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.selected_keys = ids.into_iter().map(Into::into).collect();
        self
    }

    /// Configures the disabled keys option.
    pub fn disabled_keys(mut self, ids: impl IntoIterator<Item = impl Into<SharedString>>) -> Self {
        self.disabled_keys = ids.into_iter().map(Into::into).collect();
        self
    }

    /// Configures the multiple option.
    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    /// Configures the filterable option.
    pub fn filterable(mut self, filterable: bool) -> Self {
        self.filterable = filterable;
        self
    }

    /// Configures the placeholder option.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Configures the max panel height option.
    pub fn max_panel_height(mut self, height: impl Into<gpui::Pixels>) -> Self {
        self.max_panel_height = height.into().max(px(120.0));
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(
        mut self,
        cb: impl Fn(Vec<SharedString>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(cb));
        self
    }

    /// Configures the selected keys option.
    pub fn selected_keys(&self) -> Vec<SharedString> {
        let mut keys = self.selected_keys.iter().cloned().collect::<Vec<_>>();
        keys.sort();
        keys
    }

    /// Updates the stored filter query value and keeps the existing component identity.
    pub fn set_filter_query(&mut self, query: impl Into<SharedString>, cx: &mut Context<Self>) {
        let query = query.into();
        if self.filter_query == query {
            return;
        }
        self.filter_query = query;
        cx.notify();
    }

    fn toggle_open(&mut self, cx: &mut Context<Self>) {
        self.is_open = !self.is_open;
        cx.notify();
    }

    fn select_node(&mut self, id: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if self.disabled_keys.contains(&id) {
            return;
        }
        if self.multiple {
            if !self.selected_keys.remove(&id) {
                self.selected_keys.insert(id);
            }
        } else {
            self.selected_keys.clear();
            self.selected_keys.insert(id);
            self.is_open = false;
        }
        let selected = self.selected_keys();
        if let Some(ref cb) = self.on_change {
            cb(selected, window, cx);
        }
        cx.notify();
    }

    fn selected_label(&self) -> SharedString {
        let labels = node_label_map(&self.nodes);
        if self.selected_keys.is_empty() {
            return self.placeholder.clone();
        }
        let mut selected = self
            .selected_keys
            .iter()
            .filter_map(|id| labels.get(id).cloned())
            .collect::<Vec<_>>();
        selected.sort();
        SharedString::from(selected.join(if self.multiple { ", " } else { "" }))
    }

    fn render_nodes(
        &self,
        nodes: &[TreeSelectNode],
        depth: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Vec<gpui::AnyElement> {
        nodes
            .iter()
            .filter(|node| node_matches_filter(node, self.filter_query.as_ref()))
            .flat_map(|node| {
                let mut out = Vec::new();
                out.push(self.render_node_row(node, depth, window, cx));
                out.extend(self.render_nodes(&node.children, depth + 1, window, cx));
                out
            })
            .collect()
    }

    fn render_node_row(
        &self,
        node: &TreeSelectNode,
        depth: usize,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> gpui::AnyElement {
        let theme = cx.global::<Config>().theme.clone();
        let id = node.id.clone();
        let selected = self.selected_keys.contains(&id);
        let disabled = self.disabled_keys.contains(&id);
        let has_children = !node.children.is_empty();
        let multiple = self.multiple;
        div()
            .id(element_id(format!("tree-select-node-{}", id)))
            .flex()
            .items_center()
            .gap_2()
            .min_h(px(30.0))
            .pl(px(10.0 + depth as f32 * 18.0))
            .pr_3()
            .rounded_sm()
            .text_color(if disabled {
                theme.neutral.text_disabled
            } else if selected {
                theme.primary.base
            } else {
                theme.neutral.text_1
            })
            .bg(if selected {
                theme.primary.base.opacity(0.1)
            } else {
                gpui::transparent_black()
            })
            .when(!disabled, |s| {
                s.cursor_pointer().hover(|s| s.bg(theme.neutral.hover))
            })
            .when(disabled, |s| s.cursor_not_allowed().opacity(0.58))
            .child(
                Icon::new(if has_children {
                    IconName::ChevronRight
                } else {
                    IconName::Minus
                })
                .size(px(13.0))
                .color(theme.neutral.text_3),
            )
            .when(multiple, |s| {
                s.child(
                    Icon::new(if selected {
                        IconName::Check
                    } else {
                        IconName::Square
                    })
                    .size(px(15.0))
                    .color(if selected {
                        theme.primary.base
                    } else {
                        theme.neutral.icon
                    }),
                )
            })
            .when(!multiple && selected, |s| {
                s.child(
                    Icon::new(IconName::Check)
                        .size(px(15.0))
                        .color(theme.primary.base),
                )
            })
            .child(div().flex_1().text_sm().child(node.label.clone()))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.select_node(id.clone(), window, cx);
            }))
            .into_any_element()
    }
}

impl Render for TreeSelect {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let filter_input = self.filter_input.clone();
        let view = cx.entity().clone();
        cx.update_entity(&filter_input, |input, cx| {
            input.set_placeholder("Search tree...", cx);
            input.set_on_change(move |value, cx| {
                view.update(cx, |view: &mut TreeSelect, cx| {
                    view.set_filter_query(value.to_string(), cx)
                });
            });
        });

        let label = self.selected_label();
        div()
            .id(liora_core::unique_id("tree-select"))
            .relative()
            .flex()
            .flex_col()
            .gap_2()
            .child(
                div()
                    .id("tree-select-trigger")
                    .flex()
                    .items_center()
                    .justify_between()
                    .min_h(px(34.0))
                    .rounded_md()
                    .border_1()
                    .border_color(if self.is_open {
                        theme.primary.base
                    } else {
                        theme.neutral.border
                    })
                    .bg(theme.neutral.card)
                    .px_3()
                    .cursor_pointer()
                    .child(
                        div()
                            .truncate()
                            .text_sm()
                            .text_color(if self.selected_keys.is_empty() {
                                theme.neutral.text_3
                            } else {
                                theme.neutral.text_1
                            })
                            .child(label),
                    )
                    .child(
                        Icon::new(if self.is_open {
                            IconName::ChevronUp
                        } else {
                            IconName::ChevronDown
                        })
                        .size(px(16.0))
                        .color(theme.neutral.icon),
                    )
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_open(cx))),
            )
            .when(self.is_open, |s| {
                s.child(
                    div()
                        .id("tree-select-panel")
                        .rounded_md()
                        .border_1()
                        .border_color(theme.neutral.border)
                        .bg(theme.neutral.card)
                        .shadow_lg()
                        .p_2()
                        .max_h(self.max_panel_height)
                        .overflow_y_scroll()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .when(self.filterable, |s| s.child(filter_input))
                        .children(self.render_nodes(&self.nodes, 0, window, cx)),
                )
            })
    }
}

/// Configures the node label map option.
pub fn node_label_map(nodes: &[TreeSelectNode]) -> HashMap<SharedString, String> {
    let mut map = HashMap::new();
    fn walk(node: &TreeSelectNode, map: &mut HashMap<SharedString, String>) {
        map.insert(node.id.clone(), node.label.to_string());
        for child in &node.children {
            walk(child, map);
        }
    }
    for node in nodes {
        walk(node, &mut map);
    }
    map
}

/// Configures the node matches filter option.
pub fn node_matches_filter(node: &TreeSelectNode, query: &str) -> bool {
    if query.trim().is_empty() {
        return true;
    }
    let query = query.to_lowercase();
    node.label.to_lowercase().contains(&query)
        || node.id.to_lowercase().contains(&query)
        || node
            .children
            .iter()
            .any(|child| node_matches_filter(child, &query))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn nodes() -> Vec<TreeSelectNode> {
        vec![
            TreeSelectNode::new("docs", "Docs")
                .child(TreeSelectNode::new("quick-start", "Quick Start")),
            TreeSelectNode::new("charts", "Charts"),
        ]
    }
    #[test]
    fn tree_select_filter_keeps_matching_parents() {
        assert!(node_matches_filter(&nodes()[0], "quick"));
        assert!(!node_matches_filter(&nodes()[1], "quick"));
    }
    #[test]
    fn tree_select_label_map_flattens_tree() {
        let labels = node_label_map(&nodes());
        assert_eq!(
            labels.get("quick-start").map(String::as_str),
            Some("Quick Start")
        );
    }
}
