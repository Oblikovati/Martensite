// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use num_bigint::BigInt;

fn old_outward_binary64(value: &ExactRatio) -> Option<[f64; 2]> {
    if value.sign() != Ordering::Less {
        return positive_bounds(value);
    }
    let positive = positive_bounds(&value.negated())?;
    Some([-positive[1], -positive[0]])
}
fn positive_bounds(value: &ExactRatio) -> Option<[f64; 2]> {
    if value > &ExactRatio::from_finite(f64::MAX) {
        return None;
    }
    let floor = floor_bits(value);
    let lower = f64::from_bits(floor);
    if &ExactRatio::from_finite(lower) == value {
        return Some([lower, lower]);
    }
    Some([lower, f64::from_bits(floor + 1)])
}
fn floor_bits(value: &ExactRatio) -> u64 {
    // Positive finite binary64 bit patterns are ordered; every comparison is exact.
    // Unlike a floating quotient, this also certifies underflow and extreme ratios.
    greatest_bits_where(|candidate| ExactRatio::from_finite(candidate) <= *value)
}

fn exact_bits(range: Option<[f64; 2]>) -> Option<[u64; 2]> {
    range.map(|endpoints| endpoints.map(f64::to_bits))
}

fn compare_old(value: ExactRatio) {
    let actual = outward_binary64(&value);
    assert_eq!(
        exact_bits(actual),
        exact_bits(old_outward_binary64(&value)),
        "{value:?}"
    );
    if let Some([lower, upper]) = actual {
        assert!(ExactRatio::from_finite(lower) <= value);
        assert!(ExactRatio::from_finite(upper) >= value);
    }
}

#[test]
fn exact_binary64_across_all_exponents_and_signs_matches_the_old_search() {
    for sign in [0, 1 << 63] {
        for exponent in 0..=2046 {
            for fraction in [0, 1, (1 << 51), (1 << 52) - 1] {
                let bits = sign | (exponent << 52) | fraction;
                compare_old(ExactRatio::from_finite(f64::from_bits(bits)));
            }
        }
    }
}

#[test]
fn exact_straddles_of_subnormal_normal_and_maximum_boundaries() {
    let tiny = ExactRatio::from_finite(f64::from_bits(1));
    let fraction = tiny.divide(&ExactRatio::from_finite(3.));
    for anchor in [
        0.,
        f64::from_bits(1),
        f64::from_bits(f64::MIN_POSITIVE.to_bits() - 1),
        f64::MIN_POSITIVE,
        0.5,
        1.,
        2.,
        f64::MAX,
    ] {
        let center = ExactRatio::from_finite(anchor);
        for value in [
            center.subtract(&fraction),
            center,
            ExactRatio::from_finite(anchor).add(&fraction),
        ] {
            compare_old(value.negated());
            compare_old(value);
        }
    }
}

#[test]
fn exact_quotients_across_integer_limb_counts_match_the_old_search() {
    let mut rng = 0x8f6aff18b852a21d_u64;
    for up in [
        0usize, 1, 52, 53, 63, 64, 127, 512, 1023, 1074, 2048, 8192, 16384,
    ] {
        for down in [
            0usize, 1, 18, 53, 63, 64, 128, 511, 1022, 1075, 2048, 8192, 16384,
        ] {
            for _ in 0..8 {
                rng ^= rng << 13;
                rng ^= rng >> 7;
                rng ^= rng << 17;
                let numerator = (BigInt::from(rng as i64) << up) - 1;
                let denominator = (BigInt::from(rng.rotate_left(31)) << down) + 1;
                compare_old(ExactRatio::new(numerator, denominator));
            }
        }
    }
}
