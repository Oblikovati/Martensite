//! An owned witness proves the caller's context outlives failing workers.
use crate::int_tools::Context;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

/// Retain a real Context with an owned lifetime witness; `DropTrackedContext::new(drops)`.
pub(super) struct DropTrackedContext {
    pub context: Context,
    drops: Arc<AtomicUsize>,
}

impl DropTrackedContext {
    /// Keep the caller context alive outside the operation; `DropTrackedContext::new(drops)`.
    pub fn new(drops: Arc<AtomicUsize>) -> Self {
        Self {
            context: Context::default(),
            drops,
        }
    }
}

impl Drop for DropTrackedContext {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}
