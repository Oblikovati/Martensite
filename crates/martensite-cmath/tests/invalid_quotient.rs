//! Invalid divisions give one NaN word on every target (#201).
use martensite_cmath::{INVALID_OPERATION_NAN, quotient};

const INVALID_WORD: u64 = 0xfff8_0000_0000_0000;

#[test]
fn invalid_quotients_return_the_documented_word() {
    let cases: [(f64, f64); 4] = [
        (0.0, 0.0),
        (-0.0, 0.0),
        (f64::INFINITY, f64::INFINITY),
        (f64::NEG_INFINITY, f64::INFINITY),
    ];
    for (numerator, denominator) in cases {
        let word = quotient(numerator, denominator).to_bits();
        assert_eq!(
            word, INVALID_WORD,
            "{numerator}/{denominator} gave {word:#018x}"
        );
    }
    assert_eq!(INVALID_OPERATION_NAN.to_bits(), INVALID_WORD);
}

#[test]
fn valid_quotients_and_nan_operands_are_unchanged() {
    assert_eq!(quotient(1.0, 3.0), 1.0 / 3.0);
    assert_eq!(quotient(1.0, 0.0), f64::INFINITY);
    assert_eq!(quotient(-1.0, 0.0), f64::NEG_INFINITY);
    assert_eq!(quotient(1.0, f64::INFINITY).to_bits(), 0);
    let positive_nan = f64::from_bits(0x7ff8_0000_0000_0001);
    assert_eq!(
        quotient(positive_nan, 2.0).to_bits(),
        positive_nan.to_bits()
    );
}
