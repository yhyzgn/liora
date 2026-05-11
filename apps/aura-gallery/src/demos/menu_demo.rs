use aura_components::{Card, Col, Menu, MenuMode, Row, Space, Text};
use aura_core::Config;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Entity, Render, Window, prelude::*};

use aura_components::layout_helpers::{page, section};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| {
        let horizontal_content = cx.new(|_| MenuContent::new("水平模式", "1"));
        let vertical_content = cx.new(|_| MenuContent::new("垂直模式", "1"));
        let collapsed_content = cx.new(|_| MenuContent::new("折叠模式", "1"));

        MenuDemo {
            horizontal: cx.new({
                let content = horizontal_content.clone();
                |_| {
                    Menu::new()
                        .id("menu-demo-horizontal")
                        .mode(MenuMode::Horizontal)
                        .default_active("1")
                        .on_select(move |id, _, cx| {
                            let _ = content.update(cx, |content, cx| {
                                content.set_active("水平模式", id.as_ref());
                                cx.notify();
                            });
                        })
                        .item("1", "处理中心", Some(IconName::List))
                        .submenu("2", "我的工作台", Some(IconName::Briefcase), |s| {
                            s.item("2-1", "选项1", None)
                                .item("2-2", "选项2", None)
                                .item("2-3", "选项3", None)
                        })
                        .item("3", "消息中心", Some(IconName::Bell))
                        .item("4", "订单管理", Some(IconName::FileText))
                }
            }),
            vertical: cx.new({
                let content = vertical_content.clone();
                |_| {
                    Menu::new()
                        .id("menu-demo-vertical")
                        .mode(MenuMode::Vertical)
                        .default_active("1")
                        .on_select(move |id, _, cx| {
                            let _ = content.update(cx, |content, cx| {
                                content.set_active("垂直模式", id.as_ref());
                                cx.notify();
                            });
                        })
                        .item("1", "导航一", Some(IconName::House))
                        .submenu("2", "导航二", Some(IconName::Settings), |s| {
                            s.item("2-1", "选项1", None)
                                .item("2-2", "选项2", None)
                                .group("分组一", |g| {
                                    g.item("2-3", "选项3", None).item("2-4", "选项4", None)
                                })
                        })
                        .item("3", "导航三", Some(IconName::MessageSquare))
                }
            }),
            collapsed: cx.new({
                let content = collapsed_content.clone();
                |_| {
                    Menu::new()
                        .id("menu-demo-collapsed")
                        .mode(MenuMode::Vertical)
                        .collapse(true)
                        .default_active("1")
                        .on_select(move |id, _, cx| {
                            let _ = content.update(cx, |content, cx| {
                                content.set_active("折叠模式", id.as_ref());
                                cx.notify();
                            });
                        })
                        .item("1", "导航一", Some(IconName::House))
                        .submenu("2", "导航二", Some(IconName::Settings), |s| {
                            s.item("2-1", "选项1", None).item("2-2", "选项2", None)
                        })
                        .item("3", "导航三", Some(IconName::MessageSquare))
                }
            }),
            horizontal_content,
            vertical_content,
            collapsed_content,
        }
    })
    .into()
}

struct MenuDemo {
    horizontal: Entity<Menu>,
    vertical: Entity<Menu>,
    collapsed: Entity<Menu>,
    horizontal_content: Entity<MenuContent>,
    vertical_content: Entity<MenuContent>,
    collapsed_content: Entity<MenuContent>,
}

struct MenuContent {
    scope: String,
    active_id: String,
    title: String,
    description: String,
}

impl MenuContent {
    fn new(scope: &str, active_id: &str) -> Self {
        let mut content = Self {
            scope: String::new(),
            active_id: String::new(),
            title: String::new(),
            description: String::new(),
        };
        content.set_active(scope, active_id);
        content
    }

    fn set_active(&mut self, scope: &str, active_id: &str) {
        let (title, description) = menu_content(active_id);
        self.scope = scope.to_string();
        self.active_id = active_id.to_string();
        self.title = title.to_string();
        self.description = description.to_string();
    }
}

fn menu_content(id: &str) -> (&'static str, &'static str) {
    match id {
        "1" => (
            "导航一 / 处理中心",
            "展示当前用户最常用的处理入口和快捷操作。",
        ),
        "2-1" => ("工作台 · 选项1", "这里可以放置待办任务、审批流和今日事项。"),
        "2-2" => (
            "工作台 · 选项2",
            "这里可以展示项目统计、配置表单或二级页面内容。",
        ),
        "2-3" => ("工作台 · 选项3", "这里可以展示分组下的业务详情或数据报表。"),
        "2-4" => ("工作台 · 选项4", "这里可以展示更多分组菜单对应的内容区域。"),
        "3" => (
            "消息中心 / 导航三",
            "展示通知、消息列表和需要关注的系统动态。",
        ),
        "4" => ("订单管理", "展示订单筛选、列表和管理操作入口。"),
        _ => ("菜单内容", "点击不同菜单项后，这里会切换为对应内容。"),
    }
}

impl Render for MenuContent {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;

        Card::new(
            Space::new()
                .vertical()
                .gap_md()
                .child(
                    Text::new(format!("{} · active = {}", self.scope, self.active_id))
                        .text_color(theme.neutral.text_3)
                        .nowrap(),
                )
                .child(
                    Text::new(self.title.clone())
                        .text_color(theme.neutral.text_1)
                        .bold(),
                )
                .child(Text::new(self.description.clone()).text_color(theme.neutral.text_2)),
        )
        .no_shadow()
    }
}

impl Render for MenuDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "Menu 导航菜单",
            "为网站提供导航轮廓。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "水平模式",
                    "适用于顶部导航，点击菜单项后内容区域同步切换。",
                    Space::new()
                        .vertical()
                        .gap_md()
                        .child(self.horizontal.clone())
                        .child(self.horizontal_content.clone()),
                ))
                .child(section(
                    "垂直与折叠模式",
                    "垂直菜单适合侧边导航，折叠菜单适合紧凑侧栏。",
                    Space::new()
                        .vertical()
                        .gap_lg()
                        .child(
                            Row::new()
                                .column(
                                    Col::new(4).child(
                                        Space::new()
                                            .vertical()
                                            .gap_md()
                                            .child(Text::new("垂直模式").bold())
                                            .child(self.vertical.clone()),
                                    ),
                                )
                                .column(Col::new(20).child(self.vertical_content.clone())),
                        )
                        .child(
                            Row::new()
                                .column(
                                    Col::new(2).child(
                                        Space::new()
                                            .vertical()
                                            .gap_md()
                                            .child(Text::new("折叠").bold())
                                            .child(self.collapsed.clone()),
                                    ),
                                )
                                .column(Col::new(22).child(self.collapsed_content.clone())),
                        ),
                )),
        )
    }
}
