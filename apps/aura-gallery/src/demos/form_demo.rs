use aura_components::{Checkbox, CheckboxGroup, Form, FormItem, Input, InputNumber, Radio, RadioGroup, Rate, Select, Slider, Switch, Textarea};
use aura_icons::Icon;
use gpui::{
    div, prelude::*, App, Context, Entity, IntoElement, Render, Window, px
};

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
    radio_group_disabled: Entity<RadioGroup>,
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
        let protocol_select = cx.new(|cx| Select::new(vec!["http://", "https://", "ftp://"], Some(1), cx));
        
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
            cb_group: cx.new(|cx| CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx)),
            radio_checked: cx.new(|cx| Radio::new(true, cx)),
            radio_unchecked: cx.new(|cx| Radio::new(false, cx)),
            radio_labeled: cx.new(|cx| Radio::new(false, cx).label("Label")),
            radio_disabled: cx.new(|cx| Radio::new(false, cx).disabled(true)),
            radio_disabled_checked: cx.new(|cx| Radio::new(true, cx).disabled(true)),
            radio_group: cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
            radio_group_disabled: cx.new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)),
            input_plain: cx.new(|cx| Input::new("", cx)),
            input_placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
            input_password: cx.new(|cx| Input::new("", cx).password().placeholder("Password")),
            input_password_custom: cx.new(|cx| Input::new("secret", cx).password().mask_char('*')),
            input_maxlength: cx.new(|cx| Input::new("", cx).placeholder("Max 5 chars").max_length(5)),
            input_prepend: cx.new(|cx| Input::new("", cx).prepend(|_, _| div().px_3().child("http://").into_any_element())),
            input_append: cx.new(|cx| Input::new("", cx).append(|_, _| div().px_3().child(".com").into_any_element())),
            input_composite: cx.new(|cx| {
                Input::new("", cx)
                    .prepend(|_, _| gpui::div().px_3().flex().items_center().child(Icon::new(aura_icons_lucide::IconName::User).size(px(14.0))).into_any_element())
                    .append(|_, _| div().px_3().text_size(px(12.0)).child("Admin").into_any_element())
            }),
            input_select_prepend: cx.new(|cx| {
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
            input_icon: cx.new(|cx| Input::new("", cx).placeholder("Search").icon_prefix(aura_icons_lucide::IconName::Search).clearable(true)),
            input_clearable: cx.new(|cx| Input::new("Clear me", cx).clearable(true)),
            input_disabled: cx.new(|cx| Input::new("Disabled", cx).disabled(true)),
            input_number: cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(10.0)),
            input_number_vertical: cx.new(|cx| InputNumber::new(5.0, cx).min(0.0).max(10.0).controls_position(aura_components::InputNumberControlsPosition::Right)),
            input_number_precision: cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01)),
            textarea: cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3, cx)),
            textarea_limit: cx.new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2, cx)),
            slider_basic: cx.new(|cx| Slider::new(50.0, cx)),
            slider_step: cx.new(|cx| Slider::new(20.0, cx).step(10.0)),
            rate_basic: cx.new(|cx| Rate::new(3.0, cx)),
            rate_custom: cx.new(|cx| Rate::new(4.0, cx).max(10)),
            select_basic: cx.new(|cx| Select::new(vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"], Some(1), cx)),
        }
    }
}

impl Render for FormDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let _theme = &cx.global::<aura_core::Config>().theme;

        Form::new()
            .child(FormItem::new().label("Switch 开关").child(
                div().flex().flex_row().gap_4().items_center()
                    .child(self.switch_on.clone())
                    .child(self.switch_off.clone())
                    .child(self.switch_disabled.clone())
                    .child(self.switch_disabled_on.clone())
            ))
            .child(FormItem::new().label("Checkbox 多选").required(true).child(
                div().flex().flex_row().gap_4().items_center()
                    .child(self.cb_checked.clone())
                    .child(self.cb_unchecked.clone())
                    .child(self.cb_labeled.clone())
                    .child(self.cb_disabled.clone())
                    .child(self.cb_disabled_checked.clone())
            ))
            .child(FormItem::new().label("CheckboxGroup 多选组").child(self.cb_group.clone()))
            .child(FormItem::new().label("Radio 单选").child(
                div().flex().flex_row().gap_4().items_center()
                    .child(self.radio_checked.clone())
                    .child(self.radio_unchecked.clone())
                    .child(self.radio_labeled.clone())
                    .child(self.radio_disabled.clone())
                    .child(self.radio_disabled_checked.clone())
            ))
            .child(FormItem::new().label("RadioGroup 单选组").child(
                div().flex().flex_col().gap_2()
                    .child(self.radio_group.clone())
                    .child(self.radio_group_disabled.clone())
            ))
            .child(FormItem::new().label("Select 下拉选择").child(self.select_basic.clone()))
            .child(FormItem::new().label("Input 输入框").required(true).child(
                div().flex().flex_col().gap_2()
                    .child(self.input_plain.clone())
                    .child(self.input_placeholder.clone())
                    .child(self.input_password.clone())
                    .child(self.input_password_custom.clone())
                    .child(self.input_maxlength.clone())
                    .child(self.input_prepend.clone())
                    .child(self.input_append.clone())
                    .child(self.input_select_prepend.clone())
                    .child(self.input_composite.clone())
                    .child(self.input_icon.clone())
                    .child(self.input_clearable.clone())
                    .child(self.input_disabled.clone())
            ))
            .child(FormItem::new().label("InputNumber 数字输入").child(
                div().flex().flex_col().gap_2()
                    .child(self.input_number.clone())
                    .child(self.input_number_vertical.clone())
                    .child(self.input_number_precision.clone())
            ))
            .child(FormItem::new().label("Textarea 文本域").error("This is an error message").child(
                div().flex().flex_col().gap_2()
                    .child(self.textarea.clone())
                    .child(self.textarea_limit.clone())
            ))
            .child(FormItem::new().label("Slider 滑块").child(
                div().flex().flex_col().gap_2()
                    .child(self.slider_basic.clone())
                    .child(self.slider_step.clone())
            ))
            .child(FormItem::new().label("Rate 评分").child(
                div().flex().flex_col().gap_2()
                    .child(self.rate_basic.clone())
                    .child(self.rate_custom.clone())
            ))
    }
}
