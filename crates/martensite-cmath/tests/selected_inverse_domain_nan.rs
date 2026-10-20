//! Selected owned domain representations preserve serialized expression constants (#201, #203).
use martensite_cmath::{acosh, atanh};
const NEGATIVE_QUIET_NAN: u64 = 0xfff8_0000_0000_0000;

#[test]
fn inverse_cosh_real_domain_errors_use_the_owned_word() {
    for value in [
        f64::NEG_INFINITY,
        -2.0,
        -0.0,
        0.0,
        0.25,
        f64::from_bits(0x3fefffffffffffff),
    ] {
        assert_eq!(acosh(value).to_bits(), NEGATIVE_QUIET_NAN);
    }
}

#[test]
fn inverse_tanh_real_domain_errors_include_both_domain_infinities() {
    for value in [
        f64::NEG_INFINITY,
        f64::INFINITY,
        -2.0,
        2.0,
        f64::from_bits(0x3ff0000000000001),
        f64::from_bits(0xbff0000000000001),
    ] {
        assert_eq!(atanh(value).to_bits(), NEGATIVE_QUIET_NAN);
    }
}

#[test]
fn valid_endpoints_keep_their_specified_words() {
    assert_eq!(acosh(1.0).to_bits(), 0);
    assert_eq!(acosh(f64::INFINITY), f64::INFINITY);
    assert_eq!(atanh(1.0), f64::INFINITY);
    assert_eq!(atanh(-1.0), f64::NEG_INFINITY);
    assert_eq!(atanh(0.0).to_bits(), 0);
    assert_eq!(atanh(-0.0).to_bits(), 0x8000000000000000);
}

#[test]
fn nan_operands_have_no_new_sign_payload_or_quieting_promise() {
    for bits in [
        0x7ff8000000000123,
        0xfff8000000000456,
        0x7ff0000000000001,
        0xfff0000000000001,
    ] {
        let operand = f64::from_bits(bits);
        assert!(acosh(operand).is_nan());
        assert!(atanh(operand).is_nan());
    }
}

#[test]
fn valid_neighbors_keep_independently_certified_finite_results() {
    assert_eq!(
        acosh(f64::from_bits(0x3ff0000000000001)).to_bits(),
        0x3e56a09e667f3bcc
    );
    assert_eq!(
        atanh(f64::from_bits(0x3fefffffffffffff)).to_bits(),
        0x4032b708872320e2
    );
    assert_eq!(
        atanh(f64::from_bits(0xbfefffffffffffff)).to_bits(),
        0xc032b708872320e2
    );
}
