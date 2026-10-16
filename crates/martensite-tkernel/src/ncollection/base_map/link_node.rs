//! Non-owning link shared by custom bucket nodes.

/// The list-node link prefix; no node or successor ownership is implied.
/// Example: `let link = MapLinkNode::new(std::ptr::null_mut());`.
#[repr(C)]
#[derive(Debug)]
pub struct MapLinkNode {
    successor: *mut Self,
}

impl MapLinkNode {
    /// Stores the successor without dereferencing it; `MapLinkNode::new(next)`.
    pub fn new(successor: *mut Self) -> Self {
        Self { successor }
    }
    /// Reads the const next pointer; `assert!(link.next().is_null())`.
    pub fn next(&self) -> *mut Self {
        self.successor
    }
    /// Borrows the mutable next pointer; `*link.next_mut() = next`.
    pub fn next_mut(&mut self) -> &mut *mut Self {
        &mut self.successor
    }
}
