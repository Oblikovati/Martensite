//! An indexed lookup borrows structural fields without borrowing the inspection cache.
use super::{SeqNode, SequenceNodeLiveness};

pub(in crate::ncollection::sequence) struct SequenceSearch<'a> {
    pub(in crate::ncollection::sequence) first: *mut SeqNode,
    pub(in crate::ncollection::sequence) last: *mut SeqNode,
    pub(in crate::ncollection::sequence) size: usize,
    pub(in crate::ncollection::sequence) clear_scope: Option<&'a SequenceNodeLiveness>,
}

impl SequenceSearch<'_> {
    #[inline]
    pub(in crate::ncollection::sequence) unsafe fn find(
        &self,
        index: usize,
        current: *mut SeqNode,
        cached_index: usize,
    ) -> *mut SeqNode {
        let (mut node, start) = self.search_origin(index, current, cached_index);
        for _ in index..start {
            // SAFETY: find's caller guarantees every link is live, and the walk stays within
            // the `index` steps from a live origin.
            node = unsafe { (*self.resolve_live_link(node)).previous() };
        }
        for _ in start..index {
            // SAFETY: find's caller guarantees every link is live, and the walk stays within
            // the `index` steps from a live origin.
            node = unsafe { (*self.resolve_live_link(node)).next() };
        }
        self.resolve_live_link(node)
    }
    fn search_origin(
        &self,
        index: usize,
        current: *mut SeqNode,
        cached_index: usize,
    ) -> (*mut SeqNode, usize) {
        if index <= cached_index {
            return if index < cached_index / 2 {
                (self.first, 1)
            } else {
                (current, cached_index)
            };
        }
        if index < cached_index.wrapping_add(self.size) / 2 {
            return (current, cached_index);
        }
        (self.last, self.size)
    }
    #[inline]
    pub(in crate::ncollection::sequence) fn resolve_live_link(
        &self,
        node: *mut SeqNode,
    ) -> *mut SeqNode {
        self.clear_scope
            .map_or(node, |scope| scope.resolve_live_link(node, self.size))
    }
}
