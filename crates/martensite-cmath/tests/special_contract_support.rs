//! Shared comparisons keep NaN classification separate from specified result words.

pub(super) fn assert_unary_words(name: &str, operation: fn(f64) -> f64, cases: &[(u64, u64)]) {
    for &(argument, expected) in cases {
        let actual = operation(f64::from_bits(argument)).to_bits();
        assert_eq!(
            actual, expected,
            "{name}({argument:#018x}): expected exact result word"
        );
    }
}

pub(super) fn assert_binary_words(
    name: &str,
    operation: fn(f64, f64) -> f64,
    cases: &[(f64, f64, u64)],
) {
    for &(first, second, expected) in cases {
        let actual = operation(first, second).to_bits();
        assert_eq!(
            actual, expected,
            "{name}({first:?}, {second:?}): expected exact result word"
        );
    }
}

pub(super) fn assert_unary_nan_class(name: &str, operation: fn(f64) -> f64, arguments: &[f64]) {
    for &argument in arguments {
        assert!(
            operation(argument).is_nan(),
            "{name}({argument:?}): expected NaN classification"
        );
    }
}

pub(super) fn signed_nan_arguments() -> [f64; 4] {
    [
        0x7ff8_0000_0000_0123,
        0xfff8_0000_0000_0456,
        0x7ff0_0000_0000_0001,
        0xfff0_0000_0000_0001,
    ]
    .map(f64::from_bits)
}

// C Annex F gives the same zero/infinity/parity rules in both formats; all
// power-table operands and expected non-NaN results embed exactly in binary32.
pub(super) fn assert_power_words(cases: &[(f64, f64, u64)]) {
    assert_binary_words("pow", martensite_cmath::pow, cases);
    for &(first, second, expected) in cases {
        let expected = f64::from_bits(expected) as f32;
        let actual = martensite_cmath::powf(first as f32, second as f32);
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "powf({first:?}, {second:?}): expected exact binary32 special result"
        );
    }
}
