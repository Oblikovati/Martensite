//! Miri-verified Send/Sync concurrency contracts for audited unsafe impls (#63).
use martensite_tkernel::ncollection::{
    AllocatedBytes, BaseAllocator, BoundingVolume, List, PackedMap, Sequence, SparseArray, UBTree,
};
use std::sync::Arc;
use std::thread;

fn assert_send<T: Send>() {}
fn assert_sync<T: Sync>() {}

#[derive(Clone, Debug, Default)]
struct IntervalBound(f64, f64);

impl BoundingVolume for IntervalBound {
    fn add(&mut self, other: &Self) {
        self.0 = self.0.min(other.0);
        self.1 = self.1.max(other.1);
    }
    fn is_out(&self, other: &Self) -> bool {
        self.1 < other.0 || other.1 < self.0
    }
    fn square_extent(&self) -> f64 {
        let span = self.1 - self.0;
        span * span
    }
}

#[test]
fn audited_collection_and_allocator_types_implement_expected_auto_traits() {
    assert_send::<List<i32>>();
    assert_sync::<List<i32>>();
    assert_send::<Sequence<i32>>();
    assert_sync::<Sequence<i32>>();
    assert_send::<PackedMap<u32>>();
    assert_sync::<PackedMap<u32>>();
    assert_send::<SparseArray<i32>>();
    assert_sync::<SparseArray<i32>>();
    assert_send::<UBTree<i32, IntervalBound>>();
    assert_sync::<UBTree<i32, IntervalBound>>();
    assert_send::<BaseAllocator>();
    assert_sync::<BaseAllocator>();
    assert_send::<AllocatedBytes>();
    assert_sync::<AllocatedBytes>();
}

#[test]
fn list_send_sync_and_cross_thread_drop_are_sound() {
    let mut list = List::default();
    for value in 0..16 {
        list.append(value);
    }
    let shared = Arc::new(list);
    let readers: Vec<_> = (0..3)
        .map(|_| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                let sum: i32 = shared.iter().copied().sum();
                assert_eq!(sum, 120);
            })
        })
        .collect();
    for handle in readers {
        handle.join().unwrap();
    }
    let owned = Arc::try_unwrap(shared).ok().expect("sole owner");
    thread::spawn(move || drop(owned)).join().unwrap();
}

#[test]
fn sequence_send_sync_concurrent_indexed_reads_and_drop_are_sound() {
    let mut seq = Sequence::default();
    for value in 1..=16 {
        seq.append(value);
    }
    let shared = Arc::new(seq);
    let readers: Vec<_> = (0..3)
        .map(|offset| {
            let shared = Arc::clone(&shared);
            thread::spawn(move || {
                for step in 0..16 {
                    let index = (step + offset * 5) % 16 + 1;
                    assert_eq!(*shared.value(index).unwrap(), index as i32);
                }
                assert_eq!(shared.base_access().size(), 16);
            })
        })
        .collect();
    for handle in readers {
        handle.join().unwrap();
    }
    let owned = Arc::try_unwrap(shared).ok().expect("sole owner");
    thread::spawn(move || drop(owned)).join().unwrap();
}

#[test]
fn packed_map_and_sparse_array_send_sync_and_drop_are_sound() {
    let mut map = PackedMap::<u32>::new(16);
    for key in [1, 7, 33, 64, 127] {
        map.add(key);
    }
    let shared_map = Arc::new(map);
    let mut sparse = SparseArray::new(4);
    for index in [0, 3, 7, 11] {
        sparse.set_value(index, index as i32 * 10);
    }
    let shared_sparse = Arc::new(sparse);
    let handles: Vec<_> = (0..3)
        .map(|_| {
            let m = Arc::clone(&shared_map);
            let s = Arc::clone(&shared_sparse);
            thread::spawn(move || {
                assert_eq!(m.extent(), 5);
                assert!(m.contains(33));
                assert_eq!(m.iter().count(), 5);
                assert_eq!(*s.value(7).unwrap(), 70);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let owned_map = Arc::try_unwrap(shared_map).ok().expect("sole owner");
    let owned_sparse = Arc::try_unwrap(shared_sparse).ok().expect("sole owner");
    thread::spawn(move || drop((owned_map, owned_sparse)))
        .join()
        .unwrap();
}

#[test]
fn ubtree_and_allocators_send_sync_are_sound() {
    let mut tree = UBTree::<i32, IntervalBound>::default();
    for i in 0..8 {
        tree.add(i, IntervalBound(i as f64, (i + 1) as f64));
    }
    let shared_tree = Arc::new(tree);
    let acc = BaseAllocator::accumulating(256).unwrap();
    let inc = BaseAllocator::incremental(256);
    let handles: Vec<_> = (0..2)
        .map(|_| {
            let t = Arc::clone(&shared_tree);
            let a = acc.clone();
            let b = inc.clone();
            thread::spawn(move || {
                assert!(!t.is_empty());
                let bytes_a = a.allocate(32).unwrap();
                let bytes_b = b.allocate(32).unwrap();
                assert_eq!(bytes_a.as_slice().len(), 32);
                assert_eq!(bytes_b.as_slice().len(), 32);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let owned_tree = Arc::try_unwrap(shared_tree).ok().expect("sole owner");
    thread::spawn(move || drop(owned_tree)).join().unwrap();
}
