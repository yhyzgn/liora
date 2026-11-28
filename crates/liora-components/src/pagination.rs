//! Pagination module.
//!
//! This public module implements the Liora pagination component for page-based data navigation. It keeps the reusable
//! component logic inside `liora-components` rather than host applications so
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
//! the component, and avoid app-specific host-application resources in this SDK
//! crate.

use crate::Select;
use gpui::{App, Context, IntoElement, Render, SharedString, Window, div, prelude::*, px};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

/// Fluent native GPUI component for rendering Liora pagination.
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
    /// Creates `Pagination` initialized from the supplied total.
    pub fn new(total: usize) -> Self {
        Self {
            id: liora_core::unique_id("pagination"),
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

    /// Assigns a stable element id used by GPUI state, hit testing, and automated interaction tests.
    pub fn id(mut self, id: impl Into<SharedString>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the page size value used by the component.
    pub fn page_size(mut self, size: usize) -> Self {
        self.page_size = size.max(1);
        self
    }

    /// Sets the current page value used by the component.
    pub fn current_page(mut self, page: usize) -> Self {
        self.current_page = page.max(1);
        self
    }

    /// Toggles or applies the component background treatment.
    pub fn background(mut self, bg: bool) -> Self {
        self.background = bg;
        self
    }

    /// Selects the layout used by the component.
    pub fn layout(mut self, l: impl Into<SharedString>) -> Self {
        self.layout = l.into();
        self
    }

    /// Sets the page sizes value used by the component.
    pub fn page_sizes(mut self, sizes: impl Into<Vec<usize>>) -> Self {
        self.page_sizes = sizes.into().into_iter().map(|s| s.max(1)).collect();
        self.page_sizes.sort_unstable();
        self.page_sizes.dedup();
        self
    }

    /// Registers a callback that runs when change occurs.
    pub fn on_change(mut self, f: impl Fn(usize, &mut Window, &mut App) + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }

    /// Registers a callback that runs when page size change occurs.
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
        self.total.div_ceil(self.page_size)
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
        let current_page = self.current_page.clamp(1, page_count.max(1));
        self.current_page = current_page;
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
                .id(id.clone())
                .debug_selector(move || id.to_string())
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
                    s.tab_index(0)
                        .focus_visible(|style| {
                            style.shadow(vec![
                                gpui::BoxShadow::new(px(0.0), px(0.0), theme.primary.base)
                                    .spread_radius(px(2.0)),
                            ])
                        })
                        .on_key_down(|event, window, cx| {
                            let modifiers = event.keystroke.modifiers;
                            if modifiers.control
                                || modifiers.alt
                                || modifiers.platform
                                || modifiers.function
                            {
                                return;
                            }
                            if event.keystroke.key == "tab" {
                                if modifiers.shift {
                                    window.focus_prev(cx);
                                } else {
                                    window.focus_next(cx);
                                }
                                window.prevent_default();
                                cx.stop_propagation();
                            } else if !modifiers.shift
                                && matches!(event.keystroke.key.as_str(), "enter" | "space")
                            {
                                // GPUI emits the existing native click once on key-up.
                                cx.stop_propagation();
                            }
                        })
                        .cursor_pointer()
                        .hover(move |s| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::{Cell, RefCell},
        rc::Rc,
    };

    struct KeyboardPage {
        before: gpui::FocusHandle,
        after: gpui::FocusHandle,
        pagination: gpui::Entity<Pagination>,
        modified_tabs: Rc<Cell<usize>>,
    }

    impl Render for KeyboardPage {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let modified_tabs = self.modified_tabs.clone();
            div()
                .flex()
                .flex_col()
                .on_key_down(move |event, window, cx| {
                    if event.keystroke.key != "tab" {
                        return;
                    }
                    let modifiers = event.keystroke.modifiers;
                    if modifiers.control
                        || modifiers.alt
                        || modifiers.platform
                        || modifiers.function
                    {
                        modified_tabs.set(modified_tabs.get() + 1);
                    } else if modifiers.shift {
                        window.focus_prev(cx);
                    } else {
                        window.focus_next(cx);
                    }
                })
                .child(div().id("before").size(px(20.0)).track_focus(&self.before))
                .child(self.pagination.clone())
                .child(div().id("after").size(px(20.0)).track_focus(&self.after))
        }
    }

    struct KeyboardFixture {
        app: gpui::TestAppContext,
        window: gpui::VisualTestContext,
        pagination: gpui::Entity<Pagination>,
        before: gpui::FocusHandle,
        after: gpui::FocusHandle,
        changes: Rc<RefCell<Vec<usize>>>,
        modified_tabs: Rc<Cell<usize>>,
    }

    impl KeyboardFixture {
        fn new(total: usize, page: usize) -> Self {
            let mut app = gpui::TestAppContext::single();
            app.update(crate::init_liora);
            let before = app.update(|cx| cx.focus_handle().tab_stop(true));
            let after = app.update(|cx| cx.focus_handle().tab_stop(true));
            let changes = Rc::new(RefCell::new(Vec::new()));
            let modified_tabs = Rc::new(Cell::new(0));
            let pagination = app.new({
                let changes = changes.clone();
                move |_| {
                    Pagination::new(total)
                        .id("keyboard-pagination")
                        .current_page(page)
                        .on_change(move |page, _, _| changes.borrow_mut().push(page))
                }
            });
            let handle = app.add_window(|_, _| KeyboardPage {
                before: before.clone(),
                after: after.clone(),
                pagination: pagination.clone(),
                modified_tabs: modified_tabs.clone(),
            });
            let mut window = gpui::VisualTestContext::from_window(handle.into(), &app);
            window.update(|window, cx| {
                window.draw(cx).clear(cx);
                before.focus(window, cx);
                window.draw(cx).clear(cx);
            });
            Self {
                app,
                window,
                pagination,
                before,
                after,
                changes,
                modified_tabs,
            }
        }

        fn draw(&mut self) {
            self.window.update(|window, cx| window.draw(cx).clear(cx));
        }

        fn press(&mut self, key: &str) {
            self.window.simulate_keystrokes(key);
            self.release(key);
        }

        fn release(&mut self, key: &str) {
            self.window.update(|window, cx| {
                let keystroke = gpui::Keystroke::parse_with_os(key, cx.platform_info().os).unwrap();
                window.dispatch_event(
                    gpui::PlatformInput::KeyUp(gpui::KeyUpEvent { keystroke }),
                    cx,
                );
                window.draw(cx).clear(cx);
            });
        }

        fn focus(&mut self) -> gpui::FocusHandle {
            self.window
                .update(|window, cx| window.focused(cx).expect("focused tab stop"))
        }

        fn tabs(&mut self, count: usize) {
            for _ in 0..count {
                self.press("tab");
            }
        }
    }

    #[test]
    fn keyboard_traverses_all_actions_in_both_directions_and_retains_focus() {
        let mut fixture = KeyboardFixture::new(100, 6);
        let mut stops = vec![fixture.before.clone()];
        for _ in 0..9 {
            fixture.press("tab");
            let focus = fixture.focus();
            assert_ne!(
                focus, fixture.after,
                "enabled action must precede following stop"
            );
            assert!(
                !stops.contains(&focus),
                "each action owns one native focus identity"
            );
            stops.push(focus);
        }
        fixture.press("tab");
        assert_eq!(fixture.focus(), fixture.after);
        for expected in stops.iter().rev() {
            fixture.press("shift-tab");
            assert_eq!(&fixture.focus(), expected);
        }
        assert!(fixture.changes.borrow().is_empty());

        fixture.tabs(5); // Current page 6, between both ellipsis controls.
        let focused = fixture.focus();
        fixture
            .pagination
            .update(&mut fixture.app, |_, cx| cx.notify());
        fixture.draw();
        assert_eq!(
            fixture.focus(),
            focused,
            "same action retains focus across render"
        );
        for (index, key) in [
            "ctrl-tab",
            "alt-tab",
            "cmd-tab",
            "fn-tab",
            "ctrl-shift-tab",
            "alt-shift-tab",
            "cmd-shift-tab",
            "fn-shift-tab",
        ]
        .iter()
        .enumerate()
        {
            fixture.press(key);
            assert_eq!(fixture.focus(), focused, "{key} must not move focus");
            assert_eq!(
                fixture.modified_tabs.get(),
                index + 1,
                "{key} reaches parent"
            );
        }
        for key in ["enter", "space", "ctrl-enter", "shift-space"] {
            fixture.press(key);
        }
        let current = fixture
            .window
            .debug_bounds("keyboard-pagination-pager-6")
            .unwrap();
        fixture
            .window
            .simulate_click(current.center(), gpui::Modifiers::none());
        fixture.draw();
        assert!(
            fixture.changes.borrow().is_empty(),
            "current page never emits a callback"
        );
        fixture.app.quit();
    }

    #[test]
    fn enter_and_space_activate_every_action_once_on_release_with_mouse_parity() {
        // Previous, first page, -5 ellipsis, numbered pages, +5 ellipsis, last page, next.
        for (tabs, suffix, expected) in [
            (1, "prev-btn", 5),
            (2, "pager-1", 1),
            (3, "prev-more", 1),
            (4, "pager-5", 5),
            (6, "pager-7", 7),
            (7, "next-more", 10),
            (8, "pager-10", 10),
            (9, "next-btn", 7),
        ] {
            for key in ["enter", "space"] {
                let mut fixture = KeyboardFixture::new(100, 6);
                fixture.tabs(tabs);
                let focused = fixture.focus();
                fixture.window.simulate_keystrokes(key);
                assert!(
                    fixture.changes.borrow().is_empty(),
                    "{suffix} {key} down must not activate"
                );
                fixture.draw();
                fixture.release(key);
                assert_eq!(
                    *fixture.changes.borrow(),
                    [expected],
                    "{suffix} {key} activates once"
                );
                assert_eq!(
                    fixture
                        .pagination
                        .read_with(&fixture.app, |pagination, _| pagination.current_page),
                    expected
                );
                if matches!(suffix, "prev-btn" | "next-btn" | "pager-5" | "pager-7") {
                    assert_eq!(
                        fixture.focus(),
                        focused,
                        "same action keeps focus after page rerender"
                    );
                }
                fixture.release(key);
                assert_eq!(
                    *fixture.changes.borrow(),
                    [expected],
                    "unpaired key-up must not repeat"
                );
                fixture.app.quit();
            }
            let mut fixture = KeyboardFixture::new(100, 6);
            let selector = match suffix {
                "prev-btn" => "keyboard-pagination-prev-btn",
                "pager-1" => "keyboard-pagination-pager-1",
                "prev-more" => "keyboard-pagination-prev-more",
                "pager-5" => "keyboard-pagination-pager-5",
                "pager-7" => "keyboard-pagination-pager-7",
                "next-more" => "keyboard-pagination-next-more",
                "pager-10" => "keyboard-pagination-pager-10",
                "next-btn" => "keyboard-pagination-next-btn",
                _ => unreachable!(),
            };
            let bounds = fixture.window.debug_bounds(selector).unwrap();
            fixture
                .window
                .simulate_click(bounds.center(), gpui::Modifiers::none());
            fixture.draw();
            assert_eq!(
                *fixture.changes.borrow(),
                [expected],
                "{suffix} mouse parity"
            );
            fixture.app.quit();
        }
    }

    #[test]
    fn ellipsis_moves_exactly_five_pages_in_each_direction() {
        for (tabs, expected) in [(3, 5), (7, 15)] {
            for key in ["enter", "space"] {
                let mut fixture = KeyboardFixture::new(200, 10);
                fixture.tabs(tabs);
                let focused = fixture.focus();
                fixture.press(key);
                assert_eq!(*fixture.changes.borrow(), [expected]);
                assert_eq!(
                    fixture.focus(),
                    focused,
                    "rendered ellipsis retains its native focus"
                );
                fixture.app.quit();
            }
        }
    }

    #[test]
    fn modified_activation_strokes_leave_actionable_page_unchanged() {
        let mut fixture = KeyboardFixture::new(200, 10);
        fixture.tabs(1); // Enabled previous action would change page if activated.
        let focused = fixture.focus();
        for key in [
            "shift-enter",
            "ctrl-enter",
            "alt-enter",
            "cmd-enter",
            "fn-enter",
            "shift-space",
            "ctrl-space",
            "alt-space",
            "cmd-space",
            "fn-space",
        ] {
            fixture.press(key);
            assert!(
                fixture.changes.borrow().is_empty(),
                "{key} must not activate"
            );
            assert_eq!(
                fixture
                    .pagination
                    .read_with(&fixture.app, |pagination, _| pagination.current_page),
                10
            );
            assert_eq!(fixture.focus(), focused);
        }
        fixture.app.quit();
    }

    #[test]
    fn disabled_boundaries_and_empty_totals_are_unreachable_and_inert() {
        for (total, page, enabled, disabled) in [
            (30, 1, 4, "prev-btn"),
            (30, 3, 4, "next-btn"),
            (0, 1, 0, "prev-btn"),
        ] {
            let mut fixture = KeyboardFixture::new(total, page);
            fixture.tabs(enabled + 1);
            assert_eq!(
                fixture.focus(),
                fixture.after,
                "forward traversal skips disabled boundaries"
            );
            for _ in 0..enabled {
                fixture.press("shift-tab");
            }
            fixture.press("shift-tab");
            assert_eq!(
                fixture.focus(),
                fixture.before,
                "reverse traversal skips disabled boundaries"
            );
            for suffix in [disabled, if total == 0 { "next-btn" } else { disabled }] {
                let selector = if suffix == "prev-btn" {
                    "keyboard-pagination-prev-btn"
                } else {
                    "keyboard-pagination-next-btn"
                };
                let bounds = fixture.window.debug_bounds(selector).unwrap();
                fixture
                    .window
                    .simulate_click(bounds.center(), gpui::Modifiers::none());
                fixture.draw();
                fixture.press("enter");
                fixture.press("space");
                assert!(
                    fixture.changes.borrow().is_empty(),
                    "disabled {suffix} must be inert"
                );
            }
            assert_eq!(
                fixture
                    .pagination
                    .read_with(&fixture.app, |pagination, _| pagination.current_page),
                page
            );
            fixture.app.quit();
        }
    }

    #[test]
    fn render_clamps_current_page_using_final_page_size() {
        let mut app = gpui::TestAppContext::single();
        app.update(crate::init_liora);

        for (total, page, expected) in [(20, 8, 4), (20, 3, 3), (0, 8, 1)] {
            let (pagination, cx) = app.add_window_view(|_, _| {
                Pagination::new(total)
                    .current_page(page)
                    .page_size(5)
                    .layout("jumper")
            });
            cx.update(|window, cx| {
                window.draw(cx).clear(cx);
                assert_eq!(pagination.read(cx).current_page, expected);
            });
        }
        app.quit();
    }

    #[test]
    fn page_count_keeps_partial_page_for_large_totals() {
        let pagination = Pagination::new(16_777_217).page_size(8_388_608);
        assert_eq!(pagination.page_count(), 3);
    }
}
