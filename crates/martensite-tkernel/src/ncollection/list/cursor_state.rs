//! List current/previous state without dereferencing detached addresses.
use super::{BaseListIterator, BaseListRawIterator, List, ListCursor, ListIterMut};
use crate::standard::Failure;

impl PartialEq for ListCursor {
    fn eq(&self, other: &Self) -> bool {
        self.raw.current == other.raw.current
    }
}
impl Eq for ListCursor {}
impl std::fmt::Debug for ListCursor {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format
            .debug_struct("ListCursor")
            .field("address", &self.current_node())
            .field("previous", &self.previous_node())
            .finish()
    }
}
impl ListCursor {
    /// Returns the opaque current-node identity; `cursor.current_node()`.
    pub fn current_node(&self) -> Option<usize> {
        super::address(self.raw.current)
    }
    /// Returns the last remembered predecessor, including at end; `cursor.previous_node()`.
    pub fn previous_node(&self) -> Option<usize> {
        super::address(self.raw.previous)
    }
    /// Copies a mutable standard iterator's two links; `ListCursor::from_mut_iterator(&iterator)`.
    pub fn from_mut_iterator<T>(iterator: &ListIterMut<'_, T>) -> Self {
        Self {
            raw: BaseListRawIterator {
                current: iterator.next,
                previous: iterator.previous,
            },
            revision: iterator.revision,
        }
    }
}
impl<T> List<T> {
    /// Constructs a detached standard iterator; `list.base_iterator::<false>()`.
    pub fn base_iterator<const MUTABLE: bool>(&self) -> BaseListIterator<T, MUTABLE> {
        BaseListIterator::new(self)
    }
    /// Inserts an owned item and updates the previous link; `list.insert_before_at(7, &mut cursor)?`.
    pub fn insert_before_at(
        &mut self,
        value: T,
        cursor: &mut ListCursor,
    ) -> Result<&mut T, Failure> {
        self.emplace_before_at(cursor, || value)
    }
}
impl<T: Clone> List<T> {
    /// Copies after allocation and updates the previous link; `list.insert_before_at_ref(&7, &mut cursor)?`.
    pub fn insert_before_at_ref(
        &mut self,
        value: &T,
        cursor: &mut ListCursor,
    ) -> Result<&mut T, Failure> {
        self.emplace_before_at(cursor, || value.clone())
    }
}
