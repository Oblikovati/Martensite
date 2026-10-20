#![allow(dead_code)] // Each test imports the subset it needs.
//! Sign-aware binary64 assertion semantics, including signed zero and NaNs.

/// Requires at most four ULPs between non-NaN inputs, including across zero.
/// `expect_double_eq(0.0, -0.0)` succeeds without overflowing signed arithmetic.
pub fn expect_double_eq(a: f64, b: f64) {
    let accepted =
        !a.is_nan() && !b.is_nan() && ordered_float_bits(a).abs_diff(ordered_float_bits(b)) <= 4;
    assert!(
        accepted,
        "{a} != {b}; expected non-NaN values within four ULPs"
    );
}

fn ordered_float_bits(value: f64) -> u64 {
    let bits = value.to_bits();
    if bits >> 63 == 0 {
        return bits | (1 << 63);
    }
    (!bits).wrapping_add(1)
}

/// Tests an inclusive absolute tolerance without accepting NaNs.
/// `assert!(within_absolute_error(1.0, 1.5, 0.5));`
pub fn within_absolute_error(a: f64, b: f64, tol: f64) -> bool {
    (a - b).abs() <= tol
}

#[cfg(test)]
mod absolute_error_tests {
    use super::within_absolute_error;

    #[test]
    fn inclusive_boundary_and_signed_zero_match_the_original_predicate() {
        assert!(within_absolute_error(1., 1.5, 0.5));
        assert!(!within_absolute_error(1., 1.50000001, 0.5));
        assert!(within_absolute_error(-0., 0., 0.));
        assert!(!within_absolute_error(1., 1., -0.1));
    }

    #[test]
    fn nonfinite_differences_are_rejected_with_finite_tolerance() {
        assert!(!within_absolute_error(f64::NAN, 1., 1.));
        assert!(!within_absolute_error(1., f64::NAN, 1.));
        assert!(!within_absolute_error(f64::INFINITY, f64::INFINITY, 1.));
        assert!(!within_absolute_error(f64::INFINITY, 1., 1.));
    }
}
