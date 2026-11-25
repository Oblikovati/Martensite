// SPDX-License-Identifier: GPL-2.0-only
//! Find adjacent binary64 bounds by one exact scaled division.
use super::{ExactRatio, MAX_FINITE_BITS};
use num_bigint::{BigUint, Sign};
use std::{borrow::Cow, cmp::Ordering};

const NORMAL_SIGNIFICAND: u64 = 1 << 52;

pub(super) fn outward_binary64(value: &ExactRatio) -> Option<[f64; 2]> {
    let numerator = value.numerator();
    let [floor, ceiling] = magnitude_bits(numerator.magnitude(), value.denominator().magnitude())?;
    if numerator.sign() == Sign::Minus {
        // A negative value below the smallest subnormal still has a negative-zero ceiling.
        return Some([-f64::from_bits(ceiling), -f64::from_bits(floor)]);
    }
    Some([f64::from_bits(floor), f64::from_bits(ceiling)])
}

fn magnitude_bits(top: &BigUint, bottom: &BigUint) -> Option<[u64; 2]> {
    if top.bits() == 0 {
        return Some([0, 0]);
    }
    // The bit-length difference differs from floor(log2(top/bottom)) by at most one.
    let estimate = i128::from(top.bits()) - i128::from(bottom.bits());
    if estimate > 1024 {
        return None;
    }
    if estimate < -1074 {
        return Some([0, 1]);
    }
    let exponent = floor_exponent(top, bottom, estimate as i32);
    magnitude_at_exponent(top, bottom, exponent)
}

fn floor_exponent(top: &BigUint, bottom: &BigUint, estimate: i32) -> i32 {
    let less = if estimate < 0 {
        (top << (-estimate) as usize) < *bottom
    } else {
        *top < (bottom << estimate as usize)
    };
    estimate - i32::from(less)
}

fn magnitude_at_exponent(top: &BigUint, bottom: &BigUint, exponent: i32) -> Option<[u64; 2]> {
    if exponent > 1023 {
        return None;
    }
    if exponent < -1074 {
        return Some([0, 1]);
    }
    // The scaled quotient is the binary64 significand, including its leading bit for normals.
    let (top, bottom) = scale_operands(top, bottom, 52 - exponent.max(-1022));
    let quotient = top.as_ref() / bottom.as_ref();
    let significand = u64::try_from(&quotient).ok()?;
    let floor = if exponent < -1022 {
        significand
    } else {
        (((exponent + 1023) as u64) << 52) | (significand - NORMAL_SIGNIFICAND)
    };
    let exact = bottom.as_ref() * quotient == *top;
    let ceiling = floor + u64::from(!exact);
    (ceiling <= MAX_FINITE_BITS).then_some([floor, ceiling])
}

fn scale_operands<'a>(
    top: &'a BigUint,
    bottom: &'a BigUint,
    shift: i32,
) -> (Cow<'a, BigUint>, Cow<'a, BigUint>) {
    match shift.cmp(&0) {
        Ordering::Less => (Cow::Borrowed(top), Cow::Owned(bottom << (-shift) as usize)),
        Ordering::Equal => (Cow::Borrowed(top), Cow::Borrowed(bottom)),
        Ordering::Greater => (Cow::Owned(top << shift as usize), Cow::Borrowed(bottom)),
    }
}
