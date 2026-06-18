use liora_components::{Autocomplete, AutocompleteItem};

#[test]
fn autocomplete_item_defaults_label_to_value() {
    let item = AutocompleteItem::new("Rust");
    assert_eq!(item.value.as_ref(), "Rust");
    assert_eq!(item.label.as_ref(), "Rust");
}

#[test]
fn autocomplete_item_supports_custom_label() {
    let item = AutocompleteItem::labeled("rs", "Rust");
    assert_eq!(item.value.as_ref(), "rs");
    assert_eq!(item.label.as_ref(), "Rust");
}

#[test]
fn autocomplete_filters_by_value_or_label_case_insensitive() {
    let items = vec![
        AutocompleteItem::labeled("rs", "Rust"),
        AutocompleteItem::labeled("gpui", "GPUI"),
        AutocompleteItem::labeled("element", "Element Plus"),
    ];

    let matches = Autocomplete::matching_items_for(&items, "RU", 8);
    assert_eq!(matches, vec![items[0].clone()]);

    let matches = Autocomplete::matching_items_for(&items, "ui", 8);
    assert_eq!(matches, vec![items[1].clone()]);
}

#[test]
fn autocomplete_limits_filtered_results() {
    let items = vec![
        AutocompleteItem::new("one"),
        AutocompleteItem::new("two"),
        AutocompleteItem::new("three"),
    ];

    let matches = Autocomplete::matching_items_for(&items, "", 2);
    assert_eq!(matches.len(), 2);
}
