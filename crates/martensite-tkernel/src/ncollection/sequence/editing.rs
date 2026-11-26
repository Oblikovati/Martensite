//! Typed edits delegate topology and cache transitions to the canonical base.
use super::{Sequence, SequenceCursor, SequenceNodeOwner, no_such_node, out_of_range, owner};
use crate::standard::Failure;
mod removal;
mod transfer;

impl<T> Sequence<T> {
    #[inline]
    fn construct_node(&self, construct: impl FnOnce() -> T) -> SequenceNodeOwner<T> {
        if self.common_nodes {
            return SequenceNodeOwner::construct_common(construct);
        }
        SequenceNodeOwner::construct(self.allocator(), construct)
    }

    #[inline]
    pub(super) fn own_link(&mut self, owner: SequenceNodeOwner<T>) -> *mut super::SeqNode {
        let link = owner.into_link();
        if let Some(registry) = &mut self.clear_nodes {
            registry.register(link);
        }
        link
    }

    pub(super) unsafe fn take_unlinked(
        &mut self,
        link: *mut super::SeqNode,
    ) -> SequenceNodeOwner<T> {
        let link = self.clear_nodes.as_mut().map_or(link, |registry| {
            registry
                .release_link(link)
                .expect("detached clear link belongs to this sequence")
        });
        // SAFETY: caller transfers a live unlinked node from this owning sequence.
        unsafe { SequenceNodeOwner::from_unlinked(link) }
    }

    /// Appends an independently allocated item; `*sequence.append(10) == 10`.
    #[inline]
    pub fn append(&mut self, value: T) -> &mut T {
        self.insert_node(self.len(), value)
    }
    /// Prepends an independently allocated item; `*sequence.prepend(10) == 10`.
    pub fn prepend(&mut self, value: T) -> &mut T {
        self.insert_node(0, value)
    }
    /// Inserts before a one-based position; `sequence.insert_before(1, 10)?`.
    pub fn insert_before(&mut self, index: usize, value: T) -> Result<&mut T, Failure> {
        if index == 0 {
            return Err(out_of_range("InsertBefore", index, "an index >= 1"));
        }
        self.insert_after(index - 1, value)
    }
    /// Inserts after an index, with zero meaning prepend; `sequence.insert_after(0, 10)?`.
    pub fn insert_after(&mut self, index: usize, value: T) -> Result<&mut T, Failure> {
        if index > self.len() {
            return Err(out_of_range(
                "InsertAfter",
                index,
                &format!("0..={}", self.len()),
            ));
        }
        Ok(self.insert_node(index, value))
    }
    /// Inserts after the cursor's node; an end cursor prepends; `sequence.insert_after_cursor(&cursor, 10)?`.
    pub fn insert_after_cursor(
        &mut self,
        cursor: &SequenceCursor,
        value: T,
    ) -> Result<&mut T, Failure> {
        self.insert_cursor_with(cursor, || value)
    }
    /// Swaps two nodes using the cache transitions; `sequence.exchange(2, 4)?`.
    pub fn exchange(&mut self, first: usize, second: usize) -> Result<(), Failure> {
        self.offset("Exchange", first)?;
        self.offset("Exchange", second)?;
        self.topology
            // SAFETY: both offsets were checked, and this sequence exclusively owns every
            // linked node.
            .edit_preserving_positions(|base| unsafe { base.exchange(first, second) })
    }
    /// Reverses links and adjusts the cached index; `sequence.reverse()`.
    pub fn reverse(&mut self) {
        // SAFETY: this sequence exclusively owns every linked node, and its links match its
        // size.
        self.topology
            .edit_preserving_positions(|base| unsafe { base.reverse() });
    }
    pub(super) fn require_cursor_membership(
        &self,
        member: &str,
        cursor: &SequenceCursor,
    ) -> Result<(), Failure> {
        if !cursor.more() {
            return Ok(());
        }
        self.owned_node(*cursor)
            .ok_or_else(|| no_such_node(member, *cursor))?;
        Ok(())
    }
    pub(super) fn insert_node(&mut self, offset: usize, value: T) -> &mut T {
        self.insert_node_with(offset, || value)
    }
    pub(super) fn insert_node_with(
        &mut self,
        offset: usize,
        construct: impl FnOnce() -> T,
    ) -> &mut T {
        let node = self.construct_node(construct);
        let link = self.own_link(node);
        self.link_at(offset, link);
        // SAFETY: `link` is live and owned by this sequence, and `&mut self` bounds the
        // exclusive item borrow.
        unsafe { owner::item_mut(link) }
    }
    fn link_at(&mut self, offset: usize, link: *mut super::SeqNode) {
        if offset == self.len() && self.clear_nodes.is_none() {
            // SAFETY: the node is fresh, and the normal typed topology always owns live links.
            unsafe { self.topology.append_live_node(link) };
            return;
        }
        // SAFETY: offset was checked before construction; the clear scope checks retired links.
        self.topology
            .edit_preserving_positions(|base| unsafe { base.insert_after(offset, link) })
            .expect("checked insertion index");
    }
    pub(super) fn insert_cursor_with(
        &mut self,
        cursor: &SequenceCursor,
        construct: impl FnOnce() -> T,
    ) -> Result<&mut T, Failure> {
        self.require_cursor_membership("InsertAfter", cursor)?;
        let current = self.owned_node(*cursor).unwrap_or(std::ptr::null_mut());
        let raw = super::BaseSequenceRawIterator {
            current,
            previous: std::ptr::null_mut(),
        };
        let node = self.construct_node(construct);
        let link = self.own_link(node);
        self.topology
            // SAFETY: `link` is a fresh detached node owned by this sequence, and `raw` names a
            // node of this sequence or its end.
            .edit_preserving_positions(|base| unsafe { base.insert_after_cursor(&raw, link) });
        // SAFETY: `link` is live and owned by this sequence, and `&mut self` bounds the
        // exclusive item borrow.
        Ok(unsafe { owner::item_mut(link) })
    }
}
impl<T: Clone> Sequence<T> {
    /// Allocates before copying the borrowed item; `sequence.append_ref(&10)`.
    pub fn append_ref(&mut self, value: &T) -> &mut T {
        self.insert_node_with(self.len(), || value.clone())
    }
}
