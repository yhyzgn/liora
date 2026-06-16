use aura_components::{
    Checkbox, CheckboxGroup, CheckboxOptionStyle, Input, InputNumber, Radio, RadioGroup,
    RadioOptionStyle, Rate, Select, Slider, Space, Switch, Text, Textarea,
};
use aura_icons::Icon;
use aura_icons_lucide::IconName;
use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, prelude::*, px, rgb};

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    Space::new()
        .vertical()
        .gap_md()
        .child(Text::new(title).bold())
        .child(content)
}

fn control_row(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_lg().children(children)
}

fn control_stack(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().vertical().gap_md().children(children)
}

pub fn render_input(cx: &mut App) -> AnyView {
    cx.new(|cx| InputUsage::new(cx)).into()
}

struct InputUsage {
    plain: Entity<Input>,
    placeholder: Entity<Input>,
    password: Entity<Input>,
    password_custom: Entity<Input>,
    maxlength: Entity<Input>,
    prepend: Entity<Input>,
    append: Entity<Input>,
    composite: Entity<Input>,
    select_prepend: Entity<Input>,
    icon: Entity<Input>,
    clearable: Entity<Input>,
    disabled: Entity<Input>,
}

impl InputUsage {
    fn new(cx: &mut Context<Self>) -> Self {
        let protocol_select = cx.new(|cx| {
            Select::new(vec!["http://", "https://", "ftp://"], Some(1), cx)
                .borderless()
                .radius_right_none()
                .width_xs()
                .text_sm()
                .padding_x_sm()
        });

        Self {
            plain: cx.new(|cx| Input::new("", cx)),
            placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
            password: cx.new(|cx| Input::new("", cx).password().placeholder("Password")),
            password_custom: cx.new(|cx| Input::new("secret", cx).password().mask_char('*')),
            maxlength: cx.new(|cx| Input::new("", cx).placeholder("Max 5 chars").max_length(5)),
            prepend: cx.new(|cx| Input::new("", cx).prepend_text("http://")),
            append: cx.new(|cx| Input::new("", cx).append_text(".com")),
            composite: cx.new(|cx| {
                Input::new("", cx)
                    .prepend_icon(aura_icons_lucide::IconName::User)
                    .append_text("Admin")
            }),
            select_prepend: cx.new(|cx| {
                let sel = protocol_select.clone();
                Input::new("", cx)
                    .prepend(move |_, _| sel.clone().into_any_element())
                    .placeholder("domain.com")
            }),
            icon: cx.new(|cx| {
                Input::new("", cx)
                    .placeholder("Search")
                    .icon_prefix(aura_icons_lucide::IconName::Search)
                    .clearable(true)
            }),
            clearable: cx.new(|cx| Input::new("Clear me", cx).clearable(true)),
            disabled: cx.new(|cx| Input::new("Disabled", cx).disabled(true)),
        }
    }
}

impl Render for InputUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        control_stack(vec![
            self.plain.clone().into_any_element(),
            self.placeholder.clone().into_any_element(),
            self.password.clone().into_any_element(),
            self.password_custom.clone().into_any_element(),
            self.maxlength.clone().into_any_element(),
            self.prepend.clone().into_any_element(),
            self.append.clone().into_any_element(),
            self.select_prepend.clone().into_any_element(),
            self.composite.clone().into_any_element(),
            self.icon.clone().into_any_element(),
            self.clearable.clone().into_any_element(),
            self.disabled.clone().into_any_element(),
        ])
    }
}

pub fn render_checkbox(cx: &mut App) -> AnyView {
    cx.new(|cx| CheckboxUsage::new(cx)).into()
}

struct CheckboxUsage {
    checked: Entity<Checkbox>,
    unchecked: Entity<Checkbox>,
    labeled: Entity<Checkbox>,
    disabled: Entity<Checkbox>,
    disabled_checked: Entity<Checkbox>,
    group: Entity<CheckboxGroup>,
    buttons_large: Entity<CheckboxGroup>,
    buttons_default: Entity<CheckboxGroup>,
    buttons_small: Entity<CheckboxGroup>,
    buttons_stretch: Entity<CheckboxGroup>,
    styled_cards: Entity<CheckboxGroup>,
    styled_chips: Entity<CheckboxGroup>,
    rich_options: Entity<CheckboxGroup>,
}

impl CheckboxUsage {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            checked: cx.new(|cx| Checkbox::new(true, cx)),
            unchecked: cx.new(|cx| Checkbox::new(false, cx)),
            labeled: cx.new(|cx| Checkbox::new(false, cx).label("Label")),
            disabled: cx.new(|cx| Checkbox::new(false, cx).disabled(true)),
            disabled_checked: cx.new(|cx| Checkbox::new(true, cx).disabled(true)),
            group: cx.new(|cx| {
                CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx)
            }),
            buttons_large: cx.new(|cx| city_checkbox_group(cx).large()),
            buttons_default: cx.new(city_checkbox_group),
            buttons_small: cx.new(|cx| city_checkbox_group(cx).small()),
            buttons_stretch: cx.new(|cx| city_checkbox_group(cx).stretch(true)),
            styled_cards: cx.new(|cx| {
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
            }),
            styled_chips: cx.new(|cx| {
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
            }),
            rich_options: cx.new(|cx| {
                CheckboxGroup::new(vec!["Analytics", "Alerts", "Exports"], vec![0, 1], cx)
                    .horizontal()
                    .option_style(
                        CheckboxOptionStyle::new()
                            .selected_bg(rgb(0xf0fdf4).into())
                            .selected_text_color(rgb(0x166534).into())
                            .selected_border_color(rgb(0x22c55e).into())
                            .hover_bg(rgb(0xf8fafc).into())
                            .radius(px(14.0))
                            .padding(px(14.0), px(12.0))
                            .gap(px(10.0)),
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
            }),
        }
    }
}

impl Render for CheckboxUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_xl()
            .child(section(
                "Basic",
                control_row(vec![
                    self.checked.clone().into_any_element(),
                    self.unchecked.clone().into_any_element(),
                    self.labeled.clone().into_any_element(),
                    self.disabled.clone().into_any_element(),
                    self.disabled_checked.clone().into_any_element(),
                ]),
            ))
            .child(section(
                "Group",
                control_stack(vec![
                    self.group.clone().into_any_element(),
                    self.buttons_large.clone().into_any_element(),
                    self.buttons_default.clone().into_any_element(),
                    self.buttons_small.clone().into_any_element(),
                    self.buttons_stretch.clone().into_any_element(),
                ]),
            ))
            .child(section(
                "Custom option style / layout",
                control_stack(vec![
                    self.styled_cards.clone().into_any_element(),
                    self.styled_chips.clone().into_any_element(),
                    self.rich_options.clone().into_any_element(),
                ]),
            ))
    }
}

pub fn render_radio(cx: &mut App) -> AnyView {
    cx.new(|cx| RadioUsage::new(cx)).into()
}

struct RadioUsage {
    checked: Entity<Radio>,
    unchecked: Entity<Radio>,
    labeled: Entity<Radio>,
    disabled: Entity<Radio>,
    disabled_checked: Entity<Radio>,
    group: Entity<RadioGroup>,
    buttons_large: Entity<RadioGroup>,
    buttons_default: Entity<RadioGroup>,
    buttons_small: Entity<RadioGroup>,
    buttons_stretch: Entity<RadioGroup>,
    group_disabled: Entity<RadioGroup>,
    styled_cards: Entity<RadioGroup>,
    styled_chips: Entity<RadioGroup>,
    rich_options: Entity<RadioGroup>,
}

impl RadioUsage {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            checked: cx.new(|cx| Radio::new(true, cx)),
            unchecked: cx.new(|cx| Radio::new(false, cx)),
            labeled: cx.new(|cx| Radio::new(false, cx).label("Label")),
            disabled: cx.new(|cx| Radio::new(false, cx).disabled(true)),
            disabled_checked: cx.new(|cx| Radio::new(true, cx).disabled(true)),
            group: cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
            buttons_large: cx.new(|cx| city_radio_group(cx).large()),
            buttons_default: cx.new(city_radio_group),
            buttons_small: cx.new(|cx| city_radio_group(cx).small()),
            buttons_stretch: cx.new(|cx| city_radio_group(cx).stretch(true)),
            group_disabled: cx
                .new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)),
            styled_cards: cx.new(|cx| {
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
            }),
            styled_chips: cx.new(|cx| {
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
            }),
            rich_options: cx.new(|cx| {
                RadioGroup::new(vec!["Starter", "Team", "Enterprise"], 1, cx)
                    .horizontal()
                    .option_style(
                        RadioOptionStyle::new()
                            .selected_bg(rgb(0xfffbeb).into())
                            .selected_text_color(rgb(0x92400e).into())
                            .selected_border_color(rgb(0xf59e0b).into())
                            .hover_bg(rgb(0xfffbeb).into())
                            .radius(px(14.0))
                            .padding(px(14.0), px(12.0))
                            .gap(px(10.0)),
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
                                    .child(
                                        gpui::div()
                                            .flex()
                                            .items_center()
                                            .gap_1()
                                            .child(option.label.clone())
                                            .when(option.selected, |s| {
                                                s.child(Icon::new(IconName::CircleCheck).size_xs())
                                            }),
                                    )
                                    .child(gpui::div().text_xs().child(description)),
                            )
                            .into_any_element()
                    })
            }),
        }
    }
}

impl Render for RadioUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Space::new()
            .vertical()
            .gap_xl()
            .child(section(
                "Basic",
                control_row(vec![
                    self.checked.clone().into_any_element(),
                    self.unchecked.clone().into_any_element(),
                    self.labeled.clone().into_any_element(),
                    self.disabled.clone().into_any_element(),
                    self.disabled_checked.clone().into_any_element(),
                ]),
            ))
            .child(section(
                "Group",
                control_stack(vec![
                    self.group.clone().into_any_element(),
                    self.buttons_large.clone().into_any_element(),
                    self.buttons_default.clone().into_any_element(),
                    self.buttons_small.clone().into_any_element(),
                    self.buttons_stretch.clone().into_any_element(),
                    self.group_disabled.clone().into_any_element(),
                ]),
            ))
            .child(section(
                "Custom option style / layout",
                control_stack(vec![
                    self.styled_cards.clone().into_any_element(),
                    self.styled_chips.clone().into_any_element(),
                    self.rich_options.clone().into_any_element(),
                ]),
            ))
    }
}

pub fn render_switch(cx: &mut App) -> AnyView {
    cx.new(|cx| SwitchUsage {
        on: cx.new(|cx| Switch::new(true, cx)),
        off: cx.new(|cx| Switch::new(false, cx)),
        disabled: cx.new(|cx| Switch::new(false, cx).disabled(true)),
        disabled_on: cx.new(|cx| Switch::new(true, cx).disabled(true)),
    })
    .into()
}

struct SwitchUsage {
    on: Entity<Switch>,
    off: Entity<Switch>,
    disabled: Entity<Switch>,
    disabled_on: Entity<Switch>,
}

impl Render for SwitchUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        control_row(vec![
            self.on.clone().into_any_element(),
            self.off.clone().into_any_element(),
            self.disabled.clone().into_any_element(),
            self.disabled_on.clone().into_any_element(),
        ])
    }
}

pub fn render_input_number(cx: &mut App) -> AnyView {
    cx.new(|cx| InputNumberUsage {
        basic: cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(10.0)),
        vertical: cx.new(|cx| {
            InputNumber::new(5.0, cx)
                .min(0.0)
                .max(10.0)
                .controls_position(aura_components::InputNumberControlsPosition::Right)
        }),
        precision: cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01)),
    })
    .into()
}

struct InputNumberUsage {
    basic: Entity<InputNumber>,
    vertical: Entity<InputNumber>,
    precision: Entity<InputNumber>,
}

impl Render for InputNumberUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        control_stack(vec![
            self.basic.clone().into_any_element(),
            self.vertical.clone().into_any_element(),
            self.precision.clone().into_any_element(),
        ])
    }
}

pub fn render_textarea(cx: &mut App) -> AnyView {
    cx.new(|cx| TextareaUsage {
        basic: cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3)),
        limit: cx.new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2)),
    })
    .into()
}

struct TextareaUsage {
    basic: Entity<Textarea>,
    limit: Entity<Textarea>,
}

impl Render for TextareaUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        control_stack(vec![
            self.basic.clone().into_any_element(),
            self.limit.clone().into_any_element(),
        ])
    }
}

pub fn render_slider(cx: &mut App) -> AnyView {
    cx.new(|cx| SliderUsage {
        basic: cx.new(|cx| Slider::new(50.0, cx)),
        step: cx.new(|cx| Slider::new(20.0, cx).step(10.0)),
    })
    .into()
}

struct SliderUsage {
    basic: Entity<Slider>,
    step: Entity<Slider>,
}

impl Render for SliderUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        control_stack(vec![
            self.basic.clone().into_any_element(),
            self.step.clone().into_any_element(),
        ])
    }
}

pub fn render_rate(cx: &mut App) -> AnyView {
    cx.new(|cx| RateUsage {
        basic: cx.new(|cx| Rate::new(3.0, cx)),
        custom: cx.new(|cx| Rate::new(4.0, cx).max(10)),
    })
    .into()
}

struct RateUsage {
    basic: Entity<Rate>,
    custom: Entity<Rate>,
}

impl Render for RateUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        control_stack(vec![
            self.basic.clone().into_any_element(),
            self.custom.clone().into_any_element(),
        ])
    }
}

pub fn render_select(cx: &mut App) -> AnyView {
    cx.new(|cx| SelectUsage {
        basic: cx.new(|cx| {
            Select::new(
                vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"],
                Some(1),
                cx,
            )
        }),
    })
    .into()
}

struct SelectUsage {
    basic: Entity<Select>,
}

impl Render for SelectUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        self.basic.clone()
    }
}

fn city_radio_group(cx: &mut Context<RadioGroup>) -> RadioGroup {
    RadioGroup::new(
        vec!["New York", "Washington", "Los Angeles", "Chicago"],
        1,
        cx,
    )
    .button()
}

fn city_checkbox_group(cx: &mut Context<CheckboxGroup>) -> CheckboxGroup {
    CheckboxGroup::new(
        vec!["New York", "Washington", "Los Angeles", "Chicago"],
        vec![1],
        cx,
    )
    .button()
}
