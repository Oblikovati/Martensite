//! Unsigned overflow defines zero-stride metadata; retained typed storage is tracked separately.
use super::{BaseAllocator, DynamicArray, LinearVector};
impl<T> DynamicArray<T> {
    pub(super) fn with_rounded_increment(
        increment: usize,
        allocator: Option<BaseAllocator>,
    ) -> Self {
        Self {
            blocks: LinearVector::default(),
            allocator,
            increment,
            increment_shift: increment.trailing_zeros(),
            increment_mask: increment.wrapping_sub(1),
            used: 0,
            needs_tail_scan: false,
        }
    }
    pub(super) fn require_element_stride(&self) {
        assert!(
            self.increment != 0,
            "block stride {}: expected nonzero stride for element access/allocation; reset the empty array with SetIncrement",
            self.increment
        );
    }
    #[inline(always)]
    pub(super) fn element_coordinates(&self, index: usize) -> (usize, usize) {
        if self.increment == 0 {
            // log2 of 0 is 0 and the unsigned mask wraps to usize::MAX.
            return (index, index);
        }
        // `increment` is always a power of two; shift/mask replaces hardware DIV/REM (#589).
        (index >> self.increment_shift, index & self.increment_mask)
    }
}
#[cfg(test)]
mod tests;
