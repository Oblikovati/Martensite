use super::super::TdfData;
use super::*;
use martensite_tkernel::standard::Handle;

fn children() -> (Handle<TdfData>, Vec<Arc<TdfLabelNode>>) {
    let data = TdfData::new();
    let nodes = (1..=3)
        .map(|tag| {
            data.root()
                .find_child(tag, true)
                .unwrap()
                .node()
                .unwrap()
                .clone()
        })
        .collect();
    (data, nodes)
}
fn assert_first_is(
    route: impl CommitRoute,
    parent: &Arc<TdfLabelNode>,
    expected: &Arc<TdfLabelNode>,
) {
    let first = route.first_child(parent).unwrap();
    assert!(Arc::ptr_eq(&first, expected));
}
#[test]
fn a_single_exact_node_selects_only_its_ancestor_path() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    let path = walk.capture_path(data.core.root_node()).unwrap();
    assert_first_is(walk.cursor(&path), data.core.root_node(), &nodes[2]);
    assert!(
        walk.cursor(&path)
            .descend()
            .first_child(&nodes[2])
            .is_none()
    );
    walk.complete();
}
#[test]
fn a_second_identity_falls_back_to_the_original_first_child() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.mark(&nodes[2]);
    hint.mark(&nodes[1]);
    let mut walk = hint.enter();
    assert!(walk.capture_path(data.core.root_node()).is_none());
    assert!(Arc::ptr_eq(
        &NativeCommitRoute
            .first_child(data.core.root_node())
            .unwrap(),
        &nodes[0]
    ));
    walk.complete();
}
#[test]
fn repeated_marks_of_the_same_node_keep_the_single_hint() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.mark(&nodes[2]);
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    assert!(walk.capture_path(data.core.root_node()).is_some());
    walk.complete();
}
#[test]
fn manual_flags_are_a_permanent_conservative_fallback() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.manual_flags();
    hint.open_transaction(data.core.root_node());
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    assert!(walk.capture_path(data.core.root_node()).is_none());
    walk.complete();
}
#[test]
fn nested_walk_invalidation_cannot_rearm_the_outer_snapshot() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.mark(&nodes[2]);
    let mut outer = hint.enter();
    let path = outer.capture_path(data.core.root_node()).unwrap();
    let mut inner = hint.enter();
    assert!(inner.capture_path(data.core.root_node()).is_none());
    inner.complete();
    drop(inner);
    hint.open_transaction(data.core.root_node());
    hint.mark(&nodes[2]);
    assert_first_is(outer.cursor(&path), data.core.root_node(), &nodes[0]);
    outer.complete();
}
#[test]
fn a_failed_walk_permanently_refuses_future_shortcuts() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.mark(&nodes[2]);
    drop(hint.enter());
    hint.open_transaction(data.core.root_node());
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    assert!(walk.capture_path(data.core.root_node()).is_none());
    walk.complete();
}
#[test]
fn a_point_outside_the_captured_root_is_not_a_path() {
    let (data, _) = children();
    let hint = SparseCommitHint::default();
    let foreign = Arc::new(TdfLabelNode::default());
    hint.mark(&foreign);
    let mut walk = hint.enter();
    assert!(walk.capture_path(data.core.root_node()).is_none());
    walk.complete();
}
#[test]
fn revision_exhaustion_refuses_without_wrapping() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.state.write().revision = u64::MAX - 1;
    hint.open_transaction(data.core.root_node());
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    assert!(walk.capture_path(data.core.root_node()).is_none());
    walk.complete();
}
#[test]
fn the_maximum_eligible_path_reaches_its_leaf_without_searching_prefixes() {
    let data = TdfData::new();
    let mut label = data.root();
    for _ in 0..16 {
        label = label.find_child(1, true).unwrap();
    }
    let hint = SparseCommitHint::default();
    hint.mark(label.node().unwrap());
    let mut walk = hint.enter();
    let path = walk.capture_path(data.core.root_node()).unwrap();
    let (mut node, mut cursor) = (data.core.root_node().clone(), walk.cursor(&path));
    for _ in 0..16 {
        node = cursor.first_child(&node).unwrap();
        cursor = cursor.descend();
    }
    assert!(Arc::ptr_eq(&node, label.node().unwrap()));
    assert!(cursor.first_child(&node).is_none());
    walk.complete();
}
#[test]
fn document_owned_tracking_is_send_and_sync() {
    let hint = SparseCommitHint::default();
    let _: &(dyn Send + Sync) = &hint;
    assert_eq!(hint.state.read().active, 0);
}
fn release_after_blocked_writer<L>(
    hint: &SparseCommitHint,
    lease: L,
    writer: impl FnOnce() + Send,
    before_release: impl FnOnce(),
) {
    std::thread::scope(|scope| {
        let (started, ready) = std::sync::mpsc::channel();
        let (done, finished) = std::sync::mpsc::channel();
        scope.spawn(move || signal_writer(started, done, writer));
        ready.recv().unwrap();
        assert!(hint.state.try_write().is_none());
        assert!(finished.try_recv().is_err());
        before_release();
        drop(lease);
        finished.recv().unwrap();
    });
}
fn signal_writer(
    started: std::sync::mpsc::Sender<()>,
    done: std::sync::mpsc::Sender<()>,
    writer: impl FnOnce(),
) {
    started.send(()).unwrap();
    writer();
    done.send(()).unwrap();
}
#[test]
fn a_many_mark_lease_blocks_reset_until_ancestor_publication_finishes() {
    let (data, nodes) = children();
    let hint = &data.core.commit_hint;
    hint.mark(&nodes[0]);
    hint.mark(&nodes[1]);
    let lease = hint.mark(&nodes[2]);
    release_after_blocked_writer(
        hint,
        lease,
        || hint.open_transaction(data.core.root_node()),
        || nodes[2].transaction_attributes(None, true),
    );
    assert!(matches!(
        hint.state.read().collection,
        DirtyCollection::Many
    ));
}
#[test]
fn a_selection_read_lease_blocks_a_new_distinct_mark() {
    let (data, nodes) = children();
    let hint = &data.core.commit_hint;
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    let selection = hint.current(walk.revision).unwrap();
    release_after_blocked_writer(hint, selection, || drop(hint.mark(&nodes[1])), || {});
    assert!(hint.current(walk.revision).is_none());
    walk.complete();
}

#[test]
fn an_active_cursor_falls_back_after_the_concurrent_mark_lease_drops() {
    let (data, nodes) = children();
    let hint = &data.core.commit_hint;
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    let path = walk.capture_path(data.core.root_node()).unwrap();
    std::thread::scope(|scope| {
        scope
            .spawn(|| nodes[1].transaction_attributes(Some(&data.core), true))
            .join()
            .unwrap();
    });
    assert_first_is(walk.cursor(&path), data.core.root_node(), &nodes[0]);
    assert!(nodes[1].may_be_modified());
    walk.complete();
}

#[test]
fn revision_exhaustion_never_rearms_an_old_cursor() {
    let (data, nodes) = children();
    let hint = SparseCommitHint::default();
    hint.mark(&nodes[2]);
    let mut walk = hint.enter();
    let revision = walk.revision;
    hint.state.write().revision = u64::MAX - 1;
    for _ in 0..4 {
        hint.open_transaction(data.core.root_node());
    }
    assert!(hint.current(revision).is_none());
    assert_eq!(hint.state.read().revision, u64::MAX);
    assert!(hint.state.read().forbidden);
    walk.complete();
}

#[test]
fn deeper_paths_decline_the_shortcut_without_losing_the_native_walk() {
    let data = TdfData::new();
    let mut leaf = data.root();
    for _ in 0..17 {
        leaf = leaf.find_child(1, true).unwrap();
    }
    let hint = SparseCommitHint::default();
    hint.mark(leaf.node().unwrap());
    let mut walk = hint.enter();
    assert!(!walk.has_sparse_hint());
    assert!(walk.capture_path(data.core.root_node()).is_none());
    assert!(
        NativeCommitRoute
            .first_child(data.core.root_node())
            .is_some()
    );
    walk.complete();
}
