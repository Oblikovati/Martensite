use super::{policies, prepared_vertices};
use martensite_tkbo::bop_ds::{BOPDSSubIterator, DataStructure};
use martensite_tkernel::ncollection::{BaseAllocator, List};

#[test]
fn constructor_accepts_null_without_changing_strong_resource_interface() {
    let default = BOPDSSubIterator::new();
    assert_eq!(default.allocator(), Some(&BaseAllocator::common()));
    for policy in policies() {
        let iterator = BOPDSSubIterator::with_nullable_allocator(policy.clone());
        assert_eq!(iterator.allocator(), policy.as_ref());
        assert_eq!(iterator.expected_length(), 0);
        if let Some(resource) = policy {
            let strong = BOPDSSubIterator::with_allocator(resource.clone());
            assert_eq!(strong.allocator(), Some(&resource));
        }
    }
}

fn subset_indices() -> (List<i32>, List<i32>) {
    let mut first = List::default();
    let mut second = List::default();
    for index in [0, 0, 1] {
        first.append(index);
    }
    for index in [1, 0, 1] {
        second.append(index);
    }
    (first, second)
}

fn bound_iterator<'a>(
    policy: Option<BaseAllocator>,
    ds: &'a DataStructure,
    first: &'a List<i32>,
    second: &'a List<i32>,
) -> BOPDSSubIterator<'a> {
    let mut iterator = BOPDSSubIterator::with_nullable_allocator(policy);
    iterator.set_ds(Some(ds));
    iterator.set_subset1(first);
    iterator.set_subset2(second);
    assert!(std::ptr::eq(iterator.ds().unwrap(), ds));
    assert!(std::ptr::eq(iterator.subset1().unwrap(), first));
    assert!(std::ptr::eq(iterator.subset2().unwrap(), second));
    iterator
}

fn assert_subset_pair(iterator: &mut BOPDSSubIterator<'_>) {
    iterator.prepare().unwrap();
    iterator.initialize();
    assert_eq!(iterator.expected_length(), 1);
    assert!(iterator.more());
    assert_eq!(iterator.value().unwrap(), (0, 1));
    iterator.next();
    assert!(!iterator.more());
}

fn assert_second_intersection(iterator: &mut BOPDSSubIterator<'_>) {
    iterator.intersect().unwrap();
    iterator.initialize();
    assert_eq!(iterator.expected_length(), 2);
    for _ in 0..2 {
        assert!(iterator.more());
        assert_eq!(iterator.value().unwrap(), (0, 1));
        iterator.next();
    }
    assert!(!iterator.more());
}

#[test]
fn repeated_preparation_keeps_policy_and_fences_only_each_intersection() {
    let ds = prepared_vertices();
    let (first, second) = subset_indices();
    for policy in policies() {
        let mut iterator = bound_iterator(policy.clone(), &ds, &first, &second);
        assert_subset_pair(&mut iterator);
        assert_second_intersection(&mut iterator);
        assert_subset_pair(&mut iterator);
        assert_eq!(iterator.allocator(), policy.as_ref());
    }
}

fn assert_empty_preparation(iterator: &mut BOPDSSubIterator<'_>) {
    iterator.prepare().unwrap();
    iterator.initialize();
    assert_eq!(iterator.expected_length(), 0);
    assert!(!iterator.more());
}

fn assert_unbound_preparation(iterator: &mut BOPDSSubIterator<'_>, policy: &Option<BaseAllocator>) {
    iterator.set_ds(None);
    assert_empty_preparation(iterator);
    assert!(iterator.ds().is_err());
    assert_eq!(iterator.allocator(), policy.as_ref());
}

#[test]
fn empty_unbound_and_rebound_preparation_keep_resource() {
    let ds = prepared_vertices();
    let (first, second) = subset_indices();
    let empty = List::default();
    for policy in policies() {
        let mut iterator = bound_iterator(policy.clone(), &ds, &first, &second);
        assert_subset_pair(&mut iterator);
        iterator.set_subset1(&empty);
        assert_empty_preparation(&mut iterator);
        assert_unbound_preparation(&mut iterator, &policy);
        iterator.set_ds(Some(&ds));
        iterator.set_subset1(&first);
        assert_subset_pair(&mut iterator);
        assert_eq!(iterator.allocator(), policy.as_ref());
    }
}

#[test]
fn invalid_index_reports_error_and_recovers_without_policy_change() {
    let ds = prepared_vertices();
    let (first, second) = subset_indices();
    let mut invalid = List::default();
    invalid.append(-1);
    for policy in policies() {
        let mut iterator = bound_iterator(policy.clone(), &ds, &invalid, &second);
        assert!(iterator.prepare().is_err());
        assert_eq!(iterator.allocator(), policy.as_ref());
        iterator.set_subset1(&first);
        assert_subset_pair(&mut iterator);
        assert_eq!(iterator.allocator(), policy.as_ref());
    }
}
