//! The numbers a mesh format writes as text, so that each reads back bit-identical.
use martensite_tkernel::storage::format_general;

/// Significant digits that bring any single-precision value back exactly: with fewer, large
/// coordinates round to a coarser grid than a 32-bit float holds, and small ones are lost.
const SINGLE_PRECISION_DIGITS: usize = 9;
/// Significant digits that bring any double back exactly.
const DOUBLE_PRECISION_DIGITS: usize = 17;

/// A single-precision value as text, the shortest form of 9 significant digits;
/// `single_precision_text(0.5)` is `"0.5"`.
pub fn single_precision_text(value: f32) -> String {
    format_general(f64::from(value), SINGLE_PRECISION_DIGITS)
}

/// A double as text, the shortest form of 17 significant digits;
/// `double_precision_text(0.5)` is `"0.5"`.
pub fn double_precision_text(value: f64) -> String {
    format_general(value, DOUBLE_PRECISION_DIGITS)
}
