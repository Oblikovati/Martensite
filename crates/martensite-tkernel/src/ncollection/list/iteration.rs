//! Forward borrows project item fields only, leaving raw links disjoint from yielded values.
use super::{List, ListCursor, ListIter, ListIterMut, address, owner};
// SAFETY: shared/mutable iterators borrow their List and carry the same bounds as slice iterators.
unsafe impl<T: Sync> Send for ListIter<'_, T> {}
// SAFETY: as for Send above: ListIter only hands out shared item references, which needs T:
// Sync.
unsafe impl<T: Sync> Sync for ListIter<'_, T> {}
// SAFETY: ListIterMut borrows its List exclusively and yields each item once, as a slice
// IterMut does.
unsafe impl<T: Send> Send for ListIterMut<'_, T> {}
// SAFETY: as for Send above: shared access to ListIterMut only reads, which needs T: Sync.
unsafe impl<T: Sync> Sync for ListIterMut<'_, T> {}
impl ListCursor {
    /// Repositions on the head; `cursor.init(&list)`.
    pub fn init<T>(&mut self, list: &List<T>) {
        *self = list.cursor();
    }
    /// Checks whether the cursor names a node; `cursor.more()`.
    pub fn more(self) -> bool {
        self.address.is_some()
    }
    /// Looks up the stable node in its list; `cursor.value(&list)`.
    pub fn value<T>(self, list: &List<T>) -> Option<&T> {
        let node = list.owned_node(self.address)?;
        // SAFETY: owned_node only returns a live node owned by `list`, and `&list` bounds the
        // shared borrow.
        Some(unsafe { owner::item_ref(node) })
    }
    /// Borrows the positioned item exclusively; `*cursor.change_value(&mut list).unwrap() = 9`.
    /// End or foreign cursors return None instead of dereferencing.
    pub fn change_value<'a, T>(&self, list: &'a mut List<T>) -> Option<&'a mut T> {
        let node = list.owned_node(self.address)?;
        // SAFETY: owned_node only returns a live node owned by `list`, and `&mut list` bounds
        // the exclusive borrow.
        Some(unsafe { owner::item_mut(node) })
    }
    /// Advances while remembering current, including transition to end; `cursor.next(&list)`.
    pub fn next<T>(&mut self, list: &List<T>) {
        if !self.more() {
            return;
        }
        self.previous = self.address;
        self.address = list
            .owned_node(self.address)
            // SAFETY: owned_node only returns a live node owned by `list`, and `&list` keeps
            // its link unchanged.
            .and_then(|node| address(unsafe { (*node).next() }));
    }
}
impl<'a, T> Iterator for ListIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        let node = self.next;
        self.previous = address(node);
        // SAFETY: `node` is a non-null node of the borrowed list's live chain, which the
        // iterator's borrow keeps unchanged.
        self.next = unsafe { (*node).next() };
        // SAFETY: `node` is a live node of the borrowed list, and the iterator's shared borrow
        // of the list bounds the item borrow.
        Some(unsafe { owner::item_ref(node) })
    }
}
impl<T> PartialEq for ListIter<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.next == other.next
    }
}
impl<T> Eq for ListIter<'_, T> {}
impl<'a, T> Iterator for ListIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.next.is_null() {
            return None;
        }
        let node = self.next;
        self.previous = address(node);
        // SAFETY: `node` is a non-null node of the exclusively borrowed list's live chain.
        self.next = unsafe { (*node).next() };
        // SAFETY: each node is yielded once, so the exclusive item borrows never alias, and the
        // iterator's exclusive borrow of the list bounds them.
        Some(unsafe { owner::item_mut(node) })
    }
}
impl<'a, T> IntoIterator for &'a List<T> {
    type Item = &'a T;
    type IntoIter = ListIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl<'a, T> IntoIterator for &'a mut List<T> {
    type Item = &'a mut T;
    type IntoIter = ListIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
