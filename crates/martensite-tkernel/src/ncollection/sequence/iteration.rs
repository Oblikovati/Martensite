use super::{Sequence, SequenceCursor, SequenceIter, SequenceIterMut, owner};
use std::marker::PhantomData;

impl SequenceCursor {
    /// Positions at the first item or past the last, remembering the final node; `Self::new(&sequence, false)`.
    pub fn new<T>(sequence: &Sequence<T>, is_start: bool) -> Self {
        Self {
            raw: sequence.topology.cursor(is_start),
            revision: sequence.topology.revision(),
        }
    }
    /// Resets either endpoint; `cursor.init(&sequence, true)`.
    pub fn init<T>(&mut self, sequence: &Sequence<T>, is_start: bool) {
        *self = Self::new(sequence, is_start);
    }
    /// Reports an unexhausted position; `cursor.more()`.
    pub fn more(self) -> bool {
        !self.raw.current.is_null()
    }
    /// Borrows a currently owned node's item; `cursor.value(&sequence)`.
    pub fn value<T>(self, sequence: &Sequence<T>) -> Option<&T> {
        let node = sequence.owned_node(self)?;
        // SAFETY: owned_node only returns a live node owned by `sequence`, and `&sequence`
        // bounds the shared borrow.
        Some(unsafe { owner::item_ref(node) })
    }
    /// Borrows a positioned item exclusively; `*cursor.change_value(&mut sequence)? = 3`.
    pub fn change_value<T>(self, sequence: &mut Sequence<T>) -> Option<&mut T> {
        let node = sequence.owned_node(self)?;
        // SAFETY: owned_node only returns a live node owned by `sequence`, and `&mut sequence`
        // bounds the exclusive borrow.
        Some(unsafe { owner::item_mut(node) })
    }
    /// Follows the actual next link; exhausted cursors remain unchanged; `cursor.next(&sequence)`.
    pub fn next<T>(&mut self, sequence: &Sequence<T>) {
        if !self.more() {
            return;
        }
        if let Some(node) = sequence.owned_node(*self) {
            self.step_to_next(sequence, node);
        } else {
            self.raw.previous = self.raw.current;
            self.raw.current = std::ptr::null_mut();
        }
    }
    /// Follows the remembered previous node; `cursor.previous(&sequence)`.
    pub fn previous<T>(&mut self, sequence: &Sequence<T>) {
        let previous = sequence.previous_link(*self);
        self.raw.current = self.raw.previous;
        if let Some(node) = previous {
            self.step_to_previous(sequence, node);
        } else {
            self.raw.previous = std::ptr::null_mut();
        }
    }
    pub(super) fn step_to_next<T>(&mut self, sequence: &Sequence<T>, node: *mut super::SeqNode) {
        // SAFETY: callers resolved node through this shared Sequence borrow.
        self.raw.current = unsafe { (*node).next() };
        self.raw.previous = node;
        self.revision = sequence.topology.revision();
    }
    pub(super) fn step_to_previous<T>(
        &mut self,
        sequence: &Sequence<T>,
        node: *mut super::SeqNode,
    ) {
        // SAFETY: callers resolved node through this shared Sequence borrow.
        self.raw.previous = unsafe { (*node).previous() };
        self.raw.current = node;
        self.revision = sequence.topology.revision();
    }
    pub(super) fn record_detachment<T>(
        &mut self,
        sequence: &Sequence<T>,
        predecessor: Option<*mut super::SeqNode>,
    ) {
        // A removed and reused remembered address may name the removed allocation itself.
        // The caller supplies only predecessors that remain attached after removal.
        if let Some(predecessor) = predecessor {
            self.raw.previous = predecessor;
            self.revision = sequence.topology.revision();
        } else if self.raw.previous.is_null() {
            self.revision = sequence.topology.revision();
        }
    }
}
pub(super) fn pointer_address(node: *mut super::SeqNode) -> Option<usize> {
    (!node.is_null()).then_some(node as usize)
}
impl PartialEq for SequenceCursor {
    fn eq(&self, other: &Self) -> bool {
        self.raw.current == other.raw.current
    }
}
impl Eq for SequenceCursor {}
impl<T> Sequence<T> {
    pub(super) fn linked_iter(&self) -> SequenceIter<'_, T> {
        let (front, back) = self.topology.endpoints();
        SequenceIter {
            clear_nodes: self.clear_nodes.as_ref(),
            front,
            back,
            remaining: self.len(),
            item: PhantomData,
        }
    }
    pub(super) fn linked_iter_mut(&mut self) -> SequenceIterMut<'_, T> {
        let (front, back) = self.topology.endpoints();
        SequenceIterMut {
            clear: self.clear_nodes.as_ref().map(|registry| {
                Box::new(ClearIteratorState {
                    registry,
                    yielded: Default::default(),
                })
            }),
            front,
            back,
            remaining: self.len(),
            item: PhantomData,
        }
    }
}
impl<'a, T> Iterator for SequenceIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let mut node = self.front;
        if let Some(nodes) = self.clear_nodes {
            let Some(live) = nodes.live_link(node) else {
                self.remaining = 0;
                return None;
            };
            node = live;
        }
        // SAFETY: `node` is owned by the borrowed sequence (checked above), which keeps its
        // links unchanged.
        self.front = unsafe { (*node).next() };
        self.remaining -= 1;
        // SAFETY: `node` is a live node owned by the borrowed sequence, and the iterator's
        // borrow bounds the item borrow.
        Some(unsafe { owner::item_ref(node) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}
impl<T> DoubleEndedIterator for SequenceIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let mut node = self.back;
        if let Some(nodes) = self.clear_nodes {
            let Some(live) = nodes.live_link(node) else {
                self.remaining = 0;
                return None;
            };
            node = live;
        }
        // SAFETY: `node` is owned by the borrowed sequence (checked above), which keeps its
        // links unchanged.
        self.back = unsafe { (*node).previous() };
        self.remaining -= 1;
        // SAFETY: `node` is a live node owned by the borrowed sequence, and the iterator's
        // borrow bounds the item borrow.
        Some(unsafe { owner::item_ref(node) })
    }
}
impl<T> ExactSizeIterator for SequenceIter<'_, T> {}
impl<T> Clone for SequenceIter<'_, T> {
    fn clone(&self) -> Self {
        Self {
            clear_nodes: self.clear_nodes,
            front: self.front,
            back: self.back,
            remaining: self.remaining,
            item: PhantomData,
        }
    }
}
impl<T> PartialEq for SequenceIter<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        let current = (self.remaining > 0).then_some(self.front);
        let other = (other.remaining > 0).then_some(other.front);
        current == other
    }
}
impl<T> Eq for SequenceIter<'_, T> {}
pub(super) struct ClearIteratorState<'a, T> {
    registry: &'a super::owner_registry::SequenceOwnerRegistry<T>,
    yielded: std::collections::HashSet<usize>,
}

impl<T> ClearIteratorState<'_, T> {
    fn resolve(&mut self, node: *mut super::SeqNode) -> Option<*mut super::SeqNode> {
        // A reused address in historical endpoints must not yield the same &mut twice.
        let live = self.registry.live_link(node)?;
        self.yielded.insert(live.addr()).then_some(live)
    }
}

impl<'a, T> Iterator for SequenceIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let mut node = self.front;
        if let Some(clear) = &mut self.clear {
            let Some(live) = clear.resolve(node) else {
                self.remaining = 0;
                return None;
            };
            node = live;
        }
        // SAFETY: `node` is owned by the exclusively borrowed sequence (checked above).
        self.front = unsafe { (*node).next() };
        self.remaining -= 1;
        // SAFETY: `node` is owned by the exclusively borrowed sequence and is yielded once, so item borrows never alias. Only value is borrowed; the link prefix remains available to the iterator.
        Some(unsafe { owner::item_mut(node) })
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}
impl<T> DoubleEndedIterator for SequenceIterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let mut node = self.back;
        if let Some(clear) = &mut self.clear {
            let Some(live) = clear.resolve(node) else {
                self.remaining = 0;
                return None;
            };
            node = live;
        }
        // SAFETY: `node` is owned by the exclusively borrowed sequence (checked above).
        self.back = unsafe { (*node).previous() };
        self.remaining -= 1;
        // SAFETY: `node` is owned by the exclusively borrowed sequence and is yielded once, so
        // item borrows never alias.
        Some(unsafe { owner::item_mut(node) })
    }
}
impl<T> ExactSizeIterator for SequenceIterMut<'_, T> {}
// SAFETY: lifetimes retain the sequence borrow; links cannot change while iterators exist.
unsafe impl<T: Sync> Send for SequenceIter<'_, T> {}
// SAFETY: as for Send above: SequenceIter only hands out shared item references, which needs T:
// Sync.
unsafe impl<T: Sync> Sync for SequenceIter<'_, T> {}
// SAFETY: SequenceIterMut borrows its Sequence exclusively and yields each item once, as a
// slice IterMut does.
unsafe impl<T: Send> Send for SequenceIterMut<'_, T> {}
// SAFETY: as for Send above: shared access to SequenceIterMut only reads, which needs T: Sync.
unsafe impl<T: Sync> Sync for SequenceIterMut<'_, T> {}
impl<'a, T> IntoIterator for &'a Sequence<T> {
    type Item = &'a T;
    type IntoIter = SequenceIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut Sequence<T> {
    type Item = &'a mut T;
    type IntoIter = SequenceIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
