use super::{Sequence, SequenceCursor, SequenceIter, SequenceIterMut, owner};
use std::marker::PhantomData;

impl SequenceCursor {
    /// Positions at the first item or past the last, remembering the final node; `Self::new(&sequence, false)`.
    pub fn new<T>(sequence: &Sequence<T>, is_start: bool) -> Self {
        let (head, tail) = sequence.topology.endpoints();
        let first = pointer_address(head);
        let last = pointer_address(tail);
        if is_start {
            return Self {
                address: first,
                previous: None,
            };
        }
        Self {
            address: None,
            previous: last,
        }
    }
    /// Resets either endpoint; `cursor.init(&sequence, true)`.
    pub fn init<T>(&mut self, sequence: &Sequence<T>, is_start: bool) {
        *self = Self::new(sequence, is_start);
    }
    /// Reports an unexhausted position; `cursor.more()`.
    pub fn more(self) -> bool {
        self.address.is_some()
    }
    /// Borrows a currently owned node's item; `cursor.value(&sequence)`.
    pub fn value<T>(self, sequence: &Sequence<T>) -> Option<&T> {
        let node = sequence.owned_node(self.address)?;
        // SAFETY: owned_node only returns a live node owned by `sequence`, and `&sequence`
        // bounds the shared borrow.
        Some(unsafe { owner::item_ref(node) })
    }
    /// Borrows a positioned item exclusively; `*cursor.change_value(&mut sequence)? = 3`.
    pub fn change_value<T>(self, sequence: &mut Sequence<T>) -> Option<&mut T> {
        let node = sequence.owned_node(self.address)?;
        // SAFETY: owned_node only returns a live node owned by `sequence`, and `&mut sequence`
        // bounds the exclusive borrow.
        Some(unsafe { owner::item_mut(node) })
    }
    /// Follows the actual next link; exhausted cursors remain unchanged; `cursor.next(&sequence)`.
    pub fn next<T>(&mut self, sequence: &Sequence<T>) {
        if self.address.is_none() {
            return;
        }
        let node = sequence.owned_node(self.address);
        self.previous = self.address;
        // SAFETY: owned_node only returns a live node owned by `sequence`, and `&sequence`
        // keeps its link unchanged.
        self.address = node.and_then(|node| pointer_address(unsafe { (*node).next() }));
    }
    /// Follows the remembered previous node; `cursor.previous(&sequence)`.
    pub fn previous<T>(&mut self, sequence: &Sequence<T>) {
        self.address = self.previous;
        if self.address.is_none() {
            return;
        }
        self.previous = sequence
            .owned_node(self.address)
            // SAFETY: owned_node only returns a live node owned by `sequence`, and `&sequence`
            // keeps its link unchanged.
            .and_then(|node| pointer_address(unsafe { (*node).previous() }));
    }
}
pub(super) fn pointer_address(node: *mut super::SeqNode) -> Option<usize> {
    (!node.is_null()).then_some(node as usize)
}
impl PartialEq for SequenceCursor {
    fn eq(&self, other: &Self) -> bool {
        self.address == other.address
    }
}
impl Eq for SequenceCursor {}
impl<T> Sequence<T> {
    pub(super) fn linked_iter(&self) -> SequenceIter<'_, T> {
        let (front, back) = self.topology.endpoints();
        SequenceIter {
            owners: &self.owners,
            front,
            back,
            remaining: self.len(),
            item: PhantomData,
        }
    }
    pub(super) fn linked_iter_mut(&mut self) -> SequenceIterMut<'_, T> {
        let (front, back) = self.topology.endpoints();
        SequenceIterMut {
            owners: &self.owners,
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
        let node = self.front;
        if !self.owners.contains_key(&(node as usize)) {
            self.remaining = 0;
            return None;
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
        let node = self.back;
        if !self.owners.contains_key(&(node as usize)) {
            self.remaining = 0;
            return None;
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
            owners: self.owners,
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
impl<'a, T> Iterator for SequenceIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let node = self.front;
        if !self.owners.contains_key(&(node as usize)) {
            self.remaining = 0;
            return None;
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
        let node = self.back;
        if !self.owners.contains_key(&(node as usize)) {
            self.remaining = 0;
            return None;
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
