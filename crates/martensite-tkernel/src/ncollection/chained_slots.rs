//! Safe bucket chains for DataMap and Map: slot indices replace node pointers.
//!
//! A hashed map keeps a table of `NbBuckets + 1` list heads, and each node links to the next
//! node of its bucket. Here the nodes live in one `Vec` and every link is a slot index (see
//! `slot_chains`), so the observable contracts stay those of linked nodes: the bucket count and its prime
//! steps, head insertion inside a bucket, traversal (bucket `0..=NbBuckets`, then each chain),
//! rehash and removal order, a null table before the first insertion or after `clear(true)`, and
//! a retained table after `clear(false)`. Node addresses and the per-node allocator calls are not
//! observable through the public API, so the map keeps only the
//! allocator identity.
use super::{
    BaseAllocator, BaseMap, HashBucketTable,
    slot_chains::{ChainView, SlotChains, SlotPayload, SlotRangeMut},
};
mod indexed;
mod lookup;
mod resize;
mod topology;

/// Bucket heads, slots and allocator identity of one hashed map.
pub(super) struct ChainedSlots<K, V> {
    allocator: BaseAllocator,
    bucket_count: usize,
    chains: SlotChains,
    payload: SlotPayload<K, V>,
}

impl<K, V> ChainedSlots<K, V> {
    /// Constructors keep even a zero bucket count until the first insertion resizes.
    pub(super) fn new(bucket_count: usize, allocator: BaseAllocator) -> Self {
        Self {
            allocator,
            bucket_count,
            chains: SlotChains::default(),
            payload: SlotPayload::default(),
        }
    }
    /// The keys and values of every slot.
    pub(super) fn payload(&self) -> &SlotPayload<K, V> {
        &self.payload
    }
    pub(super) fn payload_mut(&mut self) -> &mut SlotPayload<K, V> {
        &mut self.payload
    }
    /// The bucket chains, for cursors that follow traversal order.
    pub(super) fn chains(&self) -> &SlotChains {
        &self.chains
    }
    /// The table and slots together, for shared ranges.
    pub(super) fn view(&self) -> ChainView<'_, K, V> {
        ChainView::new(&self.chains, &self.payload)
    }
    /// Exclusive traversal in bucket order, lending each value once.
    pub(super) fn range_mut(&mut self) -> SlotRangeMut<'_, K, V> {
        SlotRangeMut::new(self.chains.order(), &mut self.payload)
    }
    /// The allocated table, or `None` before one exists.
    pub(super) fn buckets(&self) -> Option<HashBucketTable<'_, K, V>> {
        HashBucketTable::of(self.view())
    }
    /// Clears with an allocator: a new allocator releases the
    /// table; `None` (a null handle) selects the common allocator.
    pub(super) fn clear_with_allocator(&mut self, requested: Option<BaseAllocator>) {
        self.clear_with_borrowed_allocator(requested.as_ref());
    }
    /// The same with a borrowed handle, so no handle is
    /// added while the entries drop; the new allocator is adopted only afterwards.
    pub(super) fn clear_with_borrowed_allocator(&mut self, requested: Option<&BaseAllocator>) {
        let release = requested != Some(&self.allocator);
        self.clear(release);
        if release {
            self.allocator = requested.cloned().unwrap_or_else(BaseAllocator::common);
        }
    }
}

impl<K, V> BaseMap for ChainedSlots<K, V> {
    fn nb_buckets(&self) -> usize {
        self.bucket_count
    }
    fn size(&self) -> usize {
        self.payload.len()
    }
    fn allocator(&self) -> &BaseAllocator {
        &self.allocator
    }
}

// No API pins an entry in place: growth moves slots, so the map is Unpin whatever K and V are,
// as boxed nodes would be.
impl<K, V> Unpin for ChainedSlots<K, V> {}

impl<K, V> Drop for ChainedSlots<K, V> {
    fn drop(&mut self) {
        // Entries drop in bucket/chain order, not slot order.
        self.clear(true);
    }
}
