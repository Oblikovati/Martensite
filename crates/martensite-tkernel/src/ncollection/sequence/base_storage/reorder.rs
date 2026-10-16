use super::{BaseSequence, SeqNode};
use crate::standard::Failure;

impl BaseSequence {
    /// Reverses node links while retaining the cached node identity; `unsafe { base.reverse() }`.
    /// # Safety
    /// All retained nodes must be live, exclusively owned, and linked consistently with size.
    pub unsafe fn reverse(&mut self) {
        let mut node = self.first;
        while !node.is_null() {
            // SAFETY: the caller guarantees every node of this chain is live and exclusively
            // owned, and `node` is non-null here.
            let next = unsafe { self.guarded_link(node).next() };
            // SAFETY: `node` is a live non-null node of this exclusively owned chain.
            let previous = unsafe { self.guarded_link(node).previous() };
            // SAFETY: `node` is a live node of this exclusively owned chain, so rewriting its
            // links aliases nothing.
            unsafe {
                self.guarded_link_mut(node).set_next(previous);
                self.guarded_link_mut(node).set_previous(next);
            }
            node = next;
        }
        std::mem::swap(&mut self.first, &mut self.last);
        if self.size != 0 {
            self.current_index = self.size + 1 - self.current_index;
        }
    }
    /// Exchanges node identities at two checked indices; `unsafe { base.exchange(1, 3)? }`.
    /// # Safety
    /// Nodes, links and cached metadata must be live, consistent, and exclusively managed here.
    pub unsafe fn exchange(&mut self, first: usize, second: usize) -> Result<(), Failure> {
        self.check_index(first)?;
        self.check_index(second)?;
        if first == second {
            return Ok(());
        }
        let (first, second) = (first.min(second), first.max(second));
        // SAFETY: check_index proved both indices are in 1..=size, and the caller guarantees
        // the links and cache are valid.
        let left = unsafe { self.find(first) };
        // SAFETY: check_index proved both indices are in 1..=size, and the caller guarantees
        // the links and cache are valid.
        let right = unsafe { self.find(second) };
        // SAFETY: `left` and `right` are distinct live nodes of this base with `left` before
        // `right`.
        unsafe { self.swap_nodes(left, right) };
        if self.current_index == first {
            self.current = right;
        }
        if self.current_index == second {
            self.current = left;
        }
        Ok(())
    }
    /// Transfers the indexed suffix without clearing or rebinding the destination; `unsafe { base.split(2, &mut tail)? }`.
    /// # Safety
    /// Source nodes/cache must be live and exclusive. Destination must be empty to retain node ownership.
    pub unsafe fn split(&mut self, index: usize, destination: &mut Self) -> Result<(), Failure> {
        self.check_index(index)?;
        // SAFETY: check_index proved `index` is in 1..=size, and the caller guarantees the
        // links and cache are valid.
        let node = unsafe { self.find(index) };
        destination.first = node;
        destination.last = self.last;
        destination.size = self.size - index + 1;
        destination.cache_first();
        // SAFETY: `node` is the live node at `index` of this base.
        self.last = unsafe { self.guarded_link(node).previous() };
        // SAFETY: `self.last` is null or the live node before the split point, and the suffix
        // now belongs to `destination`.
        unsafe {
            self.truncate_prefix(index);
            self.guarded_link_mut(node)
                .set_previous(std::ptr::null_mut());
        }
        Ok(())
    }
    /// Represents self-split's checked `Standard_DomainError` without aliased mutable borrows; `base.split_self(1)`.
    pub fn split_self(&mut self, index: usize) -> Result<(), Failure> {
        self.check_index(index)?;
        Err(Failure::with_type(
            "Standard_DomainError",
            &format!("BaseSequence self split at {index}; expected a distinct destination"),
            "",
        ))
    }
    unsafe fn truncate_prefix(&mut self, index: usize) {
        if self.last.is_null() {
            self.nullify();
            return;
        }
        // SAFETY: `self.last` is non-null here and is a live node of this base.
        unsafe {
            self.guarded_link_mut(self.last)
                .set_next(std::ptr::null_mut())
        };
        self.size = index - 1;
        if self.current_index >= index {
            self.cache_first();
        }
    }
    unsafe fn swap_nodes(&mut self, left: *mut SeqNode, right: *mut SeqNode) {
        // SAFETY: callers pass `left`, a live node of this base.
        let before = unsafe { self.guarded_link(left).previous() };
        // SAFETY: callers pass `right`, a live node of this base.
        let after = unsafe { self.guarded_link(right).next() };
        // SAFETY: callers pass `left`, a live node of this base.
        let left_next = unsafe { self.guarded_link(left).next() };
        // SAFETY: callers pass `right`, a live node of this base.
        let right_previous = unsafe { self.guarded_link(right).previous() };
        // SAFETY: `before` and `after` are null or live neighbours of the distinct live nodes
        // `left` and `right`.
        unsafe { self.swap_outer_links(before, after, left, right) };
        // SAFETY: `left_next` and `right_previous` are live nodes of this base (or `right` and
        // `left` themselves when adjacent).
        unsafe { self.swap_inner_links(left, right, left_next, right_previous) };
        // SAFETY: `left` and `right` are live nodes of this base, and `before`/`after` are null
        // or live.
        unsafe {
            self.guarded_link_mut(right).set_previous(before);
            self.guarded_link_mut(left).set_next(after);
        }
    }
    unsafe fn swap_inner_links(
        &mut self,
        left: *mut SeqNode,
        right: *mut SeqNode,
        left_next: *mut SeqNode,
        right_previous: *mut SeqNode,
    ) {
        if left_next == right {
            // SAFETY: `left` and `right` are adjacent live nodes of this base.
            unsafe {
                self.guarded_link_mut(right).set_next(left);
                self.guarded_link_mut(left).set_previous(right);
            }
            return;
        }
        // SAFETY: `left_next` and `right_previous` are live nodes between the separated `left`
        // and `right`.
        unsafe { self.swap_separated_inner(left, right, left_next, right_previous) };
    }
    unsafe fn swap_separated_inner(
        &mut self,
        left: *mut SeqNode,
        right: *mut SeqNode,
        left_next: *mut SeqNode,
        right_previous: *mut SeqNode,
    ) {
        // SAFETY: callers pass live nodes `left`, `right`, `left_next` and `right_previous` of
        // this base.
        unsafe {
            self.guarded_link_mut(left_next).set_previous(right);
            self.guarded_link_mut(right_previous).set_next(left);
            self.guarded_link_mut(right).set_next(left_next);
            self.guarded_link_mut(left).set_previous(right_previous);
        }
    }
    unsafe fn swap_outer_links(
        &mut self,
        before: *mut SeqNode,
        after: *mut SeqNode,
        left: *mut SeqNode,
        right: *mut SeqNode,
    ) {
        if before.is_null() {
            self.first = right;
        } else {
            // SAFETY: `before` is non-null here and is a live node of this base.
            unsafe { self.guarded_link_mut(before).set_next(right) };
        }
        if after.is_null() {
            self.last = left;
        } else {
            // SAFETY: `after` is non-null here and is a live node of this base.
            unsafe { self.guarded_link_mut(after).set_previous(left) };
        }
    }
}
