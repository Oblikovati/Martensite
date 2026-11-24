//! Assignment identity, storage exchange and the public bucket table.
use super::{Debug, HashPolicy, IndexedMap};
use crate::ncollection::HashBucketTable;

impl<K: Debug, H: HashPolicy<K>> IndexedMap<K, H> {
    /// The key table (chains yield `(&key, &())`), or `None` before
    /// one exists. The index table stays private.
    pub(crate) fn buckets(&self) -> Option<HashBucketTable<'_, K, ()>> {
        self.storage.buckets()
    }
    /// Exchanges both maps as move assignment does; `target.move_from(&mut source)`.
    pub(crate) fn move_from(&mut self, source: &mut Self) -> &mut Self {
        self.exchange(source);
        self
    }

    /// Preserves self-assignment without creating aliased references; `map.assign_self()`.
    pub(crate) fn assign_self(&mut self) -> &mut Self {
        self
    }
}
