use aura_components::{Cascader, CascaderOption};

fn fixture() -> Vec<CascaderOption> {
    vec![
        CascaderOption::new("guide", "Guide")
            .child(CascaderOption::new("install", "Install"))
            .child(
                CascaderOption::new("advanced", "Advanced")
                    .child(CascaderOption::new("perf", "Performance")),
            ),
        CascaderOption::new("disabled", "Disabled").disabled(true),
    ]
}

#[test]
fn resolves_labels_for_a_selected_value_path() {
    let options = fixture();
    let labels = Cascader::labels_for_path(
        &options,
        &["guide".into(), "advanced".into(), "perf".into()],
    );

    assert_eq!(labels, vec!["Guide", "Advanced", "Performance"]);
}

#[test]
fn rejects_unknown_or_disabled_value_paths() {
    let options = fixture();

    assert!(!Cascader::is_selectable_path(
        &options,
        &["guide".into(), "missing".into()]
    ));
    assert!(!Cascader::is_selectable_path(
        &options,
        &["disabled".into()]
    ));
}
