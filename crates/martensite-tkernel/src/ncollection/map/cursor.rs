//! The map cursor and the allocator-replacing `clear`.
use super::super::BaseAllocator;
use super::super::base_map::exhausted_iterator;
use super::{Debug, HashPolicy, Map, MapIter};
use crate::standard::Failure;

/// A read-only cursor over bucket order; `let mut it = map.cursor();`.
pub struct MapIterator<'a, K> {
    keys: MapIter<'a, K>,
}

impl<K: Debug, H: HashPolicy<K>> Map<K, H> {
    /// A cursor on the first key in bucket order.
    ///
    /// ```
    /// # use martensite_tkernel::ncollection::Map;
    /// let mut map = Map::<i32>::default();
    /// map.add(3);
    /// assert_eq!(map.cursor().key().unwrap(), &3);
    /// ```
    pub fn cursor(&self) -> MapIterator<'_, K> {
        MapIterator { keys: self.iter() }
    }

    /// Clears, releasing buckets only when
    /// the allocator changes; `None` (a null handle) selects the common allocator.
    ///
    /// ```
    /// # use martensite_tkernel::ncollection::{BaseAllocator, Map};
    /// let mut map = Map::<i32>::default();
    /// map.add(1);
    /// map.clear_with_allocator(None);
    /// assert!(map.is_empty() && map.allocator() == &BaseAllocator::common());
    /// ```
    pub fn clear_with_allocator(&mut self, resource: Option<BaseAllocator>) {
        self.storage.clear_with_allocator(resource);
    }
}

impl<'a, K> MapIterator<'a, K> {
    /// True while the cursor is on a key.
    pub fn more(&self) -> bool {
        self.keys.entries.current().is_some()
    }
    /// `Next`: moves to the following key in bucket order.
    pub fn next(&mut self) {
        self.keys.next();
    }
    /// `Value`: the current key, or Standard_NoSuchObject past the end.
    pub fn value(&self) -> Result<&'a K, Failure> {
        self.current("NCollection_Map::Iterator::Value")
    }
    /// `Key`: the current key, or Standard_NoSuchObject past the end.
    pub fn key(&self) -> Result<&'a K, Failure> {
        self.current("NCollection_Map::Iterator::Key")
    }
    /// Restarts on another map.
    pub fn initialize<H: HashPolicy<K>>(&mut self, map: &'a Map<K, H>)
    where
        K: Debug,
    {
        *self = map.cursor();
    }
    /// `Reset`: restarts on the map the cursor was created for.
    pub fn reset(&mut self) {
        self.keys.entries.reset();
    }
    /// Both cursors are on the same key, or both are past the end.
    pub fn is_equal(&self, other: &Self) -> bool {
        self.keys == other.keys
    }
    fn current(&self, method: &str) -> Result<&'a K, Failure> {
        self.keys
            .entries
            .current()
            .map(|(key, _)| key)
            .ok_or_else(|| exhausted_iterator(method))
    }
}

/// A cursor attached to no map, whose `more()` is false.
impl<K> Default for MapIterator<'_, K> {
    fn default() -> Self {
        Self {
            keys: MapIter {
                entries: Default::default(),
            },
        }
    }
}

impl<K> Clone for MapIterator<'_, K> {
    fn clone(&self) -> Self {
        Self {
            keys: self.keys.clone(),
        }
    }
}

impl<K> MapIterator<'_, K> {
    /// Advances and returns this cursor, as prefix increment; `cursor.advance().key()`.
    pub fn advance(&mut self) -> &mut Self {
        self.next();
        self
    }
    /// Returns the previous cursor and advances, as postfix increment; `cursor.advance_post().key()`.
    pub fn advance_post(&mut self) -> Self {
        let previous = self.clone();
        self.next();
        previous
    }
}
