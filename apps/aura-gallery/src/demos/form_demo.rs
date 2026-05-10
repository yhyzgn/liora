use aura_components::{
    Checkbox, CheckboxGroup, Form, FormItem, Input, InputNumber, Radio, RadioGroup, Rate, Select,
    Slider, Space, Switch, Textarea,
};
use gpui::{App, Context, Entity, IntoElement, Render, Window, prelude::*};

pub fn render(cx: &mut App) -> Entity<FormDemo> {
    cx.new(|cx| FormDemo::new(cx))
}

pub struct FormDemo {
    switch_on: Entity<Switch>,
    switch_off: Entity<Switch>,
    switch_disabled: Entity<Switch>,
    switch_disabled_on: Entity<Switch>,
    cb_checked: Entity<Checkbox>,
    cb_unchecked: Entity<Checkbox>,
    cb_labeled: Entity<Checkbox>,
    cb_disabled: Entity<Checkbox>,
    cb_disabled_checked: Entity<Checkbox>,
    cb_group: Entity<CheckboxGroup>,
    radio_checked: Entity<Radio>,
    radio_unchecked: Entity<Radio>,
    radio_labeled: Entity<Radio>,
    radio_disabled: Entity<Radio>,
    radio_disabled_checked: Entity<Radio>,
    radio_group: Entity<RadioGroup>,
    radio_group_buttons_large: Entity<RadioGroup>,
    radio_group_buttons_default: Entity<RadioGroup>,
    radio_group_buttons_small: Entity<RadioGroup>,
    radio_group_buttons_stretch: Entity<RadioGroup>,
    radio_group_disabled: Entity<RadioGroup>,
    cb_group_buttons_large: Entity<CheckboxGroup>,
    cb_group_buttons_default: Entity<CheckboxGroup>,
    cb_group_buttons_small: Entity<CheckboxGroup>,
    cb_group_buttons_stretch: Entity<CheckboxGroup>,
    input_plain: Entity<Input>,
    input_placeholder: Entity<Input>,
    input_password: Entity<Input>,
    input_password_custom: Entity<Input>,
    input_maxlength: Entity<Input>,
    input_prepend: Entity<Input>,
    input_append: Entity<Input>,
    input_composite: Entity<Input>,
    input_select_prepend: Entity<Input>,
    input_icon: Entity<Input>,
    input_clearable: Entity<Input>,
    input_disabled: Entity<Input>,
    input_number: Entity<InputNumber>,
    input_number_vertical: Entity<InputNumber>,
    input_number_precision: Entity<InputNumber>,
    textarea: Entity<Textarea>,
    textarea_limit: Entity<Textarea>,
    slider_basic: Entity<Slider>,
    slider_step: Entity<Slider>,
    rate_basic: Entity<Rate>,
    rate_custom: Entity<Rate>,
    select_basic: Entity<Select>,
}

impl FormDemo {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let protocol_select = cx.new(|cx| {
            Select::new(vec!["http://", "https://", "ftp://"], Some(1), cx)
                .borderless()
                .radius_right_none()
                .width_xs()
                .text_sm()
                .padding_x_sm()
        });

        Self {
            switch_on: cx.new(|cx| Switch::new(true, cx)),
            switch_off: cx.new(|cx| Switch::new(false, cx)),
            switch_disabled: cx.new(|cx| Switch::new(false, cx).disabled(true)),
            switch_disabled_on: cx.new(|cx| Switch::new(true, cx).disabled(true)),
            cb_checked: cx.new(|cx| Checkbox::new(true, cx)),
            cb_unchecked: cx.new(|cx| Checkbox::new(false, cx)),
            cb_labeled: cx.new(|cx| Checkbox::new(false, cx).label("Label")),
            cb_disabled: cx.new(|cx| Checkbox::new(false, cx).disabled(true)),
            cb_disabled_checked: cx.new(|cx| Checkbox::new(true, cx).disabled(true)),
            cb_group: cx.new(|cx| {
                CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx)
            }),
            radio_checked: cx.new(|cx| Radio::new(true, cx)),
            radio_unchecked: cx.new(|cx| Radio::new(false, cx)),
            radio_labeled: cx.new(|cx| Radio::new(false, cx).label("Label")),
            radio_disabled: cx.new(|cx| Radio::new(false, cx).disabled(true)),
            radio_disabled_checked: cx.new(|cx| Radio::new(true, cx).disabled(true)),
            radio_group: cx
                .new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
            radio_group_buttons_large: cx.new(|cx| {
                RadioGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    1,
                    cx,
                )
                .button()
                .large()
            }),
            radio_group_buttons_default: cx.new(|cx| {
                RadioGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    1,
                    cx,
                )
                .button()
            }),
            radio_group_buttons_small: cx.new(|cx| {
                RadioGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    1,
                    cx,
                )
                .button()
                .small()
            }),
            radio_group_buttons_stretch: cx.new(|cx| {
                RadioGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    1,
                    cx,
                )
                .button()
                .stretch(true)
            }),
            radio_group_disabled: cx
                .new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)),
            cb_group_buttons_large: cx.new(|cx| {
                CheckboxGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    vec![1],
                    cx,
                )
                .button()
                .large()
            }),
            cb_group_buttons_default: cx.new(|cx| {
                CheckboxGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    vec![1],
                    cx,
                )
                .button()
            }),
            cb_group_buttons_small: cx.new(|cx| {
                CheckboxGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    vec![1],
                    cx,
                )
                .button()
                .small()
            }),
            cb_group_buttons_stretch: cx.new(|cx| {
                CheckboxGroup::new(
                    vec!["New York", "Washington", "Los Angeles", "Chicago"],
                    vec![1],
                    cx,
                )
                .button()
                .stretch(true)
            }),
            input_plain: cx.new(|cx| Input::new("", cx)),
            input_placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
            input_password: cx.new(|cx| Input::new("", cx).password().placeholder("Password")),
            input_password_custom: cx.new(|cx| Input::new("secret", cx).password().mask_char('*')),
            input_maxlength: cx
                .new(|cx| Input::new("", cx).placeholder("Max 5 chars").max_length(5)),
            input_prepend: cx.new(|cx| Input::new("", cx).prepend_text("http://")),
            input_append: cx.new(|cx| Input::new("", cx).append_text(".com")),
            input_composite: cx.new(|cx| {
                Input::new("", cx)
                    .prepend_icon(aura_icons_lucide::IconName::User)
                    .append_text("Admin")
            }),
            input_select_prepend: cx.new(|cx| {
                let sel = protocol_select.clone();
                Input::new("", cx)
                    .prepend(move |_, _| sel.clone().into_any_element())
                    .placeholder("domain.com")
            }),
            input_icon: cx.new(|cx| {
                Input::new("", cx)
                    .placeholder("Search")
                    .icon_prefix(aura_icons_lucide::IconName::Search)
                    .clearable(true)
            }),
            input_clearable: cx.new(|cx| Input::new("Clear me", cx).clearable(true)),
            input_disabled: cx.new(|cx| Input::new("Disabled", cx).disabled(true)),
            input_number: cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(10.0)),
            input_number_vertical: cx.new(|cx| {
                InputNumber::new(5.0, cx)
                    .min(0.0)
                    .max(10.0)
                    .controls_position(aura_components::InputNumberControlsPosition::Right)
            }),
            input_number_precision: cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01)),
            textarea: cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3)),
            textarea_limit: cx
                .new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2)),
            slider_basic: cx.new(|cx| Slider::new(50.0, cx)),
            slider_step: cx.new(|cx| Slider::new(20.0, cx).step(10.0)),
            rate_basic: cx.new(|cx| Rate::new(3.0, cx)),
            rate_custom: cx.new(|cx| Rate::new(4.0, cx).max(10)),
            select_basic: cx.new(|cx| {
                Select::new(
                    vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"],
                    Some(1),
                    cx,
                )
            }),
        }
    }
}

impl Render for FormDemo {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        Form::new()
            .child(FormItem::new().label("Switch 开关").child(control_row(vec![
                self.switch_on.clone().into_any_element(),
                self.switch_off.clone().into_any_element(),
                self.switch_disabled.clone().into_any_element(),
                self.switch_disabled_on.clone().into_any_element(),
            ])))
            .child(
                FormItem::new()
                    .label("Checkbox 多选")
                    .required(true)
                    .child(control_row(vec![
                        self.cb_checked.clone().into_any_element(),
                        self.cb_unchecked.clone().into_any_element(),
                        self.cb_labeled.clone().into_any_element(),
                        self.cb_disabled.clone().into_any_element(),
                        self.cb_disabled_checked.clone().into_any_element(),
                    ])),
            )
            .child(
                FormItem::new()
                    .label("CheckboxGroup 多选组")
                    .child(control_stack(vec![
                        self.cb_group.clone().into_any_element(),
                        self.cb_group_buttons_large.clone().into_any_element(),
                        self.cb_group_buttons_default.clone().into_any_element(),
                        self.cb_group_buttons_small.clone().into_any_element(),
                        self.cb_group_buttons_stretch.clone().into_any_element(),
                    ])),
            )
            .child(FormItem::new().label("Radio 单选").child(control_row(vec![
                self.radio_checked.clone().into_any_element(),
                self.radio_unchecked.clone().into_any_element(),
                self.radio_labeled.clone().into_any_element(),
                self.radio_disabled.clone().into_any_element(),
                self.radio_disabled_checked.clone().into_any_element(),
            ])))
            .child(
                FormItem::new()
                    .label("RadioGroup 单选组")
                    .child(control_stack(vec![
                        self.radio_group.clone().into_any_element(),
                        self.radio_group_buttons_large.clone().into_any_element(),
                        self.radio_group_buttons_default.clone().into_any_element(),
                        self.radio_group_buttons_small.clone().into_any_element(),
                        self.radio_group_buttons_stretch.clone().into_any_element(),
                        self.radio_group_disabled.clone().into_any_element(),
                    ])),
            )
            .child(
                FormItem::new()
                    .label("Select 下拉选择")
                    .child(self.select_basic.clone()),
            )
            .child(
                FormItem::new()
                    .label("Input 输入框")
                    .required(true)
                    .child(control_stack(vec![
                        self.input_plain.clone().into_any_element(),
                        self.input_placeholder.clone().into_any_element(),
                        self.input_password.clone().into_any_element(),
                        self.input_password_custom.clone().into_any_element(),
                        self.input_maxlength.clone().into_any_element(),
                        self.input_prepend.clone().into_any_element(),
                        self.input_append.clone().into_any_element(),
                        self.input_select_prepend.clone().into_any_element(),
                        self.input_composite.clone().into_any_element(),
                        self.input_icon.clone().into_any_element(),
                        self.input_clearable.clone().into_any_element(),
                        self.input_disabled.clone().into_any_element(),
                    ])),
            )
            .child(
                FormItem::new()
                    .label("InputNumber 数字输入")
                    .child(control_stack(vec![
                        self.input_number.clone().into_any_element(),
                        self.input_number_vertical.clone().into_any_element(),
                        self.input_number_precision.clone().into_any_element(),
                    ])),
            )
            .child(
                FormItem::new()
                    .label("Textarea 文本域")
                    .error("This is an error message")
                    .child(control_stack(vec![
                        self.textarea.clone().into_any_element(),
                        self.textarea_limit.clone().into_any_element(),
                    ])),
            )
            .child(
                FormItem::new()
                    .label("Slider 滑块")
                    .child(control_stack(vec![
                        self.slider_basic.clone().into_any_element(),
                        self.slider_step.clone().into_any_element(),
                    ])),
            )
            .child(FormItem::new().label("Rate 评分").child(control_stack(vec![
                self.rate_basic.clone().into_any_element(),
                self.rate_custom.clone().into_any_element(),
            ])))
    }
}

fn control_row(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().wrap().gap_lg().children(children)
}

fn control_stack(children: Vec<impl IntoElement>) -> impl IntoElement {
    Space::new().vertical().gap_sm().children(children)
}
