//! A cursor's canonical state can be observed during removal without aliasing a mutable cursor.
use super::{BaseListRawIterator, List, ListCursor, removal::resume_drop_outcome};
use crate::standard::Failure;
use std::sync::{Arc, Mutex, PoisonError};
struct NativeCursorState(Mutex<ListCursor>);
impl NativeCursorState {
    fn read(&self) -> ListCursor {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner)
    }
    fn write(&self, position: ListCursor) {
        *self.0.lock().unwrap_or_else(PoisonError::into_inner) = position;
    }
}
/// Owned cursor state; unlike the Copy `ListCursor`, it supports live observer handles.
/// Example: `let mut cursor = ListNativeCursor::new(&list); list.remove_native(&mut cursor)?`.
/// Cloning copies the links into independent state; it does not share cursor mutation.
pub struct ListNativeCursor {
    shared: Arc<NativeCursorState>,
}
/// Read-only access to a cursor's actual current/previous links.
/// Example: `let observer = cursor.observer(); assert!(observer.more())`.
#[derive(Clone)]
pub struct ListCursorObserver {
    shared: Arc<NativeCursorState>,
}
impl ListNativeCursor {
    /// Starts at the owner's head, with null previous; `ListNativeCursor::new(&list)`.
    pub fn new<T>(list: &List<T>) -> Self {
        Self::from_position(list.cursor())
    }
    fn from_position(position: ListCursor) -> Self {
        Self {
            shared: Arc::new(NativeCursorState(Mutex::new(position))),
        }
    }
    /// Creates a live read-only view without borrowing this cursor; `cursor.observer()`.
    pub fn observer(&self) -> ListCursorObserver {
        ListCursorObserver {
            shared: self.shared.clone(),
        }
    }
    /// Returns the current detached safe cursor expression; `cursor.position().value(&list)`.
    pub fn position(&self) -> ListCursor {
        self.shared.read()
    }
    /// Advances the links after checking live owner membership; `cursor.next(&list)?`.
    pub fn next<T>(&mut self, list: &List<T>) -> Result<(), Failure> {
        let mut position = self.position();
        if !position.more() {
            return Ok(());
        }
        let actual = list.locate_cursor(position, "Next")?;
        position.raw.previous = actual.current;
        // SAFETY: locate_cursor proved actual.current belongs to the live List chain.
        position.raw.current = unsafe { (*actual.current).next() };
        position.revision = list.topology.revision();
        self.shared.write(position);
        Ok(())
    }
}
impl Default for ListNativeCursor {
    fn default() -> Self {
        Self::from_position(ListCursor::default())
    }
}
impl Clone for ListNativeCursor {
    fn clone(&self) -> Self {
        Self::from_position(self.shared.read())
    }
}
impl ListCursorObserver {
    /// Reads More without dereferencing current; `observer.more()`.
    pub fn more(&self) -> bool {
        self.shared.read().more()
    }
    /// Reads opaque current identity; `observer.current_node()`.
    pub fn current_node(&self) -> Option<usize> {
        self.shared.read().current_node()
    }
    /// Reads remembered predecessor identity, including at end; `observer.previous_node()`.
    pub fn previous_node(&self) -> Option<usize> {
        self.shared.read().previous_node()
    }
}
impl<T> List<T> {
    /// Removes through an observable canonical cursor; `list.remove_native(&mut cursor)?`.
    pub fn remove_native(&mut self, cursor: &mut ListNativeCursor) -> Result<(), Failure> {
        let position = cursor.position();
        let actual = self.locate_cursor(position, "Remove")?;
        if position.raw.previous != actual.previous {
            return Err(super::no_such_object(
                "Remove",
                "a cursor with stale previous link",
            ));
        }
        let (next, outcome) = self.remove_in_phases(actual);
        cursor.shared.write(ListCursor {
            raw: BaseListRawIterator {
                current: next,
                previous: actual.previous,
            },
            revision: self.topology.revision(),
        });
        resume_drop_outcome(outcome);
        Ok(())
    }
}
