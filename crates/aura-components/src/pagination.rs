use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px};

pub struct Pagination {
    total: usize,
    page_size: usize,
    current_page: usize,
    background: bool,
    layout: SharedString,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Pagination {
    pub fn new(total: usize) -> Self {
        Self {
            total,
            page_size: 10,
            current_page: 1,
            background: false,
            layout: "prev, pager, next".into(),
            on_change: None,
        }
    }

    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(1);
        self
    }

    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    pub fn background(mut self, bg: bool) -> Self {
        self.background = bg;
        self
    }

    pub fn layout(mut self, l: impl Into<SharedString>) -> Self {
        self.layout = l.into();
        self
    }

    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    fn change_page(&mut self, page: usize, window: &mut Window, cx: &mut Context<Self>) {
        let max_page = self.page_count();
        let page = page.clamp(1, max_page.max(1));
        if page != self.current_page {
            self.current_page = page;
            if let Some(ref on_change) = self.on_change {
                (on_change)(page, window, cx);
            }
            cx.notify();
        }
    }

    fn page_count(&self) -> usize {
        (self.total as f32 / self.page_size as f32).ceil() as usize
    }
}

enum PagerItem {
    Page(usize),
    PrevMore,
    NextMore,
}

impl Render for Pagination {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let page_count = self.page_count();
        let current_page = self.current_page;
        let background = self.background;

        let render_btn = |text: Option<SharedString>,
                          icon: Option<IconName>,
                          active: bool,
                          disabled: bool,
                          _cx: &mut Context<Self>| {
            let bg_color = if background {
                if active {
                    theme.primary.base
                } else if disabled {
                    theme.neutral.hover
                } else {
                    theme.neutral.border
                }
            } else {
                gpui::transparent_black()
            };

            let text_color = if disabled {
                theme.neutral.text_3
            } else if active {
                if background {
                    gpui::white()
                } else {
                    theme.primary.base
                }
            } else {
                theme.neutral.text_2
            };

            div()
                .flex()
                .items_center()
                .justify_center()
                .min_w(px(32.0))
                .h(px(32.0))
                .px_1()
                .bg(bg_color)
                .rounded(px(theme.radius.sm))
                .text_color(text_color)
                .when(!disabled && !active, |s| {
                    s.cursor_pointer()
                        .hover(|s| s.text_color(theme.primary.base))
                })
                .when_some(text, |s, t| {
                    s.child(
                        div()
                            .text_sm()
                            .font_weight(if active {
                                gpui::FontWeight::BOLD
                            } else {
                                gpui::FontWeight::NORMAL
                            })
                            .child(t),
                    )
                })
                .when_some(icon, |s, i| {
                    s.child(Icon::new(i).size(px(14.0)).color(text_color))
                })
                .into_any_element()
        };

        // Calculate pagers
        let mut pagers = vec![];
        if page_count <= 7 {
            for i in 1..=page_count {
                pagers.push(PagerItem::Page(i));
            }
        } else {
            if current_page < 5 {
                for i in 1..=5 {
                    pagers.push(PagerItem::Page(i));
                }
                pagers.push(PagerItem::NextMore);
                pagers.push(PagerItem::Page(page_count));
            } else if current_page >= page_count - 3 {
                pagers.push(PagerItem::Page(1));
                pagers.push(PagerItem::PrevMore);
                for i in (page_count - 4)..=page_count {
                    pagers.push(PagerItem::Page(i));
                }
            } else {
                pagers.push(PagerItem::Page(1));
                pagers.push(PagerItem::PrevMore);
                pagers.push(PagerItem::Page(current_page - 1));
                pagers.push(PagerItem::Page(current_page));
                pagers.push(PagerItem::Page(current_page + 1));
                pagers.push(PagerItem::NextMore);
                pagers.push(PagerItem::Page(page_count));
            }
        }

        let parts = self.layout.to_string();
        let mut container = div().flex().flex_row().items_center().gap_2();

        for part in parts.split(',') {
            let part = part.trim();
            match part {
                "total" => {
                    container = container.child(
                        div()
                            .text_sm()
                            .text_color(theme.neutral.text_3)
                            .child(format!("共 {} 条", self.total)),
                    );
                }
                "prev" => {
                    let disabled = current_page <= 1;
                    container = container.child(
                        div()
                            .id("prev-btn")
                            .child(render_btn(
                                None,
                                Some(IconName::ChevronLeft),
                                false,
                                disabled,
                                cx,
                            ))
                            .when(!disabled, |s| {
                                s.on_click(cx.listener({
                                    move |this, _, window, cx| {
                                        let current = this.current_page;
                                        this.change_page(current.saturating_sub(1), window, cx);
                                    }
                                }))
                            }),
                    );
                }
                "pager" => {
                    for item in &pagers {
                        match item {
                            PagerItem::Page(p) => {
                                let p = *p;
                                let active = p == current_page;
                                container = container.child(
                                    div()
                                        .id(format!("pager-{}", p))
                                        .child(render_btn(
                                            Some(p.to_string().into()),
                                            None,
                                            active,
                                            false,
                                            cx,
                                        ))
                                        .when(!active, |s| {
                                            s.on_click(cx.listener(move |this, _, window, cx| {
                                                this.change_page(p, window, cx);
                                            }))
                                        }),
                                );
                            }
                            PagerItem::PrevMore => {
                                container = container.child(
                                    div()
                                        .id("prev-more")
                                        .child(render_btn(
                                            None,
                                            Some(IconName::Ellipsis),
                                            false,
                                            false,
                                            cx,
                                        ))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            let current = this.current_page;
                                            this.change_page(current.saturating_sub(5), window, cx);
                                        })),
                                );
                            }
                            PagerItem::NextMore => {
                                container = container.child(
                                    div()
                                        .id("next-more")
                                        .child(render_btn(
                                            None,
                                            Some(IconName::Ellipsis),
                                            false,
                                            false,
                                            cx,
                                        ))
                                        .on_click(cx.listener(move |this, _, window, cx| {
                                            let current = this.current_page;
                                            this.change_page(current + 5, window, cx);
                                        })),
                                );
                            }
                        }
                    }
                }
                "next" => {
                    let disabled = current_page >= page_count;
                    container = container.child(
                        div()
                            .id("next-btn")
                            .child(render_btn(
                                None,
                                Some(IconName::ChevronRight),
                                false,
                                disabled,
                                cx,
                            ))
                            .when(!disabled, |s| {
                                s.on_click(cx.listener({
                                    move |this, _, window, cx| {
                                        let current = this.current_page;
                                        this.change_page(current + 1, window, cx);
                                    }
                                }))
                            }),
                    );
                }
                "jumper" => {
                    container = container.child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.neutral.text_3)
                                    .child("前往"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .w(px(50.0))
                                    .h(px(32.0))
                                    .border_1()
                                    .border_color(theme.neutral.border)
                                    .rounded(px(theme.radius.sm))
                                    .child(
                                        div()
                                            .text_sm()
                                            .text_color(theme.neutral.text_1)
                                            .child(current_page.to_string()),
                                    ),
                            )
                            .child(div().text_sm().text_color(theme.neutral.text_3).child("页")),
                    );
                }
                _ => {}
            }
        }

        container
    }
}
