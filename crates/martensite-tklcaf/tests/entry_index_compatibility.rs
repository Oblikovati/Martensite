//! Entry indexing remains independent of the public automatic-registration flag (#489).
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tklcaf::tdf::{TdfData, TdfLabel};

fn child(data: &Handle<TdfData>, tag: i32) -> TdfLabel {
    // Keep the exact label identity across manual index transitions.
    data.root().find_child(tag, true).unwrap()
}
fn assert_entry(data: &Handle<TdfData>, entry: &str, expected: &TdfLabel) {
    let mut found = TdfLabel::default();
    assert!(data.get_label(&AsciiString::new(entry), &mut found));
    assert!(found == *expected);
}
#[test]
fn manual_registration_works_with_automatic_indexing_disabled() {
    let data = TdfData::new();
    let label = child(&data, 7);
    data.register_label(&label);
    assert!(!data.is_access_by_entries());
    assert_entry(&data, "0:7", &label);
}
#[test]
fn enable_disable_and_rebuild_preserve_existing_and_new_bindings() {
    let data = TdfData::new();
    let first = child(&data, 7);
    data.set_access_by_entries(true).unwrap();
    let second = child(&data, 9);
    assert_entry(&data, "0:7", &first);
    assert_entry(&data, "0:9", &second);
    data.set_access_by_entries(false).unwrap();
    let mut unchanged = first.clone();
    assert!(!data.get_label(&AsciiString::new("0:7"), &mut unchanged));
    assert!(unchanged == first);
    data.set_access_by_entries(true).unwrap();
    assert_entry(&data, "0:7", &first);
    assert_entry(&data, "0:9", &second);
}
#[test]
fn manual_registration_after_disable_does_not_register_unrequested_children() {
    let data = TdfData::new();
    let label = child(&data, 2);
    data.set_access_by_entries(true).unwrap();
    data.set_access_by_entries(false).unwrap();
    data.register_label(&label);
    assert_entry(&data, "0:2", &label);
    child(&data, 3);
    let mut unchanged = label.clone();
    assert!(!data.get_label(&AsciiString::new("0:3"), &mut unchanged));
    assert!(unchanged == label);
    assert!(!data.is_access_by_entries());
}
