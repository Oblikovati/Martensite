//! Destructor observations occur after detachment and before final metadata updates.
use super::owner::ListNodeOwner;
use super::{
    BaseListRawIterator, BaseListStorage, List, ListCursor, ListLinkNode, topology::ListTopology,
};
use crate::standard::Failure;
use std::{
    marker::PhantomData,
    panic::{AssertUnwindSafe, catch_unwind},
};
struct ListClearMetadata<'a>(&'a mut ListTopology);
impl Drop for ListClearMetadata<'_> {
    fn drop(&mut self) {
        // Reset before remaining owners unwind, so later observations never traverse freed nodes.
        self.0.edit(BaseListStorage::reset_links);
    }
}
impl<T> List<T> {
    /// Removes current and advances to its successor; `list.remove(&mut cursor)?`.
    pub fn remove(&mut self, cursor: &mut ListCursor) -> Result<(), Failure> {
        let raw = self.locate_cursor(*cursor, "Remove")?;
        let (next, outcome) = self.remove_in_phases(raw);
        // Membership repair must not rewrite the safe cursor's remembered predecessor.
        cursor.raw.current = next;
        if cursor.raw.previous == raw.previous {
            cursor.raw.previous = raw.previous;
            cursor.revision = self.topology.revision();
        }
        resume_drop_outcome(outcome);
        Ok(())
    }
    pub(super) fn remove_in_phases(
        &mut self,
        raw: BaseListRawIterator,
    ) -> (*mut ListLinkNode, std::thread::Result<()>) {
        let phase = self
            .topology
            // SAFETY: callers pass a cursor that locate_cursor validated in this exclusively
            // borrowed list, so it names a live node and its actual predecessor.
            .edit(|base| unsafe { base.detach_removal(raw) });
        // No lock or metadata borrow survives this boundary into arbitrary user destruction.
        // SAFETY: detach_removal transferred exactly one live node from this List<T>.
        let owner = unsafe { ListNodeOwner::<T>::from_unlinked(phase.node) };
        let outcome = catch_unwind(AssertUnwindSafe(|| drop(owner)));
        self.topology.edit(|base| base.finish_removal(phase));
        (phase.next, outcome)
    }
    pub(super) fn clear_owned_chain(&mut self) {
        let mut chain = ListRetiredChain::<T> {
            node: self.base_storage().first,
            item: PhantomData,
        };
        let reset = ListClearMetadata(&mut self.topology);
        // Clearing keeps the original metadata visible until every destructor has run.
        chain.clear();
        drop(reset);
    }
}

// On unwinding, the later-declared metadata guard resets the list before this guard
// releases the unvisited part of the chain.
struct ListRetiredChain<T> {
    node: *mut ListLinkNode,
    item: PhantomData<T>,
}
impl<T> ListRetiredChain<T> {
    fn clear(&mut self) {
        while !self.node.is_null() {
            let node = self.node;
            // SAFETY: this guard uniquely owns the detached chain. Advance before Drop so
            // an unwinding payload cannot cause a second retirement of the same node.
            unsafe {
                self.node = (*node).next();
                drop(ListNodeOwner::<T>::from_unlinked(node));
            }
        }
    }
}
impl<T> Drop for ListRetiredChain<T> {
    fn drop(&mut self) {
        self.clear();
    }
}
pub(super) fn resume_drop_outcome(outcome: std::thread::Result<()>) {
    if let Err(payload) = outcome {
        std::panic::resume_unwind(payload);
    }
}
