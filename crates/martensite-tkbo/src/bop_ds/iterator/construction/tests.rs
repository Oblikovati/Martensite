use super::*;

fn assert_outer_and_inner(iterator: &mut BOPDSIterator, outer: Option<&BaseAllocator>) {
    assert_eq!(iterator.lists.allocator(), outer);
    assert_eq!(iterator.extended_lists.allocator(), outer);
    assert_eq!(
        (iterator.lists.len(), iterator.extended_lists.len()),
        (10, 4)
    );
    assert_eq!(iterator.lists.storage_extension().block_count(), 1);
    assert_eq!(iterator.extended_lists.storage_extension().block_count(), 1);
    assert_eq!(iterator.lists.storage_extension().available_size(), 16);
    assert_eq!(
        iterator.extended_lists.storage_extension().available_size(),
        4
    );
    for group in iterator.lists.iter().chain(iterator.extended_lists.iter()) {
        assert!(group.allocator().is_none() && group.is_empty());
    }
}

#[test]
fn default_outer_storage_is_null_despite_common_iterator_resource() {
    let mut iterator = BOPDSIterator::default();
    assert_eq!(iterator.allocator(), Some(&BaseAllocator::common()));
    assert_outer_and_inner(&mut iterator, None);
}

#[test]
fn explicit_outer_storage_keeps_policy_while_all_children_start_null() {
    for policy in [
        None,
        Some(BaseAllocator::common()),
        Some(BaseAllocator::fresh_heap()),
    ] {
        let mut iterator = BOPDSIterator::with_allocator(policy.clone());
        assert_outer_and_inner(&mut iterator, policy.as_ref());
        let mut own_check = super::super::BOPDSIteratorSI::with_allocator(policy.clone());
        assert_outer_and_inner(&mut own_check, policy.as_ref());
    }
}
