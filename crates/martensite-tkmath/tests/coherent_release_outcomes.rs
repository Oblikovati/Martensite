//! CoherentTriPtr::remove_member/RemoveList and CoherentNode::remove_triangle_in/Clear when
//! the release allocator is not the owner. Each case checks one line of the recorded
//! release outcomes.
use martensite_tkernel::ncollection::BaseAllocator;
use martensite_tkmath::gp::Xyz;
use martensite_tkmath::poly::{CoherentNode, CoherentTriPtr};

/// The ring 1 -> 4 -> 7, every member owned by `owner`.
fn ring(owner: &BaseAllocator) -> CoherentTriPtr {
    let mut ring = CoherentTriPtr::new_in(1, owner.clone());
    let head = ring.head_member().unwrap();
    ring.append_member(head, 7, owner.clone()).unwrap();
    ring.append_member(head, 4, owner.clone()).unwrap();
    assert_eq!(ring.iter().collect::<Vec<_>>(), [1, 4, 7]);
    ring
}

/// Removes member 4 through `freer`; the error type when its `Free` raises.
fn remove_middle(ring: &mut CoherentTriPtr, freer: &BaseAllocator) -> Option<&'static str> {
    let middle = ring.next_member(ring.head_member().unwrap()).unwrap();
    ring.remove_member(middle, freer)
        .err()
        .map(|failure| failure.exception_type())
}

fn foreign_accumulating() -> BaseAllocator {
    BaseAllocator::accumulating(1024).unwrap()
}

#[test]
fn remove_unlinks_then_applies_each_defined_free_outcome() {
    let common = BaseAllocator::common();
    let own_acc = foreign_accumulating();
    let cases = [
        // remove_common_inc returns / remove_common_acc raises / remove_aligned_common returns
        (common.clone(), BaseAllocator::incremental(4096), None),
        (
            common.clone(),
            foreign_accumulating(),
            Some("Standard_ProgramError"),
        ),
        (BaseAllocator::aligned(16), common.clone(), None),
        // remove_acc_own returns / remove_inc_inc returns
        (own_acc.clone(), own_acc, None),
        (
            BaseAllocator::incremental(4096),
            BaseAllocator::incremental(4096),
            None,
        ),
    ];
    for (owner, freer, raised) in cases {
        let mut ring = ring(&owner);
        assert_eq!(remove_middle(&mut ring, &freer), raised);
        assert_eq!(ring.iter().collect::<Vec<_>>(), [1, 7]);
    }
}

#[test]
fn remove_list_raising_on_the_head_frees_nothing() {
    // remove_list_acc raises; remove_list_acc 1 4
    let common = BaseAllocator::common();
    let mut ring = CoherentTriPtr::new_in(1, common.clone());
    let head = ring.head_member().unwrap();
    ring.append_member(head, 4, common).unwrap();
    let failure = ring.remove_list_in(&foreign_accumulating()).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert_eq!(ring.iter().collect::<Vec<_>>(), [1, 4]);
}

#[test]
fn remove_triangle_frees_the_node_before_the_raising_release() {
    // remove_triangle_acc raises 1
    let mut node = CoherentNode::new(Xyz::new(1.0, 2.0, 3.0));
    node.add_triangle_in(1, BaseAllocator::common()).unwrap();
    let failure = node
        .remove_triangle_in(1, &foreign_accumulating())
        .unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert!(node.is_free_node());
}

#[test]
fn clear_keeps_ring_and_point_when_release_raises_then_clears_with_ignored_free() {
    // clear_acc raises 0 1; clear_inc 0 0 0
    let mut node = CoherentNode::new(Xyz::new(1.0, 2.0, 3.0));
    node.add_triangle_in(1, BaseAllocator::common()).unwrap();
    node.add_triangle_in(4, BaseAllocator::common()).unwrap();
    let failure = node.clear_in(&foreign_accumulating()).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert!(!node.is_free_node());
    assert_eq!(node.x(), 1.0);
    node.clear_in(&BaseAllocator::incremental(4096)).unwrap();
    assert!(node.is_free_node());
    assert_eq!((node.x(), node.y(), node.z()), (0.0, 0.0, 0.0));
}

/// Unchecked, a mid-list raise leaves the freed head linked (dangling); Rust drops the
/// released members and keeps the raising member and the rest as a valid ring.
#[test]
fn remove_list_raising_after_the_head_keeps_the_unreleased_members() {
    let arena = foreign_accumulating();
    let mut ring = CoherentTriPtr::new_in(1, arena.clone());
    let head = ring.head_member().unwrap();
    ring.append_member(head, 4, BaseAllocator::common())
        .unwrap();
    let failure = ring.remove_list_in(&arena).unwrap_err();
    assert_eq!(failure.exception_type(), "Standard_ProgramError");
    assert_eq!(ring.iter().collect::<Vec<_>>(), [4]);
}

#[test]
fn undefined_release_is_refused_before_the_ring_changes() {
    let mut ring = ring(&BaseAllocator::incremental(4096));
    let failure = remove_middle(&mut ring, &BaseAllocator::common());
    assert_eq!(failure, Some("Standard_ProgramError"));
    assert_eq!(ring.iter().collect::<Vec<_>>(), [1, 4, 7]);
    let error = ring
        .remove_list_in(&BaseAllocator::global_heap())
        .unwrap_err();
    assert!(error.message().contains("undefined"), "{}", error.message());
    assert_eq!(ring.len(), 3);
}
