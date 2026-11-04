//! Interior state of a transient that every handle may change.
//!
//! A transient may be mutated through any handle that shares it, so
//! a setter's change is visible through every other handle. A [`Handle`](super::Handle)
//! only lends `&T`, so a transient keeps its mutable fields in one [`SharedState`] and
//! every member, setters included, takes `&self`. Readers on several threads do
//! not block each other, and the object stays `Send + Sync`.
//!
//! A member takes the lock once and never calls another handle while it holds it,
//! because that handle can be the same object.
use std::sync::{PoisonError, RwLock, RwLockReadGuard, RwLockWriteGuard, TryLockError};

/// The locked fields of one transient; `SharedState::new(5.0).read()`.
///
/// A poisoned lock is recovered: no code panics while it holds a guard, so the
/// state is always consistent.
#[derive(Debug, Default)]
pub struct SharedState<S> {
    lock: RwLock<S>,
}
/// Why a nonblocking coherent capture could not acquire a read lease.
///
/// `SharedReadFailure::Busy` leaves the caller free to report incomplete work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SharedReadFailure {
    /// A writer owns or is waiting for the mutable cell.
    Busy,
    /// A previous writer unwound while holding the mutable cell.
    Poisoned,
}

impl<S> SharedState<S> {
    /// Wraps the initial fields; `SharedState::new(fields)`.
    pub const fn new(state: S) -> Self {
        Self {
            lock: RwLock::new(state),
        }
    }
    /// Shared access for const members; `state.read().radius`.
    pub fn read(&self) -> RwLockReadGuard<'_, S> {
        self.lock.read().unwrap_or_else(PoisonError::into_inner)
    }
    /// Borrow a coherent cell without waiting or recovering poison; `state.try_read()?`.
    pub fn try_read(&self) -> Result<RwLockReadGuard<'_, S>, SharedReadFailure> {
        match self.lock.try_read() {
            Ok(fields) => Ok(fields),
            Err(TryLockError::WouldBlock) => Err(SharedReadFailure::Busy),
            Err(TryLockError::Poisoned(_)) => Err(SharedReadFailure::Poisoned),
        }
    }
    /// Attempt a brief exclusive lease without blocking; `if let Some(mut fields)=state.try_write(){*fields=2}`.
    pub fn try_write(&self) -> Option<RwLockWriteGuard<'_, S>> {
        match self.lock.try_write() {
            Ok(fields) => Some(fields),
            Err(TryLockError::Poisoned(error)) => Some(error.into_inner()),
            Err(TryLockError::WouldBlock) => None,
        }
    }
    /// Exclusive access for mutating members; `state.write().radius = 2.0`.
    pub fn write(&self) -> RwLockWriteGuard<'_, S> {
        self.lock.write().unwrap_or_else(PoisonError::into_inner)
    }
}
impl<S: Clone> SharedState<S> {
    /// Copies the whole state, as a copy constructor reads it; `state.snapshot()`.
    pub fn snapshot(&self) -> S {
        self.read().clone()
    }
}

#[cfg(test)]
mod nonblocking_state_tests {
    use super::{SharedReadFailure, SharedState};
    #[test]
    fn nonblocking_read_preserves_values_and_refuses_an_actual_writer() {
        let state = SharedState::new(3);
        assert_eq!(*state.try_read().unwrap(), 3);
        let writer = state.write();
        assert_eq!(state.try_read().unwrap_err(), SharedReadFailure::Busy);
        drop(writer);
        assert_eq!(*state.try_read().unwrap(), 3);
    }
    #[test]
    fn nonblocking_read_reports_poison_instead_of_recovering_it() {
        let state = SharedState::new(3);
        let panic = std::panic::catch_unwind(|| {
            let mut writer = state.write();
            *writer = 7;
            panic!("NamedReadPoison fault=writer; expected explicit poison refusal");
        });
        assert!(panic.is_err());
        assert_eq!(state.try_read().unwrap_err(), SharedReadFailure::Poisoned);
        assert_eq!(*state.read(), 7);
    }
    #[test]
    fn nonblocking_write_observes_read_and_write_leases_without_waiting() {
        let state = SharedState::new(3);
        let read = state.read();
        assert!(state.try_write().is_none());
        drop(read);
        let write = state.write();
        assert!(state.try_write().is_none());
        drop(write);
        *state.try_write().unwrap() = 7;
        assert_eq!(*state.read(), 7);
    }
    #[test]
    fn nonblocking_write_recovers_the_actual_poisoned_state() {
        let state = SharedState::new(3);
        let panic = std::panic::catch_unwind(|| {
            *state.write() = 7;
            let _lease = state.write();
            panic!("NamedPoisonedState fault=writer; expected recovery after unwinding");
        });
        assert!(panic.is_err());
        *state.try_write().unwrap() = 11;
        assert_eq!(*state.read(), 11);
    }
}
