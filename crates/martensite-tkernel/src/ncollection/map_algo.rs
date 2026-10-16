//! Set operations preserving operand traversal order and destination policy.
//!
//! Aliased operands (`unite(map, map)`) have defined results;
//! Rust borrowing makes a destination alias impossible, so the two-operand forms
//! (`unite`, `intersect`, `subtract`, `differ`) carry the aliased behaviour instead.
use super::{HashPolicy, Map};
use std::fmt::Debug;

/// Every key of `other` is in `map` (an empty `other` is always contained).
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, b) = (Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// assert!(map_algo::contains(&a, &b));
/// ```
pub fn contains<K: Debug, H: HashPolicy<K>>(map: &Map<K, H>, other: &Map<K, H>) -> bool {
    if std::ptr::eq(map, other) || other.is_empty() {
        return true;
    }
    map.len() >= other.len() && other.iter().all(|key| map.contains(key))
}

/// Both maps hold the same keys.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, mut b) = (Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// b.add(1);
/// assert!(map_algo::is_equal(&a, &b));
/// ```
pub fn is_equal<K: Debug, H: HashPolicy<K>>(map: &Map<K, H>, other: &Map<K, H>) -> bool {
    std::ptr::eq(map, other) || (map.len() == other.len() && contains(map, other))
}

/// Some key is in both maps; the smaller map (by size) is scanned.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, mut b) = (Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// b.add(2);
/// assert!(!map_algo::has_intersection(&a, &b));
/// ```
pub fn has_intersection<K: Debug, H: HashPolicy<K>>(map: &Map<K, H>, other: &Map<K, H>) -> bool {
    let (scanned, probed) = if other.len() < map.len() {
        (other, map)
    } else {
        (map, other)
    };
    scanned.iter().any(|key| probed.contains(key))
}

/// `result` becomes the keys of `left` followed by those of `right`.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut out, mut a, b) = (Map::<i32>::default(), Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// map_algo::union(&mut out, &a, &b);
/// assert!(out.contains(&1));
/// ```
pub fn union<K: Debug + Clone, H: HashPolicy<K>>(
    result: &mut Map<K, H>,
    left: &Map<K, H>,
    right: &Map<K, H>,
) {
    if std::ptr::eq(left, right) {
        result.assign(left);
        return;
    }
    result.clear(false);
    for key in left.iter().chain(right.iter()) {
        result.add_ref(key);
    }
}

/// Adds the keys of `other` to `map`; true when `map` grew.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, mut b) = (Map::<i32>::default(), Map::<i32>::default());
/// b.add(1);
/// assert!(map_algo::unite(&mut a, &b));
/// ```
pub fn unite<K: Debug + Clone, H: HashPolicy<K>>(map: &mut Map<K, H>, other: &Map<K, H>) -> bool {
    let old_extent = map.len();
    for key in other {
        map.add_ref(key);
    }
    old_extent != map.len()
}

/// `result` becomes the keys common to both, in the order of the operand
/// with fewer keys (`right` on ties).
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut out, mut a, mut b) = (Map::<i32>::default(), Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// b.add(1);
/// map_algo::intersection(&mut out, &a, &b);
/// assert!(out.contains(&1));
/// ```
pub fn intersection<K: Debug + Clone, H: HashPolicy<K>>(
    result: &mut Map<K, H>,
    left: &Map<K, H>,
    right: &Map<K, H>,
) {
    if std::ptr::eq(left, right) {
        result.assign(left);
        return;
    }
    result.clear(false);
    let (smaller, larger) = if left.len() < right.len() {
        (left, right)
    } else {
        (right, left)
    };
    for key in smaller.iter().filter(|key| larger.contains(key)) {
        result.add_ref(key);
    }
}

/// Keeps only the keys of `map` also in `other`; true when `map` shrank.
///
/// `map` is first exchanged with a fresh one-bucket map built with a default
/// hasher and `map`'s allocator, then rebuilt by intersection; the rebuilt map keeps
/// that default hasher and the rebuilt bucket order.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, b) = (Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// assert!(map_algo::intersect(&mut a, &b) && a.is_empty());
/// ```
pub fn intersect<K: Debug + Clone, H: HashPolicy<K> + Default>(
    map: &mut Map<K, H>,
    other: &Map<K, H>,
) -> bool {
    if map.is_empty() {
        return false;
    }
    let old_extent = map.len();
    let copy = take_into_fresh(map);
    intersection(map, &copy, other);
    old_extent != map.len()
}

/// Removes the keys of `other` from `map`; true when `map` shrank.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, mut b) = (Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// b.add(1);
/// assert!(map_algo::subtract(&mut a, &b) && a.is_empty());
/// ```
pub fn subtract<K: Debug, H: HashPolicy<K>>(map: &mut Map<K, H>, other: &Map<K, H>) -> bool {
    let old_extent = map.len();
    for key in other {
        map.remove(key);
    }
    old_extent != map.len()
}

/// `result` becomes the keys of `left` absent from `right`.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut out, mut a, b) = (Map::<i32>::default(), Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// map_algo::subtraction(&mut out, &a, &b);
/// assert!(out.contains(&1));
/// ```
pub fn subtraction<K: Debug + Clone, H: HashPolicy<K>>(
    result: &mut Map<K, H>,
    left: &Map<K, H>,
    right: &Map<K, H>,
) {
    result.assign(left);
    subtract(result, right);
}

/// `Difference`: `result` becomes the keys in exactly one operand, `left`'s first.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut out, mut a, b) = (Map::<i32>::default(), Map::<i32>::default(), Map::<i32>::default());
/// a.add(1);
/// map_algo::difference(&mut out, &a, &b);
/// assert!(out.contains(&1));
/// ```
pub fn difference<K: Debug + Clone, H: HashPolicy<K>>(
    result: &mut Map<K, H>,
    left: &Map<K, H>,
    right: &Map<K, H>,
) {
    result.clear(false);
    if std::ptr::eq(left, right) {
        return;
    }
    let only_left = left.iter().filter(|key| !right.contains(key));
    let only_right = right.iter().filter(|key| !left.contains(key));
    for key in only_left.chain(only_right) {
        result.add_ref(key);
    }
}

/// `map` becomes its symmetric difference with `other`; true when its size changed.
///
/// As for `intersect`, `map` is rebuilt from a fresh default-hasher map.
///
/// ```
/// # use martensite_tkernel::ncollection::{Map, map_algo};
/// let (mut a, mut b) = (Map::<i32>::default(), Map::<i32>::default());
/// b.add(1);
/// assert!(map_algo::differ(&mut a, &b) && a.contains(&1));
/// ```
pub fn differ<K: Debug + Clone, H: HashPolicy<K> + Default>(
    map: &mut Map<K, H>,
    other: &Map<K, H>,
) -> bool {
    let old_extent = map.len();
    let copy = take_into_fresh(map);
    difference(map, &copy, other);
    old_extent != map.len()
}

/// Exchanges `map` with a fresh one-bucket map on its allocator.
fn take_into_fresh<K: Debug, H: HashPolicy<K> + Default>(map: &mut Map<K, H>) -> Map<K, H> {
    let fresh = Map::with_allocator(H::default(), 1, map.allocator().clone());
    std::mem::replace(map, fresh)
}
