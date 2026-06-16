//! RadioGroup custom option layout and selected styles.

use aura_components::{RadioGroup, RadioOptionStyle, Space};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::prelude::FluentBuilder;
use gpui::{AppContext, Context, IntoElement, ParentElement, Render, Styled, Window, px, rgb};

fn styled_cards(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Daily", "Weekly", "Monthly"], 1, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .bg(rgb(0xf8fafc).into())
                .selected_bg(rgb(0xecfeff).into())
                .selected_text_color(rgb(0x0e7490).into())
                .selected_border_color(rgb(0x06b6d4).into())
                .hover_bg(rgb(0xf0fdfa).into())
                .radius(px(12.0))
                .padding(px(14.0), px(10.0)),
        )
}

fn styled_chips(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Low", "Medium", "High"], 2, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .bg(gpui::transparent_black())
                .selected_bg(rgb(0x7c3aed).into())
                .selected_text_color(gpui::white())
                .selected_border_color(rgb(0x7c3aed).into())
                .radius(px(999.0))
                .padding(px(16.0), px(8.0))
                .show_indicator(false),
        )
}

fn rich_cards(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(vec!["Starter", "Team", "Enterprise"], 1, cx)
        .horizontal()
        .option_style(
            RadioOptionStyle::new()
                .selected_bg(rgb(0xfffbeb).into())
                .selected_text_color(rgb(0x92400e).into())
                .selected_border_color(rgb(0xf59e0b).into())
                .hover_bg(rgb(0xfffbeb).into())
                .radius(px(14.0))
                .padding(px(14.0), px(12.0)),
        )
        .option_renderer(|option| {
            let (icon, description) = match option.index {
                0 => (IconName::Rocket, "个人试用与轻量项目"),
                1 => (IconName::Users, "团队协作与权限控制"),
                _ => (IconName::Building2, "审计、SLA 和专属支持"),
            };
            gpui::div()
                .flex()
                .items_start()
                .gap_2()
                .child(Icon::new(icon).size_md())
                .child(
                    gpui::div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(option.label.clone())
                        .child(gpui::div().text_xs().child(description)),
                )
                .into_any_element()
        })
}

struct RadioCustomDemo {
    cards: gpui::Entity<RadioGroup>,
    chips: gpui::Entity<RadioGroup>,
    rich: gpui::Entity<RadioGroup>,
}

impl RadioCustomDemo {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            cards: cx.new(styled_cards),
            chips: cx.new(styled_chips),
            rich: cx.new(rich_cards),
        }
    }
}

impl Render for RadioCustomDemo {
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
