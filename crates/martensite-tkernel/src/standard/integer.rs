//! Integer helpers with C truncated-remainder semantics.
//!
//! Where a signed `int` operation overflows (undefined behavior in C), the result is the
//! two's-complement result that x86-64 produces instead of panicking in debug builds;
//! each such function says so.

/// `value % 2 == 0`.
///
/// ```
/// assert!(martensite_tkernel::standard::integer::is_even(-4));
/// ```
pub const fn is_even(value: i32) -> bool {
    value % 2 == 0
}

/// `value % 2 == 1`, so negative odd values are neither odd nor even (the
/// truncated remainder is -1).
///
/// ```
/// use martensite_tkernel::standard::integer::is_odd;
/// assert!(is_odd(3) && !is_odd(-3));
/// ```
pub const fn is_odd(value: i32) -> bool {
    value % 2 == 1
}

/// `value < 0 ? -value : value`.
///
/// `-INT_MIN` overflows; the result wraps to `INT_MIN`, the x86-64 result.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::abs(-7), 7);
/// ```
pub const fn abs(value: i32) -> i32 {
    if value < 0 {
        value.wrapping_neg()
    } else {
        value
    }
}

/// `Max(int, int)`: `std::max`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::max(-1, 2), 2);
/// ```
pub const fn max(first: i32, second: i32) -> i32 {
    if first < second { second } else { first }
}

/// `Min(int, int)`: `std::min`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::min(-1, 2), -1);
/// ```
pub const fn min(first: i32, second: i32) -> i32 {
    if second < first { second } else { first }
}

/// The truncated remainder `value % divisor`, with the dividend's sign.
///
/// A zero divisor and `INT_MIN % -1` have no defined result (x86-64 `idiv` raises SIGFPE);
/// both panic, as Rust's `%` does.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::modulus(-7, 3), -1);
/// ```
pub const fn modulus(value: i32, divisor: i32) -> i32 {
    value % divisor
}

/// `value * value`.
///
/// The header documents overflow as undefined; the result wraps like x86-64 `imul`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::square(-9), 81);
/// ```
pub const fn square(value: i32) -> i32 {
    value.wrapping_mul(value)
}

/// `INT_MIN`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::integer_first(), i32::MIN);
/// ```
pub const fn integer_first() -> i32 {
    i32::MIN
}

/// `INT_MAX`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::integer_last(), i32::MAX);
/// ```
pub const fn integer_last() -> i32 {
    i32::MAX
}

/// The size of an `int` in bits, `CHAR_BIT * sizeof(int)`.
///
/// ```
/// assert_eq!(martensite_tkernel::standard::integer::integer_size(), 32);
/// ```
pub const fn integer_size() -> i32 {
    (u8::BITS as usize * size_of::<i32>()) as i32
}
