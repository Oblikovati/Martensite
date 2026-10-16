mod self_splice;
use super::{BaseSequence, BaseSequenceRawIterator, SeqNode};
use crate::standard::Failure;

impl BaseSequence {
    /// Links one node at the tail; `unsafe { base.append_node(node) }`.
    /// # Safety
    /// Node must be nonnull, exclusively owned and initially detached; existing links must be valid.
    pub unsafe fn append_node(&mut self, node: *mut SeqNode) {
        if self.size == 0 {
            self.first = node;
            self.last = node;
            self.size = 1;
            self.cache_first();
            return;
        }
        let tail = self.last;
        // SAFETY: the base is nonempty, so `self.last` is a live node, and the caller
        // guarantees `node` is a detached node it owns exclusively.
        unsafe {
            self.guarded_link_mut(self.last).set_next(node);
            (*node).set_previous(tail);
            (*node).set_next(std::ptr::null_mut());
        }
        self.last = node;
        self.size += 1;
    }
    /// Links one node at the head; `unsafe { base.prepend_node(node) }`.
    /// # Safety
    /// Node must be nonnull, exclusively owned and initially detached; existing links must be valid.
    pub unsafe fn prepend_node(&mut self, node: *mut SeqNode) {
        if self.size == 0 {
            // SAFETY: this function's caller gives append_node's guarantees: a detached,
            // exclusive `node` and valid existing links.
            unsafe { self.append_node(node) };
            return;
        }
        let head = self.first;
        // SAFETY: the base is nonempty, so `self.first` is a live node, and the caller
        // guarantees `node` is a detached node it owns exclusively.
        unsafe {
            self.guarded_link_mut(self.first).set_previous(node);
            (*node).set_next(head);
            (*node).set_previous(std::ptr::null_mut());
        }
        self.first = node;
        self.size += 1;
        self.current_index += 1;
    }
    /// Transfers another base's topology without allocator checks or copying; `unsafe { base.append_sequence(&mut other) }`.
    /// # Safety
    /// Both bases must have disjoint live exclusive nodes and valid links; callers retain allocation provenance.
    pub unsafe fn append_sequence(&mut self, other: &mut Self) {
        if other.size == 0 {
            return;
        }
        if self.size == 0 {
            self.first = other.first;
            self.cache_first();
        } else {
            // SAFETY: the base is nonempty, so `self.last` is live, and the caller guarantees
            // `other` holds disjoint live nodes with valid links.
            unsafe {
                self.guarded_link_mut(self.last).set_next(other.first);
                other.guarded_link_mut(other.first).set_previous(self.last);
            }
        }
        self.last = other.last;
        self.size += other.size;
        other.nullify();
    }
    /// Transfers another base before the head, preserving this allocator; `unsafe { base.prepend_sequence(&mut other) }`.
    /// # Safety
    /// Both bases must have disjoint live exclusive nodes and valid links; callers retain allocation provenance.
    pub unsafe fn prepend_sequence(&mut self, other: &mut Self) {
        if other.size == 0 {
            return;
        }
        if self.size == 0 {
            // SAFETY: this function's caller gives append_sequence's guarantees: disjoint live
            // exclusive nodes with valid links.
            unsafe { self.append_sequence(other) };
            return;
        }
        // SAFETY: both bases are nonempty, so both endpoints are live, and the caller
        // guarantees the chains are disjoint and exclusive.
        unsafe {
            other.guarded_link_mut(other.last).set_next(self.first);
            self.guarded_link_mut(self.first).set_previous(other.last);
        }
        self.first = other.first;
        self.size += other.size;
        self.current_index += other.size;
        other.nullify();
    }
    /// Inserts after a raw iterator; null-current prepends; `unsafe { base.insert_after_cursor(&cursor, node) }`.
    /// # Safety
    /// Cursor must name this topology or its null end; node and existing links must be valid and exclusively owned.
    pub unsafe fn insert_after_cursor(
        &mut self,
        cursor: &BaseSequenceRawIterator,
        node: *mut SeqNode,
    ) {
        if cursor.current.is_null() {
            // SAFETY: this function's caller gives prepend_node's guarantees for `node` and the
            // existing links.
            unsafe { self.prepend_node(node) };
            return;
        }
        // SAFETY: the caller guarantees `cursor.current` is a live node of this base and `node`
        // is detached and exclusive.
        unsafe { self.link_after(cursor.current, node, node) };
        self.size += 1;
        self.cache_first();
    }
    /// Inserts after a one-based index, with zero meaning prepend; `unsafe { base.insert_after(0, node)? }`.
    /// # Safety
    /// Node must be detached and exclusive; all existing nodes and cache links must be valid.
    pub unsafe fn insert_after(&mut self, index: usize, node: *mut SeqNode) -> Result<(), Failure> {
        self.check_insertion(index)?;
        if index == 0 {
            // SAFETY: this function's caller gives prepend_node's guarantees for `node` and the
            // existing links.
            unsafe { self.prepend_node(node) };
            return Ok(());
        }
        // SAFETY: check_insertion proved `index` is in 1..=size here, and the caller guarantees
        // the links and cache are valid.
        let previous = unsafe { self.find(index) };
        // SAFETY: `previous` is the live node at `index`, and the caller guarantees `node` is
        // detached and exclusive.
        unsafe { self.link_after(previous, node, node) };
        self.size += 1;
        if index < self.current_index {
            self.current_index += 1;
        }
        Ok(())
    }
    /// Transfers another sequence after an index; validates even empty sources; `unsafe { base.insert_sequence_after(0, &mut other)? }`.
    /// # Safety
    /// Both bases must have disjoint live exclusive nodes and valid cache/link metadata.
    pub unsafe fn insert_sequence_after(
        &mut self,
        index: usize,
        other: &mut Self,
    ) -> Result<(), Failure> {
        self.check_insertion(index)?;
        if other.size == 0 {
            return Ok(());
        }
        if index == 0 {
            // SAFETY: this function's caller gives prepend_sequence's guarantees: disjoint live
            // exclusive nodes with valid links.
            unsafe { self.prepend_sequence(other) };
            return Ok(());
        }
        // SAFETY: check_insertion proved `index` is in 1..=size, and the caller guarantees both
        // bases' nodes are live, disjoint and exclusive.
        unsafe { self.transfer_after(index, other) };
        Ok(())
    }
    unsafe fn transfer_after(&mut self, index: usize, other: &mut Self) {
        // SAFETY: transfer_after's callers pass an `index` checked to be in 1..=size, and the
        // links of this base are valid.
        let previous = unsafe { self.find(index) };
        // SAFETY: `previous` is a live node of this base, and `other`'s endpoints are live
        // nodes of a disjoint chain.
        unsafe { self.link_after(previous, other.first, other.last) };
        self.size += other.size;
        if index < self.current_index {
            self.current_index += other.size;
        }
        other.nullify();
    }
    unsafe fn link_after(&mut self, before: *mut SeqNode, first: *mut SeqNode, last: *mut SeqNode) {
        // SAFETY: callers pass a live node `before` of this base.
        let after = unsafe { self.guarded_link(before).next() };
        // SAFETY: `first` and `last` are live endpoints of a detached chain and `before` is a
        // live node of this base, all exclusively managed here.
        unsafe {
            (*first).set_previous(before);
            (*last).set_next(after);
            self.guarded_link_mut(before).set_next(first);
        }
        if after.is_null() {
            self.last = last;
        } else {
            // SAFETY: `after` is non-null here and is the live successor of `before` in this
            // base.
            unsafe { self.guarded_link_mut(after).set_previous(last) };
        }
    }
    fn check_insertion(&self, index: usize) -> Result<(), Failure> {
        if index > self.size {
            return Err(Failure::out_of_range(&format!(
                "BaseSequence insertion {index}; expected 0..={}",
                self.size
            )));
        }
        Ok(())
    }
}
