use aura_components::{Checkbox, CheckboxGroup, Form, FormItem, Input, InputNumber, Radio, RadioGroup, Rate, Select, Slider, Switch, Textarea};
use aura_icons::Icon;
use gpui::{
    div, prelude::*, App, Context, Entity, IntoElement, Render, Window, px
};

pub fn render(cx: &mut App) -> Entity<FormDemo> {
    cx.new(|cx| FormDemo::new(cx))
}

pub struct FormDemo {
    form: Entity<Form>,
}

impl FormDemo {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let protocol_select = cx.new(|cx| Select::new(vec!["http://", "https://", "ftp://"], Some(1), cx));
        
        let switch_on = cx.new(|cx| Switch::new(true, cx));
        let switch_off = cx.new(|cx| Switch::new(false, cx));
        let switch_disabled = cx.new(|cx| Switch::new(false, cx).disabled(true));
        let switch_disabled_on = cx.new(|cx| Switch::new(true, cx).disabled(true));
        
        let cb_checked = cx.new(|cx| Checkbox::new(true, cx));
        let cb_unchecked = cx.new(|cx| Checkbox::new(false, cx));
        let cb_labeled = cx.new(|cx| Checkbox::new(false, cx).label("Label"));
        let cb_disabled = cx.new(|cx| Checkbox::new(false, cx).disabled(true));
        let cb_disabled_checked = cx.new(|cx| Checkbox::new(true, cx).disabled(true));
        let cb_group = cx.new(|cx| CheckboxGroup::new(vec!["Option 1", "Option 2", "Option 3"], vec![0, 2], cx));
        
        let radio_checked = cx.new(|cx| Radio::new(true, cx));
        let radio_unchecked = cx.new(|cx| Radio::new(false, cx));
        let radio_labeled = cx.new(|cx| Radio::new(false, cx).label("Label"));
        let radio_disabled = cx.new(|cx| Radio::new(false, cx).disabled(true));
        let radio_disabled_checked = cx.new(|cx| Radio::new(true, cx).disabled(true));
        let radio_group = cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx));
        let radio_group_disabled = cx.new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true));
        
        let input_plain = cx.new(|cx| Input::new("", cx));
        let input_placeholder = cx.new(|cx| Input::new("", cx).placeholder("Type something..."));
        let input_password = cx.new(|cx| Input::new("", cx).password().placeholder("Password"));
        let input_password_custom = cx.new(|cx| Input::new("secret", cx).password().mask_char('*'));
        let input_maxlength = cx.new(|cx| Input::new("", cx).placeholder("Max 5 chars").max_length(5));
        
        let input_prepend = cx.new(|cx| Input::new("", cx).prepend(|_, _| div().px_3().child("http://").into_any_element()));
        let input_append = cx.new(|cx| Input::new("", cx).append(|_, _| div().px_3().child(".com").into_any_element()));
        let input_composite = cx.new(|cx| {
            Input::new("", cx)
                .prepend(|_, _| gpui::div().px_3().flex().items_center().child(Icon::new(aura_icons_lucide::IconName::User).size(px(14.0))).into_any_element())
                .append(|_, _| div().px_3().text_size(px(12.0)).child("Admin").into_any_element())
        });
        
        let input_select_prepend = cx.new(|cx| {
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
        });
        
        let input_icon = cx.new(|cx| Input::new("", cx).placeholder("Search").icon_prefix(aura_icons_lucide::IconName::Search).clearable(true));
        let input_clearable = cx.new(|cx| Input::new("Clear me", cx).clearable(true));
        let input_disabled = cx.new(|cx| Input::new("Disabled", cx).disabled(true));
        
        let input_number = cx.new(|cx| InputNumber::new(10.0, cx).min(0.0).max(10.0));
        let input_number_vertical = cx.new(|cx| InputNumber::new(5.0, cx).min(0.0).max(10.0).controls_position(aura_components::InputNumberControlsPosition::Right));
        let input_number_precision = cx.new(|cx| InputNumber::new(1.23, cx).precision(2).step(0.01));
        
        let textarea = cx.new(|cx| Textarea::new("Line 1\nLine 2", cx).rows(3, cx));
        let textarea_limit = cx.new(|cx| Textarea::new("Limited content", cx).max_length(50).rows(2, cx));
        
        let slider_basic = cx.new(|cx| Slider::new(50.0, cx));
        let slider_step = cx.new(|cx| Slider::new(20.0, cx).step(10.0));
        
        let rate_basic = cx.new(|cx| Rate::new(3.0, cx));
        let rate_custom = cx.new(|cx| Rate::new(4.0, cx).max(10));
        
        let select_basic = cx.new(|cx| Select::new(vec!["Apple", "Banana", "Orange", "Grape", "Watermelon"], Some(1), cx));

        let form = cx.new(|cx| {
            let mut f = Form::new(cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Switch 开关").child(
                div().flex().flex_row().gap_4().items_center().child(switch_on).child(switch_off).child(switch_disabled).child(switch_disabled_on)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Checkbox 多选").required(true).child(
                div().flex().flex_row().gap_4().items_center().child(cb_checked).child(cb_unchecked).child(cb_labeled).child(cb_disabled).child(cb_disabled_checked)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("CheckboxGroup 多选组").child(cb_group)), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Radio 单选").child(
                div().flex().flex_row().gap_4().items_center().child(radio_checked).child(radio_unchecked).child(radio_labeled).child(radio_disabled).child(radio_disabled_checked)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("RadioGroup 单选组").child(
                div().flex().flex_col().gap_2().child(radio_group).child(radio_group_disabled)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Select 下拉选择").child(select_basic)), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Input 输入框").required(true).child(
                div().flex().flex_col().gap_2()
                    .child(input_plain).child(input_placeholder).child(input_password).child(input_password_custom)
                    .child(input_maxlength).child(input_prepend).child(input_append).child(input_select_prepend)
                    .child(input_composite).child(input_icon).child(input_clearable).child(input_disabled)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("InputNumber 数字输入").child(
                div().flex().flex_col().gap_2().child(input_number).child(input_number_vertical).child(input_number_precision)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Textarea 文本域").error("This is an error message").child(
                div().flex().flex_col().gap_2().child(textarea).child(textarea_limit)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Slider 滑块").child(
                div().flex().flex_col().gap_2().child(slider_basic).child(slider_step)
            )), cx);
            
            f.add_item(cx.new(|cx| FormItem::new(cx).label("Rate 评分").child(
                div().flex().flex_col().gap_2().child(rate_basic).child(rate_custom)
            )), cx);
            
            f
        });

        Self { form }
    }
}

impl Render for FormDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div().child(self.form.clone())
    }
}
