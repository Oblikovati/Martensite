use super::{Sequence, SequenceCursor, out_of_range};
use crate::standard::Failure;
impl<T> Sequence<T> {
    /// Constructs an appended item; `sequence.emplace_append(|| Pair(1,2.0))`.
    pub fn emplace_append(&mut self, construct: impl FnOnce() -> T) -> &mut T {
        self.insert_node_with(self.len(), construct)
    }
    /// Constructs a prepended item; `sequence.emplace_prepend(|| Pair(1,2.0))`.
    pub fn emplace_prepend(&mut self, construct: impl FnOnce() -> T) -> &mut T {
        self.insert_node_with(0, construct)
    }
    /// Constructs after a one-based index (0 prepends), checking the index before
    /// constructing; `sequence.emplace_after(1, || Pair(1,2.0))?`.
    pub fn emplace_after(
        &mut self,
        index: usize,
        construct: impl FnOnce() -> T,
    ) -> Result<&mut T, Failure> {
        if index > self.len() {
            let expected = format!("0..={}", self.len());
            return Err(out_of_range("EmplaceAfter", index, &expected));
        }
        Ok(self.insert_node_with(index, construct))
    }
    /// Constructs before a one-based index; `sequence.emplace_before(2, || Pair(1,2.0))?`.
    pub fn emplace_before(
        &mut self,
        index: usize,
        construct: impl FnOnce() -> T,
    ) -> Result<&mut T, Failure> {
        if index == 0 {
            return Err(out_of_range("EmplaceBefore", index, "an index >= 1"));
        }
        self.emplace_after(index - 1, construct)
    }
    /// Constructs after a node cursor; an end cursor prepends;
    /// `sequence.emplace_after_cursor(&cursor, || Pair(1,2.0))?`.
    pub fn emplace_after_cursor(
        &mut self,
        cursor: &SequenceCursor,
        construct: impl FnOnce() -> T,
    ) -> Result<&mut T, Failure> {
        self.insert_cursor_with(cursor, construct)
    }
}
impl<T: Clone> Sequence<T> {
    /// Appends and empties a source sequence; `sequence.append_sequence(&mut source)`.
    pub fn append_sequence(&mut self, source: &mut Self) {
        self.splice_sequence(self.len(), source);
    }
    /// Prepends and empties a source, including empty sources; `sequence.prepend_sequence(&mut source)`.
    pub fn prepend_sequence(&mut self, source: &mut Self) {
        self.splice_sequence(0, source);
    }
    /// Splices after a one-based index, copying when allocators differ. An empty
    /// source returns before the index is checked; `sequence.insert_sequence_after(1,&mut source)?`.
    /// Distinct allocators keep the unsigned MAX+1 prepend behavior. Other out-of-range
    /// indices remain checked errors instead of following invalid links.
    pub fn insert_sequence_after(
        &mut self,
        index: usize,
        source: &mut Self,
    ) -> Result<(), Failure> {
        if source.is_empty() {
            return Ok(());
        }
        let index = native_sequence_copy_offset(index, self.allocator() == source.allocator());
        if index > self.len() {
            let expected = format!("0..={}", self.len());
            return Err(out_of_range("InsertAfter", index, &expected));
        }
        self.splice_sequence(index, source);
        Ok(())
    }
    /// Splices before a one-based index; index 0 is OutOfRange even for an empty source;
    /// `sequence.insert_sequence_before(1, &mut source)?`.
    pub fn insert_sequence_before(
        &mut self,
        index: usize,
        source: &mut Self,
    ) -> Result<(), Failure> {
        if index == 0 {
            return Err(out_of_range("InsertBefore", index, "an index >= 1"));
        }
        self.insert_sequence_after(index - 1, source)
    }
    /// Moves (same allocator) or copies and clears (other allocator) `source` after `offset`.
    fn splice_sequence(&mut self, offset: usize, source: &mut Self) {
        if self.allocator() == source.allocator() {
            if !source.is_empty() {
                self.check_splice_transfer(source);
            }
            let transferred = self.transfer_clear_links(source);
            self.topology
                // SAFETY: both sequences are exclusively borrowed with the same allocator
                // (checked above), their chains are disjoint and live, and the donor's owners
                // move to `self`.
                .edit_pair(&mut source.topology, |base, donor| unsafe {
                    base.insert_sequence_after(offset, donor)
                })
                .expect("checked sequence insertion index");
            self.finish_clear_transfer(source, transferred);
            return;
        }
        // Each successful copy is linked before the next copy begins.
        for (index, value) in source.iter().enumerate() {
            self.insert_node_with(offset + index, || value.clone());
        }
        source.clear(None);
    }
    fn check_splice_transfer(&self, source: &Self) {
        let result = self
            .check_clear_transfer("Splice")
            .and_then(|()| source.check_clear_transfer("Splice"));
        crate::standard::raise_native_failure(result);
    }
    fn transfer_clear_links(&self, source: &Self) -> Vec<*mut super::SeqNode> {
        let mut links = Vec::new();
        if self.clear_nodes.is_none() && source.clear_nodes.is_none() {
            return links;
        }
        let mut link = source.topology.endpoints().0;
        while !link.is_null() && links.len() < source.len() {
            link = source.topology.resolve_transfer_link(link);
            links.push(link);
            // SAFETY: checked above if the source's chain can retain retired links.
            link = unsafe { (*link).next() };
        }
        if !link.is_null() || links.len() != source.len() {
            let error = crate::standard::exceptions::program_error(
                "sequence splice encountered stale source topology during clear; expected every source node to remain live",
            );
            return crate::standard::raise_native_failure(Err(error));
        }
        links
    }
    fn finish_clear_transfer(&mut self, source: &mut Self, transferred: Vec<*mut super::SeqNode>) {
        for link in transferred {
            if let Some(nodes) = &mut source.clear_nodes {
                nodes.release_link(link);
            }
            if let Some(nodes) = &mut self.clear_nodes {
                nodes.register(link);
            }
        }
    }
}
impl<T: Clone> Clone for Sequence<T> {
    fn clone(&self) -> Self {
        Self::from_copied_with(self, T::clone)
    }
    fn clone_from(&mut self, source: &Self) {
        self.assign(source);
    }
}

// The index is incremented for a prepend only when allocator identities differ.
fn native_sequence_copy_offset(index: usize, same_allocator: bool) -> usize {
    if index == usize::MAX && !same_allocator {
        return 0;
    }
    index
}

#[cfg(test)]
#[path = "splice_max_tests.rs"]
mod splice_max_tests;
