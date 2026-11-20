// SPDX-License-Identifier: GPL-2.0-only
//! Enclose the authored polar derivative and the retained cap Bernstein derivative.
use super::super::super::{
    developed::directional_margin, inverse::cross_polynomial, spatial::SpatialCapture,
    squared_distance,
};
use super::*;
use crate::predicates::roots::{
    certificate_arithmetic::{
        bernstein_curve::{align_planar, capture_planar, derivative},
        bounded_dyadic::PositiveSqrtFailure,
        planar_hulls::candidate_directions,
    },
    metric_bounds::cylindrical_profile::algebra::hull,
};

pub(super) struct Projection {
    authored: [[Interval; 2]; 2],
    cap: [[u64; 2]; 2],
    rates: Vec<[Interval; 2]>,
}

impl Projection {
    pub(super) fn capture(
        uv: &[[u64; 2]],
        retained: &SpatialCapture,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<Self, CombinedFailure> {
        let original = retained
            .comparison
            .as_ref()
            .and_then(|proof| proof.restricted.as_ref())
            .ok_or(CombinedFailure::MissingOriginal)?;
        let [first, .., last] = uv else {
            return Err(CombinedFailure::JoinMismatch);
        };
        let authored = [
            std::array::from_fn(|axis| original[axis][0].clone()),
            std::array::from_fn(|axis| original[axis][original[axis].len() - 1].clone()),
        ];
        let mut rates = cap_rates(uv, arithmetic)?;
        include_authored_corners(
            &mut rates,
            authored_rate(original, &retained.sine, arithmetic)?,
        );
        Ok(Self {
            authored,
            cap: [*first, *last],
            rates,
        })
    }

    pub(super) fn join(
        &self,
        next: &Self,
        arithmetic: &mut Arithmetic<'_>,
    ) -> Result<[f64; 2], CombinedFailure> {
        if self.cap[1] != next.cap[0] || !exact_join(&self.authored[1], &next.authored[0]) {
            return Err(CombinedFailure::JoinMismatch);
        }
        common_direction(&self.rates, &next.rates, arithmetic)
    }
}

fn common_direction(
    left: &[[Interval; 2]],
    right: &[[Interval; 2]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[f64; 2], CombinedFailure> {
    let derivatives = [left, right];
    let zero = arithmetic.zero();
    for direction in candidate_directions(&derivatives, arithmetic.fractional_bits()) {
        if directional_margin(left, direction, &zero, arithmetic)?
            && directional_margin(right, direction, &zero, arithmetic)?
        {
            return Ok(direction);
        }
    }
    Err(CombinedFailure::DirectionUnresolved)
}

fn exact_join(last: &[Interval; 2], first: &[Interval; 2]) -> bool {
    last.iter()
        .zip(first)
        .all(|(a, b)| a.lower == a.upper && b.lower == b.upper && a.lower == b.lower)
}

fn cap_rates(
    uv: &[[u64; 2]],
    arithmetic: &mut Arithmetic<'_>,
) -> Result<Vec<[Interval; 2]>, CombinedFailure> {
    let points = uv
        .iter()
        .map(|pair| pair.map(f64::from_bits))
        .collect::<Vec<_>>();
    let [u, v] = capture_planar(&points, arithmetic)?;
    Ok(align_planar(
        &derivative(&u, arithmetic)?,
        &derivative(&v, arithmetic)?,
    ))
}

fn authored_rate(
    source: &[Vec<Interval>; 2],
    sine: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Result<[Interval; 2], CombinedFailure> {
    let squares = squared_distance(source, arithmetic)?;
    let q = hull(&squares);
    let radius = arithmetic.positive_sqrt(&q).map_err(sqrt_failure)?;
    let count = source[0].len().max(source[1].len());
    let cross = cross_polynomial(source, count, arithmetic).map_err(sqrt_failure)?;
    // With q = x²+y², the slant derivative is q'/2sqrt(q).
    let q_prime = hull(&derivative(&squares, arithmetic)?);
    let angular_denominator = arithmetic.multiply(&q, sine)?;
    let radial_denominator = arithmetic.fraction(&radius, 2, 1)?;
    Ok([
        arithmetic.divide(&hull(&cross), &angular_denominator)?,
        arithmetic.divide(&q_prime, &radial_denominator)?,
    ])
}

fn sqrt_failure(cause: PositiveSqrtFailure) -> CombinedFailure {
    match cause {
        PositiveSqrtFailure::Arithmetic(cause) => cause.into(),
        PositiveSqrtFailure::InvalidInterval { .. } => CombinedFailure::DirectionUnresolved,
    }
}

fn include_authored_corners(points: &mut Vec<[Interval; 2]>, [u, v]: [Interval; 2]) {
    // The projected interval derivative attains its extrema at corners of this rectangle.
    for x in [&u.lower, &u.upper] {
        for y in [&v.lower, &v.upper] {
            points.push([x, y].map(|n| Interval {
                lower: n.clone(),
                upper: n.clone(),
            }));
        }
    }
}

#[cfg(test)]
mod tests;
