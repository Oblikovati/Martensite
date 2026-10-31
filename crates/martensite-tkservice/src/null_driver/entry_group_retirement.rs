//! Retire one moved entry batch in order, including the remaining nodes during unwinding (#324).
use crate::graphic3d::Graphic3dGroup;
use martensite_tkernel::{
    ncollection::Sequence,
    standard::{Handle, raise_native_failure},
};

pub(super) struct EntryGroupRetirement {
    groups: Sequence<Handle<dyn Graphic3dGroup>>,
}
impl EntryGroupRetirement {
    /// Own the actual moved entry collection; `EntryGroupRetirement::new(groups).finish()`.
    pub(super) fn new(groups: Sequence<Handle<dyn Graphic3dGroup>>) -> Self {
        // Moving this owner preserves its original nodes and allocator; it creates no snapshot.
        Self { groups }
    }
    /// Retire nodes without an enclosing field guard; `retirement.finish()`.
    pub(super) fn finish(&mut self) {
        while !self.groups.is_empty() {
            let mut cursor = self.groups.cursor();
            let retired = raise_native_failure(self.groups.detach_cursor(&mut cursor));
            drop(retired);
        }
    }
}
impl Drop for EntryGroupRetirement {
    fn drop(&mut self) {
        // If one payload panics in finish(), its detached node is gone; retire the remaining chain.
        self.finish();
    }
}
