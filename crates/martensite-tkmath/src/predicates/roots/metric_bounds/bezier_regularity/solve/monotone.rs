// SPDX-License-Identifier: GPL-2.0-only
//! Exact no-root certificate for a cubic with a strictly signed derivative projection.
use super::{
    BezierRegularityCause, BezierRegularityDecision, BezierRegularityQuery, RegularityParts,
    RootIsolation, poll,
};
use crate::predicates::{exact_coordinate, roots::metric_bounds::ResidualError};
use num_bigint::Sign;

pub(super) fn classify(
    query: BezierRegularityQuery<'_>,
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<RegularityParts>, BezierRegularityCause> {
    let Ok(poles) = <&[[f64; 3]; 4]>::try_from(query.poles) else {
        return Ok(None);
    };
    if !unit_cubic(query)
        || poles.iter().flatten().any(|value| !value.is_finite())
        || !(0..3).any(|axis| strictly_ordered(poles, axis))
    {
        return Ok(None);
    }
    let Some(degree) = squared_speed_degree(poles, cancelled)? else {
        return Ok(None);
    };
    check_cancelled(cancelled)?;
    Ok(Some(regular(degree)))
}

fn unit_cubic(query: BezierRegularityQuery<'_>) -> bool {
    let placement = query.placement;
    query.weights.is_none()
        && query.interval == [0., 1.]
        && query.options.max_stored_degree >= 3
        && query.options.roots.max_degree >= 4
        && placement.scale == 1.
        && placement.origin == [0.; 3]
        && placement.linear == [[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]
}

fn strictly_ordered(poles: &[[f64; 3]; 4], axis: usize) -> bool {
    // Every Bernstein derivative coefficient on this axis is nonzero and has
    // the same sign. The degree-two basis forms a nonnegative partition of unity.
    let values = poles.map(|pole| pole[axis]);
    values.windows(2).all(|pair| pair[0] < pair[1])
        || values.windows(2).all(|pair| pair[0] > pair[1])
}

fn squared_speed_degree(
    poles: &[[f64; 3]; 4],
    cancelled: &mut impl FnMut() -> bool,
) -> Result<Option<usize>, BezierRegularityCause> {
    let mut degree = 0;
    for axis in 0..3 {
        check_cancelled(cancelled)?;
        let Some(next) = derivative_degree(poles.map(|pole| pole[axis]), axis) else {
            return Ok(None);
        };
        if next == 2 {
            return Ok(Some(4));
        }
        degree = degree.max(next);
    }
    // Leading terms in the sum of squared derivatives add with positive coefficients.
    Ok(Some(2 * degree))
}

fn derivative_degree(values: [f64; 4], axis: usize) -> Option<usize> {
    let [a, b, c, d] = [0, 1, 2, 3].map(|index| exact_coordinate(values[index], index, axis).ok());
    let [a, b, c, d] = [a?, b?, c?, d?];
    let third = &d - &a + (&b - &c) * 3_i64;
    if third.sign() != Sign::NoSign {
        return Some(2);
    }
    let second = c - (&b << 1_usize) + a;
    Some(usize::from(second.sign() != Sign::NoSign))
}

fn regular(speed_degree: usize) -> RegularityParts {
    let empty = RootIsolation {
        roots: Vec::new(),
        bisections: 0,
    };
    RegularityParts {
        denominator: empty.clone(),
        denominator_degree: 0,
        speed_degree: Some(speed_degree),
        decision: BezierRegularityDecision::Regular {
            zero_speed_isolation: empty,
        },
    }
}

fn check_cancelled(cancelled: &mut impl FnMut() -> bool) -> Result<(), BezierRegularityCause> {
    poll(cancelled)
        .map_err(|cause| BezierRegularityCause::Coefficients(ResidualError::Roots(cause)))
}

#[cfg(test)]
mod tests;
