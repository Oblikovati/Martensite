//! Equality of `time_t` values.
//!
//! The comparison exists only when `time_t` is a distinct type from `size_t` and
//! `int`, which holds on every supported profile, where `time_t` is a signed 64-bit count.

/// `std::time_t` on the supported profiles.
pub type TimeT = i64;

/// Value equality.
///
/// ```
/// assert!(martensite_tkernel::standard::time::is_equal(1_700_000_000, 1_700_000_000));
/// ```
pub const fn is_equal(first: TimeT, second: TimeT) -> bool {
    first == second
}
