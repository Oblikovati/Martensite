//! Detached observers retain public metadata and the final allocator lease.
use super::BaseAllocator;
use std::sync::{Mutex, OnceLock, PoisonError};

pub(super) struct DetachedObservation<T> {
    snapshot: Mutex<T>,
    retired_allocator: OnceLock<BaseAllocator>,
}

impl<T: Copy> DetachedObservation<T> {
    pub(super) fn new(snapshot: T) -> Self {
        Self {
            snapshot: Mutex::new(snapshot),
            retired_allocator: OnceLock::new(),
        }
    }

    pub(super) fn read(&self) -> T {
        *self.snapshot.lock().unwrap_or_else(PoisonError::into_inner)
    }

    pub(super) fn publish(&self, snapshot: impl FnOnce() -> T) {
        let mut current = self.snapshot.lock().unwrap_or_else(PoisonError::into_inner);
        *current = snapshot();
    }

    pub(super) fn retain_final_allocator(&self, allocator: BaseAllocator) {
        let _ = self.retired_allocator.set(allocator);
    }
}
