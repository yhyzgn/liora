use aura_components::{Checkbox, Input, Radio, RadioGroup, Switch};
use gpui::{
    div, prelude::*, App, Context, Entity, IntoElement, Render, Window,
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
    radio_checked: Entity<Radio>,
    radio_unchecked: Entity<Radio>,
    radio_labeled: Entity<Radio>,
    radio_disabled: Entity<Radio>,
    radio_disabled_checked: Entity<Radio>,
    radio_group: Entity<RadioGroup>,
    radio_group_disabled: Entity<RadioGroup>,
    input_plain: Entity<Input>,
    input_placeholder: Entity<Input>,
    input_icon: Entity<Input>,
    input_clearable: Entity<Input>,
    input_disabled: Entity<Input>,
    textarea: Entity<Input>,
}

impl FormDemo {
    pub fn new(cx: &mut Context<Self>) -> Self {
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
            radio_checked: cx.new(|cx| Radio::new(true, cx)),
            radio_unchecked: cx.new(|cx| Radio::new(false, cx)),
            radio_labeled: cx.new(|cx| Radio::new(false, cx).label("Label")),
            radio_disabled: cx.new(|cx| Radio::new(false, cx).disabled(true)),
            radio_disabled_checked: cx.new(|cx| Radio::new(true, cx).disabled(true)),
            radio_group: cx.new(|cx| RadioGroup::new(vec!["Option A", "Option B", "Option C"], 1, cx)),
            radio_group_disabled: cx.new(|cx| RadioGroup::new(vec!["Disabled A", "Disabled B"], 0, cx).disabled(true)),
            input_plain: cx.new(|cx| Input::new("", cx)),
            input_placeholder: cx.new(|cx| Input::new("", cx).placeholder("Type something...")),
            input_icon: cx.new(|cx| {
                Input::new("", cx)
                    .placeholder("Search")
                    .icon_prefix(aura_icons_lucide::IconName::Search)
                    .clearable(true)
            }),
            input_clearable: cx.new(|cx| Input::new("Clear me", cx).clearable(true)),
            input_disabled: cx.new(|cx| Input::new("Disabled", cx).disabled(true)),
            textarea: cx.new(|cx| Input::new("Line 1\nLine 2\nLine 3", cx).placeholder("Multi-line textarea...")),
        }
    }
}

impl Render for FormDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &cx.global::<aura_core::Config>().theme;

        div().flex().flex_col().gap_6()
            .child(section(theme, "Switch 开关", "ON/OFF 切换开关", div().flex().flex_row().gap_4().items_center()
                .child(self.switch_on.clone())
                .child(self.switch_off.clone())
                .child(self.switch_disabled.clone())
                .child(self.switch_disabled_on.clone())
            ))
            .child(section(theme, "Checkbox 多选", "多选框", div().flex().flex_row().gap_4().items_center()
                .child(self.cb_checked.clone())
                .child(self.cb_unchecked.clone())
                .child(self.cb_labeled.clone())
                .child(self.cb_disabled.clone())
                .child(self.cb_disabled_checked.clone())
            ))
            .child(section(theme, "Radio 单选", "单个单选按钮", div().flex().flex_row().gap_4().items_center()
                .child(self.radio_checked.clone())
                .child(self.radio_unchecked.clone())
                .child(self.radio_labeled.clone())
                .child(self.radio_disabled.clone())
                .child(self.radio_disabled_checked.clone())
            ))
            .child(section(theme, "RadioGroup 单选组", "一组单选按钮，支持方向键导航", div().flex().flex_col().gap_4()
                .child(self.radio_group.clone())
                .child(self.radio_group_disabled.clone())
            ))
            .child(section(theme, "Input 输入框", "支持搜索图标、清空、禁用、多行等状态", div().flex().flex_col().gap_2()
                .child(self.input_plain.clone())
                .child(self.input_placeholder.clone())
                .child(self.input_icon.clone())
                .child(self.input_clearable.clone())
                .child(self.input_disabled.clone())
                .child(self.textarea.clone())
            ))
    }
}

fn section(theme: &aura_theme::Theme, title: &str, desc: &str, content: impl IntoElement) -> impl IntoElement {
    div().flex().flex_col().gap_2()
        .child(div().flex().flex_col().gap_1()
            .child(div().text_size(gpui::px(theme.font_size.lg)).text_color(theme.neutral.text_1).font_weight(gpui::FontWeight::BOLD).child(title.to_string()))
            .child(div().text_size(gpui::px(theme.font_size.sm)).text_color(theme.neutral.text_3).child(desc.to_string())))
        .child(content)
}
