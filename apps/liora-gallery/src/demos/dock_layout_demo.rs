use gpui::{AnyView, App, Context, Render, Window, prelude::*, px};
use liora_components::dock_layout::DockState;
use liora_components::layout_helpers::{page, section};
use liora_components::{Button, DockEdge, DockLayout, DockPanel, DockTab, Text};
use std::{cell::RefCell, rc::Rc};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| DockLayoutDemo {
        snapshot: Rc::new(RefCell::new(None)),
        restored: None,
        generation: 0,
    })
    .into()
}

struct DockLayoutDemo {
    snapshot: Rc<RefCell<Option<String>>>,
    restored: Option<DockState>,
    generation: usize,
}

impl Render for DockLayoutDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let snapshot = self.snapshot.clone();
        let mut workbench = DockLayout::new()
            .id(format!("gallery-dock-{}", self.generation))
            .height_lg().panel_gap(px(6.))
            .panel(DockPanel::new("explorer", "Explorer", DockEdge::Left, Text::new("src / crates / apps")).size(px(160.)))
            .panel(DockPanel::new("terminal", "Terminal", DockEdge::Bottom, Text::new("Host-owned terminal area")).size(px(90.)))
            .panel(DockPanel::new("center-panel", "Center inspector", DockEdge::Center, Text::new("Center panel content is rendered")).size(px(160.)))
            .tab(DockTab::new("main", "main.rs", Text::new("fn main() {\n    init_liora(cx);\n}")).closable(false))
            .tab(DockTab::new("readme", "README", Text::new("# Native GPUI SDK")))
            .tab(DockTab::new("notes", "Notes", Text::new("Drag my tab onto another header to reorder, into a group to move, or onto Split arrows to split.")))
            .on_layout_change(move |state| {
                if let Ok(json) = state.to_json() {
                    *snapshot.borrow_mut() = Some(json);
                }
            });
        if let Some(state) = &self.restored {
            workbench = workbench.layout_state(state.clone());
        }
        page(
            "DockLayout 工作台",
            "点击选择、关闭；原生拖动 tab 重排、跨组移动或分屏。布局描述不包含业务内容。",
            gpui::div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    Button::new("Restore last changed layout").on_click(cx.listener(
                        |this, _, _, cx| {
                            if let Some(json) = this.snapshot.borrow().as_ref() {
                                if let Ok(restored) = DockState::from_json(json) {
                                    this.restored = Some(restored);
                                    this.generation += 1;
                                }
                            }
                            cx.notify();
                        },
                    )),
                )
                .child(section(
                    "Native workbench",
                    "拖动标签到 Split ← / → / ↑ / ↓；修改布局后可恢复到新实例。",
                    workbench,
                )),
        )
    }
}
