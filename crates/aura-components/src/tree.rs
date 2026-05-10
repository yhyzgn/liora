use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, IntoElement, Pixels, Render, SharedString, Window, div, prelude::*,
    px,
};
use std::collections::HashSet;

pub struct TreeNode {
    pub id: SharedString,
    pub label: SharedString,
    pub children: Vec<TreeNode>,
}

pub struct Tree {
    data: Vec<TreeNode>,
    expanded_keys: HashSet<SharedString>,
    selected_keys: HashSet<SharedString>,
    multiple: bool,
    indent: Pixels,
    show_checkbox: bool,
    on_node_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl TreeNode {
    pub fn new(id: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            children: vec![],
        }
    }

    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child);
        self
    }
}

impl Tree {
    pub fn new(data: Vec<TreeNode>) -> Self {
        Self {
            data,
            expanded_keys: HashSet::new(),
            selected_keys: HashSet::new(),
            multiple: false,
            indent: px(18.0),
            show_checkbox: false,
            on_node_click: None,
        }
    }

    pub fn indent(mut self, indent: impl Into<Pixels>) -> Self {
        self.indent = indent.into();
        self
    }

    pub fn show_checkbox(mut self, show: bool) -> Self {
        self.show_checkbox = show;
        self
    }

    pub fn multiple(mut self, multiple: bool) -> Self {
        self.multiple = multiple;
        self
    }

    pub fn on_node_click(
        mut self,
        f: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_node_click = Some(Box::new(f));
        self
    }

    fn toggle_expand(&mut self, id: SharedString, cx: &mut Context<Self>) {
        if self.expanded_keys.contains(&id) {
            self.expanded_keys.remove(&id);
        } else {
            self.expanded_keys.insert(id);
        }
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

        if let Some(ref on_click) = self.on_node_click {
            (on_click)(id, window, cx);
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
        }
        self.select_node(id, window, cx);
    }

    fn render_node(
        &self,
        node: &TreeNode,
        depth: u32,
        theme: &aura_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = node.id.clone();
        let is_expanded = self.expanded_keys.contains(&id);
        let is_selected = self.selected_keys.contains(&id);
        let has_children = !node.children.is_empty();
        let padding_left = px(f32::from(self.indent) * depth as f32);

        div()
            .flex()
            .flex_col()
            .child(
                div()
                    .id(id.clone())
                    .cursor_pointer()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .h(px(32.0))
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
                        // Expand Icon
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w(px(20.0))
                            .id(format!("expand-{}", id.clone()))
                            .when(has_children, |s| {
                                s.on_click(cx.listener({
                                    let id = id.clone();
                                    move |this, _, _, cx| {
                                        this.toggle_expand(id.clone(), cx);
                                        cx.stop_propagation();
                                    }
                                }))
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
                    .on_click(cx.listener({
                        let id = id.clone();
                        move |this, _, window, cx| {
                            this.click_node(id.clone(), has_children, window, cx);
                        }
                    }))
                    .child(
                        div()
                            .flex_1()
                            .id(format!("content-{}", id.clone()))
                            .child(div().text_sm().child(node.label.clone())),
                    ),
            )
            .when(is_expanded && has_children, |s| {
                s.children(
                    node.children
                        .iter()
                        .map(|child| self.render_node(child, depth + 1, theme, cx)),
                )
            })
            .into_any_element()
    }
}

impl Render for Tree {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        div().flex().flex_col().w_full().children(
            self.data
                .iter()
                .map(|node| self.render_node(node, 0, &theme, cx)),
        )
    }
}
