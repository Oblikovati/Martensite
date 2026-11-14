// SPDX-License-Identifier: GPL-2.0-only
//! Outward Taylor bounds with exact dyadic centers and interval argument reduction.
use super::{algebra::*, *};
use crate::predicates::roots::DyadicBound;
use num_bigint::BigInt;

pub(in crate::predicates::roots::metric_bounds) fn enclose(
    phase: &[Interval],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    // A shared coefficient interval encloses the whole phase even if its width is nonzero.
    if phase.windows(2).all(|pair| pair[0] == pair[1]) {
        let values = scalar(&phase[0], arithmetic)?;
        return Ok([vec![values[0].clone()], vec![values[1].clone()]]);
    }
    let range = hull(phase);
    let grid: BigInt = (&range.lower + &range.upper) / 2;
    let center = Interval {
        lower: grid.clone(),
        upper: grid,
    };
    let delta = phase
        .iter()
        .map(|p| Ok(arithmetic.subtract(p, &center)?))
        .collect::<Checked<Vec<_>>>()?;
    let values = scalar(&center, arithmetic)?;
    let mut result = series(&delta, &values, arithmetic)?;
    let error = remainder(&magnitude(&delta), 9, arithmetic)?;
    widen(&mut result[0], &error, arithmetic)?;
    widen(&mut result[1], &error, arithmetic)?;
    Ok(result)
}

fn series(
    delta: &[Interval],
    values: &[Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Vec<Interval>; 2]> {
    let mut result = [vec![values[0].clone()], vec![values[1].clone()]];
    let mut power = vec![arithmetic.scalar(1.)?];
    let mut factorial = 1u64;
    for order in 1..=8u64 {
        power = polynomial::product(&power, delta, arithmetic)?;
        factorial *= order;
        for (axis, result) in result.iter_mut().enumerate() {
            let coefficient = coefficient(values, axis, order, factorial, arithmetic)?;
            let term = polynomial::scale(&power, &coefficient, arithmetic)?;
            *result = sum(result, &term, arithmetic)?;
        }
    }
    Ok(result)
}

fn coefficient(
    values: &[Interval; 2],
    axis: usize,
    order: u64,
    factorial: u64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let value = &values[(axis + order as usize) % 2];
    let negative = if axis == 0 {
        order.div_ceil(2) % 2 == 1
    } else {
        (order / 2) % 2 == 1
    };
    let value = if negative {
        arithmetic.subtract(&arithmetic.zero(), value)?
    } else {
        value.clone()
    };
    Ok(arithmetic.fraction(&value, 1, factorial)?)
}

fn reduced(center: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let pi = crate::predicates::roots::certificate_arithmetic::circular_constants::pi(arithmetic)?;
    let period = arithmetic.fraction(&pi, 2, 1)?;
    let middle: BigInt = (&period.lower + &period.upper) / 2;
    let turns = arithmetic.integer_quotient(&center.lower, &middle)?;
    let shift = arithmetic.integer_scale(&period, &turns)?;
    let reduced = arithmetic.subtract(center, &shift)?;
    let limit = arithmetic.scalar(7.)?;
    if reduced.magnitude() > limit.upper {
        let exponent = -i32::from(arithmetic.fractional_bits());
        return Err(CylindricalProfileFailure::TrigonometricRange {
            lower: DyadicBound::new(reduced.lower, exponent),
            upper: DyadicBound::new(reduced.upper, exponent),
        });
    }
    Ok(reduced)
}

fn scalar(center: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<[Interval; 2]> {
    if center.lower == BigInt::from(0) && center.upper == BigInt::from(0) {
        return Ok([arithmetic.scalar(1.)?, arithmetic.zero()]);
    }
    let argument = reduced(center, arithmetic)?;
    let square = arithmetic.multiply(&argument, &argument)?;
    let mut terms = [arithmetic.scalar(1.)?, argument.clone()];
    let mut result = terms.clone();
    for k in 1..=32u64 {
        for axis in 0..2 {
            let product = arithmetic.multiply(&terms[axis], &square)?;
            terms[axis] = arithmetic.fraction(
                &product,
                1,
                (2 * k - 1 + axis as u64) * (2 * k + axis as u64),
            )?;
            result[axis] = if k % 2 == 0 {
                arithmetic.add(&result[axis], &terms[axis])?
            } else {
                arithmetic.subtract(&result[axis], &terms[axis])?
            };
        }
    }
    scalar_tail(&argument, result, arithmetic)
}

fn scalar_tail(
    argument: &Interval,
    mut values: [Interval; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[Interval; 2]> {
    let bound = argument.magnitude();
    let amplitude = Interval {
        lower: bound.clone(),
        upper: bound,
    };
    let error = remainder(&amplitude, 65, arithmetic)?;
    let offset = Interval {
        lower: -error.upper.clone(),
        upper: error.upper,
    };
    for value in &mut values {
        *value = arithmetic.add(value, &offset)?;
    }
    Ok(values)
}

#[cfg(test)]
#[path = "trigonometric_tests.rs"]
mod tests;
