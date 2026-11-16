//! Ordered callback histories for sparse commits and their conservative fallback (#489).
#[path = "sparse_transaction_history/probe.rs"]
mod probe;
use martensite_tklcaf::tdf::TdfAttributeState;
use probe::{CommitAction, HistoryFixture};

#[test]
fn a_callback_commits_a_newly_dirty_later_sibling_in_order() {
    let fixture = HistoryFixture::new(&[&[1], &[2], &[3]]);
    assert_later_sibling_history(fixture);
}
fn assert_later_sibling_history(fixture: HistoryFixture) {
    let mut transaction = fixture.open();
    fixture.arm(1, CommitAction::Touch(fixture.label(2)));
    fixture.touch(1);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:2", "0:3"]);
    HistoryFixture::assert_delta(&delta, &["0:2", "0:3"]);
    assert_eq!(fixture.probe(2).transaction(), 0);
}

#[test]
fn a_callback_does_not_revisit_a_newly_dirty_earlier_sibling() {
    let fixture = HistoryFixture::new(&[&[1], &[2], &[3]]);
    let mut transaction = fixture.open();
    fixture.arm(1, CommitAction::Touch(fixture.label(0)));
    fixture.touch(1);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:2"]);
    HistoryFixture::assert_delta(&delta, &["0:2"]);
    assert_eq!(fixture.probe(0).transaction(), 1);
    assert!(fixture.label(0).may_be_modified());
}

#[test]
fn public_flag_clearing_in_a_callback_skips_a_later_dirty_sibling() {
    let fixture = HistoryFixture::new(&[&[1], &[2], &[3]]);
    let mut transaction = fixture.open();
    fixture.arm(1, CommitAction::ClearMay(fixture.label(2)));
    fixture.touch(1);
    fixture.touch(2);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:2"]);
    HistoryFixture::assert_delta(&delta, &["0:2"]);
    assert_eq!(fixture.probe(2).transaction(), 1);
}

#[test]
fn ancestor_callbacks_visit_newly_dirty_descendants_after_their_parent() {
    let fixture = HistoryFixture::new(&[&[2], &[2, 4]]);
    let mut transaction = fixture.open();
    fixture.arm(0, CommitAction::Touch(fixture.label(1)));
    fixture.touch(0);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:2", "0:2:4"]);
    HistoryFixture::assert_delta(&delta, &["0:2", "0:2:4"]);
}

#[test]
fn descendant_callbacks_do_not_revisit_their_already_processed_ancestor() {
    let fixture = HistoryFixture::new(&[&[2], &[2, 4]]);
    let mut transaction = fixture.open();
    fixture.arm(1, CommitAction::Touch(fixture.label(0)));
    fixture.touch(1);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:2:4"]);
    HistoryFixture::assert_delta(&delta, &["0:2:4"]);
    assert_eq!(fixture.probe(0).transaction(), 1);
    assert!(fixture.label(0).may_be_modified());
}

#[test]
fn manual_flags_keep_an_unmodified_probe_visible_to_commit_callbacks() {
    let fixture = HistoryFixture::new(&[&[1], &[2]]);
    let mut transaction = fixture.open();
    fixture.touch(0);
    let second = fixture.label(1);
    second.node().unwrap().set_attributes_modified(true);
    second.node().unwrap().set_may_be_modified(false);
    second.node().unwrap().set_may_be_modified(true);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:1", "0:2"]);
    HistoryFixture::assert_delta(&delta, &["0:1"]);
}

#[test]
fn nested_commit_keeps_outer_backups_until_the_outer_abort() {
    let fixture = HistoryFixture::new(&[&[1], &[2]]);
    assert_nested_abort_history(fixture);
}
fn assert_nested_abort_history(fixture: HistoryFixture) {
    let mut outer = fixture.open();
    fixture.touch(0);
    let mut inner = fixture.open();
    fixture.touch(1);
    inner.commit(true).unwrap();
    fixture.assert_history(&["0:1", "0:2"]);
    assert_eq!(fixture.probe(0).transaction(), 1);
    assert_eq!(fixture.probe(1).transaction(), 1);
    fixture.clear_history();
    outer.abort().unwrap();
    assert_eq!(fixture.data.transaction(), 0);
    assert_eq!(fixture.probe(0).transaction(), 0);
    assert_eq!(fixture.probe(1).transaction(), 0);
}

#[test]
fn a_failing_callback_preserves_the_unprocessed_transaction_and_failure() {
    let fixture = HistoryFixture::new(&[&[1], &[2]]);
    let mut transaction = fixture.open();
    fixture.arm(0, CommitAction::TouchThenFail(fixture.label(1)));
    fixture.touch(0);
    let failure = transaction.commit(true).unwrap_err();
    assert_eq!(
        failure.to_string(),
        "Standard_Failure: sparse callback refusal"
    );
    fixture.assert_history(&["0:1"]);
    assert_eq!(fixture.data.transaction(), 1);
    assert_eq!(fixture.probe(0).transaction(), 1);
    assert_eq!(fixture.probe(1).transaction(), 1);
}

#[test]
fn preexisting_ancestor_and_descendant_modifications_keep_depth_first_order() {
    let fixture = HistoryFixture::new(&[&[2], &[2, 4]]);
    let mut transaction = fixture.open();
    fixture.touch(1);
    fixture.touch(0);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:2", "0:2:4"]);
    HistoryFixture::assert_delta(&delta, &["0:2", "0:2:4"]);
    assert_eq!(fixture.probe(0).transaction(), 0);
    assert_eq!(fixture.probe(1).transaction(), 0);
}

#[test]
fn a_callback_nested_commit_preserves_outer_resume_order_and_delta() {
    let fixture = HistoryFixture::new(&[&[1], &[2]]);
    let mut transaction = fixture.open();
    fixture.arm(0, CommitAction::NestedCommit(fixture.label(1)));
    fixture.touch(0);
    let delta = transaction.commit(true).unwrap();
    fixture.assert_history(&["0:1", "0:2", "0:2"]);
    HistoryFixture::assert_delta(&delta, &["0:1", "0:2"]);
    assert_eq!(fixture.data.transaction(), 0);
    assert_eq!(fixture.probe(0).transaction(), 0);
    assert_eq!(fixture.probe(1).transaction(), 0);
}

#[test]
fn a_sparse_modification_commits_through_a_256_label_ancestor_chain() {
    let path = vec![1; 256];
    let fixture = HistoryFixture::new(&[&path]);
    let mut transaction = fixture.open();
    fixture.touch(0);
    let delta = transaction.commit(true).unwrap();
    let expected = format!("0{}", ":1".repeat(256));
    fixture.assert_history(&[&expected]);
    HistoryFixture::assert_delta(&delta, &[&expected]);
    assert_eq!(fixture.data.transaction(), 0);
    assert_eq!(fixture.probe(0).transaction(), 0);
    assert!(!fixture.data.root().may_be_modified());
}

fn indexed_fixture(paths: &[&[i32]]) -> HistoryFixture {
    let fixture = HistoryFixture::new(paths);
    fixture.data.set_access_by_entries(true).unwrap();
    fixture
}
#[test]
fn indexed_documents_preserve_later_sibling_callback_history() {
    let fixture = indexed_fixture(&[&[1], &[2], &[3]]);
    assert_later_sibling_history(fixture);
}
#[test]
fn indexed_documents_preserve_nested_commit_and_abort_history() {
    let fixture = indexed_fixture(&[&[1], &[2]]);
    assert_nested_abort_history(fixture);
}
