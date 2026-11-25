// SPDX-License-Identifier: GPL-2.0-only
//! Independent injectivity witnesses for advancing, stationary-ended and planar spans.
use super::{
    Arithmetic, HelixSpanProgress, HelixSplineSource, HelixTraceFailure as Failure, Interval,
    Poles, PreparedLaw, ResultOf, model::point, polynomial,
};
use crate::predicates::roots::certificate_arithmetic::bernstein_curve::derivative;
use num_bigint::Sign;

pub(super) enum Progress {
    Advancing,
    Ramp,
    Flat([Interval; 3]),
}

impl Progress {
    pub(super) fn new(
        mode: HelixSpanProgress,
        spline: HelixSplineSource<'_>,
        law: &PreparedLaw,
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<Self> {
        match mode {
            HelixSpanProgress::Advancing => Ok(Self::Advancing),
            HelixSpanProgress::Ramp => {
                ramp(spline, law.z_axis(), arithmetic)?;
                Ok(Self::Ramp)
            }
            HelixSpanProgress::Flat => {
                Ok(Self::Flat(flat(spline.poles, law.z_axis(), arithmetic)?))
            }
        }
    }

    pub(super) fn check(
        &self,
        poles: &Poles,
        width: &Interval,
        law: &PreparedLaw,
        arithmetic: &mut Arithmetic<'_>,
    ) -> ResultOf<f64> {
        match self {
            Self::Advancing => strictly_increasing(poles, law.z_axis(), width, arithmetic)?
                .ok_or(Failure::AxialProgress),
            Self::Ramp => Ok(0.),
            Self::Flat(chord) => strictly_increasing(poles, chord, width, arithmetic)?
                .map(|_| 0.)
                .ok_or(Failure::FlatProgress),
        }
    }
}

fn strictly_increasing(
    poles: &Poles,
    direction: &[Interval; 3],
    width: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> ResultOf<Option<f64>> {
    let mut projection = vec![arithmetic.zero(); poles[0].len()];
    for (axis, factor) in poles.iter().zip(direction) {
        let scaled = polynomial::scale(axis, factor, arithmetic)?;
        projection = polynomial::sum(&projection, &scaled, arithmetic)?;
    }
    let rates = derivative(&projection, arithmetic)?;
    let lower = polynomial::lower(&rates);
    if lower.sign() != Sign::Plus {
        return Ok(None);
    }
    let enclosed = arithmetic.divide(
        &Interval {
            lower: lower.clone(),
            upper: lower,
        },
        width,
    )?;
    let bound = -arithmetic.upper_f64(-enclosed.lower)?;
    Ok((bound > 0.).then_some(bound))
}

fn flat(
    xyz: &[[f64; 3]],
    normal: &[Interval; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> ResultOf<[Interval; 3]> {
    let first = point(xyz[0], arithmetic)?;
    let last = point(xyz[xyz.len() - 1], arithmetic)?;
    for &pole in &xyz[1..] {
        let difference = vector(point(pole, arithmetic)?, &first, arithmetic)?;
        if dot(&difference, normal, arithmetic)? != arithmetic.zero() {
            return Err(Failure::FlatProgress);
        }
    }
    vector(last, &first, arithmetic)
}

fn ramp(
    spline: HelixSplineSource<'_>,
    direction: &[Interval; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> ResultOf<()> {
    let mut positive = Vec::with_capacity(spline.poles.len() - 1);
    for points in spline.poles.windows(2) {
        let center = point(points[0], arithmetic)?;
        let delta = vector(point(points[1], arithmetic)?, &center, arithmetic)?;
        let projected = dot(&delta, direction, arithmetic)?;
        if projected.lower.sign() == Sign::Minus {
            return Err(Failure::AxialProgress);
        }
        positive.push(projected.lower.sign() == Sign::Plus);
    }
    for (index, knots) in spline.knots.windows(2).enumerate() {
        if knots[0] == knots[1] {
            continue;
        }
        let support = index
            .checked_sub(spline.degree)
            .and_then(|first| positive.get(first..index))
            .ok_or(Failure::InvalidSpline)?;
        if !support.contains(&true) {
            return Err(Failure::AxialProgress);
        }
    }
    Ok(())
}

fn vector(
    [x, y, z]: [Interval; 3],
    origin: &[Interval; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> ResultOf<[Interval; 3]> {
    Ok([
        arithmetic.subtract(&x, &origin[0])?,
        arithmetic.subtract(&y, &origin[1])?,
        arithmetic.subtract(&z, &origin[2])?,
    ])
}

fn dot(
    vector: &[Interval; 3],
    direction: &[Interval; 3],
    arithmetic: &mut Arithmetic<'_>,
) -> ResultOf<Interval> {
    let mut sum = arithmetic.zero();
    for (component, axis) in vector.iter().zip(direction) {
        let value = arithmetic.multiply(component, axis)?;
        sum = arithmetic.add(&sum, &value)?;
    }
    Ok(sum)
}
