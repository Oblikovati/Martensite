//! More/Next/Value over the map's bucket chains in bucket order.
use super::super::base_map::exhausted_iterator;
use super::{DataMap, DataMapItems, DataMapStorage, Debug, HashPolicy};
use crate::{ncollection::slot_chains::ChainPosition, standard::Failure};
/// Read-only cursor; `let mut cursor=map.cursor(); while cursor.more() { cursor.next(); }`.
pub struct DataMapIterator<'a, K, V> {
    items: DataMapItems<'a, K, V>,
}
/// Exclusive cursor; `*map.cursor_mut().change_value()?=7`.
pub struct DataMapIteratorMut<'a, K, V> {
    source: Option<&'a mut DataMapStorage<K, V>>,
    position: ChainPosition,
}
impl<K: Debug, V, H: HashPolicy<K>> DataMap<K, V, H> {
    /// A cursor on the first binding in bucket order.
    ///
    /// ```
    /// # use martensite_tkernel::ncollection::DataMap;
    /// let mut map = DataMap::<i32, i32>::default();
    /// map.bind(1, 10);
    /// let it = map.cursor();
    /// assert_eq!((it.key().unwrap(), it.value().unwrap()), (&1, &10));
    /// ```
    pub fn cursor(&self) -> DataMapIterator<'_, K, V> {
        DataMapIterator {
            items: self.items(),
        }
    }
    /// A cursor that may also change items.
    ///
    /// ```
    /// # use martensite_tkernel::ncollection::DataMap;
    /// let mut map = DataMap::<i32, i32>::default();
    /// map.bind(1, 10);
    /// *map.cursor_mut().change_value().unwrap() = 5;
    /// assert_eq!(map.seek(&1), Some(&5));
    /// ```
    pub fn cursor_mut(&mut self) -> DataMapIteratorMut<'_, K, V> {
        DataMapIteratorMut {
            position: ChainPosition::first(self.storage.chains()),
            source: Some(&mut self.storage),
        }
    }
}
impl<'a, K, V> DataMapIterator<'a, K, V> {
    /// Reports whether the cursor has an item; `cursor.more()`.
    pub fn more(&self) -> bool {
        self.items.current().is_some()
    }
    /// Advances in bucket/chain order; `cursor.next()`.
    pub fn next(&mut self) {
        self.items.next();
    }
    /// Borrows the value or reports exhaustion; `cursor.value()?`.
    pub fn value(&self) -> Result<&'a V, Failure> {
        Ok(self.entry("NCollection_DataMap::Iterator::Value")?.1)
    }
    /// Borrows the key or reports exhaustion; `cursor.key()?`.
    pub fn key(&self) -> Result<&'a K, Failure> {
        Ok(self.entry("NCollection_DataMap::Iterator::Key")?.0)
    }
    /// Rebinds to a read-only map; `cursor.initialize(&map)`.
    pub fn initialize<H: HashPolicy<K>>(&mut self, map: &'a DataMap<K, V, H>)
    where
        K: Debug,
    {
        *self = map.cursor();
    }
    /// Restarts on the bound graph (default remains empty); `cursor.reset()`.
    pub fn reset(&mut self) {
        self.items.reset();
    }
    /// Compares node identity, including exhausted cursors; `cursor.is_equal(&other)`.
    pub fn is_equal(&self, other: &Self) -> bool {
        self.items == other.items
    }
    fn entry(&self, method: &str) -> Result<(&'a K, &'a V), Failure> {
        self.items
            .current()
            .ok_or_else(|| exhausted_iterator(method))
    }
}
impl<K, V> Default for DataMapIterator<'_, K, V> {
    fn default() -> Self {
        Self {
            items: DataMapItems::default(),
        }
    }
}
impl<K, V> Clone for DataMapIterator<'_, K, V> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
        }
    }
}
impl<K, V> DataMapIterator<'_, K, V> {
    /// Advances and returns this cursor, as prefix increment; `cursor.advance().value()`.
    pub fn advance(&mut self) -> &mut Self {
        self.next();
        self
    }
    /// Returns the previous cursor and advances, as postfix increment; `cursor.advance_post().value()`.
    pub fn advance_post(&mut self) -> Self {
        let previous = self.clone();
        self.next();
        previous
    }
}
impl<'a, K, V> DataMapIteratorMut<'a, K, V> {
    /// Reports whether the cursor has an item; `cursor.more()`.
    pub fn more(&self) -> bool {
        self.position.slot().is_some()
    }
    /// Advances; `cursor.next()`.
    pub fn next(&mut self) {
        if let Some(source) = self.source.as_deref() {
            self.position.advance(source.chains());
        }
    }
    /// Borrows the value without permitting changes; `cursor.value()?`.
    pub fn value(&self) -> Result<&V, Failure> {
        Ok(self.entry("NCollection_DataMap::Iterator::Value")?.1)
    }
    /// Borrows the value exclusively; `*cursor.change_value()?=8`.
    pub fn change_value(&mut self) -> Result<&mut V, Failure> {
        let (Some(source), Some(slot)) = (self.source.as_deref_mut(), self.position.slot()) else {
            return Err(exhausted_iterator(
                "NCollection_DataMap::Iterator::ChangeValue",
            ));
        };
        Ok(source.payload_mut().value_mut(slot))
    }
    /// Borrows the key; `cursor.key()?`.
    pub fn key(&self) -> Result<&K, Failure> {
        Ok(self.entry("NCollection_DataMap::Iterator::Key")?.0)
    }
    /// Rebinds to a mutable map; `cursor.initialize(&mut map)`.
    pub fn initialize<H: HashPolicy<K>>(&mut self, map: &'a mut DataMap<K, V, H>)
    where
        K: Debug,
    {
        *self = map.cursor_mut();
    }
    /// Restarts on this map's first node; `cursor.reset()`.
    pub fn reset(&mut self) {
        if let Some(source) = self.source.as_deref() {
            self.position = ChainPosition::first(source.chains());
        }
    }
    fn entry(&self, method: &str) -> Result<(&K, &V), Failure> {
        let (Some(source), Some(slot)) = (self.source.as_deref(), self.position.slot()) else {
            return Err(exhausted_iterator(method));
        };
        Ok(source.payload().entry(slot))
    }
}
impl<K, V> Default for DataMapIteratorMut<'_, K, V> {
    fn default() -> Self {
        Self {
            source: None,
            position: ChainPosition::default(),
        }
    }
}
impl<K, V> DataMapIteratorMut<'_, K, V> {
    /// Compares node identity, with all exhausted cursors equal; `first.is_equal(&second)`.
    pub fn is_equal(&self, other: &Self) -> bool {
        self.identity() == other.identity()
    }
    /// Advances and returns the writable cursor; `cursor.advance().change_value()`.
    pub fn advance(&mut self) -> &mut Self {
        self.next();
        self
    }
    /// Returns a read-only previous position while advancing; `cursor.advance_post().value()`.
    /// The snapshot borrows this cursor to prevent aliased writable item references.
    pub fn advance_post(&mut self) -> DataMapIterator<'_, K, V> {
        let position = self.position;
        self.next();
        DataMapIterator {
            items: DataMapItems::from_position(self.source.as_deref(), position),
        }
    }
    // Node identity: the owning map plus the slot, or nothing once exhausted.
    fn identity(&self) -> Option<(*const DataMapStorage<K, V>, usize)> {
        let source = self.source.as_deref()?;
        Some((std::ptr::from_ref(source), self.position.slot()?))
    }
}
