//! Lookup: `hash(key) % NbBuckets + 1` selects the bucket, then the chain is compared.
use super::ChainedSlots;
use crate::ncollection::{BaseMap, HashPolicy, slot_chains::SlotLink};

impl<K, V> ChainedSlots<K, V> {
    /// Finds a bound key; an empty map skips the user hasher.
    #[inline]
    pub(in crate::ncollection) fn lookup(
        &self,
        key: &K,
        hasher: &impl HashPolicy<K>,
    ) -> Option<(usize, usize)> {
        if self.size() == 0 {
            return None;
        }
        let (bucket, slot) = self.lookup_with_bucket(key, hasher);
        slot.map(|slot| (bucket, slot))
    }
    /// Hashes once and returns the bucket with the matching slot; callers prepared a table.
    #[inline]
    pub(in crate::ncollection) fn lookup_with_bucket(
        &self,
        key: &K,
        hasher: &impl HashPolicy<K>,
    ) -> (usize, SlotLink) {
        let bucket = hasher.hash(key) % self.bucket_count + 1;
        let slot = self
            .chains
            .find(bucket, |slot| hasher.equal(self.payload.key(slot), key));
        (bucket, slot)
    }
    /// Finds a bound key using a precomputed hash and custom equality without constructing `K`.
    #[inline]
    pub(in crate::ncollection) fn lookup_by(
        &self,
        hash: usize,
        equal: impl FnMut(&K) -> bool,
    ) -> Option<(usize, usize)> {
        if self.size() == 0 {
            return None;
        }
        let (bucket, slot) = self.lookup_with_bucket_by(hash, equal);
        slot.map(|slot| (bucket, slot))
    }
    /// Returns the bucket and matching slot for a precomputed hash and custom equality.
    #[inline]
    pub(in crate::ncollection) fn lookup_with_bucket_by(
        &self,
        hash: usize,
        mut equal: impl FnMut(&K) -> bool,
    ) -> (usize, SlotLink) {
        let bucket = hash % self.bucket_count + 1;
        let slot = self
            .chains
            .find(bucket, |slot| equal(self.payload.key(slot)));
        (bucket, slot)
    }
}
