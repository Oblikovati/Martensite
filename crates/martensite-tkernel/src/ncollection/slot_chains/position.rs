//! Iteration follows one chain, then scans forward for the next nonempty bucket; a map that walks
//! its keys in the order added goes from slot to slot instead.
use super::{SlotChains, SlotLink};

/// A traversal point: the current bucket and the current slot (`None` once exhausted).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::ncollection) struct ChainPosition {
    bucket: usize,
    slot: SlotLink,
}

impl ChainPosition {
    /// The first entry in bucket order, or an exhausted position for an empty or unallocated map.
    pub(in crate::ncollection) fn first(chains: &SlotChains) -> Self {
        if chains.walks_slots() {
            return Self {
                bucket: 0,
                slot: (chains.slot_count() > 0).then_some(0),
            };
        }
        let mut position = Self::default();
        position.seek(chains);
        position
    }
    /// The current slot; `None` means `more()` is false.
    pub(in crate::ncollection) fn slot(&self) -> SlotLink {
        self.slot
    }
    /// Advances: the chain successor, else the head of the next nonempty bucket.
    pub(in crate::ncollection) fn advance(&mut self, chains: &SlotChains) {
        let Some(slot) = self.slot else {
            return;
        };
        if chains.walks_slots() {
            self.slot = (slot + 1 < chains.slot_count()).then_some(slot + 1);
            return;
        }
        self.slot = chains.next(slot);
        if self.slot.is_some() {
            return;
        }
        self.bucket += 1;
        self.seek(chains);
    }
    fn seek(&mut self, chains: &SlotChains) {
        let table = chains.table();
        while let Some(&head) = table.get(self.bucket) {
            if head.is_some() {
                self.slot = head;
                return;
            }
            self.bucket += 1;
        }
        self.slot = None;
    }
}
