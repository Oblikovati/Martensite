//! A policy can make a map walk its entries in the order they were added, for keys hashed by an
//! address, which differs between runs of a program. Other maps keep their bucket order.
use martensite_tkernel::ncollection::{DataMap, HashPolicy, Map, PackedMap};

/// A hash that scatters like an address does, for keys that a test can name.
#[derive(Clone, Copy, Debug, Default)]
struct AddressLike;

impl HashPolicy<i32> for AddressLike {
    fn hash(&self, key: &i32) -> usize {
        (*key as usize).wrapping_mul(0x9E37_79B9_7F4A_7C15)
    }
    fn equal(&self, first: &i32, second: &i32) -> bool {
        first == second
    }
    fn walks_in_order_added(&self) -> bool {
        true
    }
}

const ADDED: [i32; 8] = [70, 3, 500, 41, 8, 999, 12, 256];

fn address_map() -> Map<i32, AddressLike> {
    let mut map = Map::with_hasher(AddressLike, 1);
    for key in ADDED {
        map.add(key);
    }
    map
}

fn address_data_map() -> DataMap<i32, i32, AddressLike> {
    let mut map = DataMap::with_hasher(AddressLike, 1);
    for key in ADDED {
        map.bind(key, key * 2);
    }
    map
}

#[test]
fn a_set_keyed_by_address_walks_in_the_order_of_adding() {
    let map = address_map();
    assert_eq!(map.iter().copied().collect::<Vec<_>>(), ADDED);
    let mut walked = Vec::new();
    let mut cursor = map.cursor();
    while cursor.more() {
        walked.push(*cursor.value().unwrap());
        cursor.next();
    }
    assert_eq!(walked, ADDED);
}

#[test]
fn a_map_keyed_by_address_walks_in_the_order_of_binding() {
    let mut map = address_data_map();
    let keys: Vec<i32> = map.items().map(|(key, _)| *key).collect();
    assert_eq!(keys, ADDED);
    let mut walked = Vec::new();
    let mut cursor = map.cursor();
    while cursor.more() {
        walked.push(*cursor.key().unwrap());
        cursor.next();
    }
    assert_eq!(walked, ADDED);
    let mut changing = map.cursor_mut();
    let mut changed = Vec::new();
    while changing.more() {
        changed.push(*changing.key().unwrap());
        changing.next();
    }
    assert_eq!(changed, ADDED);
    let lent: Vec<i32> = map.items_mut().map(|(key, _)| *key).collect();
    assert_eq!(lent, ADDED);
}

/// A removal moves the last entry into the freed place, in every run the same.
#[test]
fn a_removal_moves_the_last_entry_into_its_place() {
    let mut map = address_map();
    assert!(map.remove(&500));
    assert_eq!(
        map.iter().copied().collect::<Vec<_>>(),
        [70, 3, 256, 41, 8, 999, 12]
    );
}

#[test]
fn growing_and_clearing_keep_the_order_of_adding() {
    let mut map = address_map();
    map.resize(5000);
    assert_eq!(map.iter().copied().collect::<Vec<_>>(), ADDED);
    map.clear(false);
    for key in [9, 1, 5] {
        map.add(key);
    }
    assert_eq!(map.iter().copied().collect::<Vec<_>>(), [9, 1, 5]);
    let mut copy = Map::with_hasher(AddressLike, 1);
    copy.assign(&map);
    assert_eq!(copy.iter().copied().collect::<Vec<_>>(), [9, 1, 5]);
}

#[test]
fn keys_by_default_keep_their_bucket_order() {
    let mut by_value = Map::<i32>::default();
    for key in [30, 10, 20] {
        by_value.add(key);
    }
    assert_eq!(by_value.iter().copied().collect::<Vec<_>>(), [10, 20, 30]);
    let values = [5_i32, 6];
    let mut by_pointer = Map::<*const i32>::default();
    let first = std::ptr::from_ref(&values[0]);
    let second = std::ptr::from_ref(&values[1]);
    by_pointer.add(second);
    by_pointer.add(first);
    let bucket = |key: *const i32| key as usize % by_pointer.bucket_count();
    let expected = if bucket(first) < bucket(second) {
        [first, second]
    } else {
        [second, first]
    };
    assert_eq!(by_pointer.iter().copied().collect::<Vec<_>>(), expected);
}

/// A map that shrank from its largest size: blocks 0, 2 and 102, and two that share a bucket.
fn packed_map_past_its_largest_size() -> PackedMap<i32> {
    let mut map = PackedMap::default();
    for block in 0..104 {
        map.add(block * 32);
    }
    let bucket_mate = (102 + 1009) * 32;
    map.add(bucket_mate);
    for block in (0..104).filter(|block| ![0, 2, 102].contains(block)) {
        map.remove(block * 32);
    }
    map
}

#[test]
fn a_packed_copy_that_keeps_the_walk_differs_from_a_plain_clone() {
    let map = packed_map_past_its_largest_size();
    assert_eq!(map.bucket_count(), 1009);
    let copy = map.same_walk_copy();
    assert_eq!(copy.bucket_count(), 1009);
    assert_eq!(copy.extent(), map.extent());
    let walk: Vec<i32> = map.iter().collect();
    assert_eq!(copy.iter().collect::<Vec<i32>>(), walk);
    assert_ne!(map.clone().iter().collect::<Vec<i32>>(), walk);
}

#[test]
fn an_empty_packed_copy_keeps_the_bucket_count() {
    let mut map = packed_map_past_its_largest_size();
    map.clear();
    let mut copy = map.same_walk_copy();
    assert!(copy.is_empty());
    assert_eq!(copy.bucket_count(), 1009);
    assert!(copy.add(5));
    assert!(copy.contains(5));
}
