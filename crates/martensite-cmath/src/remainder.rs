//! Pure-Rust IEEE 754 binary64 remainder (`remainder(x, y)`), zero `unsafe`.
//!
//! Computes `x - n * y` where `n` is the integer nearest to `x / y`, with ties
//! rounded to the nearest even integer. Every finite valid result is exact in
//! binary64 and matches C99/IEEE 754 `remainder(x, y)` bit-for-bit (#631).

const SIGN_BIT: u64 = 1 << 63;
const FRAC_MASK: u64 = (1 << 52) - 1;
const IMPLICIT_BIT: u64 = 1 << 52;
const EXP_MASK: u64 = 0x7ff;

/// Decodes a finite nonzero positive `f64` into `(significand, exponent)` such
/// that `value = significand * 2^(exponent - 1075)` and `significand` lies in
/// `[2^52, 2^53)`.
fn decode_positive(value: f64) -> (u64, i32) {
    let bits: u64 = value.to_bits() & !SIGN_BIT;
    let exp_bits: i32 = ((bits >> 52) & EXP_MASK) as i32;
    let frac: u64 = bits & FRAC_MASK;
    if exp_bits > 0 {
        return (IMPLICIT_BIT | frac, exp_bits);
    }
    let shift: u32 = frac.leading_zeros() - 11;
    (frac << shift, 1 - shift as i32)
}

/// Encodes `significand * 2^(exponent - 1075)` with `sign_bit` into `f64`.
/// `significand` must be nonzero and `< 2^53`, and the value must be an exact
/// binary64 number (guaranteed for IEEE 754 `remainder` outputs).
fn encode_exact(significand: u64, exponent: i32, sign_bit: u64) -> f64 {
    let lead: u32 = significand.leading_zeros() - 11;
    let norm_sig: u64 = significand << lead;
    let norm_exp: i32 = exponent - lead as i32;
    if norm_exp >= 1 {
        let bits: u64 = sign_bit | ((norm_exp as u64) << 52) | (norm_sig & FRAC_MASK);
        return f64::from_bits(bits);
    }
    let shift: u32 = (1 - norm_exp) as u32;
    f64::from_bits(sign_bit | (norm_sig >> shift))
}

/// Computes `(sx * 2^exp_diff) mod (2 * sy)` without overflowing `u128`.
/// Since `2 * sy < 2^54`, shifting by at most 64 bits keeps products `< 2^118`.
fn reduce_mod_double(sx: u64, sy: u64, mut exp_diff: u32) -> u64 {
    let two_sy: u128 = u128::from(sy) << 1;
    let mut rem: u128 = u128::from(sx) % two_sy;
    while exp_diff > 0 {
        let step: u32 = exp_diff.min(64);
        rem = (rem << step) % two_sy;
        exp_diff -= step;
    }
    rem as u64
}

/// Reduces `|x| >= |y|` (or same exponent) using exact modulo `2 * |y|` to keep
/// both the remainder and the parity of `floor(|x| / |y|)`.
fn remainder_ge_exp(sx: u64, sy: u64, ey: i32, exp_diff: u32, sign_x: u64) -> f64 {
    let r_mod2: u64 = reduce_mod_double(sx, sy, exp_diff);
    if r_mod2 == 0 {
        return f64::from_bits(sign_x);
    }
    let q_odd: bool = r_mod2 >= sy;
    let rem: u64 = if q_odd { r_mod2 - sy } else { r_mod2 };
    if rem == 0 {
        return f64::from_bits(sign_x);
    }
    let twice_rem: u64 = rem << 1;
    if twice_rem < sy || (twice_rem == sy && !q_odd) {
        return encode_exact(rem, ey, sign_x);
    }
    encode_exact(sy - rem, ey, sign_x ^ SIGN_BIT)
}

/// IEEE 754 remainder: `numerator - n * denominator` with `n` the integer
/// nearest to `numerator / denominator` (ties to even).
pub(crate) fn remainder(numerator: f64, denominator: f64) -> f64 {
    if numerator.is_nan() || denominator.is_nan() {
        return f64::NAN;
    }
    if numerator.is_infinite() || denominator == 0.0 {
        return f64::NAN;
    }
    if denominator.is_infinite() || numerator == 0.0 {
        return numerator;
    }
    let sign_x: u64 = numerator.to_bits() & SIGN_BIT;
    let (sx, ex): (u64, i32) = decode_positive(numerator);
    let (sy, ey): (u64, i32) = decode_positive(denominator);
    if ex >= ey {
        return remainder_ge_exp(sx, sy, ey, (ex - ey) as u32, sign_x);
    }
    if ex == ey - 1 && sx > sy {
        return encode_exact((sy << 1) - sx, ex, sign_x ^ SIGN_BIT);
    }
    numerator
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod differential_tests {
    use super::{FRAC_MASK, remainder};

    fn exact_tiny_scaled_int(v: f64) -> u128 {
        let bits: u64 = v.to_bits() & !super::SIGN_BIT;
        let exp: u64 = bits >> 52;
        let frac: u64 = bits & FRAC_MASK;
        match exp {
            0 => u128::from(frac),
            1 => u128::from((1u64 << 52) | frac),
            2 => u128::from((1u64 << 52) | frac) << 1,
            _ => unreachable!("exact_tiny_scaled_int only handles |v| < 4 * MIN_POSITIVE"),
        }
    }

    fn exact_tiny_pair_bits(x: f64, y: f64) -> u64 {
        let sign_x: u64 = x.to_bits() & super::SIGN_BIT;
        let a: u128 = exact_tiny_scaled_int(x);
        let b: u128 = exact_tiny_scaled_int(y);
        let q: u128 = a / b;
        let r: u128 = a % b;
        let twice_r: u128 = r << 1;
        let (mag, sign) = if twice_r > b || (twice_r == b && (q & 1) == 1) {
            (b - r, sign_x ^ super::SIGN_BIT)
        } else {
            (r, sign_x)
        };
        if mag == 0 {
            sign_x
        } else if mag < (1u128 << 52) {
            sign | (mag as u64)
        } else {
            sign | (1u64 << 52) | ((mag as u64) & FRAC_MASK)
        }
    }

    fn assert_matches_libm(x: f64, y: f64) {
        let actual: f64 = remainder(x, y);
        let tiny_threshold: f64 = f64::MIN_POSITIVE * 2.0;
        // Independently certify all finite tiny pairs (subnormals and the lowest normal
        // binade) using exact `u128` integer arithmetic. Windows UCRT `remainder` has a
        // known precision bug whenever `0.5 * y` underflows into subnormals
        // (`|y| < 2 * MIN_POSITIVE`), e.g. `remainder(1e-323, 1.5e-323)` and
        // `remainder(MIN_POSITIVE, 2 * MIN_POSITIVE - ulp)`.
        if x.is_finite() && y.is_finite() && y != 0.0 && y.abs() < tiny_threshold {
            assert!(
                actual.is_finite() && 2.0 * actual.abs() <= y.abs(),
                "remainder({x:?}, {y:?}) = {actual:?} violated |r| <= |y|/2"
            );
            if x.abs() < f64::MIN_POSITIVE * 4.0 {
                let exact_bits: u64 = exact_tiny_pair_bits(x, y);
                assert_eq!(
                    actual.to_bits(),
                    exact_bits,
                    "tiny oracle remainder({:#018x} ({x:?}), {:#018x} ({y:?})): actual={:#018x}, exact={:#018x}",
                    x.to_bits(),
                    y.to_bits(),
                    actual.to_bits(),
                    exact_bits,
                );
            }
            if cfg!(windows) {
                return;
            }
        }
        let expected: f64 = crate::ffi::remainder(x, y);
        if expected.is_nan() {
            assert!(
                actual.is_nan(),
                "remainder({x:?}, {y:?}) = {actual:?}, expected NaN"
            );
            return;
        }
        assert_eq!(
            actual.to_bits(),
            expected.to_bits(),
            "remainder({:#018x} ({x:?}), {:#018x} ({y:?})): actual={:#018x} ({actual:?}), libm={:#018x} ({expected:?})",
            x.to_bits(),
            y.to_bits(),
            actual.to_bits(),
            expected.to_bits(),
        );
    }

    fn edge_values() -> Vec<f64> {
        let mut values: Vec<f64> = vec![
            0.0,
            -0.0,
            f64::from_bits(1),
            f64::from_bits(2),
            f64::from_bits(3),
            f64::from_bits(7),
            f64::from_bits((1 << 51) - 1),
            f64::from_bits(1 << 51),
            f64::from_bits((1 << 52) - 1),
            f64::MIN_POSITIVE,
            f64::from_bits(f64::MIN_POSITIVE.to_bits() + 1),
            f64::from_bits(f64::MIN_POSITIVE.to_bits() | FRAC_MASK),
            0.25,
            0.5,
            0.75,
            1.0,
            1.5,
            2.0,
            2.5,
            3.0,
            3.5,
            std::f64::consts::PI,
            std::f64::consts::TAU,
            1e10,
            1e100,
            1e300,
            f64::from_bits(0x7fdf_ffff_ffff_ffff),
            f64::from_bits(0x7fe0_0000_0000_0000),
            f64::MAX / 2.0,
            f64::MAX,
            f64::INFINITY,
            f64::NAN,
        ];
        let negated: Vec<f64> = values.iter().map(|&v| -v).collect();
        values.extend(negated);
        values
    }

    #[test]
    fn exhaustive_edge_matrix_matches_platform_libm() {
        let values: Vec<f64> = edge_values();
        for &x in &values {
            for &y in &values {
                assert_matches_libm(x, y);
            }
        }
    }

    #[test]
    fn splitmix_random_pairs_and_extreme_exponents_match_platform_libm() {
        let mut state: u64 = 0x243f_6a88_85a3_08d3;
        let mut next_u64 = || {
            state = state.wrapping_add(0x9e37_79b9_7f4a_7c15);
            let mut z: u64 = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
            z ^ (z >> 31)
        };
        for _ in 0..20_000 {
            let x: f64 = f64::from_bits(next_u64());
            let y: f64 = f64::from_bits(next_u64());
            assert_matches_libm(x, y);
        }
        // Forced ties and near-ties across all binary64 exponent scales.
        for exp in [-1074i32, -1023, -1022, -500, -1, 0, 1, 53, 500, 1021, 1022] {
            let base: f64 = f64::from_bits(((exp + 1023).clamp(0, 2046) as u64) << 52)
                + f64::from_bits(0x0005_5555_5555_5555 & FRAC_MASK);
            for mult in [0.5, 1.0, 1.5, 2.0, 2.5, 3.0, 3.5, 4.5] {
                let x: f64 = base * mult;
                assert_matches_libm(x, base);
                assert_matches_libm(-x, base);
                assert_matches_libm(x, -base);
                assert_matches_libm(x.next_up(), base);
                assert_matches_libm(x.next_down(), base);
            }
        }
    }
}
