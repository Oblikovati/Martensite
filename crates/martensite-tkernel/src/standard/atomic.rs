//! The deprecated full-barrier integer operations.
//!
//! The GCC branch uses `__sync_*` builtins, which are sequentially consistent and wrap
//! on overflow; `AtomicI32` with `SeqCst` gives the same values and ordering on every
//! platform, so the per-platform branches of the header collapse into one.

use std::sync::atomic::{AtomicI32, Ordering};

/// Adds one and returns the new value (wrapping).
///
/// ```
/// use std::sync::atomic::AtomicI32;
/// let counter = AtomicI32::new(1);
/// assert_eq!(martensite_tkernel::standard::atomic::atomic_increment(&counter), 2);
/// ```
pub fn atomic_increment(value: &AtomicI32) -> i32 {
    value.fetch_add(1, Ordering::SeqCst).wrapping_add(1)
}

/// Subtracts one and returns the new value (wrapping).
///
/// ```
/// use std::sync::atomic::AtomicI32;
/// let counter = AtomicI32::new(1);
/// assert_eq!(martensite_tkernel::standard::atomic::atomic_decrement(&counter), 0);
/// ```
pub fn atomic_decrement(value: &AtomicI32) -> i32 {
    value.fetch_sub(1, Ordering::SeqCst).wrapping_sub(1)
}

/// Stores `new_value` when the
/// current value is `old_value`, and returns whether it did.
///
/// ```
/// use std::sync::atomic::AtomicI32;
/// let counter = AtomicI32::new(5);
/// assert!(martensite_tkernel::standard::atomic::atomic_compare_and_swap(&counter, 5, 9));
/// ```
pub fn atomic_compare_and_swap(value: &AtomicI32, old_value: i32, new_value: i32) -> bool {
    value
        .compare_exchange(old_value, new_value, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}
