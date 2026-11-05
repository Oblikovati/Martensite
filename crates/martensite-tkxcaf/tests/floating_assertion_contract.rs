//! Independent binary64 boundaries for the shared four-ULP comparator.
use float_assertions::expect_double_eq;
use martensite_test_support::floating as float_assertions;

const SIGN: u64 = 0x8000_0000_0000_0000;
const ONE: u64 = 0x3ff0_0000_0000_0000;
const INFINITY: u64 = 0x7ff0_0000_0000_0000;
const NAN: u64 = 0x7ff8_0000_0000_0001;

fn assert_pair(a: u64, b: u64, expected: bool) {
    let result = std::panic::catch_unwind(|| {
        expect_double_eq(f64::from_bits(a), f64::from_bits(b));
    });
    assert_eq!(result.is_ok(), expected, "pair {a:016x}, {b:016x}");
}

#[test]
fn four_ulp_comparison_has_the_exact_signed_zero_and_subnormal_boundary() {
    for (a, b, accepted) in [
        (0, SIGN, true),
        (1, SIGN | 1, true),
        (2, SIGN | 2, true),
        (3, SIGN | 2, false),
        (SIGN, 4, true),
        (SIGN, 5, false),
    ] {
        assert_pair(a, b, accepted);
    }
}

#[test]
fn four_ulp_comparison_has_the_exact_positive_and_negative_finite_boundary() {
    for sign in [0, SIGN] {
        assert_pair(sign | ONE, sign | (ONE + 4), true);
        assert_pair(sign | ONE, sign | (ONE + 5), false);
    }
}

#[test]
fn four_ulp_comparison_keeps_infinity_edges_and_refuses_all_nan_pairs() {
    for sign in [0, SIGN] {
        assert_pair(sign | INFINITY, sign | INFINITY, true);
        assert_pair(sign | INFINITY, sign | (INFINITY - 1), true);
        assert_pair(sign | INFINITY, sign | (INFINITY - 5), false);
        assert_pair(sign | NAN, sign | NAN, false);
        assert_pair(sign | NAN, sign | INFINITY, false);
    }
    assert_pair(INFINITY, SIGN | INFINITY, false);
    assert_pair(NAN, ONE, false);
    assert_pair(ONE, NAN, false);
}
