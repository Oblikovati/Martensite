// SPDX-License-Identifier: GPL-2.0-only
//! Whole native circular trims and endpoint joins using outward Taylor values.
use super::{
    Arithmetic, Checked, Circle3, EVec, ExactPlacement, ExactVector, Interval, R, SphericalBand,
    SphericalBandError as Error, SphericalBandTraceInput, exact_point, exact_vector,
    positive_radius, scalar_trig,
};
use crate::predicates::roots::certificate_arithmetic::circular_constants;
use num_bigint::BigInt;
use std::f64::consts::TAU;

pub(super) fn prove(
    band: &SphericalBand,
    input: SphericalBandTraceInput,
    place: &ExactPlacement,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let actual = source(input, arithmetic)?;
    let chart = chart_circle(band, [0, 2], arithmetic)?;
    let seam = circle(input.meridian, place, arithmetic)?;
    let turn = difference(&actual.offset, arithmetic)?;
    let peer = difference(&R::from_finite(TAU), arithmetic)?;
    let seam_phase = phase_error(&chart, &turn, arithmetic)?;
    let longitude = chart_circle(band, [0, 1], arithmetic)?;
    let peer_phase = phase_error(&longitude, &peer, arithmetic)?;
    let trace = circle_error(&chart, &seam, arithmetic)? + seam_phase + &peer_phase;
    let rims = rims_and_joins(band, input, &seam, place, arithmetic)?;
    Ok(arithmetic.upper_f64(trace.max(rims + peer_phase))?)
}

struct Source {
    offset: R,
}

fn source(input: SphericalBandTraceInput, arithmetic: &mut Arithmetic<'_>) -> Checked<Source> {
    let [lo, hi] = input.latitude;
    if !lo.is_finite() || !hi.is_finite() || lo >= hi {
        return Err(Error::Latitude(input.latitude));
    }
    let pi = circular_constants::pi(arithmetic)?;
    let half = arithmetic.fraction(&pi, 1, 2)?;
    if arithmetic.scalar(lo)?.lower <= -&half.lower || arithmetic.scalar(hi)?.upper >= half.lower {
        return Err(Error::Latitude(input.latitude));
    }
    if !input.meridian_offset.is_finite()
        || input.meridian_range.iter().any(|value| !value.is_finite())
    {
        return Err(Error::Meridian);
    }
    let offset = R::from_finite(input.meridian_offset);
    let [a, b] = input
        .meridian_range
        .map(|value| R::from_finite(value).add(&offset));
    if a != R::from_finite(lo) || b != R::from_finite(hi) || input.anchors[0] == input.anchors[1] {
        return Err(Error::Meridian);
    }
    Ok(Source {
        offset: offset.negated(),
    })
}

fn rims_and_joins(
    band: &SphericalBand,
    input: SphericalBandTraceInput,
    seam: &[EVec; 3],
    place: &ExactPlacement,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<BigInt> {
    let mut worst = BigInt::from(0);
    for i in 0..2 {
        let actual = circle(input.rims[i], place, arithmetic)?;
        let ideal = chart_rim(band, input.latitude[i], arithmetic)?;
        let trace = circle_error(&ideal, &actual, arithmetic)?;
        let vertex = enclose(&place.point(exact_vector(input.anchors[i], i)?), arithmetic)?;
        let meridian = evaluate(seam, input.meridian_range[i], arithmetic)?;
        let seam_join = vector_error(&vertex, &meridian, arithmetic)?;
        let first_join = vector_error(&vertex, &evaluate(&actual, 0., arithmetic)?, arithmetic)?;
        let last_join = vector_error(&vertex, &evaluate(&actual, TAU, arithmetic)?, arithmetic)?;
        let ideal_join = vector_error(&vertex, &evaluate(&ideal, 0., arithmetic)?, arithmetic)?;
        worst = worst
            .max(trace)
            .max(seam_join)
            .max(first_join)
            .max(last_join)
            .max(ideal_join);
    }
    Ok(worst)
}

fn chart_rim(
    band: &SphericalBand,
    latitude: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[EVec; 3]> {
    let [cosine, sine] = scalar_trig(latitude, arithmetic)?;
    let [origin, x, y] = chart_circle(band, [0, 1], arithmetic)?;
    let z = enclose(&scaled(&band.axes[2], &band.radius), arithmetic)?;
    let height = scale(&z, &sine, arithmetic)?;
    Ok([
        add(&origin, &height, arithmetic)?,
        scale(&x, &cosine, arithmetic)?,
        scale(&y, &cosine, arithmetic)?,
    ])
}

fn chart_circle(
    band: &SphericalBand,
    [a, b]: [usize; 2],
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[EVec; 3]> {
    Ok([
        enclose(&band.origin, arithmetic)?,
        enclose(&scaled(&band.axes[a], &band.radius), arithmetic)?,
        enclose(&scaled(&band.axes[b], &band.radius), arithmetic)?,
    ])
}

fn circle(
    circle: Circle3,
    place: &ExactPlacement,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<[EVec; 3]> {
    let frame = circle.position();
    let radius = positive_radius(circle.radius())?;
    let center = place.point(exact_point(circle.location(), 0)?);
    let x = place.vector(exact_vector(*frame.x_direction().coordinates(), 1)?);
    let y = place.vector(exact_vector(*frame.y_direction().coordinates(), 2)?);
    let cross = super::super::placed_plane::cross(&x, &y);
    if cross.iter().all(|row| row.sign().is_eq()) {
        return Err(Error::Frame);
    }
    Ok([
        enclose(&center, arithmetic)?,
        enclose(&scaled(&x, &radius), arithmetic)?,
        enclose(&scaled(&y, &radius), arithmetic)?,
    ])
}

fn evaluate(
    [center, x, y]: &[EVec; 3],
    phase: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<EVec> {
    let [cosine, sine] = scalar_trig(phase, arithmetic)?;
    let dx = scale(x, &cosine, arithmetic)?;
    let dy = scale(y, &sine, arithmetic)?;
    let radial = add(&dx, &dy, arithmetic)?;
    add(center, &radial, arithmetic)
}

fn difference(turn: &R, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let pi = circular_constants::pi(arithmetic)?;
    let two_pi = arithmetic.fraction(&pi, 2, 1)?;
    let stored = arithmetic.rational(turn)?;
    let error = arithmetic.subtract(&stored, &two_pi)?.magnitude();
    Ok(Interval {
        lower: 0.into(),
        upper: error,
    })
}

fn phase_error(
    [_, x, y]: &[EVec; 3],
    phase: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<BigInt> {
    // Sine and cosine are 1-Lipschitz at the stored phase shifted by one exact turn.
    let mut bound = BigInt::from(0);
    for axis in x.iter().chain(y) {
        bound += arithmetic.multiply(axis, phase)?.magnitude();
    }
    Ok(bound)
}

fn circle_error(a: &[EVec; 3], b: &[EVec; 3], arithmetic: &mut Arithmetic<'_>) -> Checked<BigInt> {
    let mut bound = BigInt::from(0);
    for (a, b) in a.iter().zip(b) {
        bound += vector_error(a, b, arithmetic)?;
    }
    Ok(bound)
}

fn vector_error(a: &EVec, b: &EVec, arithmetic: &mut Arithmetic<'_>) -> Checked<BigInt> {
    let mut bound = BigInt::from(0);
    for (a, b) in a.iter().zip(b) {
        bound += arithmetic.subtract(a, b)?.magnitude();
    }
    Ok(bound)
}

pub(super) fn enclose([x, y, z]: &ExactVector, arithmetic: &mut Arithmetic<'_>) -> Checked<EVec> {
    Ok([
        arithmetic.rational(x)?,
        arithmetic.rational(y)?,
        arithmetic.rational(z)?,
    ])
}
pub(super) fn add(a: &EVec, b: &EVec, arithmetic: &mut Arithmetic<'_>) -> Checked<EVec> {
    let [x, y, z] = [0, 1, 2].map(|i| arithmetic.add(&a[i], &b[i]));
    Ok([x?, y?, z?])
}
pub(super) fn scale(a: &EVec, b: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<EVec> {
    let [x, y, z] = a.each_ref().map(|c| arithmetic.multiply(c, b));
    Ok([x?, y?, z?])
}
fn scaled(a: &ExactVector, b: &R) -> ExactVector {
    a.each_ref().map(|c| c.multiply(b))
}
