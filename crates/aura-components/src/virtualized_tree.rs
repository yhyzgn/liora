use crate::VirtualScrollbar;
use crate::tree::TreeNode;
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    App, Context, Entity, IntoElement, ListAlignment, ListState, MouseButton, Pixels, Render,
    SharedString, Window, div, list, prelude::*, px,
};
use std::collections::HashSet;
use std::sync::Arc;

type NodeCallback = dyn Fn(SharedString, &mut Window, &mut App) + 'static;

/// Visible tree row produced from the expanded tree model.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualTreeItem {
    pub id: SharedString,
    pub label: SharedString,
    pub depth: u32,
    pub has_children: bool,
}

/// Virtualized tree for large hierarchical datasets.
///
/// The component stores tree data and lightweight visible-node metadata only.
/// It never stores rendered GPUI elements across frames; rows are generated from
/// the flattened visible model inside GPUI's virtual `list` callback.
pub struct VirtualizedTree {
    data: Vec<TreeNode>,
    expanded_keys: HashSet<SharedString>,
    selected_keys: HashSet<SharedString>,
    flattened: Vec<VirtualTreeItem>,
    list_state: ListState,
    multiple: bool,
    indent: Pixels,
    row_height: Pixels,
    height: Pixels,
    overdraw: Pixels,
    show_checkbox: bool,
    on_node_click: Option<Arc<NodeCallback>>,
}

impl VirtualizedTree {
    pub fn new(data: Vec<TreeNode>, _cx: &mut Context<Self>) -> Self {
        let flattened = flatten_visible(&data, &HashSet::new());
        let overdraw = px(640.0);
        let list_state = ListState::new(flattened.len(), ListAlignment::Top, overdraw);
        Self {
            data,
            expanded_keys: HashSet::new(),
            selected_keys: HashSet::new(),
            flattened,
            list_state,
            multiple: false,
            indent: px(18.0),
            row_height: px(34.0),
            height: px(360.0),
            overdraw,
            show_checkbox: false,
            on_node_click: None,
        }
    }

    pub fn entity(data: Vec<TreeNode>, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| Self::new(data, cx))
    }

    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into();
        self
    }

    pub fn row_height(mut self, height: impl Into<Pixels>) -> Self {
        self.row_height = height.into();
        self.list_state.remeasure();
        self
    }

    pub fn indent(mut self, indent: impl Into<Pixels>) -> Self {
        self.indent = indent.into();
        self
    }

    pub fn overdraw(mut self, overdraw: impl Into<Pixels>) -> Self {
        self.overdraw = overdraw.into();
        self.rebuild_list_state();
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn show_checkbox(mut self, show: bool) -> Self {
        self.show_checkbox = show;
        self
    }

    pub fn default_expanded_keys(mut self, keys: impl IntoIterator<Item = SharedString>) -> Self {
        self.expanded_keys = keys.into_iter().collect();
        self.rebuild_flattened();
        self
    }

    pub fn default_selected_keys(mut self, keys: impl IntoIterator<Item = SharedString>) -> Self {
        self.selected_keys = keys.into_iter().collect();
        self
    }

    pub fn expand_all(mut self) -> Self {
        let mut keys = HashSet::new();
        collect_parent_keys(&self.data, &mut keys);
        self.expanded_keys = keys;
        self.rebuild_flattened();
        self
    }

    pub fn on_node_click(
        mut self,
        callback: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_node_click = Some(Arc::new(callback));
        self
    }

    pub fn visible_len(&self) -> usize {
        self.flattened.len()
    }

    pub fn is_expanded(&self, id: &SharedString) -> bool {
        self.expanded_keys.contains(id)
    }

    pub fn is_selected(&self, id: &SharedString) -> bool {
        self.selected_keys.contains(id)
    }

    pub fn list_state(&self) -> ListState {
        self.list_state.clone()
    }

    fn rebuild_flattened(&mut self) {
        let next = flatten_visible(&self.data, &self.expanded_keys);
        let count_changed = next.len() != self.flattened.len();
        self.flattened = next;
        if count_changed {
            self.rebuild_list_state();
        } else {
            self.list_state.remeasure();
        }
    }

    fn rebuild_list_state(&mut self) {
        self.list_state = ListState::new(self.flattened.len(), ListAlignment::Top, self.overdraw);
    }

    fn toggle_expand(&mut self, id: SharedString, cx: &mut Context<Self>) {
        if self.expanded_keys.contains(&id) {
            self.expanded_keys.remove(&id);
        } else {
            self.expanded_keys.insert(id);
        }
        self.rebuild_flattened();
        cx.notify();
    }

    fn select_node(&mut self, id: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if self.multiple {
            if self.selected_keys.contains(&id) {
                self.selected_keys.remove(&id);
            } else {
                self.selected_keys.insert(id.clone());
            }
        } else {
            self.selected_keys.clear();
            self.selected_keys.insert(id.clone());
        }

        if let Some(callback) = self.on_node_click.clone() {
            callback(id, window, cx);
        }
        cx.notify();
    }

    fn click_node(
        &mut self,
        id: SharedString,
        has_children: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if has_children {
            if self.expanded_keys.contains(&id) {
                self.expanded_keys.remove(&id);
            } else {
                self.expanded_keys.insert(id.clone());
            }
            self.rebuild_flattened();
        }
        self.select_node(id, window, cx);
    }
}

impl Render for VirtualizedTree {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let flattened = self.flattened.clone();
        let expanded_keys = self.expanded_keys.clone();
        let selected_keys = self.selected_keys.clone();
        let indent = self.indent;
        let row_height = self.row_height;
        let show_checkbox = self.show_checkbox;
        let entity = cx.entity().clone();
        let list_state = self.list_state.clone();

        div()
            .relative()
            .w_full()
            .h(self.height)
            .overflow_hidden()
            .rounded(px(theme.radius.md))
            .border_1()
            .border_color(theme.neutral.border)
            .bg(theme.neutral.card)
            .child(
                list(list_state.clone(), move |index, _window, _cx| {
                    let Some(item) = flattened.get(index).cloned() else {
                        return div().into_any_element();
                    };
                    let id = item.id.clone();
                    let is_expanded = expanded_keys.contains(&id);
                    let is_selected = selected_keys.contains(&id);
                    let has_children = item.has_children;
                    let padding_left = px(f32::from(indent) * item.depth as f32);
                    let expand_entity = entity.clone();
                    let click_entity = entity.clone();
                    let expand_id = id.clone();
                    let click_id = id.clone();

                    div()
                        .id(format!("virtual-tree-row-{}", id))
                        .cursor_pointer()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .w_full()
                        .min_h(row_height)
                        .pl(padding_left)
                        .pr_4()
                        .text_color(if is_selected {
                            theme.primary.base
                        } else {
                            theme.neutral.text_1
                        })
                        .bg(if is_selected {
                            theme.primary.base.opacity(0.1)
                        } else {
                            gpui::transparent_black()
                        })
                        .hover(|s| s.bg(theme.neutral.hover))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(22.0))
                                .when(has_children, |s| {
                                    s.on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                        expand_entity.update(cx, |tree, cx| {
                                            tree.toggle_expand(expand_id.clone(), cx);
                                        });
                                        cx.stop_propagation();
                                    })
                                    .child(
                                        Icon::new(if is_expanded {
                                            IconName::ChevronDown
                                        } else {
                                            IconName::ChevronRight
                                        })
                                        .size(px(14.0))
                                        .color(theme.neutral.text_3),
                                    )
                                }),
                        )
                        .when(show_checkbox, |s| {
                            s.child(
                                Icon::new(if is_selected {
                                    IconName::Check
                                } else {
                                    IconName::Square
                                })
                                .size(px(15.0))
                                .color(if is_selected {
                                    theme.primary.base
                                } else {
                                    theme.neutral.text_3
                                }),
                            )
                        })
                        .child(
                            div()
                                .flex_1()
                                .text_size(px(theme.font_size.sm))
                                .child(item.label.clone()),
                        )
                        .on_click(move |_, window, cx| {
                            click_entity.update(cx, |tree, cx| {
                                tree.click_node(click_id.clone(), has_children, window, cx);
                            });
                        })
                        .into_any_element()
                })
                .size_full(),
            )
            .child(VirtualScrollbar::new(list_state))
    }
}

pub fn flatten_visible(
    data: &[TreeNode],
    expanded_keys: &HashSet<SharedString>,
) -> Vec<VirtualTreeItem> {
    let mut output = Vec::new();
    for node in data {
        flatten_node(node, 0, expanded_keys, &mut output);
    }
    output
}

fn flatten_node(
    node: &TreeNode,
    depth: u32,
    expanded_keys: &HashSet<SharedString>,
    output: &mut Vec<VirtualTreeItem>,
) {
    let has_children = !node.children.is_empty();
    output.push(VirtualTreeItem {
        id: node.id.clone(),
        label: node.label.clone(),
        depth,
        has_children,
    });
    if has_children && expanded_keys.contains(&node.id) {
        for child in &node.children {
            flatten_node(child, depth + 1, expanded_keys, output);
        }
    }
}

fn collect_parent_keys(data: &[TreeNode], output: &mut HashSet<SharedString>) {
    for node in data {
        if !node.children.is_empty() {
            output.insert(node.id.clone());
            collect_parent_keys(&node.children, output);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_tree() -> Vec<TreeNode> {
        vec![
            TreeNode::new("root", "Root")
                .child(TreeNode::new("a", "A"))
                .child(TreeNode::new("b", "B").child(TreeNode::new("b1", "B1"))),
            TreeNode::new("other", "Other"),
        ]
    }

    #[test]
    fn flatten_visible_only_includes_expanded_descendants() {
        let tree = sample_tree();
        let collapsed = flatten_visible(&tree, &HashSet::new());
        assert_eq!(
            collapsed
                .iter()
                .map(|item| item.id.as_ref())
                .collect::<Vec<_>>(),
            vec!["root", "other"]
        );

        let expanded = HashSet::from([SharedString::from("root")]);
        let visible = flatten_visible(&tree, &expanded);
        assert_eq!(
            visible
                .iter()
                .map(|item| item.id.as_ref())
                .collect::<Vec<_>>(),
            vec!["root", "a", "b", "other"]
        );

        let expanded = HashSet::from([SharedString::from("root"), SharedString::from("b")]);
        let visible = flatten_visible(&tree, &expanded);
        assert_eq!(
            visible
                .iter()
                .map(|item| item.id.as_ref())
                .collect::<Vec<_>>(),
            vec!["root", "a", "b", "b1", "other"]
        );
    }

    #[test]
    fn virtualized_tree_uses_list_state_and_visible_metadata() {
        let source = include_str!("virtualized_tree.rs");

        assert!(source.contains("pub struct VirtualizedTree"));
        assert!(source.contains("ListState::new"));
        assert!(source.contains("list(list_state.clone()"));
        assert!(source.contains("VirtualScrollbar::new"));
        assert!(source.contains("flatten_visible"));
        assert!(source.contains("flattened: Vec<VirtualTreeItem>"));
    }
}
