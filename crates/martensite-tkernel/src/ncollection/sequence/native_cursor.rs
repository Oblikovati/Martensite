//! Canonical cursor state can advance before Drop without aliasing its observer.
use super::{SeqNode, Sequence, SequenceCursor, SequenceNodeOwner, SequenceNodeRetirement};
use crate::standard::Failure;
use std::sync::{Arc, Mutex, PoisonError};
struct SequenceCursorState(Mutex<SequenceCursor>);
impl SequenceCursorState {
    fn read(&self) -> SequenceCursor {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
    fn write(&self, cursor: SequenceCursor) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = cursor;
    }
}
/// Observable cursor; `let mut cursor = SequenceNativeCursor::new(&sequence, true)`.
/// Clone copies links independently; observer() shares this cursor's actual state.
pub struct SequenceNativeCursor {
    shared: Arc<SequenceCursorState>,
}
/// Read-only canonical cursor links; `let observer = cursor.observer(); observer.more()`.
#[derive(Clone)]
pub struct SequenceCursorObserver {
    shared: Arc<SequenceCursorState>,
}
impl SequenceNativeCursor {
    /// Positions at first or past-last; `SequenceNativeCursor::new(&sequence, false)`.
    pub fn new<T>(sequence: &Sequence<T>, start: bool) -> Self {
        Self::from_position(SequenceCursor::new(sequence, start))
    }
    fn from_position(cursor: SequenceCursor) -> Self {
        Self {
            shared: Arc::new(SequenceCursorState(Mutex::new(cursor))),
        }
    }
    /// Creates a view of actual current/previous fields; `cursor.observer()`.
    pub fn observer(&self) -> SequenceCursorObserver {
        SequenceCursorObserver {
            shared: self.shared.clone(),
        }
    }
    /// Copies the current safe detached position; `cursor.position().value(&sequence)`.
    pub fn position(&self) -> SequenceCursor {
        self.shared.read()
    }
    /// Advances from a freshly resolved live node; `cursor.next(&sequence)?`.
    pub fn next<T>(&mut self, sequence: &Sequence<T>) -> Result<(), Failure> {
        let position = self.position();
        if !position.more() {
            return Ok(());
        }
        let node = sequence
            .owned_node(position)
            .ok_or_else(|| super::no_such_node("Next", position))?;
        let mut updated = position;
        updated.step_to_next(sequence, node);
        self.shared.write(updated);
        Ok(())
    }
    /// Steps through the remembered live predecessor; `cursor.previous(&sequence)?`.
    pub fn previous<T>(&mut self, sequence: &Sequence<T>) -> Result<(), Failure> {
        let position = self.position();
        if position.raw.previous.is_null() {
            self.shared.write(SequenceCursor::default());
            return Ok(());
        }
        let node = sequence
            .previous_link(position)
            .ok_or_else(|| super::no_such_node("Previous", position))?;
        let mut updated = position;
        updated.step_to_previous(sequence, node);
        self.shared.write(updated);
        Ok(())
    }
}
impl Default for SequenceNativeCursor {
    fn default() -> Self {
        Self::from_position(SequenceCursor::default())
    }
}
impl Clone for SequenceNativeCursor {
    fn clone(&self) -> Self {
        Self::from_position(self.shared.read())
    }
}
impl SequenceCursorObserver {
    /// Reads More without dereferencing current; `observer.more()`.
    pub fn more(&self) -> bool {
        self.shared.read().more()
    }
    /// Reads opaque current identity; `observer.current_node()`.
    pub fn current_node(&self) -> Option<usize> {
        self.shared.read().current_node()
    }
    /// Reads remembered opaque predecessor identity; `observer.previous_node()`.
    pub fn previous_node(&self) -> Option<usize> {
        self.shared.read().previous_node()
    }
}
impl<T> Sequence<T> {
    /// Updates topology/cache/cursor before item Drop; `sequence.remove_native(&mut cursor)?`.
    pub fn remove_native(&mut self, cursor: &mut SequenceNativeCursor) -> Result<(), Failure> {
        // Adapters use detach_native to retire callbacks after releasing their outer field guard.
        drop(self.detach_native(cursor)?);
        Ok(())
    }
    /// Detach and publish actual iterator advancement before callbacks; `sequence.detach_native(&mut cursor)?`.
    /// Drop the returned original node after releasing any enclosing collection guard.
    pub fn detach_native(
        &mut self,
        cursor: &mut SequenceNativeCursor,
    ) -> Result<Option<SequenceNodeRetirement<T>>, Failure> {
        let position = cursor.position();
        if !position.more() {
            return Ok(None);
        }
        let node = self
            .owned_node(position)
            .ok_or_else(|| super::no_such_node("Remove", position))?;
        Ok(Some(SequenceNodeRetirement {
            owner: self.detach_native_node(cursor, node),
        }))
    }
    fn detach_native_node(
        &mut self,
        cursor: &mut SequenceNativeCursor,
        node: *mut SeqNode,
    ) -> SequenceNodeOwner<T> {
        let mut position = cursor.shared.read();
        let mut raw = position.raw;
        raw.current = node;
        let previous = self
            .previous_link(position)
            .filter(|previous| *previous != node);
        self.prepare_owned_cursor(&mut raw);
        position.raw = raw;
        position.record_detachment(self, previous);
        cursor.shared.write(position);
        // SAFETY: prepare_owned_cursor transferred one live unlinked node.
        unsafe { self.take_unlinked(node) }
    }
}
