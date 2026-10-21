//! The four-ULP assertion contract uses independently specified binary64 pairs.
use float_assertions::expect_double_eq;
#[path = "common/floating.rs"]
mod float_assertions;

const SIGN: u64 = 1 << 63;
const ONE: u64 = 0x3ff0_0000_0000_0000;
const INFINITY: u64 = 0x7ff0_0000_0000_0000;
const NAN: u64 = 0x7ff8_0000_0000_0001;

#[test]
fn four_ulp_assertion_accepts_signed_zero_and_nearby_cross_sign_subnormals() {
    for (a, b) in [(0, SIGN), (1, SIGN | 1), (2, SIGN | 2), (0, 4)] {
        expect_double_eq(f64::from_bits(a), f64::from_bits(b));
    }
    assert_rejected(3, SIGN | 2);
    assert_rejected(0, 5);
}

#[test]
fn four_ulp_assertion_keeps_exact_positive_and_negative_boundary() {
    for sign in [0, SIGN] {
        expect_double_eq(f64::from_bits(sign | ONE), f64::from_bits(sign | (ONE + 4)));
        assert_rejected(sign | ONE, sign | (ONE + 5));
    }
}

#[test]
fn four_ulp_assertion_rejects_nans_and_distant_infinities() {
    for (a, b) in [
        (NAN, NAN),
        (NAN, ONE),
        (ONE, NAN),
        (INFINITY, SIGN | INFINITY),
    ] {
        assert_rejected(a, b);
    }
    for sign in [0, SIGN] {
        expect_double_eq(
            f64::from_bits(sign | INFINITY),
            f64::from_bits(sign | INFINITY),
        );
        expect_double_eq(
            f64::from_bits(sign | INFINITY),
            f64::from_bits(sign | (INFINITY - 1)),
        );
    }
}

fn assert_rejected(a: u64, b: u64) {
    let result = std::panic::catch_unwind(|| {
        expect_double_eq(f64::from_bits(a), f64::from_bits(b));
    });
    assert!(
        result.is_err(),
        "accepted {a:x} and {b:x}; expected rejection"
    );
}
