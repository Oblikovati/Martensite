//! Explicit constructor and default construction differ in outer-array policy.
use super::{BOPDSIterator, BaseAllocator, DynamicArray, Pair};

impl Default for BOPDSIterator {
    fn default() -> Self {
        Self::with_outer_policy(Some(BaseAllocator::common()), None)
    }
}

impl BOPDSIterator {
    /// Creates empty candidate groups retaining the nullable allocation policy.
    /// `assert!(BOPDSIterator::with_allocator(None).allocator().is_none());`
    pub fn with_allocator(allocator: Option<BaseAllocator>) -> Self {
        Self::with_outer_policy(allocator.clone(), allocator)
    }

    /// Borrows the configured resource independently of the default outer-array policy.
    /// `let resource = BOPDSIterator::default().allocator().cloned();`
    pub fn allocator(&self) -> Option<&BaseAllocator> {
        self.allocator.as_ref()
    }

    fn with_outer_policy(allocator: Option<BaseAllocator>, outer: Option<BaseAllocator>) -> Self {
        Self {
            allocator,
            preparation_kind: super::PreparationKind::AcrossArguments,
            lists: empty_groups(10, outer.clone()),
            extended_lists: empty_groups(4, outer),
            use_extended: false,
            active_pairs: Vec::new(),
            shape_types: Vec::new(),
            position: 0,
            length: 0,
            run_parallel: false,
        }
    }
}

fn empty_groups(count: i32, allocator: Option<BaseAllocator>) -> DynamicArray<DynamicArray<Pair>> {
    // The native constructors only assign the outer allocator: child arrays stay default/null.
    let mut groups = DynamicArray::with_allocator_i32(count, allocator);
    for _ in 0..count {
        groups.append(DynamicArray::default());
    }
    groups
}

#[cfg(test)]
mod tests;
