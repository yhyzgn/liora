use crate::Select;
use aura_core::Config;
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px};

pub struct Pagination {
    id: SharedString,
    total: usize,
    page_size: usize,
    current_page: usize,
    background: bool,
    layout: SharedString,
    page_sizes: Vec<usize>,
    page_size_picker: Option<gpui::Entity<Select>>,
    on_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    on_page_size_change: Option<Box<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl Pagination {
    pub fn new(total: usize) -> Self {
        Self {
            id: aura_core::unique_id("pagination"),
            total,
            page_size: 10,
            current_page: 1,
            background: false,
            layout: "prev, pager, next".into(),
            page_sizes: vec![],
            page_size_picker: None,
            on_change: None,
            on_page_size_change: None,
        }
    }

    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
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

    pub fn page_sizes(mut self, sizes: impl Into<Vec<usize>>) -> Self {
        self.page_sizes = sizes.into().into_iter().map(|s| s.max(1)).collect();
        self.page_sizes.sort_unstable();
        self.page_sizes.dedup();
        self
    }

    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    pub fn on_page_size_change(
        mut self,
        f: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_page_size_change = Some(Box::new(f));
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

    fn change_page_size(&mut self, size: usize, window: &mut Window, cx: &mut Context<Self>) {
        let size = size.max(1);
        if self.page_size == size {
            return;
        }

        self.page_size = size;
        if let Some(ref on_page_size_change) = self.on_page_size_change {
            (on_page_size_change)(size, window, cx);
        }

        let max_page = self.page_count().max(1);
        let clamped = self.current_page.clamp(1, max_page);
        if clamped != self.current_page {
            self.current_page = clamped;
            if let Some(ref on_change) = self.on_change {
                (on_change)(clamped, window, cx);
            }
        }

        cx.notify();
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

#[derive(Clone, Copy)]
enum PagerAction {
    Page(usize),
    Prev,
    Next,
    PrevMore,
    NextMore,
}

impl Render for Pagination {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Config>().theme.clone();
        let page_count = self.page_count();
        let current_page = self.current_page;
        let background = self.background;
        let page_sizes = self.page_sizes.clone();
        let picker_entity = if page_sizes.is_empty() {
            None
        } else {
            let options: Vec<SharedString> = page_sizes
                .iter()
                .map(|size| size.to_string().into())
                .collect();
            let selected_idx = options
                .iter()
                .position(|opt| opt.as_ref() == self.page_size.to_string())
                .or(Some(0));

            let picker = self.page_size_picker.get_or_insert_with(|| {
                let pagination = cx.entity().clone();
                let sizes = page_sizes.clone();
                let options_for_build = options.clone();
                cx.new(move |cx| {
                    Select::new(options_for_build, selected_idx, cx).on_change(
                        move |idx, window, cx| {
                            if let Some(size) = sizes.get(idx).copied() {
                                let _ = pagination.update(cx, |this, cx| {
                                    this.change_page_size(size, window, cx);
                                });
                            }
                        },
                    )
                })
            });

            let options_clone = options.clone();
            picker.update(cx, |select, cx| {
                select.set_options(options_clone, cx);
                select.set_selected_idx(selected_idx, cx);
                select.set_borderless(false, cx);
                select.set_radius_none(false, cx);
                select.set_width(px(88.0), cx);
                select.set_padding_x(px(10.0), cx);
                select.set_text_size(px(theme.font_size.sm), cx);
                select.set_text_color(theme.neutral.text_1, cx);
            });

            Some(picker.clone())
        };

        let render_btn = |id: SharedString,
                          text: Option<SharedString>,
                          icon: Option<IconName>,
                          active: bool,
                          disabled: bool,
                          action: Option<PagerAction>,
                          cx: &mut Context<Self>| {
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
                    theme.neutral.inverted
                } else {
                    theme.primary.base
                }
            } else {
                theme.neutral.text_2
            };

            let hover_bg = if active {
                if background {
                    theme.primary.hover
                } else {
                    theme.primary.base.opacity(0.18)
                }
            } else {
                theme.neutral.text_3.opacity(0.22)
            };
            let hover_text_color = if active {
                if background {
                    theme.neutral.inverted
                } else {
                    theme.primary.base
                }
            } else {
                theme.neutral.text_1
            };
            let hover_group = SharedString::from(format!("{}-hover", id));

            div()
                .id(id)
                .group(hover_group.clone())
                .flex()
                .items_center()
                .justify_center()
                .min_w(px(32.0))
                .h(px(32.0))
                .px_1()
                .bg(bg_color)
                .rounded(px(theme.radius.sm))
                .text_color(text_color)
                .when(action.is_some() && !disabled, |s| {
                    s.cursor_pointer().hover(move |s| {
                        s.cursor_pointer().bg(hover_bg).text_color(hover_text_color)
                    })
                })
                .when_some(action.filter(|_| !disabled), |s, action| {
                    s.on_click(cx.listener(move |this, _, window, cx| match action {
                        PagerAction::Page(page) => this.change_page(page, window, cx),
                        PagerAction::Prev => {
                            let current = this.current_page;
                            this.change_page(current.saturating_sub(1), window, cx);
                        }
                        PagerAction::Next => {
                            let current = this.current_page;
                            this.change_page(current + 1, window, cx);
                        }
                        PagerAction::PrevMore => {
                            let current = this.current_page;
                            this.change_page(current.saturating_sub(5), window, cx);
                        }
                        PagerAction::NextMore => {
                            let current = this.current_page;
                            this.change_page(current + 5, window, cx);
                        }
                    }))
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
                    s.child(
                        Icon::new(i)
                            .size(px(14.0))
                            .color(text_color)
                            .group_hover_color(hover_group, theme.primary.base),
                    )
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
                    container = container.child(render_btn(
                        format!("{}-prev-btn", self.id).into(),
                        None,
                        Some(IconName::ChevronLeft),
                        false,
                        disabled,
                        Some(PagerAction::Prev),
                        cx,
                    ));
                }
                "pager" => {
                    for item in &pagers {
                        match item {
                            PagerItem::Page(p) => {
                                let p = *p;
                                let active = p == current_page;
                                container = container.child(render_btn(
                                    format!("{}-pager-{}", self.id, p).into(),
                                    Some(p.to_string().into()),
                                    None,
                                    active,
                                    false,
                                    Some(PagerAction::Page(p)),
                                    cx,
                                ));
                            }
                            PagerItem::PrevMore => {
                                container = container.child(render_btn(
                                    format!("{}-prev-more", self.id).into(),
                                    None,
                                    Some(IconName::Ellipsis),
                                    false,
                                    false,
                                    Some(PagerAction::PrevMore),
                                    cx,
                                ));
                            }
                            PagerItem::NextMore => {
                                container = container.child(render_btn(
                                    format!("{}-next-more", self.id).into(),
                                    None,
                                    Some(IconName::Ellipsis),
                                    false,
                                    false,
                                    Some(PagerAction::NextMore),
                                    cx,
                                ));
                            }
                        }
                    }
                }
                "next" => {
                    let disabled = current_page >= page_count;
                    container = container.child(render_btn(
                        format!("{}-next-btn", self.id).into(),
                        None,
                        Some(IconName::ChevronRight),
                        false,
                        disabled,
                        Some(PagerAction::Next),
                        cx,
                    ));
                }
                "sizes" => {
                    if let Some(ref picker) = picker_entity {
                        container = container.child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .text_sm()
                                        .text_color(theme.neutral.text_3)
                                        .child("每页"),
                                )
                                .child(picker.clone())
                                .child(
                                    div().text_sm().text_color(theme.neutral.text_3).child("条"),
                                ),
                        );
                    }
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
                                    .cursor_default()
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
