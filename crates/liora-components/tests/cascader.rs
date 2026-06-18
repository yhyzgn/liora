use liora_components::{Cascader, CascaderOption};

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

#[test]
fn derives_stable_interactive_ids_from_value_paths() {
    let id = Cascader::popup_item_id("region", &["zhejiang".into(), "hangzhou".into()]);

    assert_eq!(id.as_ref(), "region-item-zhejiang-hangzhou");
}

#[test]
fn installs_children_at_a_lazy_path() {
    let mut options = vec![CascaderOption::new("remote", "Remote")];

    assert!(Cascader::set_children_in_options(
        &mut options,
        &["remote".into()],
        vec![CascaderOption::new("leaf", "Loaded leaf").leaf(true)],
    ));

    assert_eq!(
        Cascader::labels_for_path(&options, &["remote".into(), "leaf".into()]),
        vec!["Remote", "Loaded leaf"]
    );
}

#[test]
fn lazy_empty_branch_is_not_selectable_until_marked_leaf() {
    let branch = CascaderOption::new("remote", "Remote");
    let leaf = CascaderOption::new("done", "Done").leaf(true);

    assert!(Cascader::should_lazy_load_option(&branch, true));
    assert!(!Cascader::is_selectable_option(&branch, true));
    assert!(Cascader::is_selectable_option(&leaf, true));
}
