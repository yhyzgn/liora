//! CheckboxGroup custom option layout and selected styles.

use aura_components::{CheckboxGroup, CheckboxOptionStyle, Space};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::prelude::FluentBuilder;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, px, rgb};

fn styled_cards(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(vec!["CPU", "Memory", "Network"], vec![0, 2], cx)
        .horizontal()
        .option_style(
            CheckboxOptionStyle::new()
                .bg(rgb(0xf8fafc).into())
                .selected_bg(rgb(0xdbeafe).into())
                .selected_text_color(rgb(0x1d4ed8).into())
                .selected_border_color(rgb(0x3b82f6).into())
                .hover_bg(rgb(0xeff6ff).into())
                .radius(px(12.0))
                .padding(px(14.0), px(10.0)),
        )
}

fn styled_chips(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(vec!["Fast", "Stable", "Secure"], vec![1], cx)
        .horizontal()
        .option_style(
            CheckboxOptionStyle::new()
                .bg(gpui::transparent_black())
                .selected_bg(rgb(0x111827).into())
                .selected_text_color(gpui::white())
                .selected_border_color(rgb(0x111827).into())
                .radius(px(999.0))
                .padding(px(16.0), px(8.0))
                .show_indicator(false),
        )
}

fn rich_options(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(vec!["Analytics", "Alerts", "Exports"], vec![0, 1], cx)
        .horizontal()
        .option_style(
            CheckboxOptionStyle::new()
                .selected_bg(rgb(0xf0fdf4).into())
                .selected_text_color(rgb(0x166534).into())
                .selected_border_color(rgb(0x22c55e).into())
                .hover_bg(rgb(0xf8fafc).into())
                .radius(px(14.0))
                .padding(px(14.0), px(12.0)),
        )
        .option_renderer(|option| {
            let description = match option.index {
                0 => "趋势、漏斗和指标面板",
                1 => "阈值触发与通知策略",
                _ => "CSV / JSON 批量导出",
            };
            gpui::div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    gpui::div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .child(option.label.clone())
                        .when(option.selected, |s| {
                            s.child(Icon::new(IconName::BadgeCheck).size_xs())
                        }),
                )
                .child(gpui::div().text_xs().child(description))
                .into_any_element()
        })
}

struct CheckboxCustomDemo {
    cards: gpui::Entity<CheckboxGroup>,
    chips: gpui::Entity<CheckboxGroup>,
    rich: gpui::Entity<CheckboxGroup>,
}

impl CheckboxCustomDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            cards: cx.new(styled_cards),
            chips: cx.new(styled_chips),
            rich: cx.new(rich_options),
        }
    }
}

impl Render for CheckboxCustomDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl gpui::IntoElement {
        Space::new()
            .vertical()
            .gap_md()
            .child(self.cards.clone())
            .child(self.chips.clone())
            .child(self.rich.clone())
    }
}

fn main() {}
