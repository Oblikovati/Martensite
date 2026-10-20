use super::frexp;

#[test]
fn normal_boundaries_keep_the_exact_significand() {
    let cases: [(u64, u64, i32); 4] = [
        (0x3ff0000000000000, 0x3fe0000000000000, 1),
        (0xc008000000000000, 0xbfe8000000000000, 2),
        (0x0010000000000000, 0x3fe0000000000000, -1021),
        (0x7fefffffffffffff, 0x3fefffffffffffff, 1024),
    ];
    for (input, expected, exponent) in cases {
        let actual = frexp(f64::from_bits(input));
        assert_eq!((actual.0.to_bits(), actual.1), (expected, exponent));
    }
}

#[test]
fn subnormal_endpoints_are_normalized_without_rounding() {
    let cases: [(u64, u64, i32); 3] = [
        (1, 0x3fe0000000000000, -1073),
        (0x8000000000000001, 0xbfe0000000000000, -1073),
        (0x000fffffffffffff, 0x3feffffffffffffe, -1022),
    ];
    for (input, expected, exponent) in cases {
        let actual = frexp(f64::from_bits(input));
        assert_eq!((actual.0.to_bits(), actual.1), (expected, exponent));
    }
}

#[test]
fn zero_and_nonfinite_words_are_preserved() {
    let words: [u64; 6] = [
        0,
        0x8000000000000000,
        0x7ff0000000000000,
        0xfff0000000000000,
        0x7ff8000000000042,
        0xfff8000000000042,
    ];
    for word in words {
        let actual = frexp(f64::from_bits(word));
        assert_eq!((actual.0.to_bits(), actual.1), (word, 0));
    }
}
