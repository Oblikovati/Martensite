// SPDX-License-Identifier: GPL-2.0-only
use super::*;
use crate::predicates::roots::DyadicBound;
use num_bigint::Sign;

pub(super) fn publish(
    query: CylindricalMeasureQuery<'_>,
    area: Interval,
    error: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<CertifiedCylindricalNormalMeasure> {
    let bounds = calculate(query, area, error, arithmetic)?;
    Ok(CertifiedCylindricalNormalMeasure {
        bounds,
        profiles: query
            .sources
            .iter()
            .map(|s| s.certificate.clone())
            .collect(),
        depths: query.depths,
        options: query.options,
        limit: query.limit,
        operations: arithmetic.operations(),
    })
}
fn calculate(
    query: CylindricalMeasureQuery<'_>,
    area: Interval,
    error: Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<types::MeasureBounds> {
    let factor = factor(query, arithmetic)?;
    let error_bound = volume_discrepancy(&factor, &error, arithmetic)?;
    check_limit(query.limit, error_bound)?;
    let widened = widen(&area, &error, arithmetic)?;
    let volume = arithmetic.multiply(&factor, &area)?;
    let profile_volume = arithmetic.multiply(&factor, &widened)?;
    outward(
        &area,
        &error,
        [&volume, &profile_volume],
        error_bound,
        arithmetic,
    )
}
fn volume_discrepancy(
    factor: &Interval,
    error: &Interval,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<f64> {
    let result = arithmetic.multiply(factor, error)?;
    Ok(arithmetic.upper_f64(result.upper)?)
}

fn outward(
    area: &Interval,
    error: &Interval,
    volumes: [&Interval; 2],
    volume_error: f64,
    arithmetic: &Arithmetic<'_>,
) -> Checked<types::MeasureBounds> {
    Ok(types::MeasureBounds {
        authored_area: enclosure(area, arithmetic)?,
        area_error: arithmetic.upper_f64(error.upper.clone())?,
        authored_volume: enclosure(volumes[0], arithmetic)?,
        profile_volume: enclosure(volumes[1], arithmetic)?,
        volume_error,
    })
}

fn widen(value: &Interval, error: &Interval, arithmetic: &mut Arithmetic<'_>) -> Checked<Interval> {
    let delta = Interval {
        lower: -error.upper.clone(),
        upper: error.upper.clone(),
    };
    Ok(arithmetic.add(value, &delta)?)
}
fn factor(
    query: CylindricalMeasureQuery<'_>,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let radius = arithmetic.scalar(query.sources[0].support_radius)?;
    let a = ideal_radius(0, &radius, query.depths[0], arithmetic)?;
    let b = ideal_radius(1, &radius, query.depths[1], arithmetic)?;
    let first = arithmetic.scalar(query.depths[0])?;
    let last = arithmetic.scalar(query.depths[1])?;
    let thickness = arithmetic.subtract(&last, &first)?;
    let sum = arithmetic.add(&a, &b)?;
    let numerator = arithmetic.multiply(&thickness, &sum)?;
    let denominator = arithmetic.fraction(&radius, 2, 1)?;
    Ok(arithmetic.divide(&numerator, &denominator)?)
}
fn ideal_radius(
    depth: usize,
    radius: &Interval,
    offset: f64,
    arithmetic: &mut Arithmetic<'_>,
) -> Checked<Interval> {
    let offset = arithmetic.scalar(offset)?;
    let result = arithmetic.add(radius, &offset)?;
    if result.lower.sign() != Sign::Plus {
        let lower = DyadicBound::new(result.lower, -i32::from(arithmetic.fractional_bits()));
        return Err(CylindricalMeasureFailure::Radius { depth, lower });
    }
    Ok(result)
}
fn enclosure(value: &Interval, arithmetic: &Arithmetic<'_>) -> Checked<[f64; 2]> {
    let lower = arithmetic.upper_f64(-value.lower.clone())?;
    let upper = arithmetic.upper_f64(value.upper.clone())?;
    Ok([-lower, upper])
}
fn check_limit(limit: CylindricalMeasureLimit, bound: f64) -> Checked<()> {
    if let CylindricalMeasureLimit::Refuse(requested) = limit
        && bound > requested
    {
        return Err(CylindricalMeasureFailure::Uncertified { bound, requested });
    }
    Ok(())
}
