use liora_components::{Transfer, TransferItem};

fn fixture() -> Vec<TransferItem> {
    vec![
        TransferItem::new("a", "Alpha"),
        TransferItem::new("b", "Beta").disabled(true),
        TransferItem::new("c", "Gamma"),
    ]
}

#[test]
fn moves_only_enabled_checked_source_items_to_target() {
    let mut target = vec!["c".into()];
    let mut checked = vec!["a".into(), "b".into()];

    let moved = Transfer::move_to_target(&fixture(), &mut target, &mut checked);

    assert_eq!(moved, vec!["a"]);
    assert_eq!(target, vec!["c", "a"]);
    assert!(checked.is_empty());
}

#[test]
fn moves_only_enabled_checked_target_items_to_source() {
    let mut target = vec!["a".into(), "b".into(), "c".into()];
    let mut checked = vec!["b".into(), "c".into()];

    let moved = Transfer::move_to_source(&fixture(), &mut target, &mut checked);

    assert_eq!(moved, vec!["c"]);
    assert_eq!(target, vec!["a", "b"]);
    assert!(checked.is_empty());
}

#[test]
fn filters_items_by_label_or_key() {
    let result = Transfer::filter_items(&fixture(), "alp");
    assert_eq!(result, vec!["a"]);

    let result = Transfer::filter_items(&fixture(), "c");
    assert_eq!(result, vec!["c"]);
}

#[test]
fn moved_source_items_stay_checked_on_target_side() {
    let mut target = vec![];
    let mut source_checked = vec!["a".into(), "c".into()];
    let mut target_checked = vec![];

    Transfer::move_to_target_with_checked(
        &fixture(),
        &mut target,
        &mut source_checked,
        &mut target_checked,
    );

    assert!(source_checked.is_empty());
    assert_eq!(target_checked, vec!["a", "c"]);
}

#[test]
fn moved_target_items_stay_checked_on_source_side() {
    let mut target = vec!["a".into(), "c".into()];
    let mut source_checked = vec![];
    let mut target_checked = vec!["a".into()];

    Transfer::move_to_source_with_checked(
        &fixture(),
        &mut target,
        &mut target_checked,
        &mut source_checked,
    );

    assert_eq!(target, vec!["c"]);
    assert!(target_checked.is_empty());
    assert_eq!(source_checked, vec!["a"]);
}
