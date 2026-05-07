use crate::Popover;
use aura_core::{Config, Placement};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px,
};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MenuMode {
    #[default]
    Vertical,
    Horizontal,
}

pub enum MenuNode {
    Item(MenuItem),
    SubMenu(SubMenu),
    Group(MenuItemGroup),
}

pub struct MenuItem {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconName>,
}

pub struct SubMenu {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub children: Vec<MenuNode>,
}

pub struct MenuItemGroup {
    pub title: SharedString,
    pub children: Vec<MenuNode>,
}

pub struct Menu {
    id: SharedString,
    mode: MenuMode,
    is_collapsed: bool,
    active_index: Option<SharedString>,
    opened_submenus: HashSet<SharedString>,
    items: Vec<MenuNode>,
    on_select: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
}

impl Menu {
    #[track_caller]
    pub fn new() -> Self {
        let caller = std::panic::Location::caller();
        Self {
            id: format!("menu-{}", caller).into(),
            mode: MenuMode::Vertical,
            is_collapsed: false,
            active_index: None,
            opened_submenus: HashSet::new(),
            items: vec![],
            on_select: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn mode(mut self, mode: MenuMode) -> Self {
        self.mode = mode;
        self
    }

    pub fn collapse(mut self, collapsed: bool) -> Self {
        self.is_collapsed = collapsed;
        self
    }

    pub fn default_active(mut self, index: impl Into<SharedString>) -> Self {
        self.active_index = Some(index.into());
        self
    }

    pub fn on_select(mut self, f: impl Fn(SharedString, &mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(f));
        self
    }

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

    fn select_item(&mut self, id: SharedString, window: &mut Window, cx: &mut App) {
        self.active_index = Some(id.clone());
        if let Some(on_select) = &self.on_select {
            (on_select)(id, window, cx);
        }
    }

    fn render_node(
        &self,
        node: &MenuNode,
        depth: u32,
        theme: &aura_theme::Theme,
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
        theme: &aura_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = item.id.clone();
        let is_active = self.active_index.as_ref() == Some(&id);
        let padding_left = if self.is_collapsed {
            px(0.0)
        } else {
            px(20.0 + (depth as f32 * 20.0))
        };

        div()
            .id(format!("{}-item-{}", self.id, id))
            .cursor_pointer()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .when(!self.is_collapsed, |s| s.justify_start())
            .h(px(50.0))
            .pl(padding_left)
            .pr(if self.is_collapsed { px(0.0) } else { px(16.0) })
            .text_color(if is_active {
                theme.primary.base
            } else {
                theme.neutral.text_1
            })
            .bg(if is_active {
                theme.primary.base.opacity(0.1)
            } else {
                gpui::transparent_black()
            })
            .hover(|s| s.bg(theme.neutral.hover))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.select_item(id.clone(), window, cx);
                cx.notify();
            }))
            .when_some(item.icon, |s, icon| {
                s.child(Icon::new(icon).size(px(18.0)).color(if is_active {
                    theme.primary.base
                } else {
                    theme.neutral.icon
                }))
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
        theme: &aura_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = submenu.id.clone();
        let is_open = self.opened_submenus.contains(&id);
        let padding_left = if self.is_collapsed {
            px(0.0)
        } else {
            px(20.0 + (depth as f32 * 20.0))
        };

        if self.is_collapsed {
            let menu_handle = cx.entity().clone();
            Popover::new(
                div()
                    .id(format!("{}-collapsed-submenu-{}", self.id, id))
                    .cursor_pointer()
                    .flex()
                    .items_center()
                    .justify_center()
                    .h(px(50.0))
                    .w_full()
                    .text_color(theme.neutral.text_1)
                    .hover(|s| s.bg(theme.neutral.hover))
                    .when_some(submenu.icon, |s, icon| {
                        s.child(Icon::new(icon).size(px(18.0)).color(theme.neutral.icon))
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
            .placement(Placement::RightStart)
            .content({
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
                move |_window, _cx| {
                    let menu_handle = menu_handle.clone();
                    div()
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
                            div()
                                .id(format!("menu-sub-item-{}-{}", menu_handle.entity_id(), id))
                                .cursor_pointer()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py_2()
                                .rounded(px(theme.radius.sm))
                                .hover(|s| s.bg(theme.neutral.hover))
                                .on_click(move |_, window, cx| {
                                    let _ = menu_handle.update(cx, |this, cx| {
                                        this.select_item(id.clone(), window, cx);
                                        cx.notify();
                                    });
                                })
                                .when_some(icon, |s, i| {
                                    s.child(Icon::new(i).size(px(16.0)).color(theme.neutral.icon))
                                })
                                .child(div().text_sm().child(label))
                        }))
                }
            })
            .into_any_element()
        } else {
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .id(format!("{}-submenu-{}", self.id, id))
                        .cursor_pointer()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .h(px(50.0))
                        .pl(padding_left)
                        .pr_4()
                        .text_color(theme.neutral.text_1)
                        .hover(|s| s.bg(theme.neutral.hover))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.toggle_submenu(id.clone(), cx);
                        }))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .when_some(submenu.icon, |s, icon| {
                                    s.child(
                                        Icon::new(icon).size(px(18.0)).color(theme.neutral.icon),
                                    )
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
                            .color(theme.neutral.icon),
                        ),
                )
                .when(is_open, |s| {
                    s.children(
                        submenu
                            .children
                            .iter()
                            .map(|child| self.render_node(child, depth + 1, theme, cx)),
                    )
                })
                .into_any_element()
        }
    }

    fn render_vertical_group(
        &self,
        group: &MenuItemGroup,
        depth: u32,
        theme: &aura_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        if self.is_collapsed {
            return div().into_any_element();
        }
        let padding_left = px(20.0 + (depth as f32 * 20.0));

        div()
            .flex()
            .flex_col()
            .child(
                div()
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
        theme: &aura_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = item.id.clone();
        let is_active = self.active_index.as_ref() == Some(&id);

        div()
            .id(format!("{}-horizontal-item-{}", self.id, id))
            .cursor_pointer()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .h(px(60.0))
            .px_5()
            .text_color(if is_active {
                theme.primary.base
            } else {
                theme.neutral.text_1
            })
            .border_b_2()
            .border_color(if is_active {
                theme.primary.base
            } else {
                gpui::transparent_black()
            })
            .hover(|s| s.text_color(theme.primary.base))
            .on_click(cx.listener(move |this, _, window, cx| {
                this.select_item(id.clone(), window, cx);
                cx.notify();
            }))
            .when_some(item.icon, |s, icon| {
                s.child(Icon::new(icon).size(px(18.0)).color(if is_active {
                    theme.primary.base
                } else {
                    theme.neutral.icon
                }))
            })
            .child(div().text_sm().child(item.label.clone()))
            .into_any_element()
    }

    fn render_horizontal_submenu(
        &self,
        submenu: &SubMenu,
        theme: &aura_theme::Theme,
        cx: &Context<Self>,
    ) -> AnyElement {
        let id = submenu.id.clone();
        let menu_handle = cx.entity().clone();

        Popover::new(
            div()
                .id(format!("{}-horizontal-submenu-{}", self.id, id))
                .cursor_pointer()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                .h(px(60.0))
                .px_5()
                .text_color(theme.neutral.text_1)
                .hover(|s| s.text_color(theme.primary.base))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .when_some(submenu.icon, |s, icon| {
                            s.child(Icon::new(icon).size(px(18.0)).color(theme.neutral.icon))
                        })
                        .child(div().text_sm().child(submenu.label.clone()))
                        .child(
                            Icon::new(IconName::ChevronDown)
                                .size(px(12.0))
                                .color(theme.neutral.icon),
                        ),
                ),
        )
        .id(format!("{}-horizontal-popover-{}", self.id, id))
        .placement(Placement::BottomStart)
        .content({
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
            move |_window, _cx| {
                let menu_handle = menu_handle.clone();
                div()
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
                        div()
                            .id(format!(
                                "menu-horiz-sub-item-{}-{}",
                                menu_handle.entity_id(),
                                id
                            ))
                            .cursor_pointer()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .px_3()
                            .py_2()
                            .rounded(px(theme.radius.sm))
                            .hover(|s| s.bg(theme.neutral.hover))
                            .on_click(move |_, window, cx| {
                                let _ = menu_handle.update(cx, |this, cx| {
                                    this.select_item(id.clone(), window, cx);
                                    cx.notify();
                                });
                            })
                            .when_some(icon, |s, i| {
                                s.child(Icon::new(i).size(px(16.0)).color(theme.neutral.icon))
                            })
                            .child(div().text_sm().child(label))
                    }))
            }
        })
        .into_any_element()
    }
}

pub struct SubMenuBuilder {
    pub id: SharedString,
    pub label: SharedString,
    pub icon: Option<IconName>,
    pub children: Vec<MenuNode>,
}

impl SubMenuBuilder {
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

pub struct MenuGroupBuilder {
    pub title: SharedString,
    pub children: Vec<MenuNode>,
}

impl MenuGroupBuilder {
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

        div()
            .flex()
            .w_full()
            .bg(theme.neutral.card)
            .when(self.mode == MenuMode::Vertical, |s| s.flex_col())
            .when(self.mode == MenuMode::Horizontal, |s| {
                s.flex_row().border_b_1().border_color(theme.neutral.border)
            })
            .children(
                self.items
                    .iter()
                    .map(|node| self.render_node(node, 0, &theme, cx)),
            )
    }
}
