//! A policy can make a map walk its entries in the order they were added, for keys hashed by an
//! address, which differs between runs of a program. Other maps keep their bucket order.
use martensite_tkernel::ncollection::{BaseAllocator, DataMap, HashPolicy, Map, PackedMap};

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

type AddressDataMap = DataMap<i32, i32, AddressLike>;

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

fn walked<H: HashPolicy<i32>>(map: &Map<i32, H>) -> Vec<i32> {
    map.iter().copied().collect()
}

fn walked_data<H: HashPolicy<i32>>(map: &DataMap<i32, i32, H>) -> Vec<i32> {
    map.items().map(|(key, _)| *key).collect()
}

fn add_all(map: &mut Map<i32, AddressLike>) {
    for key in ADDED {
        map.add(key);
    }
}

fn bind_all(map: &mut DataMap<i32, i32, AddressLike>) {
    for key in ADDED {
        map.bind(key, key * 2);
    }
}

#[test]
fn every_way_to_build_a_set_keeps_the_order_of_adding() {
    let arena = BaseAllocator::incremental(4096);
    let mut built: Vec<Map<i32, AddressLike>> = vec![
        Map::default(),
        Map::new(0),
        Map::new(5000),
        Map::with_hasher(AddressLike, 5000),
        Map::with_allocator(AddressLike, 5000, arena.clone()),
        Map::with_hasher_signed(AddressLike, 5000).unwrap(),
        Map::with_allocator_signed(AddressLike, 5000, arena).unwrap(),
    ];
    for map in &mut built {
        add_all(map);
        assert_eq!(walked(map), ADDED);
    }
}

#[test]
fn every_way_to_build_a_map_keeps_the_order_of_binding() {
    let arena = BaseAllocator::incremental(4096);
    let mut built: Vec<DataMap<i32, i32, AddressLike>> = vec![
        DataMap::default(),
        DataMap::with_hasher(AddressLike, 0),
        DataMap::with_hasher(AddressLike, 5000),
        DataMap::with_allocator(AddressLike, 5000, arena.clone()),
        DataMap::with_hasher_signed(AddressLike, 5000).unwrap(),
        DataMap::with_allocator_signed(AddressLike, 5000, arena).unwrap(),
    ];
    for map in &mut built {
        bind_all(map);
        assert_eq!(walked_data(map), ADDED);
    }
}

#[test]
fn a_cloned_or_assigned_set_keeps_the_order_of_adding() {
    let source = address_map();
    let mut cloned = source.clone();
    assert_eq!(walked(&cloned), ADDED);
    cloned.add(-1);
    assert_eq!(walked(&cloned).last(), Some(&-1));
    let mut empty_clone = Map::<i32, AddressLike>::default().clone();
    add_all(&mut empty_clone);
    assert_eq!(walked(&empty_clone), ADDED);
    let mut assigned = Map::with_hasher(AddressLike, 1);
    assigned.add(1234);
    assigned.assign(&source);
    assert_eq!(walked(&assigned), ADDED);
    assigned.add(-1);
    assert_eq!(walked(&assigned).last(), Some(&-1));
    let mut returned = Map::with_hasher(AddressLike, 1);
    assert_eq!(walked(returned.assign_returning(&source)), ADDED);
}

#[test]
fn a_cloned_or_assigned_map_keeps_the_order_of_binding() {
    let source = address_data_map();
    let mut cloned = source.clone();
    assert_eq!(walked_data(&cloned), ADDED);
    cloned.bind(-1, 0);
    assert_eq!(walked_data(&cloned).last(), Some(&-1));
    let mut assigned = DataMap::with_hasher(AddressLike, 1);
    assigned.bind(1234, 0);
    assigned.assign(&source);
    assert_eq!(walked_data(&assigned), ADDED);
    let mut returned = DataMap::with_hasher(AddressLike, 1);
    assert_eq!(walked_data(returned.assign_returning(&source)), ADDED);
}

#[test]
fn exchanging_keeps_the_order_of_adding_on_both_sides() {
    let mut set = address_map();
    let mut other_set = Map::with_hasher(AddressLike, 1);
    other_set.add(9);
    other_set.add(1);
    set.exchange(&mut other_set);
    assert_eq!(walked(&set), [9, 1]);
    assert_eq!(walked(&other_set), ADDED);
    set.add(5);
    assert_eq!(walked(&set), [9, 1, 5]);
    other_set.move_from_returning(&mut set);
    assert_eq!(walked(&other_set), [9, 1, 5]);
    assert_eq!(walked(&set), ADDED);

    let mut map = address_data_map();
    let mut other_map = DataMap::with_hasher(AddressLike, 1);
    other_map.bind(9, 0);
    other_map.bind(1, 0);
    map.exchange(&mut other_map);
    assert_eq!(walked_data(&map), [9, 1]);
    assert_eq!(walked_data(&other_map), ADDED);
    map.move_from_returning(&mut other_map);
    assert_eq!(walked_data(&map), ADDED);
    assert_eq!(walked_data(&other_map), [9, 1]);
}

#[test]
fn clearing_and_filling_again_keeps_the_order_of_adding() {
    let clearings: [fn(&mut Map<i32, AddressLike>); 4] = [
        |map| map.clear(false),
        |map| map.clear(true),
        |map| map.clear_with_allocator(None),
        |map| map.clear_with_allocator(Some(BaseAllocator::incremental(4096))),
    ];
    for clearing in clearings {
        let mut set = address_map();
        clearing(&mut set);
        assert!(walked(&set).is_empty());
        add_all(&mut set);
        assert_eq!(walked(&set), ADDED);
    }
    let clearings: [fn(&mut AddressDataMap); 4] = [
        |map| map.clear(false),
        |map| map.clear(true),
        |map| map.clear_with_allocator(None),
        |map| map.clear_with_allocator(Some(BaseAllocator::incremental(4096))),
    ];
    for clearing in clearings {
        let mut map = address_data_map();
        clearing(&mut map);
        assert!(walked_data(&map).is_empty());
        bind_all(&mut map);
        assert_eq!(walked_data(&map), ADDED);
    }
}

#[test]
fn resizing_keeps_the_order_of_adding() {
    let mut set = Map::with_hasher(AddressLike, 1);
    set.resize(5000);
    add_all(&mut set);
    set.resize(50_000);
    assert_eq!(walked(&set), ADDED);
    set.resize_signed(100_000).unwrap();
    assert_eq!(walked(&set), ADDED);

    let mut map = DataMap::with_hasher(AddressLike, 1);
    map.resize(5000);
    bind_all(&mut map);
    map.resize(50_000);
    assert_eq!(walked_data(&map), ADDED);
    map.resize_signed(100_000).unwrap();
    assert_eq!(walked_data(&map), ADDED);
}

/// Growing past the table as keys arrive rehashes it more than once.
#[test]
fn growing_key_by_key_keeps_the_order_of_adding() {
    let keys: Vec<i32> = (0..5000).rev().map(|key| key * 7).collect();
    let mut set = Map::with_hasher(AddressLike, 1);
    let mut map = DataMap::with_hasher(AddressLike, 1);
    for &key in &keys {
        set.add(key);
        map.bind(key, key);
    }
    assert_eq!(walked(&set), keys);
    assert_eq!(walked_data(&map), keys);
}

/// The operations of sets rebuild the set, some on a new table.
#[test]
fn the_operations_of_sets_keep_the_order_of_adding() {
    let other = || {
        let mut map = Map::with_hasher(AddressLike, 1);
        for key in [999, 41, -5, 70] {
            map.add(key);
        }
        map
    };
    let mut united = address_map();
    united.unite(&other());
    assert_eq!(walked(&united), [70, 3, 500, 41, 8, 999, 12, 256, -5]);
    // An intersection walks the smaller set and adds what the larger one holds.
    let mut intersected = address_map();
    intersected.intersect(&other());
    assert_eq!(walked(&intersected), [999, 41, 70]);
    let mut differed = address_map();
    differed.differ(&other());
    assert_eq!(walked(&differed), [3, 500, 8, 12, 256, -5]);
    let mut made = Map::with_hasher(AddressLike, 1);
    made.union(&address_map(), &other());
    assert_eq!(walked(&made), [70, 3, 500, 41, 8, 999, 12, 256, -5]);
    made.intersection(&address_map(), &other());
    assert_eq!(walked(&made), [999, 41, 70]);
    // A subtraction copies the set and removes keys: each removal moves the last entry.
    made.subtraction(&address_map(), &other());
    assert_eq!(walked(&made), [256, 3, 500, 12, 8]);
    made.difference(&address_map(), &other());
    assert_eq!(walked(&made), [3, 500, 8, 12, 256, -5]);
}

/// A policy that does not ask keeps bucket order through the same operations.
#[test]
fn a_map_keeps_its_bucket_order_through_a_clone_and_an_exchange() {
    let mut by_value = Map::<i32>::default();
    for key in [30, 10, 20] {
        by_value.add(key);
    }
    let cloned = by_value.clone();
    assert_eq!(walked(&cloned), [10, 20, 30]);
    let mut other = Map::<i32>::default();
    other.exchange(&mut by_value);
    assert_eq!(walked(&other), [10, 20, 30]);
}
