//! Menu module.
//!
//! This public module implements the Liora navigation menu component with items, groups, and submenus. It keeps the reusable
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
use crate::{Flex, Popover, motion::pop_in};
use gpui::{
    AnyElement, App, Context, IntoElement, MouseButton, Render, SharedString, Window, div,
    prelude::*, px,
};
use liora_core::{Config, Placement};
use liora_icons::Icon;
use liora_icons_lucide::IconName;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Options that control menu mode behavior.
pub enum MenuMode {
    #[default]
    /// Lays out content in the vertical direction.
    Vertical,
    /// Lays out content in the horizontal direction.
    Horizontal,
}

#[derive(Clone, PartialEq, Eq)]
/// Options that control menu node behavior.
pub enum MenuNode {
    /// Uses the `Item` option for `MenuNode`.
    Item(MenuItem),
    /// Uses the `SubMenu` option for `MenuNode`.
    SubMenu(SubMenu),
    /// Uses the `Group` option for `MenuNode`.
    Group(MenuItemGroup),
}

#[derive(Clone, PartialEq, Eq)]
/// Data model used by menu item rendering.
pub struct MenuItem {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
}

#[derive(Clone, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora sub menu.
pub struct SubMenu {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
    /// Nested child items rendered beneath this item.
    pub children: Vec<MenuNode>,
}

#[derive(Clone, PartialEq, Eq)]
/// Fluent native GPUI component for rendering Liora menu item group.
pub struct MenuItemGroup {
    /// Primary heading or title text displayed by the component.
    pub title: SharedString,
    /// Nested child items rendered beneath this item.
    pub children: Vec<MenuNode>,
}

/// Fluent native GPUI component for rendering Liora menu.
pub struct Menu {
    id: SharedString,
    mode: MenuMode,
    is_collapsed: bool,
    active_index: Option<SharedString>,
    opened_submenus: HashSet<SharedString>,
    items: Vec<MenuNode>,
    on_select: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    close_on_escape: bool,
}

impl Menu {
    /// Creates `Menu` with default theme-driven styling and no optional callbacks attached.
    pub fn new() -> Self {
        Self {
            id: liora_core::unique_id("menu"),
            mode: MenuMode::Vertical,
            is_collapsed: false,
            active_index: None,
            opened_submenus: HashSet::new(),
            items: vec![],
            on_select: None,
            close_on_escape: true,
        }
    }

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Selects the rendering mode used by this component.
    pub fn mode(mut self, mode: MenuMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the collapse value used by the component.
    pub fn collapse(mut self, collapsed: bool) -> Self {
        self.is_collapsed = collapsed;
        self
    }

    /// Sets the initially active menu item.
    pub fn default_active(mut self, index: impl Into<SharedString>) -> Self {
        self.active_index = Some(index.into());
        self
    }

    /// Registers a callback that runs when select occurs.
    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

    /// Replaces the rendered menu items while preserving focus, callbacks, and submenu state.
    pub fn set_items(&mut self, items: Vec<MenuNode>, cx: &mut Context<Self>) {
        if self.items == items {
            return;
        }
        self.items = items;
        cx.notify();
    }

    /// Updates the active item without rebuilding the menu entity.
    pub fn set_active_index(&mut self, index: impl Into<SharedString>, cx: &mut Context<Self>) {
        let index = index.into();
        if self.active_index.as_ref() == Some(&index) {
            return;
        }
        self.active_index = Some(index);
        cx.notify();
    }

    /// Toggles whether the popup closes when escape occurs.
    pub fn close_on_escape(mut self, close: bool) -> Self {
        self.close_on_escape = close;
        self
    }

    /// Replaces all menu nodes while constructing a menu.
    pub fn with_items(mut self, items: Vec<MenuNode>) -> Self {
        self.items = items;
        self
    }

    /// Performs the item operation used by this component.
    pub fn item(
        mut self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
    ) -> Self {
        self.items.push(MenuNode::Item(MenuItem {
            id: id.into(),
            label: label.into(),
            icon,
        }));
        self
    }

    /// Creates a tray menu submenu item specification.
    pub fn submenu<F>(
        mut self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
        f: F,
    ) -> Self
    where
        F: FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    {
        let builder = SubMenuBuilder {
            id: id.into(),
            label: label.into(),
            icon,
            children: vec![],
        };
        let result = f(builder);
        self.items.push(MenuNode::SubMenu(SubMenu {
            id: result.id,
            label: result.label,
            icon: result.icon,
            children: result.children,
        }));
        self
    }

    /// Sets the group value used by the component.
    pub fn group<F>(mut self, title: impl Into<SharedString>, f: F) -> Self
    where
        F: FnOnce(MenuGroupBuilder) -> MenuGroupBuilder,
    {
        let builder = MenuGroupBuilder {
            title: title.into(),
            children: vec![],
        };
        let result = f(builder);
        self.items.push(MenuNode::Group(MenuItemGroup {
            title: result.title,
            children: result.children,
        }));
        self
    }

    fn toggle_submenu(&mut self, id: SharedString, cx: &mut Context<Self>) {
        if self.opened_submenus.contains(&id) {
            self.opened_submenus.remove(&id);
        } else {
            self.opened_submenus.insert(id);
        }
        cx.notify();
    }

    fn select_item(&mut self, id: SharedString, window: &mut Window, cx: &mut App) -> bool {
        let changed = self.active_index.as_ref() != Some(&id);
        if changed {
            self.active_index = Some(id.clone());
        }
        if let Some(on_select) = &self.on_select {
            (on_select)(id, window, cx);
        }
        changed
    }

    fn render_node(
        &self,
        node: &MenuNode,
        depth: u32,
        theme: &liora_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        match self.mode {
            MenuMode::Vertical => match node {
                MenuNode::Item(item) => self.render_vertical_item(item, depth, theme, cx),
                MenuNode::SubMenu(submenu) => {
                    self.render_vertical_submenu(submenu, depth, theme, cx)
                }
                MenuNode::Group(group) => self.render_vertical_group(group, depth, theme, cx),
            },
            MenuMode::Horizontal => match node {
                MenuNode::Item(item) => self.render_horizontal_item(item, theme, cx),
                MenuNode::SubMenu(submenu) => self.render_horizontal_submenu(submenu, theme, cx),
                MenuNode::Group(group) => self.render_vertical_group(group, depth, theme, cx),
            },
        }
    }

    fn render_vertical_item(
        &self,
        item: &MenuItem,
        depth: u32,
        theme: &liora_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = item.id.clone();
        let is_active = self.active_index.as_ref() == Some(&id);
        let item_color = if is_active {
            theme.primary.base
        } else {
            theme.neutral.text_1
        };
        let padding_left = if self.is_collapsed {
            px(0.0)
        } else {
            px(20.0 + (depth as f32 * 20.0))
        };

        div()
            .id(element_id(format!("{}-item-{}", self.id, id)))
            .cursor_pointer()
            .block_mouse_except_scroll()
            .flex()
            .flex_none()
            .flex_row()
            .items_center()
            .justify_center()
            .when(!self.is_collapsed, |s| s.justify_start())
            .w_full()
            .h(px(50.0))
            .pl(padding_left)
            .pr(if self.is_collapsed { px(0.0) } else { px(16.0) })
            .text_color(item_color)
            .bg(if is_active {
                theme.primary.base.opacity(0.1)
            } else {
                gpui::transparent_black()
            })
            .hover(|s| s.bg(theme.neutral.hover))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.select_item(id.clone(), window, cx) {
                        cx.notify();
                    }
                }),
            )
            .when_some(item.icon, |s, icon| {
                s.child(Icon::new(icon).size(px(18.0)).color(item_color))
            })
            .when(!self.is_collapsed, |s| {
                s.child(div().ml_2().text_sm().child(item.label.clone()))
            })
            .into_any_element()
    }

    fn render_vertical_submenu(
        &self,
        submenu: &SubMenu,
        depth: u32,
        theme: &liora_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = submenu.id.clone();
        let is_open = self.opened_submenus.contains(&id);
        let submenu_color = theme.neutral.text_1;
        let padding_left = if self.is_collapsed {
            px(0.0)
        } else {
            px(20.0 + (depth as f32 * 20.0))
        };

        if self.is_collapsed {
            let menu_handle = cx.entity().clone();
            Popover::new(
                div()
                    .id(element_id(format!("{}-collapsed-submenu-{}", self.id, id)))
                    .cursor_pointer()
                    .block_mouse_except_scroll()
                    .flex()
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .w_full()
                    .h(px(50.0))
                    .text_color(submenu_color)
                    .hover(|s| s.bg(theme.neutral.hover))
                    .when_some(submenu.icon, |s, icon| {
                        s.child(Icon::new(icon).size(px(18.0)).color(submenu_color))
                    })
                    .when(submenu.icon.is_none(), |s| {
                        s.child(
                            div().text_sm().child(
                                submenu
                                    .label
                                    .clone()
                                    .to_string()
                                    .chars()
                                    .next()
                                    .unwrap_or('?')
                                    .to_string(),
                            ),
                        )
                    }),
            )
            .id(format!("{}-collapsed-popover-{}", self.id, id))
            .close_on_escape(self.close_on_escape)
            .placement(Placement::RightStart)
            .content({
                let popover_id: SharedString =
                    format!("{}-collapsed-popover-{}", self.id, id).into();
                let children: Vec<MenuItem> = submenu
                    .children
                    .iter()
                    .filter_map(|n| {
                        if let MenuNode::Item(i) = n {
                            Some(MenuItem {
                                id: i.id.clone(),
                                label: i.label.clone(),
                                icon: i.icon,
                            })
                        } else {
                            None
                        }
                    })
                    .collect();
                let theme = theme.clone();
                let popover_id = popover_id.clone();
                move |_window, _cx| {
                    let menu_handle = menu_handle.clone();
                    div()
                        .id(element_id(format!(
                            "menu-sub-popover-content-{}",
                            menu_handle.entity_id()
                        )))
                        .cursor_default()
                        .occlude()
                        .on_hover(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .on_mouse_move(|_, _, cx| {
                            cx.stop_propagation();
                        })
                        .flex()
                        .flex_col()
                        .p_1()
                        .min_w(px(160.0))
                        .children(children.iter().map(|item| {
                            let id = item.id.clone();
                            let label = item.label.clone();
                            let icon = item.icon;
                            let theme = theme.clone();
                            let menu_handle = menu_handle.clone();
                            let is_active =
                                menu_handle.read(_cx).active_index.as_ref() == Some(&id);
                            let item_color = if is_active {
                                theme.primary.base
                            } else {
                                theme.neutral.text_1
                            };
                            div()
                                .id(element_id(format!(
                                    "menu-sub-item-{}-{}",
                                    menu_handle.entity_id(),
                                    id
                                )))
                                .cursor_pointer()
                                .block_mouse_except_scroll()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .w_full()
                                .px_3()
                                .py_2()
                                .rounded(px(theme.radius.sm))
                                .text_color(item_color)
                                .bg(if is_active {
                                    theme.primary.base.opacity(0.1)
                                } else {
                                    gpui::transparent_black()
                                })
                                .hover(|s| s.bg(theme.neutral.hover))
                                .on_click({
                                    let popover_id = popover_id.clone();
                                    move |_, window, cx| {
                                        let _ = menu_handle.update(cx, |this, cx| {
                                            if this.select_item(id.clone(), window, cx) {
                                                cx.notify();
                                            }
                                        });
                                        liora_core::clear_popover(&popover_id, cx);
                                    }
                                })
                                .when_some(icon, |s, i| {
                                    s.child(Icon::new(i).size(px(16.0)).color(item_color))
                                })
                                .child(div().text_sm().child(label))
                        }))
                }
            })
            .into_any_element()
        } else {
            let toggle_id = id.clone();
            div()
                .flex()
                .flex_none()
                .flex_col()
                .child(
                    div()
                        .id(element_id(format!("{}-submenu-{}", self.id, id)))
                        .cursor_pointer()
                        .flex()
                        .flex_none()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .w_full()
                        .h(px(50.0))
                        .pl(padding_left)
                        .pr_4()
                        .text_color(submenu_color)
                        .hover(|s| s.bg(theme.neutral.hover))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.toggle_submenu(toggle_id.clone(), cx);
                        }))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .when_some(submenu.icon, |s, icon| {
                                    s.child(Icon::new(icon).size(px(18.0)).color(submenu_color))
                                })
                                .child(div().text_sm().child(submenu.label.clone())),
                        )
                        .child(
                            Icon::new(if is_open {
                                IconName::ChevronDown
                            } else {
                                IconName::ChevronRight
                            })
                            .size(px(14.0))
                            .color(submenu_color),
                        ),
                )
                .when(is_open, |s| {
                    s.child(pop_in(
                        element_id(format!("{}-submenu-motion-{}", self.id, id)),
                        div().flex().flex_col().children(
                            submenu
                                .children
                                .iter()
                                .map(|child| self.render_node(child, depth + 1, theme, cx)),
                        ),
                    ))
                })
                .into_any_element()
        }
    }

    fn render_vertical_group(
        &self,
        group: &MenuItemGroup,
        depth: u32,
        theme: &liora_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        if self.is_collapsed {
            return div().into_any_element();
        }
        let padding_left = px(20.0 + (depth as f32 * 20.0));

        div()
            .flex()
            .flex_none()
            .flex_col()
            .child(
                div()
                    .flex_none()
                    .h(px(30.0))
                    .pl(padding_left)
                    .flex()
                    .items_center()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.neutral.text_3)
                            .child(group.title.clone()),
                    ),
            )
            .children(
                group
                    .children
                    .iter()
                    .map(|child| self.render_node(child, depth, theme, cx)),
            )
            .into_any_element()
    }

    fn render_horizontal_item(
        &self,
        item: &MenuItem,
        theme: &liora_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = item.id.clone();
        let is_active = self.active_index.as_ref() == Some(&id);
        let item_color = if is_active {
            theme.primary.base
        } else {
            theme.neutral.text_1
        };

        div()
            .id(element_id(format!("{}-horizontal-item-{}", self.id, id)))
            .cursor_pointer()
            .block_mouse_except_scroll()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(60.0))
            .px_5()
            .text_color(item_color)
            .border_b_2()
            .border_color(if is_active {
                theme.primary.base
            } else {
                gpui::transparent_black()
            })
            .hover(|s| s.bg(theme.neutral.hover))
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _, window, cx| {
                    if this.select_item(id.clone(), window, cx) {
                        cx.notify();
                    }
                }),
            )
            .when_some(item.icon, |s, icon| {
                s.child(Icon::new(icon).size(px(18.0)).color(item_color))
            })
            .child(div().text_sm().child(item.label.clone()))
            .into_any_element()
    }

    fn render_horizontal_submenu(
        &self,
        submenu: &SubMenu,
        theme: &liora_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = submenu.id.clone();
        let menu_handle = cx.entity().clone();
        let submenu_color = theme.neutral.text_1;

        Popover::new(
            div()
                .id(element_id(format!("{}-horizontal-submenu-{}", self.id, id)))
                .cursor_pointer()
                .block_mouse_except_scroll()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                .h(px(60.0))
                .px_5()
                .text_color(submenu_color)
                .hover(|s| s.bg(theme.neutral.hover))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .when_some(submenu.icon, |s, icon| {
                            s.child(Icon::new(icon).size(px(18.0)).color(submenu_color))
                        })
                        .child(div().text_sm().child(submenu.label.clone()))
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .size(px(12.0))
                                .color(submenu_color),
                        ),
                ),
        )
        .id(format!("{}-horizontal-popover-{}", self.id, id))
        .close_on_escape(self.close_on_escape)
        .placement(Placement::BottomStart)
        .content({
            let popover_id: SharedString = format!("{}-horizontal-popover-{}", self.id, id).into();
            let children: Vec<MenuItem> = submenu
                .children
                .iter()
                .filter_map(|n| {
                    if let MenuNode::Item(i) = n {
                        Some(MenuItem {
                            id: i.id.clone(),
                            label: i.label.clone(),
                            icon: i.icon,
                        })
                    } else {
                        None
                    }
                })
                .collect();
            let theme = theme.clone();
            let popover_id = popover_id.clone();
            move |_window, _cx| {
                let menu_handle = menu_handle.clone();
                div()
                    .id(element_id(format!(
                        "menu-horiz-popover-content-{}",
                        menu_handle.entity_id()
                    )))
                    .cursor_default()
                    .occlude()
                    .on_hover(|_, _, cx| {
                        cx.stop_propagation();
                    })
                    .on_mouse_move(|_, _, cx| {
                        cx.stop_propagation();
                    })
                    .flex()
                    .flex_col()
                    .p_1()
                    .min_w(px(160.0))
                    .children(children.iter().map(|item| {
                        let id = item.id.clone();
                        let label = item.label.clone();
                        let icon = item.icon;
                        let theme = theme.clone();
                        let menu_handle = menu_handle.clone();
                        let is_active = menu_handle.read(_cx).active_index.as_ref() == Some(&id);
                        let item_color = if is_active {
                            theme.primary.base
                        } else {
                            theme.neutral.text_1
                        };
                        div()
                            .id(element_id(format!(
                                "menu-horiz-sub-item-{}-{}",
                                menu_handle.entity_id(),
                                id
                            )))
                            .cursor_pointer()
                            .block_mouse_except_scroll()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .w_full()
                            .px_3()
                            .py_2()
                            .rounded(px(theme.radius.sm))
                            .text_color(item_color)
                            .bg(if is_active {
                                theme.primary.base.opacity(0.1)
                            } else {
                                gpui::transparent_black()
                            })
                            .hover(|s| s.bg(theme.neutral.hover))
                            .on_click({
                                let popover_id = popover_id.clone();
                                move |_, window, cx| {
                                    let _ = menu_handle.update(cx, |this, cx| {
                                        if this.select_item(id.clone(), window, cx) {
                                            cx.notify();
                                        }
                                    });
                                    liora_core::clear_popover(&popover_id, cx);
                                }
                            })
                            .when_some(icon, |s, i| {
                                s.child(Icon::new(i).size(px(16.0)).color(item_color))
                            })
                            .child(div().text_sm().child(label))
                    }))
            }
        })
        .into_any_element()
    }
}

/// Fluent native GPUI component for rendering Liora sub menu builder.
pub struct SubMenuBuilder {
    /// Stable identifier used for GPUI state, callbacks, and automation.
    pub id: SharedString,
    /// User-facing label rendered for this item.
    pub label: SharedString,
    /// Optional icon rendered with the item.
    pub icon: Option<IconName>,
    /// Nested child items rendered beneath this item.
    pub children: Vec<MenuNode>,
}

impl SubMenuBuilder {
    /// Replaces all child menu nodes while constructing a submenu.
    pub fn with_items(mut self, items: Vec<MenuNode>) -> Self {
        self.children = items;
        self
    }

    /// Performs the item operation used by this component.
    pub fn item(
        mut self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
    ) -> Self {
        self.children.push(MenuNode::Item(MenuItem {
            id: id.into(),
            label: label.into(),
            icon,
        }));
        self
    }

    /// Creates a tray menu submenu item specification.
    pub fn submenu<F>(
        mut self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
        f: F,
    ) -> Self
    where
        F: FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    {
        let builder = SubMenuBuilder {
            id: id.into(),
            label: label.into(),
            icon,
            children: vec![],
        };
        let result = f(builder);
        self.children.push(MenuNode::SubMenu(SubMenu {
            id: result.id,
            label: result.label,
            icon: result.icon,
            children: result.children,
        }));
        self
    }

    /// Sets the group value used by the component.
    pub fn group<F>(mut self, title: impl Into<SharedString>, f: F) -> Self
    where
        F: FnOnce(MenuGroupBuilder) -> MenuGroupBuilder,
    {
        let builder = MenuGroupBuilder {
            title: title.into(),
            children: vec![],
        };
        let result = f(builder);
        self.children.push(MenuNode::Group(MenuItemGroup {
            title: result.title,
            children: result.children,
        }));
        self
    }
}

/// Fluent native GPUI component for rendering Liora menu group builder.
pub struct MenuGroupBuilder {
    /// Primary heading or title text displayed by the component.
    pub title: SharedString,
    /// Nested child items rendered beneath this item.
    pub children: Vec<MenuNode>,
}

impl MenuGroupBuilder {
    /// Replaces all child menu nodes while constructing a menu group.
    pub fn with_items(mut self, items: Vec<MenuNode>) -> Self {
        self.children = items;
        self
    }

    /// Performs the item operation used by this component.
    pub fn item(
        mut self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
    ) -> Self {
        self.children.push(MenuNode::Item(MenuItem {
            id: id.into(),
            label: label.into(),
            icon,
        }));
        self
    }

    /// Creates a tray menu submenu item specification.
    pub fn submenu<F>(
        mut self,
        id: impl Into<SharedString>,
        label: impl Into<SharedString>,
        icon: Option<IconName>,
        f: F,
    ) -> Self
    where
        F: FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    {
        let builder = SubMenuBuilder {
            id: id.into(),
            label: label.into(),
            icon,
            children: vec![],
        };
        let result = f(builder);
        self.children.push(MenuNode::SubMenu(SubMenu {
            id: result.id,
            label: result.label,
            icon: result.icon,
            children: result.children,
        }));
        self
    }
}

impl Render for Menu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();

        match self.mode {
            MenuMode::Vertical => Flex::new()
                .id(self.id.clone())
                .column()
                .w_full()
                .min_h_0()
                .flex_1()
                .overflow_y_scroll()
                .bg(theme.neutral.card)
                .children(
                    self.items
                        .iter()
                        .map(|node| self.render_node(node, 0, &theme, cx)),
                )
                .into_any_element(),
            MenuMode::Horizontal => div()
                .flex()
                .w_full()
                .bg(theme.neutral.card)
                .border_b_1()
                .border_color(theme.neutral.border)
                .flex_row()
                .children(
                    self.items
                        .iter()
                        .map(|node| self.render_node(node, 0, &theme, cx)),
                )
                .into_any_element(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn menu_nodes_are_comparable_so_item_refreshes_can_skip_noops() {
        let items = vec![MenuNode::Item(MenuItem {
            id: "one".into(),
            label: "One".into(),
            icon: None,
        })];

        assert!(items == items.clone());
    }

    #[test]
    fn menu_source_keeps_items_full_width_and_avoids_redundant_select_notify() {
        let source = include_str!("menu.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap();

        assert!(source.contains("if self.items == items"));
        assert!(source.contains("fn select_item(&mut self"));
        assert!(source.contains("-> bool"));
        assert!(source.contains("let changed = self.active_index.as_ref() != Some(&id);"));
        assert!(source.contains("if changed {"));
        assert!(source.contains(".on_mouse_down("));
        assert!(source.contains(".block_mouse_except_scroll()"));
        assert!(source.contains("MouseButton::Left"));
        assert!(source.contains("if this.select_item(id.clone(), window, cx)"));
        assert!(source.contains(".w_full()"));
        assert!(source.contains(".overflow_y_scroll()"));
        assert!(source.contains(".min_h_0()"));
        assert!(source.contains("Flex::new()"));
        assert!(source.contains(".id(self.id.clone())"));
        assert!(
            source.matches(".flex_none()").count() >= 4,
            "vertical menu rows must opt out of flex shrinking so fixed-height items create real overflow for Menu-owned scrolling"
        );
        assert!(
            !source.contains(
                "this.select_item(id.clone(), window, cx);\n                cx.notify();"
            )
        );
    }
}
