// SPDX-License-Identifier: GPL-2.0-only
//! Safe retained initialized slot reset for operation-local mesh builders (#104).

/// Operation-local reusable slot pool that retains backing allocation across rebuilds
/// and resets active slots safely without `unsafe` code (#104).
#[derive(Clone, Debug)]
pub struct RetainedSlotPool<T: Default> {
    slots: Vec<T>,
    active: usize,
}

impl<T: Default> Default for RetainedSlotPool<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Default> RetainedSlotPool<T> {
    /// Creates an empty pool.
    pub fn new() -> Self {
        Self {
            slots: Vec::new(),
            active: 0,
        }
    }

    /// Creates a pool with pre-reserved backing capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            slots: Vec::with_capacity(capacity),
            active: 0,
        }
    }

    /// Acquires the next slot, reusing a previously initialized slot when available.
    pub fn push(&mut self, value: T) -> usize {
        let index = self.active;
        if index < self.slots.len() {
            self.slots[index] = value;
        } else {
            self.slots.push(value);
        }
        self.active += 1;
        index
    }

    /// Borrows the currently active slice.
    pub fn active_slice(&self) -> &[T] {
        &self.slots[..self.active]
    }

    /// Mutably borrows the currently active slice.
    pub fn active_slice_mut(&mut self) -> &mut [T] {
        &mut self.slots[..self.active]
    }

    /// Number of active slots.
    pub fn len(&self) -> usize {
        self.active
    }

    /// True when no slots are active.
    pub fn is_empty(&self) -> bool {
        self.active == 0
    }

    /// Retained backing capacity.
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Resets every active slot to `T::default()` while preserving backing allocation (#104).
    pub fn reset_retained(&mut self) {
        for slot in &mut self.slots[..self.active] {
            *slot = T::default();
        }
        self.active = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reset_retained_clears_active_slots_and_preserves_capacity() {
        let mut pool = RetainedSlotPool::<i32>::with_capacity(16);
        assert_eq!(pool.push(10), 0);
        assert_eq!(pool.push(20), 1);
        assert_eq!(pool.active_slice(), &[10, 20]);
        let cap_before = pool.capacity();

        pool.reset_retained();
        assert!(pool.is_empty());
        assert_eq!(pool.capacity(), cap_before);

        assert_eq!(pool.push(99), 0);
        assert_eq!(pool.active_slice(), &[99]);
    }
}
