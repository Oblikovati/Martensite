//! Walk a live handle sequence node by node while callbacks edit it.
//!
//! Native `updateAttributes` iterates the group `NCollectionSequence` and the textured
//! `SetColor`/`SetMaterial`/`UnsetMaterial` iterate the presentation sequence node by node: a
//! callback may remove earlier or later nodes or append new ones, and the walk continues
//! from the same node.
//! Removing the active node invalidates the native iterator; here advancing from it returns
//! the typed `NoSuchObject` of the sequence cursor instead of reading freed storage. The
//! cursor is detached: it holds node identities only, so no field guard and no intrusive
//! owner survive a callback, and repeated handles in different nodes stay distinct.
use martensite_tkernel::ncollection::{Sequence, SequenceNativeCursor};
use martensite_tkernel::standard::{Failure, Handle, HandleObserver, RuntimeTyped};
use martensite_tkservice::graphic3d::{
    Graphic3dGroup, Graphic3dStructureMembers as _, Graphic3dStructureSource,
};

#[cfg(test)]
type GroupSequence = Sequence<Handle<dyn Graphic3dGroup>>;

#[derive(Default)]
pub(super) struct LiveNodeCursor {
    // None until the first group is resolved; then the active node of the live sequence.
    cursor: Option<SequenceNativeCursor>,
}
impl LiveNodeCursor {
    /// Lend the next live group after callback edits; `while let Some(g) = cursor.next(prs)? {}`.
    pub(super) fn next(
        &mut self,
        presentation: &dyn Graphic3dStructureSource,
    ) -> Result<Option<HandleObserver<dyn Graphic3dGroup>>, Failure> {
        let structure = presentation.cstructure();
        let Some(structure) = structure.get() else {
            return Ok(None);
        };
        // The guard ends with this call; the caller runs the callback without it.
        let fields = structure.cstructure_base().fields.read();
        self.advance(&fields.groups)
    }
    /// Move to the first node, or past the active node, of `items`; `cursor.advance(&items)?`.
    /// Call it under a short guard of the live sequence and release the guard before the
    /// callback; the next call resolves the same node again.
    pub(super) fn advance<T: ?Sized + RuntimeTyped>(
        &mut self,
        items: &Sequence<Handle<T>>,
    ) -> Result<Option<HandleObserver<T>>, Failure> {
        match &mut self.cursor {
            None => self.cursor = Some(SequenceNativeCursor::new(items, true)),
            Some(cursor) => cursor.next(items)?,
        }
        let position = self.cursor.as_ref().map(SequenceNativeCursor::position);
        // A non-owning observer: the node keeps the only owner, as natively.
        Ok(position
            .and_then(|position| position.value(items))
            .map(Handle::observe))
    }
}

#[cfg(test)]
#[path = "textured_group_cursor_tests.rs"]
mod tests;
