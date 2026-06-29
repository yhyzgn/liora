use gpui::{AnyView, App, Context, Render, Window, prelude::*};
use liora_components::layout_helpers::{page, section, showcase_grid};
use liora_components::{FocusTrap, Space, Tag, Text};

pub fn render(cx: &mut App) -> AnyView {
    cx.new(|_| FocusTrapDemo).into()
}

struct FocusTrapDemo;

impl Render for FocusTrapDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let default_policy = FocusTrap::new();
        let strict_policy = FocusTrap::new().restore_focus(true).close_on_escape(false);
        page(
            "FocusTrap 焦点策略",
            "Overlay 组件共享的焦点策略数据，用于 modal-like surfaces 的焦点恢复和 Escape 策略。",
            Space::new().vertical().gap_xl().child(section(
                "策略对象",
                "FocusTrap 是基础设施 facade，不是重视觉控件。",
                showcase_grid(vec![
                    policy_card("Default", default_policy),
                    policy_card("Strict modal", strict_policy),
                ]),
            )),
        )
    }
}

fn policy_card(label: &'static str, policy: FocusTrap) -> gpui::AnyElement {
    Space::new()
        .vertical()
        .gap_sm()
        .child(Tag::new(label).info())
        .child(Text::new(format!("enabled: {}", policy.enabled)))
        .child(Text::new(format!(
            "restore_focus: {}",
            policy.restore_focus
        )))
        .child(Text::new(format!(
            "close_on_escape: {}",
            policy.close_on_escape
        )))
        .into_any_element()
}
