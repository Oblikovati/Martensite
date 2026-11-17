//! A broad direct-child mutation must not depend on the last-found child cache (#489).
use martensite_tkernel::{standard::Handle, tcollection::AsciiString};
use martensite_tklcaf::{
    tdata_std::TDataStdInteger,
    tdf::{TdfData, TdfLabel, TdfTransaction},
};
use std::time::Instant;
const DIRECT_LABELS: i32 = 100_000;

fn command(data: &Handle<TdfData>) -> TdfTransaction {
    let mut transaction = TdfTransaction::with_data(data, &AsciiString::new("wide direct"));
    transaction.open().unwrap();
    transaction
}
fn direct_tree(data: &Handle<TdfData>) -> TdfLabel {
    let root = data.root();
    let mut middle = TdfLabel::default();
    for tag in 1..=DIRECT_LABELS {
        let label = root.find_child(tag, true).unwrap();
        TDataStdInteger::set(&label, 1).unwrap();
        if tag == DIRECT_LABELS / 2 {
            middle = label;
        }
    }
    middle
}
fn changed_attribute(data: &Handle<TdfData>, middle: &TdfLabel, value: i32) -> TdfTransaction {
    let transaction = command(data);
    TDataStdInteger::set(middle, value).unwrap();
    // The selected label's identity must survive an unrelated later lookup.
    data.root().find_child(DIRECT_LABELS, false).unwrap();
    transaction
}
fn integer_value(label: &TdfLabel) -> i32 {
    let mut attribute = Handle::default();
    assert!(
        label
            .find_attribute(&TDataStdInteger::get_id(), &mut attribute)
            .unwrap()
    );
    attribute.cast::<TDataStdInteger>().get().unwrap().get()
}
fn commit_direct(data: &Handle<TdfData>, middle: &TdfLabel) -> u128 {
    let transaction = changed_attribute(data, middle, 2);
    let elapsed = timed_commit(transaction);
    assert_eq!(integer_value(middle), 2);
    elapsed
}
fn timed_commit(mut transaction: TdfTransaction) -> u128 {
    let started = Instant::now();
    let delta = transaction.commit(true).unwrap();
    let elapsed = started.elapsed().as_micros();
    assert_eq!(delta.attribute_deltas().len(), 1);
    elapsed
}
fn abort_direct(data: &Handle<TdfData>, middle: &TdfLabel) -> u128 {
    let mut transaction = changed_attribute(data, middle, 3);
    let started = Instant::now();
    transaction.abort().unwrap();
    let elapsed = started.elapsed().as_micros();
    assert_eq!(integer_value(middle), 2);
    elapsed
}
#[test]
fn a_sparse_direct_child_keeps_commit_and_abort_history_with_a_distant_lookup_cache() {
    let data = TdfData::new();
    measure_direct(&data, "wide_direct");
}
fn measure_direct(data: &Handle<TdfData>, kind: &str) {
    let mut creation = command(data);
    let started = Instant::now();
    let middle = direct_tree(data);
    let create_us = started.elapsed().as_micros();
    let started = Instant::now();
    creation.commit(false).unwrap();
    let create_commit_us = started.elapsed().as_micros();
    let commit_us = commit_direct(data, &middle);
    let abort_us = abort_direct(data, &middle);
    println!(
        "{{\"kind\":\"{kind}\",\"labels\":{DIRECT_LABELS},\"creation_us\":{create_us},\"creation_commit_us\":{create_commit_us},\"sparse_commit_us\":{commit_us},\"sparse_abort_us\":{abort_us}}}"
    );
    assert_eq!(data.transaction(), 0);
    assert!(!data.root().may_be_modified());
}

fn deep_tree(data: &Handle<TdfData>) -> TdfLabel {
    let mut leaf = data.root();
    for _ in 0..256 {
        leaf = leaf.find_child(1, true).unwrap();
    }
    TDataStdInteger::set(&leaf, 1).unwrap();
    leaf
}
fn repeat_deep(data: &Handle<TdfData>, leaf: &TdfLabel) -> u128 {
    let mut commit_us = 0;
    for value in 2..=101 {
        let transaction = command(data);
        TDataStdInteger::set(leaf, value).unwrap();
        commit_us += timed_commit(transaction);
    }
    commit_us
}
#[test]
fn repeated_deep_sparse_commits_preserve_the_value_and_single_attribute_delta() {
    let data = TdfData::new();
    measure_deep(&data, "deep");
}
fn measure_deep(data: &Handle<TdfData>, kind: &str) {
    let mut creation = command(data);
    let leaf = deep_tree(data);
    creation.commit(false).unwrap();
    let started = Instant::now();
    let commit_us = repeat_deep(data, &leaf);
    let total_us = started.elapsed().as_micros();
    println!(
        "{{\"kind\":\"{kind}\",\"depth\":256,\"repetitions\":100,\"total_us\":{total_us},\"commit_us\":{commit_us}}}"
    );
    assert_eq!(integer_value(&leaf), 101);
    assert!(!data.root().may_be_modified());
}

#[test]
fn indexed_direct_child_commits_preserve_the_existing_native_route() {
    let data = TdfData::new();
    data.set_access_by_entries(true).unwrap();
    measure_direct(&data, "indexed_direct");
}
#[test]
fn indexed_deep_commits_preserve_the_existing_native_route() {
    let data = TdfData::new();
    data.set_access_by_entries(true).unwrap();
    measure_deep(&data, "indexed_deep");
}
