//! A shared cell whose writes advance a revision, so a renderer can tell with one load that
//! the state is unchanged since it last read it (#594). OCCT's renderer reads its aspects again
//! on every frame; a Vulkan draw record keeps what it lowered from them and needs this proof.
use martensite_tkernel::standard::{SharedReadFailure, SharedState};
use std::ops::{Deref, DerefMut};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{RwLockReadGuard, RwLockWriteGuard};

/// `SharedState` with a write revision; `state.write().line_width = 2.0` advances
/// `state.revision()`.
#[derive(Default)]
pub struct RevisedState<S> {
    state: SharedState<S>,
    revision: AtomicU64,
}

/// Exclusive access that advances the revision when it ends, while it still holds the lock,
/// so a reader that sees the new revision also sees the new state.
pub struct RevisedWriteGuard<'a, S> {
    guard: RwLockWriteGuard<'a, S>,
    revision: &'a AtomicU64,
}

impl<S> RevisedState<S> {
    /// Wraps the initial state at revision 0; `RevisedState::new(fields)`.
    pub const fn new(state: S) -> Self {
        Self {
            state: SharedState::new(state),
            revision: AtomicU64::new(0),
        }
    }
    /// Shared access; `state.read().line_width`.
    pub fn read(&self) -> RwLockReadGuard<'_, S> {
        self.state.read()
    }
    /// Shared access without waiting; `state.try_read()?`.
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, S>, SharedReadFailure> {
        self.state.try_read()
    }
    /// Exclusive access; the revision advances when the guard drops; `state.write().x = 1`.
    pub fn write(&self) -> RevisedWriteGuard<'_, S> {
        RevisedWriteGuard {
            guard: self.state.write(),
            revision: &self.revision,
        }
    }
    /// Exclusive access without waiting; `if let Some(mut s) = state.try_write() { s.x = 1 }`.
    pub fn try_write(&self) -> Option<RevisedWriteGuard<'_, S>> {
        Some(RevisedWriteGuard {
            guard: self.state.try_write()?,
            revision: &self.revision,
        })
    }
    /// The number of write accesses that have ended. Read it before the state: an equal
    /// later value proves that no write ended in between; `state.revision()`.
    pub fn revision(&self) -> u64 {
        self.revision.load(Ordering::Acquire)
    }
}

impl<S: Clone> RevisedState<S> {
    /// Copies the whole state; `state.snapshot()`.
    pub fn snapshot(&self) -> S {
        self.state.snapshot()
    }
}

impl<S> Deref for RevisedWriteGuard<'_, S> {
    type Target = S;
    fn deref(&self) -> &S {
        &self.guard
    }
}

impl<S> DerefMut for RevisedWriteGuard<'_, S> {
    fn deref_mut(&mut self) -> &mut S {
        &mut self.guard
    }
}

impl<S> Drop for RevisedWriteGuard<'_, S> {
    fn drop(&mut self) {
        // The guard field drops after this body, so the lock is still held here.
        self.revision.fetch_add(1, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::RevisedState;

    #[test]
    fn each_ended_write_advances_the_revision_and_reads_do_not() {
        let state = RevisedState::new(1);
        assert_eq!(state.revision(), 0);
        assert_eq!(*state.read(), 1);
        assert_eq!(state.revision(), 0);
        *state.write() = 2;
        assert_eq!((*state.read(), state.revision()), (2, 1));
        let writer = state.write();
        assert_eq!(state.revision(), 1, "an open write has not ended yet");
        drop(writer);
        assert_eq!(state.revision(), 2);
    }

    #[test]
    fn a_refused_try_write_keeps_the_revision() {
        let state = RevisedState::new(1);
        let reader = state.read();
        assert!(state.try_write().is_none());
        drop(reader);
        assert_eq!(state.revision(), 0);
        if let Some(mut value) = state.try_write() {
            *value = 3;
        }
        assert_eq!((state.snapshot(), state.revision()), (3, 1));
    }
}
