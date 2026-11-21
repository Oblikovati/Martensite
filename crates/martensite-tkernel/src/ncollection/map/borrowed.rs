//! Borrowed insertion keeps the lookup and key-copy ordering.
use super::{Debug, HashPolicy, Map};

impl<K: Debug + Clone, H: HashPolicy<K>> Map<K, H> {
    /// Adds a borrowed key only when absent; lookup precedes allocation and copying.
    pub(crate) fn add_ref(&mut self, key: &K) -> bool {
        self.place_key_ref(key).0
    }
    /// Returns a stable stored key, copying the borrowed key only on insertion.
    #[cfg(test)]
    pub(crate) fn added_ref(&mut self, key: &K) -> &K {
        self.place_key_ref(key).1
    }
}
