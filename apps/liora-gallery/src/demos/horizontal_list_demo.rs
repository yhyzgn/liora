use gpui::{
    AnyElement, AnyView, App, Context, Entity, MouseButton, Render, SharedString, Window, div,
    prelude::*, px, rgb,
};
use liora_components::draggable::{DragAxis, DragState, drag_handle, reorder_indices};
use liora_components::layout_helpers::{page, section};
use liora_components::{HorizontalList, Space, Text, toast_success};
use liora_core::Config;
use liora_icons::Icon;
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|cx| HorizontalListDemo::new(cx)).into()
}

pub(super) fn render_drag_helpers(cx: &mut App) -> AnyView {
    cx.new(|_| DragHelpersDemo::new()).into()
}

struct HorizontalListDemo {
    basic: gpui::Entity<HorizontalList>,
    divider: gpui::Entity<HorizontalList>,
    draggable: gpui::Entity<HorizontalList>,
    drag_helpers: Entity<DragHelpersDemo>,
}

impl HorizontalListDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        let basic = cx.new(|_| {
            HorizontalList::new(PROCESS_STEPS.len(), step_card)
                .height(px(88.0))
                .item_gap(px(12.0))
        });
        let divider = cx.new(|_| {
            HorizontalList::new(FLOW_ITEMS.len(), flow_card)
                .height(px(92.0))
                .item_gap(px(10.0))
                .divider(|_| {
                    Icon::new(IconName::ChevronRight)
                        .size(px(18.0))
                        .color(rgb(0x94a3b8).into())
                        .into_any_element()
                })
        });
        let draggable = cx.new(|_| {
            HorizontalList::new(KANBAN_ITEMS.len(), kanban_card)
                .height(px(118.0))
                .item_gap(px(12.0))
                .draggable(true)
                .on_reorder(|from, to, _, _| {
                    toast_success!("HorizontalList reordered: {} -> {}", from + 1, to + 1);
                })
        });

        Self {
            basic,
            divider,
            draggable,
            drag_helpers: cx.new(|_| DragHelpersDemo::new()),
        }
    }
}

impl Render for HorizontalListDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page(
            "HorizontalList 水平列表",
            "横向滚动列表，支持完全自定义 item、divider 和原生拖动排序。",
            Space::new()
                .vertical()
                .gap_xl()
                .child(section(
                    "基础横向列表",
                    "适合展示流程节点、指标卡片、横向快捷入口。",
                    self.basic.clone(),
                ))
                .child(section(
                    "自定义 divider",
                    "默认 divider 是竖线，也可以改成箭头、图标或任意 GPUI 元素。",
                    self.divider.clone(),
                ))
                .child(section(
                    "拖动排序",
                    "开启 draggable 后，item 前端会显示 Grip 拖拽把手；按住把手拖到目标 item 上松开即可重排，并触发 on_reorder 回调。",
                    self.draggable.clone(),
                ))
                .child(section(
                    "公共拖拽 helpers",
                    "按住 Grip 并移动超过原生拖拽阈值开始拖动；左键在目标行松开重排，右键松开取消。偏移从拖拽启动时计算。",
                    self.drag_helpers.clone(),
                )),
        )
    }
}

struct DragHelpersDemo {
    items: Vec<&'static str>,
    drag: DragState,
    session: Option<gpui::EntityId>,
    selector_prefix: &'static str,
}

impl DragHelpersDemo {
    fn new() -> Self {
        Self::new_with_prefix("helper")
    }

    // Distinct debug selectors let multi-instance regressions measure each
    // host's own painted boxes; the default keeps the single-instance names.
    fn new_with_prefix(selector_prefix: &'static str) -> Self {
        Self {
            items: vec!["Design", "Build", "Review"],
            drag: DragState::default(),
            session: None,
            selector_prefix,
        }
    }

    fn drop_at(&mut self, index: Option<usize>) -> bool {
        self.session = None;
        let Some(index) = index else {
            self.drag.cancel();
            return false;
        };
        self.drag.set_over(index);
        self.drag
            .finish()
            .is_some_and(|(from, to)| reorder_indices(&mut self.items, from, to))
    }
}

impl Render for DragHelpersDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        let color = theme.neutral.text_2;
        let highlight = theme.primary.base;
        let (_, dy) = self.drag.offset(DragAxis::Vertical);
        let status = match (self.drag.active_index(), self.drag.over_index()) {
            (Some(_), Some(target)) => format!("Drop on row {} · Δy {dy:?}", target + 1),
            _ => "Drag a grip to reorder".into(),
        };
        Space::new()
            .vertical()
            .gap_sm()
            .child(Text::new(status))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w_full()
                    .max_w(px(320.0))
                    // Native preview release cleans up releases delivered to the window, even offscreen.
                    .children(self.items.iter().copied().enumerate().map(|(index, item)| {
                        div()
                            .flex()
                            .items_center()
                            .h(px(36.0))
                            .when(self.drag.is_over(index), |row| {
                                row.bg(highlight.opacity(0.12))
                            })
                            .debug_selector(|| format!("{}-row-{index}", self.selector_prefix))
                            .on_drag_move::<HelperDrag>(cx.listener(
                                move |this, event: &gpui::DragMoveEvent<HelperDrag>, _, cx| {
                                    if event.drag(cx).owner == cx.entity_id()
                                        && event.bounds.contains(&event.event.position)
                                    {
                                        this.drag.update_position(event.event.position);
                                        this.drag.set_over(index);
                                        cx.notify();
                                    }
                                },
                            ))
                            // Left-only painted-row release while our own
                            // gesture is still the live native drag. A stale
                            // session must not commit, and a foreign helper
                            // instance (no session) must not consume the
                            // release; propagation stops only on a real
                            // reorder, so unrelated clicks still bubble.
                            .on_mouse_up(
                                MouseButton::Left,
                                cx.listener(move |this, _: &gpui::MouseUpEvent, _, cx| {
                                    if this.session.is_some()
                                        && cx.has_active_drag()
                                        && this.drag.active_index().is_some()
                                        && this.drop_at(Some(index))
                                    {
                                        cx.stop_propagation();
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(
                                drag_handle(color, self.drag.is_active(index), px(32.0))
                                    .id(("drag-helper", index))
                                    .debug_selector(|| {
                                        format!("{}-grip-{index}", self.selector_prefix)
                                    })
                                    .on_drag(
                                        HelperDrag {
                                            owner: cx.entity_id(),
                                            index,
                                            label: item,
                                        },
                                        {
                                            let owner = cx.weak_entity();
                                            move |payload, _, window, cx| {
                                                let preview =
                                                    cx.new(|_| HelperDragPreview(payload.label));
                                                let session = preview.entity_id();
                                                owner
                                                    .update(cx, |this, cx| {
                                                        this.session = Some(session);
                                                        this.drag.start(
                                                            payload.index,
                                                            window.mouse_position(),
                                                        );
                                                        cx.observe_release(
                                                            &preview,
                                                            move |this, _, cx| {
                                                                if this.session == Some(session) {
                                                                    this.drop_at(None);
                                                                    cx.notify();
                                                                }
                                                            },
                                                        )
                                                        .detach();
                                                        cx.notify();
                                                    })
                                                    .ok();
                                                preview
                                            }
                                        },
                                    ),
                            )
                            .child(Text::new(item))
                    })),
            )
    }
}

struct HelperDrag {
    owner: gpui::EntityId,
    index: usize,
    label: &'static str,
}

// GPUI owns this session-only preview even when the originating list stops painting.
struct HelperDragPreview(&'static str);

impl Render for HelperDragPreview {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<Config>().theme;
        // Opaque elevated surface so the floating preview does not show the
        // underlying row's text and highlight through its glyphs.
        div()
            .bg(theme.neutral.card)
            .border_1()
            .border_color(theme.neutral.border)
            .rounded_sm()
            .px_1()
            .child(Text::new(self.0))
    }
}

const PROCESS_STEPS: &[(&str, &str)] = &[
    ("01", "Discover"),
    ("02", "Design"),
    ("03", "Build"),
    ("04", "Verify"),
    ("05", "Ship"),
    ("06", "Observe"),
];

const FLOW_ITEMS: &[(&str, &str)] = &[
    ("Input", "Collect data"),
    ("Validate", "Check rules"),
    ("Transform", "Normalize"),
    ("Export", "Send output"),
];

const KANBAN_ITEMS: &[(&str, &str)] = &[
    ("Inbox", "8 tasks"),
    ("Ready", "5 tasks"),
    ("Doing", "3 tasks"),
    ("Review", "2 tasks"),
    ("Done", "12 tasks"),
];

fn step_card(index: usize) -> AnyElement {
    let (number, label) = PROCESS_STEPS[index];
    div()
        .w(px(132.0))
        .h(px(72.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xdbeafe))
        .bg(rgb(0xeff6ff))
        .p_3()
        .flex()
        .flex_col()
        .justify_between()
        .child(
            Text::new(number)
                .size(px(12.0))
                .text_color(rgb(0x2563eb).into()),
        )
        .child(Text::new(label).bold().text_color(rgb(0x1e3a8a).into()))
        .into_any_element()
}

fn flow_card(index: usize) -> AnyElement {
    let (label, desc) = FLOW_ITEMS[index];
    div()
        .w(px(150.0))
        .h(px(76.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .flex()
        .flex_col()
        .gap_1()
        .child(Text::new(label).bold().text_color(rgb(0x0f172a).into()))
        .child(
            Text::new(desc)
                .size(px(12.0))
                .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}

fn kanban_card(index: usize) -> AnyElement {
    let (label, desc) = KANBAN_ITEMS[index];
    let accent = match index % 4 {
        0 => rgb(0x6366f1),
        1 => rgb(0x14b8a6),
        2 => rgb(0xf59e0b),
        _ => rgb(0xef4444),
    };
    div()
        .w(px(156.0))
        .h(px(96.0))
        .rounded_lg()
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .bg(rgb(0xffffff))
        .p_3()
        .flex()
        .flex_col()
        .gap_2()
        .child(div().w(px(28.0)).h(px(4.0)).rounded_full().bg(accent))
        .child(Text::new(SharedString::from(label)).bold())
        .child(
            Text::new(SharedString::from(desc))
                .size(px(12.0))
                .text_color(rgb(0x64748b).into()),
        )
        .into_any_element()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drag_helpers_commit_only_valid_drops() {
        let mut demo = DragHelpersDemo::new();
        let start = gpui::point(px(10.0), px(20.0));
        demo.drag.start(0, start);
        demo.drag.update_position(gpui::point(px(30.0), px(52.0)));
        assert_eq!(demo.drag.offset(DragAxis::Vertical), (px(0.0), px(32.0)));
        assert!(demo.drop_at(Some(2)));
        assert_eq!(demo.items, ["Build", "Review", "Design"]);

        demo.drag.start(1, start);
        assert!(!demo.drop_at(None));
        assert_eq!(demo.drag.active_index(), None);
        assert!(!demo.drop_at(Some(0)));

        demo.drag.start(0, start);
        assert!(!demo.drop_at(Some(3)));
        assert_eq!(demo.items, ["Build", "Review", "Design"]);
    }

    #[test]
    fn drag_helpers_cancel_unpainted_and_non_left_releases() {
        use gpui::{AppContext, Modifiers, TestAppContext, VisualTestContext, point};

        struct Surface {
            demo: Entity<DragHelpersDemo>,
            visible: bool,
        }

        impl Render for Surface {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                div()
                    .size_full()
                    .when(self.visible, |root| root.child(self.demo.clone()))
            }
        }

        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let handle = app.add_window(|_, cx| Surface {
            demo: cx.new(|_| DragHelpersDemo::new()),
            visible: true,
        });
        let root = handle.root(&mut app).unwrap();
        let demo = root.read_with(&app, |root, _| root.demo.clone());
        let mut window = VisualTestContext::from_window(handle.into(), &app);
        let draw = |window: &mut VisualTestContext| {
            window.update(|window, cx| window.draw(cx).clear(cx));
        };
        draw(&mut window);
        let start = window.debug_bounds("helper-grip-0").unwrap().center();
        let target = window.debug_bounds("helper-row-2").unwrap().center();
        let outside = point(px(480.0), target.y);
        let modifiers = Modifiers::none();
        let start_drag = |window: &mut VisualTestContext| {
            window.simulate_mouse_down(start, MouseButton::Left, modifiers);
            window.simulate_mouse_move(
                start + point(px(12.0), px(0.0)),
                MouseButton::Left,
                modifiers,
            );
            draw(window);
            window.simulate_mouse_move(target, MouseButton::Left, modifiers);
            draw(window);
        };
        let show = |app: &mut TestAppContext, window: &mut VisualTestContext, visible| {
            root.update(app, |root, cx| {
                root.visible = visible;
                cx.notify();
            });
            draw(window);
        };
        let assert_idle = |app: &TestAppContext, order| {
            demo.read_with(app, |demo, _| {
                assert_eq!(demo.drag.active_index(), None);
                assert_eq!(demo.items, order);
            });
        };
        let original = ["Design", "Build", "Review"];

        start_drag(&mut window);
        assert_eq!(
            demo.read_with(&app, |demo, _| demo.drag.active_index()),
            Some(0)
        );
        show(&mut app, &mut window, false);
        assert!(window.debug_bounds("helper-row-2").is_none());
        window.simulate_mouse_up(target, MouseButton::Left, modifiers);
        draw(&mut window);
        show(&mut app, &mut window, true);
        window.simulate_click(target, modifiers);
        assert_idle(&app, original);

        start_drag(&mut window);
        window.simulate_mouse_move(outside, MouseButton::Left, modifiers);
        window.simulate_mouse_down(outside, MouseButton::Right, modifiers);
        window.simulate_mouse_up(target, MouseButton::Right, modifiers);
        window.simulate_mouse_up(target, MouseButton::Left, modifiers);
        draw(&mut window);
        assert_idle(&app, original);

        start_drag(&mut window);
        window.simulate_mouse_up(target, MouseButton::Left, modifiers);
        draw(&mut window);
        assert_idle(&app, ["Build", "Review", "Design"]);
    }

    #[test]
    fn drag_helpers_foreign_release_bubbles_then_new_gesture_commits_once() {
        use gpui::{AppContext, Modifiers, TestAppContext, VisualTestContext, point};
        use std::cell::Cell;
        use std::rc::Rc;

        // Two independently painted helper instances share one ancestor that
        // counts bubbled releases by button. All geometry is measured from
        // each host's own debug-selector boxes.
        struct TwoHosts {
            a: Entity<DragHelpersDemo>,
            b: Entity<DragHelpersDemo>,
            left_ups: Rc<Cell<usize>>,
            right_ups: Rc<Cell<usize>>,
        }

        impl Render for TwoHosts {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let left = self.left_ups.clone();
                let right = self.right_ups.clone();
                div()
                    .size_full()
                    .flex()
                    .flex_col()
                    .on_mouse_up(MouseButton::Left, move |_, _, _| {
                        left.set(left.get() + 1);
                    })
                    .on_mouse_up(MouseButton::Right, move |_, _, _| {
                        right.set(right.get() + 1);
                    })
                    .child(self.a.clone())
                    .child(self.b.clone())
            }
        }

        let mut app = TestAppContext::single();
        app.update(liora_components::init_liora);
        let (left_ups, right_ups) = (Rc::new(Cell::new(0)), Rc::new(Cell::new(0)));
        let handle = app.add_window(|_, cx| TwoHosts {
            a: cx.new(|_| DragHelpersDemo::new_with_prefix("helper-a")),
            b: cx.new(|_| DragHelpersDemo::new_with_prefix("helper-b")),
            left_ups: left_ups.clone(),
            right_ups: right_ups.clone(),
        });
        let root = handle.root(&mut app).unwrap();
        let (a, b) = root.read_with(&app, |host, _| (host.a.clone(), host.b.clone()));
        let mut window = VisualTestContext::from_window(handle.into(), &app);
        let draw = |window: &mut VisualTestContext| {
            window.update(|window, cx| window.draw(cx).clear(cx));
        };
        draw(&mut window);
        let modifiers = Modifiers::none();
        let original = ["Design", "Build", "Review"];

        // Live A drag released over B's own painted row: no commit anywhere,
        // both orders unchanged, exactly one bubbled ancestor Left-up, and
        // A's session is cleaned up.
        let start = window.debug_bounds("helper-a-grip-0").unwrap().center();
        let target = window.debug_bounds("helper-b-row-2").unwrap().center();
        assert!(target.y > start.y, "B rows must paint below A");
        window.simulate_mouse_down(start, MouseButton::Left, modifiers);
        window.simulate_mouse_move(
            start + point(px(12.0), px(0.0)),
            MouseButton::Left,
            modifiers,
        );
        draw(&mut window);
        assert_eq!(
            a.read_with(&app, |demo, _| demo.drag.active_index()),
            Some(0)
        );
        window.simulate_mouse_move(target, MouseButton::Left, modifiers);
        draw(&mut window);
        assert_eq!(left_ups.get(), 0);
        window.simulate_mouse_up(target, MouseButton::Left, modifiers);
        draw(&mut window);
        assert_eq!(left_ups.get(), 1);
        a.read_with(&app, |demo, _| {
            assert_eq!(demo.drag.active_index(), None);
            assert_eq!(demo.items, original);
        });
        b.read_with(&app, |demo, _| assert_eq!(demo.items, original));

        // An ordinary move with no held button must not reactivate A.
        window.simulate_mouse_move(target, MouseButton::Left, modifiers);
        draw(&mut window);
        a.read_with(&app, |demo, _| {
            assert_eq!(demo.drag.active_index(), None);
            assert_eq!(demo.items, original);
        });

        // Unrelated Right-up passes through: a live A drag cancelled by a
        // right release over B's rows leaves both orders unchanged and
        // bubbles exactly one ancestor Right-up.
        window.simulate_mouse_down(start, MouseButton::Left, modifiers);
        window.simulate_mouse_move(
            start + point(px(12.0), px(0.0)),
            MouseButton::Left,
            modifiers,
        );
        draw(&mut window);
        window.simulate_mouse_move(target, MouseButton::Left, modifiers);
        draw(&mut window);
        window.simulate_mouse_down(target, MouseButton::Right, modifiers);
        window.simulate_mouse_up(target, MouseButton::Right, modifiers);
        draw(&mut window);
        assert_eq!(right_ups.get(), 1);
        a.read_with(&app, |demo, _| {
            assert_eq!(demo.drag.active_index(), None);
            assert_eq!(demo.items, original);
        });
        b.read_with(&app, |demo, _| assert_eq!(demo.items, original));

        // A fresh B gesture commits exactly once; the commit consumes the
        // release, so the ancestor Left-up count stays at 1.
        let b_start = window.debug_bounds("helper-b-grip-0").unwrap().center();
        window.simulate_mouse_down(b_start, MouseButton::Left, modifiers);
        window.simulate_mouse_move(
            b_start + point(px(12.0), px(0.0)),
            MouseButton::Left,
            modifiers,
        );
        draw(&mut window);
        window.simulate_mouse_move(target, MouseButton::Left, modifiers);
        draw(&mut window);
        window.simulate_mouse_up(target, MouseButton::Left, modifiers);
        draw(&mut window);
        assert_eq!(left_ups.get(), 1);
        assert_eq!(right_ups.get(), 1);
        b.read_with(&app, |demo, _| {
            assert_eq!(demo.drag.active_index(), None);
            assert_eq!(demo.items, ["Build", "Review", "Design"]);
        });
        a.read_with(&app, |demo, _| assert_eq!(demo.items, original));

        // Same ownership contract with the painting order reversed: swap the
        // hosts so A paints below B, then drag A's grip onto B's rows again.
        root.update(&mut app, |host, cx| {
            std::mem::swap(&mut host.a, &mut host.b);
            cx.notify();
        });
        draw(&mut window);
        let start2 = window.debug_bounds("helper-a-grip-0").unwrap().center();
        let target2 = window.debug_bounds("helper-b-row-2").unwrap().center();
        assert!(start2.y > target2.y, "A must paint below B after the swap");
        window.simulate_mouse_down(start2, MouseButton::Left, modifiers);
        window.simulate_mouse_move(
            start2 + point(px(12.0), px(0.0)),
            MouseButton::Left,
            modifiers,
        );
        draw(&mut window);
        window.simulate_mouse_move(target2, MouseButton::Left, modifiers);
        draw(&mut window);
        window.simulate_mouse_up(target2, MouseButton::Left, modifiers);
        draw(&mut window);
        assert_eq!(left_ups.get(), 2);
        a.read_with(&app, |demo, _| {
            assert_eq!(demo.drag.active_index(), None);
            assert_eq!(demo.items, original);
        });
        b.read_with(&app, |demo, _| {
            assert_eq!(demo.items, ["Build", "Review", "Design"]);
        });
    }
    #[test]
    fn horizontal_list_demo_covers_custom_divider_and_drag() {
        let source = include_str!("horizontal_list_demo.rs");
        assert!(source.contains("HorizontalList::new"));
        assert!(source.contains("divider"));
        assert!(source.contains("draggable(true)"));
        assert!(source.contains("on_reorder"));
    }
}
