use liora_components::{CheckboxGroupLayout, CheckboxGroupSize, RadioGroupLayout, RadioGroupSize};

#[test]
fn radio_group_layout_defaults_to_vertical_default_size() {
    assert_eq!(RadioGroupLayout::default(), RadioGroupLayout::Vertical);
    assert_eq!(RadioGroupSize::default(), RadioGroupSize::Default);
}

#[test]
fn checkbox_group_layout_defaults_to_vertical_default_size() {
    assert_eq!(
        CheckboxGroupLayout::default(),
        CheckboxGroupLayout::Vertical
    );
    assert_eq!(CheckboxGroupSize::default(), CheckboxGroupSize::Default);
}
