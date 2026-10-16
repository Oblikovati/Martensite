//! The public surface shared by every hashed map.
use super::{BaseAllocator, DataMap, DoubleMap, HashPolicy, IndexedDataMap, IndexedMap, Map};
use std::fmt::Debug;
// The protected base-map state (bucket table, resize protocol, destroy, the
// basic and iterator bases) is not observable through the public API: every hashed map keeps it in
// its safe slot storage (`slot_chains`). `MapLinkNode` remains the link node of the
// lists.
mod link_node;
mod resize_plan;
pub use link_node::MapLinkNode;
pub(super) use resize_plan::{planned_bucket_count, table_slot_count};
mod indexed_cursor;
mod indexed_source;
pub use indexed_cursor::BaseMapIndexedIterator;
pub use indexed_source::{IndexedCursorSource, IndexedCursorSourceMut};

/// Bucket and size queries shared by the hashed maps.
///
/// ```
/// use martensite_tkernel::ncollection::{BaseMap, Map};
/// let mut map = Map::<i32>::default();
/// map.add(7);
/// assert_eq!((BaseMap::extent(&map), BaseMap::nb_buckets(&map)), (1, 101));
/// ```
pub trait BaseMap {
    /// The hash modulus, kept even after `clear(true)` releases the buckets.
    fn nb_buckets(&self) -> usize;
    /// `Size()`: the number of stored entries.
    fn size(&self) -> usize;
    /// The node allocator.
    fn allocator(&self) -> &BaseAllocator;
    /// The entry count narrowed to `i32`.
    fn extent(&self) -> i32 {
        self.size() as i32
    }
    /// The same narrowed entry count as `extent()`.
    fn length(&self) -> i32 {
        self.size() as i32
    }
    /// True when no entry is stored.
    fn is_empty(&self) -> bool {
        self.size() == 0
    }
}

impl<K: Debug, V, H: HashPolicy<K>> BaseMap for DataMap<K, V, H> {
    fn nb_buckets(&self) -> usize {
        self.bucket_count()
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn allocator(&self) -> &BaseAllocator {
        DataMap::allocator(self)
    }
}

impl<K: Debug, H: HashPolicy<K>> BaseMap for Map<K, H> {
    fn nb_buckets(&self) -> usize {
        self.bucket_count()
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn allocator(&self) -> &BaseAllocator {
        Map::allocator(self)
    }
}

impl<K: Debug, H: HashPolicy<K>> BaseMap for IndexedMap<K, H> {
    fn nb_buckets(&self) -> usize {
        self.bucket_count()
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn allocator(&self) -> &BaseAllocator {
        IndexedMap::allocator(self)
    }
}

impl<K: Debug, V, H: HashPolicy<K>> BaseMap for IndexedDataMap<K, V, H> {
    fn nb_buckets(&self) -> usize {
        self.bucket_count()
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn allocator(&self) -> &BaseAllocator {
        IndexedDataMap::allocator(self)
    }
}

impl<A: Debug, B: Debug, H1: HashPolicy<A>, H2: HashPolicy<B>> BaseMap for DoubleMap<A, B, H1, H2> {
    fn nb_buckets(&self) -> usize {
        self.bucket_count()
    }
    fn size(&self) -> usize {
        self.len()
    }
    fn allocator(&self) -> &BaseAllocator {
        DoubleMap::allocator(self)
    }
}

/// The `Standard_NoSuchObject` a map iterator raises when read past its end.
pub(super) fn exhausted_iterator(method: &str) -> crate::standard::Failure {
    crate::standard::Failure::with_type(
        "Standard_NoSuchObject",
        &format!("{method}: iterator exhausted; expected More() to be true"),
        "",
    )
}
