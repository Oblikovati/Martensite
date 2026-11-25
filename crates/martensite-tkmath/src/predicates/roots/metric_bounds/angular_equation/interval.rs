// SPDX-License-Identifier: GPL-2.0-only
//! Fixed-grid outward bounds and nominal binary64 materialization.
use super::{
    AngularEquationError as Error, AngularEquationJet, Arithmetic, EquationJet, R,
    refusal::Checked,
    series::{FACTORIAL, Series, WorldJet},
};
use crate::predicates::roots::{
    DyadicBound,
    certificate_arithmetic::{bounded_dyadic::Interval, rounding::nearest_binary64},
};
use num_bigint::{BigInt, Sign};

pub(super) fn bounds(interval: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 2]> {
    Ok([
        -arithmetic.upper_f64(-&interval.lower)?,
        arithmetic.upper_f64(interval.upper.clone())?,
    ])
}

pub(super) fn nominal(interval: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<f64> {
    let midpoint = DyadicBound::new(
        &interval.lower + &interval.upper,
        -i32::from(arithmetic.fractional_bits()) - 1,
    );
    nearest_binary64(&R::from_dyadic(&midpoint)).ok_or_else(|| {
        super::super::BoundedArithmeticFailure::UnrepresentableBound { bound: midpoint }.into()
    })
}

pub(super) fn positive_lower(value: &Interval) -> BigInt {
    if value.lower.sign() == Sign::Plus {
        value.lower.clone()
    } else if value.upper.sign() == Sign::Minus {
        -&value.upper
    } else {
        0.into()
    }
}

pub(super) fn identically_zero(value: &Interval) -> bool {
    value.lower.sign() == Sign::NoSign && value.upper.sign() == Sign::NoSign
}

pub(super) fn negative(value: &Interval) -> Interval {
    Interval {
        lower: -&value.upper,
        upper: -&value.lower,
    }
}

pub(super) fn jet(
    world: &WorldJet,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<AngularEquationJet> {
    let mut values = [[0.; 3]; 4];
    let mut enclosure = [[[0.; 2]; 3]; 4];
    for order in 0..4 {
        for (axis, series) in world.iter().enumerate() {
            let derivative = derivative(series, order, arithmetic)?;
            values[order][axis] = nominal(&derivative, arithmetic)?;
            enclosure[order][axis] = bounds(&derivative, arithmetic)?;
        }
    }
    let [position, first, second, third] = values;
    Ok(AngularEquationJet {
        nominal: EquationJet {
            position,
            first,
            second,
            third,
        },
        bounds: enclosure,
    })
}

pub(super) fn derivative(
    series: &Series,
    order: usize,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    Ok(arithmetic.fraction(&series[order], FACTORIAL[order], 1)?)
}

pub(super) fn speed_lower(grid: BigInt, arithmetic: &Arithmetic<'_>) -> Checked<f64> {
    let value = -arithmetic.upper_f64(-grid)?;
    if value <= 0. {
        return Err(Error::RegularityUnproven);
    }
    Ok(value)
}
