use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{
    AnyElement, App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px,
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabType {
    #[default]
    Standard,
    Card,
    BorderCard,
}

pub struct TabPane {
    pub name: SharedString,
    pub label: SharedString,
    pub content: Arc<dyn Fn(&mut Window, &mut Context<Tabs>) -> AnyElement + 'static>,
    pub closable: bool,
    pub icon: Option<IconName>,
}

pub struct Tabs {
    id: SharedString,
    active_name: SharedString,
    position: TabPosition,
    tab_type: TabType,
    panes: Vec<TabPane>,
    editable: bool,
    stretch: bool,
    on_tab_click: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_remove: Option<Box<dyn Fn(SharedString, &mut Window, &mut App) + 'static>>,
    on_tab_add: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl Tabs {
    #[track_caller]
    pub fn new(active_name: impl Into<SharedString>) -> Self {
        let caller = std::panic::Location::caller();
        let name = active_name.into();
        Self {
            id: format!("tabs-{}", caller).into(),
            active_name: name,
            position: TabPosition::Top,
            tab_type: TabType::Standard,
            panes: vec![],
            editable: false,
            stretch: false,
            on_tab_click: None,
            on_tab_remove: None,
            on_tab_add: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    pub fn position(mut self, pos: TabPosition) -> Self {
        self.position = pos;
        self
    }

    pub fn type_(mut self, t: TabType) -> Self {
        self.tab_type = t;
        self
    }

    pub fn editable(mut self, e: bool) -> Self {
        self.editable = e;
        self
    }

    pub fn stretch(mut self, stretch: bool) -> Self {
        self.stretch = stretch;
        self
    }

    pub fn on_tab_click(
        mut self,
        f: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tab_click = Some(Box::new(f));
        self
    }

    pub fn on_tab_remove(
        mut self,
        f: impl Fn(SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_tab_remove = Some(Box::new(f));
        self
    }

    pub fn on_tab_add(mut self, f: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_tab_add = Some(Box::new(f));
        self
    }

    pub fn pane<F, E>(
        mut self,
        name: impl Into<SharedString>,
        label: impl Into<SharedString>,
        f: F,
    ) -> Self
    where
        F: Fn(&mut Window, &mut Context<Self>) -> E + 'static,
        E: IntoElement,
    {
        self.panes.push(TabPane {
            name: name.into(),
            label: label.into(),
            content: Arc::new(move |window, cx| f(window, cx).into_any_element()),
            closable: true,
            icon: None,
        });
        self
    }

    fn select_tab(&mut self, name: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        self.active_name = name.clone();
        if let Some(on_click) = &self.on_tab_click {
            (on_click)(name, window, cx);
        }
        cx.notify();
    }

    fn remove_tab(&mut self, name: SharedString, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(pos) = self.panes.iter().position(|p| p.name == name) {
            self.panes.remove(pos);
            if self.active_name == name {
                if let Some(new_active) =
                    self.panes.get(pos.min(self.panes.len().saturating_sub(1)))
                {
                    self.active_name = new_active.name.clone();
                }
            }
        }
        if let Some(on_remove) = &self.on_tab_remove {
            (on_remove)(name, window, cx);
        }
        cx.notify();
    }

    fn add_tab(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let mut next = self.panes.len() + 1;
        let name = loop {
            let candidate = SharedString::from(format!("tab-{}", next));
            if self.panes.iter().all(|pane| pane.name != candidate) {
                break candidate;
            }
            next += 1;
        };
        let label = SharedString::from(format!("Tab {}", next));
        let content_text = SharedString::from(format!("Content of Tab {}", next));

        self.panes.push(TabPane {
            name: name.clone(),
            label,
            content: Arc::new(move |_, _| div().child(content_text.clone()).into_any_element()),
            closable: true,
            icon: None,
        });
        self.active_name = name;

        if let Some(on_add) = &self.on_tab_add {
            (on_add)(window, cx);
        }
        cx.notify();
    }
}

impl Render for Tabs {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let tab_type = self.tab_type;
        let position = self.position;
        let is_vertical = position == TabPosition::Left || position == TabPosition::Right;

        let render_header = |this: &Self, cx: &mut Context<Self>| {
            let theme = cx.global::<Config>().theme.clone();
            div()
                .flex()
                .when(!is_vertical, |s| s.flex_row().items_center().w_full())
                .when(is_vertical, |s| s.flex_col().w(px(120.0)))
                .when(tab_type == TabType::Standard, |s| match position {
                    TabPosition::Top => s
                        .when(!this.stretch, |s| s.gap_8())
                        .border_b_1()
                        .border_color(theme.neutral.border),
                    TabPosition::Bottom => s
                        .when(!this.stretch, |s| s.gap_8())
                        .border_t_1()
                        .border_color(theme.neutral.border),
                    TabPosition::Left => s.gap_2().border_r_1().border_color(theme.neutral.border),
                    TabPosition::Right => s.gap_2().border_l_1().border_color(theme.neutral.border),
                })
                .when(
                    tab_type == TabType::Card || tab_type == TabType::BorderCard,
                    |s| {
                        s.bg(theme.neutral.hover)
                            .border_b_1()
                            .border_color(theme.neutral.border)
                    },
                )
                .children(this.panes.iter().map(|pane| {
                    let name = pane.name.clone();
                    let is_active = this.active_name == name;
                    let closable = pane.closable;

                    div()
                        .id(format!("{}-tab-{}", this.id, name))
                        .cursor_pointer()
                        .flex()
                        .items_center()
                        .justify_center()
                        .when(this.stretch && !is_vertical, |s| s.flex_1())
                        .when(!is_vertical, |s| s.h(px(40.0)))
                        .when(is_vertical, |s| s.w_full().py_3())
                        .when(tab_type == TabType::Standard, |s| {
                            s.px_2()
                                .text_color(if is_active {
                                    theme.primary.base
                                } else {
                                    theme.neutral.text_1
                                })
                                .hover(|s| s.text_color(theme.primary.base))
                                .when(is_active, |s| match position {
                                    TabPosition::Top => s.child(
                                        div()
                                            .absolute()
                                            .bottom_0()
                                            .w_full()
                                            .h(px(2.0))
                                            .bg(theme.primary.base),
                                    ),
                                    TabPosition::Bottom => s.child(
                                        div()
                                            .absolute()
                                            .top_0()
                                            .w_full()
                                            .h(px(2.0))
                                            .bg(theme.primary.base),
                                    ),
                                    TabPosition::Left => s.child(
                                        div()
                                            .absolute()
                                            .right_0()
                                            .h_full()
                                            .w(px(2.0))
                                            .bg(theme.primary.base),
                                    ),
                                    TabPosition::Right => s.child(
                                        div()
                                            .absolute()
                                            .left_0()
                                            .h_full()
                                            .w(px(2.0))
                                            .bg(theme.primary.base),
                                    ),
                                })
                        })
                        .when(
                            tab_type == TabType::Card || tab_type == TabType::BorderCard,
                            |s| {
                                s.px_5()
                                    .border_r_1()
                                    .border_color(theme.neutral.border)
                                    .bg(if is_active {
                                        theme.neutral.card
                                    } else {
                                        gpui::transparent_black()
                                    })
                                    .text_color(if is_active {
                                        theme.primary.base
                                    } else {
                                        theme.neutral.text_1
                                    })
                                    .hover(|s| s.text_color(theme.primary.base))
                                    .when(is_active, |s| {
                                        s.border_b_1().border_color(theme.neutral.card).mb(px(-1.0))
                                    })
                            },
                        )
                        .on_click(cx.listener({
                            let name = name.clone();
                            move |this, _, window, cx| {
                                this.select_tab(name.clone(), window, cx);
                            }
                        }))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(div().text_sm().child(pane.label.clone()))
                                .when(closable && this.editable, |s| {
                                    s.child(
                                        div()
                                            .id(format!("{}-close-{}", this.id, name))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .w_4()
                                            .h_4()
                                            .rounded_full()
                                            .hover(|s| s.bg(theme.neutral.hover))
                                            .on_click(cx.listener({
                                                let name = name.clone();
                                                move |this, _, window, cx| {
                                                    this.remove_tab(name.clone(), window, cx);
                                                }
                                            }))
                                            .child(
                                                Icon::new(IconName::X)
                                                    .size(px(12.0))
                                                    .color(theme.neutral.icon),
                                            ),
                                    )
                                }),
                        )
                }))
                .when(this.editable, |s| {
                    s.child(
                        div()
                            .id(format!("{}-add-tab", this.id))
                            .cursor_pointer()
                            .flex()
                            .items_center()
                            .justify_center()
                            .w_10()
                            .h_10()
                            .hover(|s| s.text_color(theme.primary.base))
                            .on_click(cx.listener(move |this, _, window, cx| {
                                this.add_tab(window, cx);
                            }))
                            .child(
                                Icon::new(IconName::Plus)
                                    .size(px(16.0))
                                    .color(theme.neutral.icon),
                            ),
                    )
                })
        };

        let content = self
            .panes
            .iter()
            .find(|p| p.name == self.active_name)
            .map(|p| (p.content)(_window, cx))
            .unwrap_or_else(|| div().into_any_element());

        div()
            .flex()
            .w_full()
            .h_full()
            .when(!is_vertical, |s| s.flex_col())
            .when(is_vertical, |s| s.flex_row())
            .when(tab_type == TabType::BorderCard, |s| {
                s.border_1()
                    .border_color(theme.neutral.border)
                    .rounded(px(theme.radius.md))
                    .overflow_hidden()
            })
            .bg(theme.neutral.card)
            .child(match position {
                TabPosition::Top | TabPosition::Left => render_header(self, cx).into_any_element(),
                _ => div().into_any_element(),
            })
            .child(div().flex_1().p_4().child(content))
            .child(match position {
                TabPosition::Bottom | TabPosition::Right => {
                    render_header(self, cx).into_any_element()
                }
                _ => div().into_any_element(),
            })
    }
}
