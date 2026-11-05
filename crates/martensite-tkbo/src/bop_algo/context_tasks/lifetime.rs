//! Owned witnesses prove cleanup after the existing SDK has joined failing workers.
use crate::int_tools::Context;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

/// Retain a real Context with an owned lifetime witness; `DropTrackedContext::main(drops)`.
pub(super) struct DropTrackedContext {
    pub context: Context,
    pub worker: bool,
    drops: Arc<AtomicUsize>,
}

impl DropTrackedContext {
    /// Keep the caller context alive outside the operation; `DropTrackedContext::main(drops)`.
    pub fn main(drops: Arc<AtomicUsize>) -> Self {
        let worker = false;
        Self::new(drops, worker)
    }

    /// Count worker-context destruction after join; `DropTrackedContext::worker(drops)`.
    pub fn worker(drops: Arc<AtomicUsize>) -> Self {
        let worker = true;
        Self::new(drops, worker)
    }

    fn new(drops: Arc<AtomicUsize>, worker: bool) -> Self {
        Self {
            context: Context::default(),
            worker,
            drops,
        }
    }
}

impl Drop for DropTrackedContext {
    fn drop(&mut self) {
        if self.worker {
            self.drops.fetch_add(1, Ordering::SeqCst);
        }
    }
}
