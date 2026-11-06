//! Owned callbacks distinguish tail-first cleanup from merely unlinking a chain.
use super::InterfaceEntityCluster;
use martensite_tkernel::{
    impl_transient,
    standard::{Handle, SharedState, StandardTransient},
};
use std::sync::Arc;

struct ClusterDeletionRecord {
    transient: StandardTransient,
    ordinal: i32,
    observations: Arc<SharedState<Vec<i32>>>,
}
impl_transient!(
    ClusterDeletionRecord,
    "ClusterDeletionRecord",
    StandardTransient,
    transient
);

impl Drop for ClusterDeletionRecord {
    fn drop(&mut self) {
        self.observations.write().push(self.ordinal);
    }
}

fn recorded_entity(
    ordinal: i32,
    observations: &Arc<SharedState<Vec<i32>>>,
) -> Handle<StandardTransient> {
    Handle::new(ClusterDeletionRecord {
        transient: StandardTransient::default(),
        ordinal,
        observations: observations.clone(),
    })
    .cast()
}

#[test]
fn destructor_clears_retained_tail_entities_before_head_entities() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let head = InterfaceEntityCluster::new();
    for ordinal in 1..=9 {
        head.append(&recorded_entity(ordinal, &observations))
            .unwrap();
    }
    let middle = head.next();
    let tail = middle.next();
    drop(head);
    assert_eq!(*observations.read(), vec![9, 5, 6, 7, 8, 1, 2, 3, 4]);
    assert_eq!((middle.nb_entities(), tail.nb_entities()), (0, 0));
    assert!(!middle.has_next());
    drop(middle);
    drop(tail);
    assert_eq!(observations.read().len(), 9);
}

#[test]
fn destructor_releases_only_its_entity_ownership() {
    let observations = Arc::new(SharedState::new(Vec::new()));
    let retained = recorded_entity(2, &observations);
    let cluster = InterfaceEntityCluster::with_entity(&recorded_entity(1, &observations));
    cluster.append(&retained).unwrap();
    drop(cluster);
    assert_eq!(*observations.read(), vec![1]);
    drop(retained);
    assert_eq!(*observations.read(), vec![1, 2]);
}

#[test]
fn destructor_handles_long_chain_without_recursive_link_destruction() {
    let mut head = Handle::default();
    for _ in 0..20_000 {
        head = Handle::new(InterfaceEntityCluster::with_next(&head));
    }
    drop(head);
}
