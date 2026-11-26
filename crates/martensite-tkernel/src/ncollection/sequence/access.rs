use super::{NodeRevision, SeqNode, Sequence, SequenceBaseAccess, SequenceCursor, owner};
use crate::standard::Failure;

impl<T> Sequence<T> {
    /// Reads coherent base metadata for inspection and raw cursor construction.
    /// Example: `let base = sequence.base_access(); assert_eq!(base.size(), 2)`.
    pub fn base_access(&self) -> SequenceBaseAccess<'_> {
        self.topology.access()
    }
    /// Creates a read-only handle to actual metadata; `sequence.observer().snapshot().size`.
    pub fn observer(&self) -> super::SequenceObserver {
        self.topology.observer()
    }
    pub(super) fn owned_node(&self, cursor: SequenceCursor) -> Option<*mut SeqNode> {
        self.resolve_link(cursor.raw.current, cursor.revision)
    }
    pub(super) fn previous_link(&self, cursor: SequenceCursor) -> Option<*mut SeqNode> {
        self.resolve_link(cursor.raw.previous, cursor.revision)
    }
    fn resolve_link(&self, target: *mut SeqNode, revision: NodeRevision) -> Option<*mut SeqNode> {
        if target.is_null() {
            return None;
        }
        if let Some(registry) = &self.clear_nodes {
            return registry.live_link(target);
        }
        if revision == self.topology.revision() {
            return Some(target);
        }
        let mut node = self.topology.endpoints().0;
        while !node.is_null() {
            if node == target {
                return Some(node);
            }
            // SAFETY: outside controlled clear, shared links retain the entire live chain.
            node = unsafe { (*node).next() };
        }
        None
    }
    #[inline]
    pub(super) fn checked_node(
        &self,
        member: &str,
        index: usize,
        cache: bool,
    ) -> Result<*mut SeqNode, Failure> {
        self.offset(member, index)?;
        Ok(self.topology.find(index, cache))
    }
    #[inline]
    pub(super) fn read_indexed(&self, index: usize) -> Result<&T, Failure> {
        let node = self.checked_node("Value", index, true)?;
        // SAFETY: this shared sequence borrow keeps its owned node live and immutable.
        Ok(unsafe { owner::item_ref(node) })
    }
    #[inline]
    pub(super) fn write_indexed(&mut self, index: usize) -> Result<&mut T, Failure> {
        let node = self.checked_node("ChangeValue", index, true)?;
        // SAFETY: exclusive sequence borrowing excludes all other item references.
        Ok(unsafe { owner::item_mut(node) })
    }
    fn endpoint(&self, first: bool) -> *mut SeqNode {
        let (head, tail) = self.topology.endpoints();
        if first { head } else { tail }
    }
    fn live_endpoint(&self, first: bool) -> Option<*mut SeqNode> {
        let node = self.endpoint(first);
        if node.is_null() {
            return None;
        }
        if let Some(registry) = &self.clear_nodes {
            return registry.live_link(node);
        }
        Some(node)
    }
    pub(super) fn endpoint_value(&self, first: bool) -> Option<&T> {
        let node = self.live_endpoint(first)?;
        // SAFETY: owned_node only returns a live node owned by this sequence, and `&self`
        // bounds the shared borrow.
        Some(unsafe { owner::item_ref(node) })
    }
    pub(super) fn endpoint_mut(&mut self, first: bool) -> Option<&mut T> {
        let node = self.live_endpoint(first)?;
        // SAFETY: owned_node only returns a live node owned by this sequence, and `&mut self`
        // bounds the exclusive borrow.
        Some(unsafe { owner::item_mut(node) })
    }
}
