//! Metadata and cursor updates finish before each removal's destructor boundary.
use super::super::{
    BaseAllocator, BaseSequenceRawIterator, SeqNode, Sequence, SequenceCursor, SequenceNodeOwner,
    SequenceNodeRetirement, topology::SequenceTopology,
};
use super::{Failure, no_such_node, out_of_range};
struct ClearMetadata<'a>(&'a mut SequenceTopology);
impl Drop for ClearMetadata<'_> {
    fn drop(&mut self) {
        // Reset before remaining owners unwind: observers must never traverse freed metadata.
        self.0.reset_retired_chain();
    }
}
impl<T> Sequence<T> {
    /// Removes an existing item through node/cache deletion; `sequence.remove(1)?`.
    pub fn remove(&mut self, index: usize) -> Result<(), Failure> {
        self.offset("Remove", index)?;
        let node = self
            .topology
            // SAFETY: the offset was checked, and this sequence exclusively owns every linked
            // node and its cache.
            .edit(|base| unsafe { base.prepare_index_removal(index) })?;
        // SAFETY: prepare_index_removal transferred a live unlinked node.
        drop(unsafe { self.take_unlinked(node) });
        Ok(())
    }
    /// Removes an inclusive one-based range in forward destruction order; `sequence.remove_range(2, 4)?`.
    pub fn remove_range(&mut self, first: usize, last: usize) -> Result<(), Failure> {
        if first == 0 || last > self.len() || first > last {
            return Err(out_of_range(
                "Remove",
                first,
                &format!("1 <= first <= last <= {} (last {last})", self.len()),
            ));
        }
        let node = self
            .topology
            // SAFETY: the range was checked, and this sequence exclusively owns every linked
            // node and its cache.
            .edit(|base| unsafe { base.prepare_range_removal(first, last) })?;
        self.destroy_detached_span(node, last - first + 1);
        Ok(())
    }
    fn destroy_detached_span(&mut self, mut node: *mut SeqNode, count: usize) {
        if self.clear_nodes.is_none() {
            let mut chain = SequenceRetiredChain {
                node,
                remaining: count,
                item: std::marker::PhantomData::<T>,
            };
            chain.clear();
            return;
        }
        // During controlled clear, withdraw all liveness entries before running callbacks.
        let mut detached = Vec::with_capacity(count);
        for _ in 0..count {
            // SAFETY: the span was detached from this owning chain. Its owner stays
            // live while the saved next pointer is read.
            let owner = unsafe { self.take_unlinked(node) };
            // SAFETY: this is the held allocation's actual next field.
            node = unsafe { (*owner.link).next() };
            detached.push(owner);
        }
        for owner in detached {
            drop(owner);
        }
    }
    /// Removes current and advances it with previous retained; `sequence.remove_cursor(&mut cursor)?`.
    pub fn remove_cursor(&mut self, cursor: &mut SequenceCursor) -> Result<(), Failure> {
        drop(self.detach_cursor(cursor)?);
        Ok(())
    }
    /// Unlink current and advance before returning its owner; `let retired=sequence.detach_cursor(&mut cursor)?`.
    /// Drop the owner after releasing any surrounding field guard, so destructors can reenter.
    pub fn detach_cursor(
        &mut self,
        cursor: &mut SequenceCursor,
    ) -> Result<Option<SequenceNodeRetirement<T>>, Failure> {
        if !cursor.more() {
            return Ok(None);
        }
        let node = self
            .owned_node(*cursor)
            .ok_or_else(|| no_such_node("Remove", *cursor))?;
        Ok(Some(self.detach_owned_cursor(cursor, node)))
    }
    fn detach_owned_cursor(
        &mut self,
        cursor: &mut SequenceCursor,
        node: *mut SeqNode,
    ) -> SequenceNodeRetirement<T> {
        let mut raw = BaseSequenceRawIterator {
            current: node,
            previous: std::ptr::null_mut(),
        };
        let previous = self
            .previous_link(*cursor)
            .filter(|previous| *previous != node);
        self.prepare_owned_cursor(&mut raw);
        cursor.raw.current = raw.current;
        cursor.record_detachment(self, previous);
        SequenceNodeRetirement {
            // SAFETY: this node was validated and unlinked before transferring its ownership.
            owner: unsafe { self.take_unlinked(node) },
        }
    }
    pub(crate) fn prepare_owned_cursor(&mut self, raw: &mut BaseSequenceRawIterator) {
        self.topology
            // SAFETY: callers pass a cursor validated in this sequence, which exclusively owns
            // every linked node.
            .edit(|base| unsafe { base.prepare_cursor_removal(raw) });
    }
    /// Frees items in link order and optionally replaces the allocator; `sequence.clear(Some(arena))`.
    pub fn clear(&mut self, allocator: Option<BaseAllocator>) {
        if self.topology.clear_scope().is_some() {
            self.reset_interrupted_clear(allocator);
            return;
        }
        self.clear_uninterrupted(allocator);
    }
    fn clear_uninterrupted(&mut self, allocator: Option<BaseAllocator>) {
        let mut chain = SequenceRetiredChain::<T> {
            node: self.topology.endpoints().0,
            remaining: self.len(),
            item: std::marker::PhantomData,
        };
        let reset = ClearMetadata(&mut self.topology);
        chain.clear();
        drop(reset);
        if let Some(allocator) = allocator {
            self.topology.edit(|base| base.allocator = allocator);
            self.common_nodes = false;
        }
    }
    fn reset_interrupted_clear(&mut self, allocator: Option<BaseAllocator>) {
        // A discarded partial traversal contains freed prefixes. Reset before cleanup;
        // re-clearing those pointers is undefined, so safe Rust never dereferences them.
        self.reset_clear_metadata();
        drop(self.clear_nodes.take());
        if let Some(allocator) = allocator {
            self.topology.edit(|base| base.allocator = allocator);
            self.common_nodes = false;
        }
    }
}
impl<T> Drop for Sequence<T> {
    fn drop(&mut self) {
        self.clear(None);
    }
}

// On unwind, the metadata guard drops first. The remaining chain still owns its nodes.
struct SequenceRetiredChain<T> {
    node: *mut SeqNode,
    remaining: usize,
    item: std::marker::PhantomData<T>,
}
impl<T> SequenceRetiredChain<T> {
    fn clear(&mut self) {
        while self.remaining != 0 {
            let node = self.node;
            // SAFETY: the guard owns only the announced contiguous span. The link after the
            // last node may still point into the original live sequence and is never used.
            self.node = unsafe { (*node).next() };
            self.remaining -= 1;
            // SAFETY: the current node is uniquely detached and drop advances the guard first.
            drop(unsafe { SequenceNodeOwner::<T>::from_unlinked(node) });
        }
    }
}
impl<T> Drop for SequenceRetiredChain<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
