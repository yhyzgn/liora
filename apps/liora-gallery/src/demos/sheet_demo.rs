use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_card, showcase_grid};
use liora_components::{Button, Sheet, Space, Text};
use liora_icons_lucide::IconName;

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| SheetDemo).into()
}
struct SheetDemo;

fn sheet_body(title: &'static str, detail: &'static str) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new(title).bold())
        .child(Text::new(detail).wrap())
        .child(
            Space::new()
                .gap_sm()
                .child(Button::new("Cancel"))
                .child(Button::new("Apply").primary()),
        )
}

fn trigger(label: &'static str, icon: IconName, sheet: fn(&mut App)) -> impl IntoElement {
    Button::new(label)
        .icon_start(icon)
        .on_click(move |_, _, cx| sheet(cx))
}

impl Render for SheetDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        page("Sheet 轻量面板", "边缘滑入的轻量流程面板，适合筛选、快速创建、检查器和局部设置。底层复用 Drawer overlay runtime，避免重复弹层状态。",
            Space::new().vertical().gap_xl().child(section(
                "Sheet showcase",
                "触发按钮统一放入卡片网格，点击后再展示真实 Sheet overlay。",
                showcase_grid(vec![
                    showcase_card(
                        "边缘位置",
                        "Right/Left/Top/Bottom 四个方向，用于不同 app-shell 场景。",
                        Space::new()
                            .wrap()
                            .gap_md()
                            .child(trigger("Right inspector", IconName::PanelRightOpen, |cx| Sheet::new().title("Inspector").right().content(|_| sheet_body("Inspector", "Review properties before applying changes.")).show(cx)))
                            .child(trigger("Left navigator", IconName::PanelLeftOpen, |cx| Sheet::new().title("Navigator").left().content(|_| sheet_body("Navigator", "Jump across workspaces without leaving context.")).show(cx)))
                            .child(trigger("Top command", IconName::PanelTopOpen, |cx| Sheet::new().title("Command").top().height_sm().content(|_| sheet_body("Command", "Run a temporary command flow.")).show(cx)))
                            .child(trigger("Bottom actions", IconName::PanelBottomOpen, |cx| Sheet::new().title("Actions").bottom().height_sm().content(|_| sheet_body("Actions", "Confirm mobile-style secondary actions.")).show(cx))),
                    )
                    .into_any_element(),
                    showcase_card(
                        "受控关闭",
                        "可以关闭 outside click 或 escape，用于必须显式完成的流程。",
                        Space::new()
                            .vertical()
                            .gap_md()
                            .child(trigger("Blocking review", IconName::ShieldCheck, |cx| Sheet::new().title("Blocking review").width_lg().close_on_click_outside(false).close_on_escape(false).content(|_| sheet_body("Explicit close only", "This sheet keeps the user in the review flow until they choose an action.")).show(cx)))
                            .child(Text::new("Sheet::close(cx) / Sheet::close_id(id, cx) can close it programmatically.").sm()),
                    )
                    .into_any_element(),
                ]),
            ))
        )
    }
}
