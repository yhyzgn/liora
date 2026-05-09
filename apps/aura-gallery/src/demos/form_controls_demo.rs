use aura_components::{
    Checkbox, CheckboxGroup, Input, InputNumber, Radio, RadioGroup, Rate, Select, Slider, Switch,
    Textarea,
};
use aura_icons::Icon;
use gpui::{AnyView, App, Context, Entity, IntoElement, Render, Window, div, prelude::*, px};

fn section(title: &'static str, content: impl IntoElement) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(div().font_weight(gpui::FontWeight::BOLD).child(title))
        .child(content)
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
        let protocol_select =
            cx.new(|cx| Select::new(vec!["http://", "https://", "ftp://"], Some(1), cx));

        Self {
            plain: cx.new(|cx| Input::new("", cx)),
            placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
            password: cx.new(|cx| Input::new("", cx).password().placeholder("Password")),
            password_custom: cx.new(|cx| Input::new("secret", cx).password().mask_char('*')),
            maxlength: cx.new(|cx| Input::new("", cx).placeholder("Max 5 chars").max_length(5)),
            prepend: cx.new(|cx| {
                Input::new("", cx).prepend(|_, _| div().px_3().child("http://").into_any_element())
            }),
            append: cx.new(|cx| {
                Input::new("", cx).append(|_, _| div().px_3().child(".com").into_any_element())
            }),
            composite: cx.new(|cx| {
                Input::new("", cx)
                    .prepend(|_, _| {
                        div()
                            .px_3()
                            .flex()
                            .items_center()
                            .child(Icon::new(aura_icons_lucide::IconName::User).size(px(14.0)))
                            .into_any_element()
                    })
                    .append(|_, _| {
                        div()
                            .px_3()
                            .text_size(px(12.0))
                            .child("Admin")
                            .into_any_element()
                    })
            }),
            select_prepend: cx.new(|cx| {
                let sel = protocol_select.clone();
                Input::new("", cx)
                    .prepend(move |_, cx| {
                        let theme = cx.global::<aura_core::Config>().theme.clone();
                        sel.update(cx, |s, cx| {
                            s.set_borderless(true, cx);
                            s.set_radius_none(true, cx);
                            s.set_radius_left_none(false, cx);
                            s.set_width(px(90.0), cx);
                            s.set_text_size(px(theme.font_size.sm), cx);
                            s.set_text_color(theme.neutral.text_3, cx);
                            s.set_padding_x(px(8.0), cx);
                        });
                        div().w(px(90.0)).child(sel.clone()).into_any_element()
                    })
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
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(self.plain.clone())
            .child(self.placeholder.clone())
            .child(self.password.clone())
            .child(self.password_custom.clone())
            .child(self.maxlength.clone())
            .child(self.prepend.clone())
            .child(self.append.clone())
            .child(self.select_prepend.clone())
            .child(self.composite.clone())
            .child(self.icon.clone())
            .child(self.clearable.clone())
            .child(self.disabled.clone())
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
        }
    }
}

impl Render for CheckboxUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(section(
                "Basic",
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .items_center()
                    .child(self.checked.clone())
                    .child(self.unchecked.clone())
                    .child(self.labeled.clone())
                    .child(self.disabled.clone())
                    .child(self.disabled_checked.clone()),
            ))
            .child(section(
                "Group",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.group.clone())
                    .child(self.buttons_large.clone())
                    .child(self.buttons_default.clone())
                    .child(self.buttons_small.clone())
                    .child(self.buttons_stretch.clone()),
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
        }
    }
}

impl Render for RadioUsage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(section(
                "Basic",
                div()
                    .flex()
                    .flex_row()
                    .gap_4()
                    .items_center()
                    .child(self.checked.clone())
                    .child(self.unchecked.clone())
                    .child(self.labeled.clone())
                    .child(self.disabled.clone())
                    .child(self.disabled_checked.clone()),
            ))
            .child(section(
                "Group",
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .child(self.group.clone())
                    .child(self.buttons_large.clone())
                    .child(self.buttons_default.clone())
                    .child(self.buttons_small.clone())
                    .child(self.buttons_stretch.clone())
                    .child(self.group_disabled.clone()),
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
        div()
            .flex()
            .flex_row()
            .gap_4()
            .items_center()
            .child(self.on.clone())
            .child(self.off.clone())
            .child(self.disabled.clone())
            .child(self.disabled_on.clone())
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
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(self.basic.clone())
            .child(self.vertical.clone())
            .child(self.precision.clone())
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
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(self.basic.clone())
            .child(self.limit.clone())
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
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(self.basic.clone())
            .child(self.step.clone())
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
        div()
            .flex()
            .flex_col()
            .gap_3()
            .child(self.basic.clone())
            .child(self.custom.clone())
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
