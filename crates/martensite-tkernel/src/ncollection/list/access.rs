//! Membership checks retain pointer provenance and repair stale safe cursor predecessors.
use super::{
    BaseListRawIterator, BaseListStorage, List, ListCursor, ListLinkNode, no_such_object, owner,
};
use crate::standard::Failure;
impl<T> List<T> {
    /// Borrows the actual list base for raw iterator construction; `list.base_storage().size()`.
    pub fn base_storage(&self) -> &BaseListStorage {
        self.topology.base()
    }
    /// Creates a handle to actual owner metadata; `let observer = list.observer()`.
    pub fn observer(&self) -> super::ListObserver {
        self.topology.observer()
    }
    pub(super) fn locate_cursor(
        &self,
        cursor: ListCursor,
        member: &str,
    ) -> Result<BaseListRawIterator, Failure> {
        self.find_cursor(cursor)
            .ok_or_else(|| no_such_object(member, &format!("{cursor:?} outside this list")))
    }
    pub(super) fn owned_node(&self, cursor: ListCursor) -> Option<*mut ListLinkNode> {
        if !cursor.more() {
            return None;
        }
        if cursor.revision == self.topology.revision() {
            return Some(cursor.raw.current);
        }
        self.find_link(cursor.raw.current).map(|raw| raw.current)
    }
    fn find_cursor(&self, cursor: ListCursor) -> Option<BaseListRawIterator> {
        if !cursor.more() {
            return None;
        }
        if cursor.revision == self.topology.revision() {
            return Some(cursor.raw);
        }
        self.find_link(cursor.raw.current)
    }
    fn find_link(&self, target: *mut ListLinkNode) -> Option<BaseListRawIterator> {
        let mut raw = BaseListRawIterator::new(self.base_storage());
        while !raw.current.is_null() {
            // Equality checks the address only. Return provenance from a live link.
            if raw.current == target {
                return Some(raw);
            }
            raw.previous = raw.current;
            // SAFETY: the List borrow protects each node reached through its owning chain.
            raw.current = unsafe { (*raw.current).next() };
        }
        None
    }
    pub(super) fn endpoint_ref(&self, node: *mut ListLinkNode) -> Option<&T> {
        if node.is_null() {
            return None;
        }
        // SAFETY: callers pass a non-null endpoint of this list, which this list owns; `&self`
        // bounds the shared borrow.
        Some(unsafe { owner::item_ref(node) })
    }
    pub(super) fn endpoint_mut(&mut self, node: *mut ListLinkNode) -> Option<&mut T> {
        if node.is_null() {
            return None;
        }
        // SAFETY: callers pass a non-null endpoint of this list, which this list owns; `&mut
        // self` bounds the exclusive borrow.
        Some(unsafe { owner::item_mut(node) })
    }
}
