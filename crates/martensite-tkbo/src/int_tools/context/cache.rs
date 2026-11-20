use super::*;
use martensite_tkbrep::toptools::ShapeMapHasher;
use martensite_tkernel::ncollection::{AllocatedBox, BaseAllocator, DataMap};
use std::sync::Arc;
/// A shared cached native service with allocator-owned storage; `context.fclass2d(&face)?.read()`.
pub type Cached<T> = Arc<AllocatedBox<SharedState<T>>>;
pub(super) struct ShapeCache<T> {
    allocator: BaseAllocator,
    entries: SharedState<DataMap<Shape, Cached<T>, ShapeMapHasher>>,
}
impl<T> ShapeCache<T> {
    pub(super) fn new(allocator: &BaseAllocator) -> Self {
        Self {
            allocator: allocator.clone(),
            entries: SharedState::new(DataMap::with_allocator(
                ShapeMapHasher,
                100,
                allocator.clone(),
            )),
        }
    }
    pub(super) fn get(
        &self,
        shape: &Shape,
        create: impl FnOnce() -> Result<T, Failure>,
    ) -> Result<Cached<T>, Failure> {
        if let Some(service) = self.entries.read().seek(shape) {
            return Ok(service.clone());
        }
        // Build outside the map lock: workers share one context (#496), and a slow build
        // under the lock would stall every lookup in this cache. When two workers race, both
        // build and the first insert wins, so all callers still share one service.
        let built = Arc::new(self.allocator.allocate_value(SharedState::new(create()?)));
        let mut entries = self.entries.write();
        if let Some(service) = entries.seek(shape) {
            return Ok(service.clone());
        }
        entries.bind(shape.clone(), built.clone());
        Ok(built)
    }
    pub(super) fn clear(&self) {
        self.entries.write().clear(true);
    }
}

#[cfg(test)]
#[path = "cache_tests.rs"]
mod tests;
